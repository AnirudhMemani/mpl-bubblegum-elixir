use rustler::{Encoder, Env, Error, NifResult, Term};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use base64::{engine::general_purpose, Engine as _};

mod atoms {
    rustler::atoms! {
        ok,
        error,
        nif_not_loaded,
        invalid_keypair,
        invalid_pubkey,
        transaction_error,
        rpc_error,
        secret_key,
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MetadataArgs {
    name: String,
    symbol: String,
    uri: String,
    seller_fee_basis_points: u16,
    creators: Option<Vec<Creator>>,
    collection: Option<Collection>,
    uses: Option<Uses>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Creator {
    address: String,
    verified: bool,
    share: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Collection {
    verified: bool,
    key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Uses {
    use_method: u8,
    remaining: u64,
    total: u64,
}

// Helper function to parse a keypair from a map
fn parse_keypair(map: Term) -> Result<Keypair, Error> {
    let env = map.get_env();
    let secret_key: Vec<u8> = map.map_get(atoms::secret_key().to_term(env))?.decode()?;
    Keypair::from_bytes(&secret_key).map_err(|_| Error::Term(Box::new(atoms::invalid_keypair())))
}

// Helper function to parse a pubkey from a string
fn parse_pubkey(pubkey_str: &str) -> Result<Pubkey, Error> {
    Pubkey::from_str(pubkey_str).map_err(|_| Error::Term(Box::new(atoms::invalid_pubkey())))
}

// Helper function to get an RPC client
fn get_rpc_client() -> RpcClient {
    // In a real implementation, this would be configurable
    let url = "https://api.devnet.solana.com";
    RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed())
}

#[rustler::nif]
fn create_tree(payer: Term, tree_creator: Term, max_depth: u32, max_buffer_size: u32, public: bool) -> NifResult<Term> {
    let env = payer.get_env();
    
    // Parse keypairs
    let payer_keypair = parse_keypair(payer)?;
    let tree_creator_keypair = parse_keypair(tree_creator)?;
    
    // Get RPC client
    let rpc_client = get_rpc_client();
    
    // Generate a new keypair for the merkle tree
    let merkle_tree = Keypair::new();
    
    // Derive the tree authority PDA
    let seeds = &[merkle_tree.pubkey().as_ref()];
    let (tree_authority, _) = Pubkey::find_program_address(seeds, &mpl_bubblegum::id());
    
    // Create a new Merkle tree instruction
    let create_tree_ix = mpl_bubblegum::instructions::CreateTreeConfig {
        tree_config: tree_authority,
        merkle_tree: merkle_tree.pubkey(),
        payer: payer_keypair.pubkey(),
        tree_creator: tree_creator_keypair.pubkey(),
        log_wrapper: spl_noop::id(),
        compression_program: spl_account_compression::id(),
        system_program: solana_program::system_program::id(),
    }.instruction(
        mpl_bubblegum::instructions::CreateTreeConfigInstructionArgs {
            max_depth,
            max_buffer_size,
            public: Some(public),
        }
    );
    
    // Create and sign transaction
    let recent_blockhash = rpc_client.get_latest_blockhash()
        .map_err(|_| Error::Term(Box::new(atoms::rpc_error())))?;
    
    let transaction = Transaction::new_signed_with_payer(
        &[create_tree_ix],
        Some(&payer_keypair.pubkey()),
        &[&payer_keypair, &tree_creator_keypair, &merkle_tree],
        recent_blockhash,
    );
    
    // Send transaction
    let signature = rpc_client.send_and_confirm_transaction(&transaction)
        .map_err(|_| Error::Term(Box::new(atoms::transaction_error())))?;
    
    Ok((atoms::ok(), signature.to_string()).encode(env))
}

#[rustler::nif]
fn mint_v1(payer: Term, tree_authority: String, leaf_owner: String, merkle_tree: String, metadata: Term) -> NifResult<Term> {
    let env = payer.get_env();
    
    // Parse keypair and pubkeys
    let payer_keypair = parse_keypair(payer)?;
    let tree_authority_pubkey = parse_pubkey(&tree_authority)?;
    let leaf_owner_pubkey = parse_pubkey(&leaf_owner)?;
    let merkle_tree_pubkey = parse_pubkey(&merkle_tree)?;
    
    // Parse metadata
    let metadata_map: Value = metadata.decode()?;
    let metadata_json = serde_json::from_value::<MetadataArgs>(metadata_map)
        .map_err(|_| Error::Term(Box::new(atoms::error())))?;
    
    // Convert to mpl-bubblegum MetadataArgs
    let creators = metadata_json.creators.map(|creators| {
        creators.into_iter().map(|creator| {
            mpl_bubblegum::types::Creator {
                address: parse_pubkey(&creator.address).unwrap(),
                verified: creator.verified,
                share: creator.share,
            }
        }).collect::<Vec<_>>()
    }).unwrap_or_default();
    
    let collection = metadata_json.collection.map(|collection| {
        mpl_bubblegum::types::Collection {
            verified: collection.verified,
            key: parse_pubkey(&collection.key).unwrap(),
        }
    });
    
    let uses = metadata_json.uses.map(|uses| {
        mpl_bubblegum::types::Uses {
            use_method: mpl_bubblegum::types::UseMethod::try_from(uses.use_method).unwrap(),
            remaining: uses.remaining,
            total: uses.total,
        }
    });
    
    let metadata_args = mpl_bubblegum::types::MetadataArgs {
        name: metadata_json.name,
        symbol: metadata_json.symbol,
        uri: metadata_json.uri,
        seller_fee_basis_points: metadata_json.seller_fee_basis_points,
        primary_sale_happened: false,
        is_mutable: true,
        edition_nonce: None,
        token_standard: Some(mpl_bubblegum::types::TokenStandard::NonFungible),
        collection,
        uses,
        token_program_version: mpl_bubblegum::types::TokenProgramVersion::Original,
        creators,
    };
    
    // Get RPC client
    let rpc_client = get_rpc_client();
    
    // Create mint instruction
    let mint_ix = mpl_bubblegum::instructions::MintV1 {
        tree_config: tree_authority_pubkey,
        leaf_owner: leaf_owner_pubkey,
        leaf_delegate: leaf_owner_pubkey,
        merkle_tree: merkle_tree_pubkey,
        payer: payer_keypair.pubkey(),
        tree_creator_or_delegate: payer_keypair.pubkey(),
        log_wrapper: spl_noop::id(),
        compression_program: spl_account_compression::id(),
        system_program: solana_program::system_program::id(),
    }.instruction(
        mpl_bubblegum::instructions::MintV1InstructionArgs {
            message: metadata_args,
        }
    );
    
    // Create and sign transaction
    let recent_blockhash = rpc_client.get_latest_blockhash()
        .map_err(|_| Error::Term(Box::new(atoms::rpc_error())))?;
    
    let transaction = Transaction::new_signed_with_payer(
        &[mint_ix],
        Some(&payer_keypair.pubkey()),
        &[&payer_keypair],
        recent_blockhash,
    );
    
    // Send transaction
    let signature = rpc_client.send_and_confirm_transaction(&transaction)
        .map_err(|_| Error::Term(Box::new(atoms::transaction_error())))?;
    
    Ok((atoms::ok(), signature.to_string()).encode(env))
}

#[rustler::nif]
fn transfer(
    payer: Term,
    tree_authority: String,
    leaf_owner: Term,
    new_leaf_owner: String,
    merkle_tree: String,
    root: Vec<u8>,
    data_hash: Vec<u8>,
    creator_hash: Vec<u8>,
    nonce: u64,
    index: u32,
) -> NifResult<Term> {
    let env = payer.get_env();
    
    // Parse keypairs and pubkeys
    let payer_keypair = parse_keypair(payer)?;
    let leaf_owner_keypair = parse_keypair(leaf_owner)?;
    let tree_authority_pubkey = parse_pubkey(&tree_authority)?;
    let new_leaf_owner_pubkey = parse_pubkey(&new_leaf_owner)?;
    let merkle_tree_pubkey = parse_pubkey(&merkle_tree)?;
    
    // Get RPC client
    let rpc_client = get_rpc_client();
    
    // Create a transfer instruction using mpl-bubblegum
    let transfer_ix = mpl_bubblegum::instructions::Transfer {
        tree_config: tree_authority_pubkey,
        leaf_owner: (leaf_owner_keypair.pubkey(), true),
        leaf_delegate: (leaf_owner_keypair.pubkey(), true),
        new_leaf_owner: new_leaf_owner_pubkey,
        merkle_tree: merkle_tree_pubkey,
        log_wrapper: spl_noop::id(),
        compression_program: spl_account_compression::id(),
        system_program: solana_program::system_program::id(),
    }.instruction(
        mpl_bubblegum::instructions::TransferInstructionArgs {
            root: root.try_into().map_err(|_| Error::Term(Box::new(atoms::error())))?,
            data_hash: data_hash.try_into().map_err(|_| Error::Term(Box::new(atoms::error())))?,
            creator_hash: creator_hash.try_into().map_err(|_| Error::Term(Box::new(atoms::error())))?,
            nonce,
            index,
        }
    );
    
    // Create and sign transaction
    let recent_blockhash = rpc_client.get_latest_blockhash()
        .map_err(|_| Error::Term(Box::new(atoms::rpc_error())))?;
    
    let transaction = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&payer_keypair.pubkey()),
        &[&payer_keypair, &leaf_owner_keypair],
        recent_blockhash,
    );
    
    // Send transaction
    let signature = rpc_client.send_and_confirm_transaction(&transaction)
        .map_err(|_| Error::Term(Box::new(atoms::transaction_error())))?;
    
    Ok((atoms::ok(), signature.to_string()).encode(env))
}

#[rustler::nif]
fn burn(
    payer: Term,
    tree_authority: String,
    leaf_owner: Term,
    merkle_tree: String,
    root: Vec<u8>,
    data_hash: Vec<u8>,
    creator_hash: Vec<u8>,
    nonce: u64,
    index: u32,
) -> NifResult<Term> {
    let env = payer.get_env();
    
    // Parse keypairs and pubkeys
    let payer_keypair = parse_keypair(payer)?;
    let leaf_owner_keypair = parse_keypair(leaf_owner)?;
    let tree_authority_pubkey = parse_pubkey(&tree_authority)?;
    let merkle_tree_pubkey = parse_pubkey(&merkle_tree)?;
    
    // Get RPC client
    let rpc_client = get_rpc_client();
    
    // Create a burn instruction using mpl-bubblegum
    let burn_ix = mpl_bubblegum::instructions::Burn {
        tree_config: tree_authority_pubkey,
        leaf_owner: (leaf_owner_keypair.pubkey(), true),
        leaf_delegate: (leaf_owner_keypair.pubkey(), true),
        merkle_tree: merkle_tree_pubkey,
        log_wrapper: spl_noop::id(),
        compression_program: spl_account_compression::id(),
        system_program: solana_program::system_program::id(),
    }.instruction(
        mpl_bubblegum::instructions::BurnInstructionArgs {
            root: root.try_into().map_err(|_| Error::Term(Box::new(atoms::error())))?,
            data_hash: data_hash.try_into().map_err(|_| Error::Term(Box::new(atoms::error())))?,
            creator_hash: creator_hash.try_into().map_err(|_| Error::Term(Box::new(atoms::error())))?,
            nonce,
            index,
        }
    );
    
    // Create and sign transaction
    let recent_blockhash = rpc_client.get_latest_blockhash()
        .map_err(|_| Error::Term(Box::new(atoms::rpc_error())))?;
    
    let transaction = Transaction::new_signed_with_payer(
        &[burn_ix],
        Some(&payer_keypair.pubkey()),
        &[&payer_keypair, &leaf_owner_keypair],
        recent_blockhash,
    );
    
    // Send transaction
    let signature = rpc_client.send_and_confirm_transaction(&transaction)
        .map_err(|_| Error::Term(Box::new(atoms::transaction_error())))?;
    
    Ok((atoms::ok(), signature.to_string()).encode(env))
}

rustler::init!("Elixir.MplBubblegum.Native", [
    create_tree,
    mint_v1,
    transfer,
    burn
]);
defmodule MplBubblegum do
  @moduledoc """
  Elixir NIFs for Metaplex Bubblegum compressed NFTs on Solana.
  
  This module provides functions to interact with the Metaplex Bubblegum program
  for creating and managing compressed NFTs on Solana.
  """

  alias MplBubblegum.Native

  @doc """
  Creates a new Merkle tree for storing compressed NFTs.
  
  ## Parameters
  
  * `payer` - The keypair that will pay for the transaction
  * `tree_creator` - The keypair that will be the creator of the tree
  * `max_depth` - The maximum depth of the Merkle tree
  * `max_buffer_size` - The maximum buffer size for the Merkle tree
  * `public` - Whether the tree is public or not
  
  ## Returns
  
  * `{:ok, signature}` - The transaction signature
  * `{:error, reason}` - If the transaction fails
  """
  @spec create_tree(map(), map(), integer(), integer(), boolean()) :: {:ok, String.t()} | {:error, any()}
  def create_tree(payer, tree_creator, max_depth, max_buffer_size, public \\ false) do
    Native.create_tree(payer, tree_creator, max_depth, max_buffer_size, public)
  end

  @doc """
  Mints a new compressed NFT to a Merkle tree.
  
  ## Parameters
  
  * `payer` - The keypair that will pay for the transaction
  * `tree_authority` - The public key of the tree authority
  * `leaf_owner` - The public key of the leaf owner
  * `merkle_tree` - The public key of the Merkle tree
  * `metadata` - The metadata for the NFT
  
  ## Returns
  
  * `{:ok, signature}` - The transaction signature
  * `{:error, reason}` - If the transaction fails
  """
  @spec mint_v1(map(), String.t(), String.t(), String.t(), map()) :: {:ok, String.t()} | {:error, any()}
  def mint_v1(payer, tree_authority, leaf_owner, merkle_tree, metadata) do
    Native.mint_v1(payer, tree_authority, leaf_owner, merkle_tree, metadata)
  end

  @doc """
  Transfers a compressed NFT to a new owner.
  
  ## Parameters
  
  * `payer` - The keypair that will pay for the transaction
  * `tree_authority` - The public key of the tree authority
  * `leaf_owner` - The keypair of the current leaf owner
  * `new_leaf_owner` - The public key of the new leaf owner
  * `merkle_tree` - The public key of the Merkle tree
  * `root` - The Merkle root
  * `data_hash` - The data hash of the NFT
  * `creator_hash` - The creator hash of the NFT
  * `nonce` - The nonce of the NFT
  * `index` - The index of the NFT in the tree
  
  ## Returns
  
  * `{:ok, signature}` - The transaction signature
  * `{:error, reason}` - If the transaction fails
  """
  @spec transfer(map(), String.t(), map(), String.t(), String.t(), binary(), binary(), binary(), integer(), integer()) :: {:ok, String.t()} | {:error, any()}
  def transfer(payer, tree_authority, leaf_owner, new_leaf_owner, merkle_tree, root, data_hash, creator_hash, nonce, index) do
    Native.transfer(payer, tree_authority, leaf_owner, new_leaf_owner, merkle_tree, root, data_hash, creator_hash, nonce, index)
  end

  @doc """
  Burns a compressed NFT.
  
  ## Parameters
  
  * `payer` - The keypair that will pay for the transaction
  * `tree_authority` - The public key of the tree authority
  * `leaf_owner` - The keypair of the leaf owner
  * `merkle_tree` - The public key of the Merkle tree
  * `root` - The Merkle root
  * `data_hash` - The data hash of the NFT
  * `creator_hash` - The creator hash of the NFT
  * `nonce` - The nonce of the NFT
  * `index` - The index of the NFT in the tree
  
  ## Returns
  
  * `{:ok, signature}` - The transaction signature
  * `{:error, reason}` - If the transaction fails
  """
  @spec burn(map(), String.t(), map(), String.t(), binary(), binary(), binary(), integer(), integer()) :: {:ok, String.t()} | {:error, any()}
  def burn(payer, tree_authority, leaf_owner, merkle_tree, root, data_hash, creator_hash, nonce, index) do
    Native.burn(payer, tree_authority, leaf_owner, merkle_tree, root, data_hash, creator_hash, nonce, index)
  end
end
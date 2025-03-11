# Metaplex Bubblegum Elixir Client

Elixir NIFs for interacting with Metaplex Bubblegum compressed NFTs on Solana.

## Installation

The package can be installed by adding `mpl_bubblegum` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:mpl_bubblegum, "~> 0.1.0"}
  ]
end
```

## Requirements

- Elixir 1.14 or later
- Rust 1.70 or later (for compiling NIFs)
- Solana CLI tools (for interacting with the Solana network)

## Docker Support

If you prefer not to install Rust locally, you can use Docker to build and test the project:

1.  Make sure Docker is installed on your system
2.  Create a `docker-compose.yml` file with the following content:

```yaml
version: '3'

services:
  app:
    build: .
    volumes:
      - .:/app
    command: mix test
```

3. Run tests with:

```bash
docker-compose up
```

4. Run other commands with:

```bash
docker-compose run app mix compile
```

## Usage

### Creating a Merkle Tree

```elixir
# Create a keypair for the payer
payer = %{
  public_key: "your_public_key",
  secret_key: <<your_secret_key_bytes>>
}

# Create a keypair for the tree creator
tree_creator = %{
  public_key: "your_public_key",
  secret_key: <<your_secret_key_bytes>>
}

# Create a new Merkle tree
{:ok, signature} = MplBubblegum.create_tree(
  payer,
  tree_creator,
  14,  # max_depth
  64,  # max_buffer_size
  false  # public
)
```

### Minting a Compressed NFT

```elixir
# Define the metadata for the NFT
metadata = %{
  name: "My Compressed NFT",
  symbol: "CNFT",
  uri: "https://example.com/metadata.json",
  seller_fee_basis_points: 500,  # 5%
  creators: [
    %{
      address: "creator_public_key",
      verified: false,
      share: 100
    }
  ]
}

# Mint a new compressed NFT
{:ok, signature} = MplBubblegum.mint_v1(
  payer,
  "tree_authority_public_key",
  "leaf_owner_public_key",
  "merkle_tree_public_key",
  metadata
)
```

### Transferring a Compressed NFT

```elixir
# Transfer a compressed NFT to a new owner
{:ok, signature} = MplBubblegum.transfer(
  payer,
  "tree_authority_public_key",
  leaf_owner,  # Keypair of the current owner
  "new_owner_public_key",
  "merkle_tree_public_key",
  <<root_bytes::binary-32>>,
  <<data_hash_bytes::binary-32>>,
  <<creator_hash_bytes::binary-32>>,
  nonce,  # u64
  index  # u32
)
```

### Burning a Compressed NFT

```elixir
# Burn a compressed NFT
{:ok, signature} = MplBubblegum.burn(
  payer,
  "tree_authority_public_key",
  leaf_owner,  # Keypair of the owner
  "merkle_tree_public_key",
  <<root_bytes::binary-32>>,
  <<data_hash_bytes::binary-32>>,
  <<creator_hash_bytes::binary-32>>,
  nonce,  # u64
  index  # u32
)
```

## Development

### Building

```bash
mix deps.get
mix compile
```

### Testing

```bash
mix test
```

### Formatting

```bash
mix format
```

## License

This project is licensed under the Apache License - see the LICENSE file for details.
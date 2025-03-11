defmodule MplBubblegum.Native do
  @moduledoc false
  # This module contains the NIF functions that are implemented in Rust
  # It should not be used directly, but through the MplBubblegum module

  use Rustler, otp_app: :mpl_bubblegum, crate: "mpl_bubblegum_nif"

  # When the NIF is loaded, it will override these functions with the actual implementations
  def create_tree(_payer, _tree_creator, _max_depth, _max_buffer_size, _public), do: :erlang.nif_error(:nif_not_loaded)
  def mint_v1(_payer, _tree_authority, _leaf_owner, _merkle_tree, _metadata), do: :erlang.nif_error(:nif_not_loaded)
  def transfer(_payer, _tree_authority, _leaf_owner, _new_leaf_owner, _merkle_tree, _root, _data_hash, _creator_hash, _nonce, _index), do: :erlang.nif_error(:nif_not_loaded)
  def burn(_payer, _tree_authority, _leaf_owner, _merkle_tree, _root, _data_hash, _creator_hash, _nonce, _index), do: :erlang.nif_error(:nif_not_loaded)
end
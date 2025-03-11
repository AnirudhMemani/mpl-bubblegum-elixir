defmodule MplBubblegum.MixProject do
  use Mix.Project

  @version "0.1.0"
  @source_url "https://github.com/metaplex-foundation/mpl-bubblegum"

  def project do
    [
      app: :mpl_bubblegum,
      version: @version,
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      description: description(),
      package: package(),
      docs: docs(),
      aliases: aliases()
    ]
  end

  def application do
    [
      extra_applications: [:logger, :crypto]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.29.1"},
      {:ex_doc, "~> 0.29", only: :dev, runtime: false},
      {:solana, "~> 0.2.0"},
      {:ex_keccak, "~> 0.7.0"}
    ]
  end

  defp description do
    "Elixir NIFs for Metaplex Bubblegum compressed NFTs on Solana"
  end

  defp package do
    [
      name: "mpl_bubblegum",
      files: ["lib", "native/mpl_bubblegum_nif", "mix.exs", "README.md", "LICENSE"],
      maintainers: ["Metaplex Contributors"],
      licenses: ["MIT"],
      links: %{"GitHub" => @source_url}
    ]
  end

  defp docs do
    [
      main: "MplBubblegum",
      source_url: @source_url,
      extras: ["README.md"]
    ]
  end

  defp aliases do
    [
      format: ["format", "cmd cargo fmt --manifest-path=native/mpl_bubblegum_nif/Cargo.toml"]
    ]
  end
end
# Tiktoken

Elixir bindings for OpenAI's [Tiktoken](https://github.com/openai/tiktoken).

Built on [@zurawiki](https://github.com/zurawiki)'s [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs).

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `tiktoken` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:tiktoken, "~> 0.1.0"}
  ]
end
```

## Required dependencies to compile the shared library

You will need to have a `Rust` compiler and `Python` installed.

For example on `debian`/`ubuntu` this means:

```bash
apt install cargo libpython3-dev
```

To install more up-to-date versions of `Rust` check [rustup](https://rustup.rs/).

The libraries the bindings are built on use `PyO3` so you may need to set the `PyO3` flag before compiling:

```bash
export PYO3_PYTHON=python
# or depending on your OS
export PYO3_PYTHON=python3
```

## Force compilation of the shared library

If the precompiled shared library are not available, you can force
`rustler_precompiled` to compile it by adding the following to you `config.exs`:

```elixir
config :rustler_precompiled, :force_build, tiktoken: true
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/tiktoken>.


FROM hexpm/elixir:1.14.4-erlang-25.3-debian-bullseye-20230227

RUN apt-get update && \
    apt-get install -y curl build-essential git && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

COPY mix.exs mix.lock ./

RUN mix local.hex --force && \
    mix local.rebar --force && \
    mix deps.get

COPY . .

CMD ["mix", "test"]
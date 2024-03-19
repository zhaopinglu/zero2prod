FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app
ENV http_proxy=http://192.168.55.199:7890
ENV https_proxy=http://192.168.55.199:7890
RUN sed -i "s/deb.debian.org/mirrors.huaweicloud.com/g" /etc/apt/sources.list.d/debian.sources

RUN apt update && apt install lld clang -y

FROM chef as planner
WORKDIR /app
COPY cargo_config.toml $CARGO_HOME/config
COPY . .
# Compute a lock-like file for our project
#RUN cargo install bunyan
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as _builder
WORKDIR /app
#COPY --from=planner /usr/local/cargo/bin/bunyan bunyan
COPY cargo_config.toml $CARGO_HOME/config
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.

COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin zero2prod

FROM debian:bookworm-slim AS runtime
WORKDIR /app
ENV http_proxy=http://192.168.55.199:7890
ENV https_proxy=http://192.168.55.199:7890
RUN sed -i "s/deb.debian.org/mirrors.huaweicloud.com/g" /etc/apt/sources.list.d/debian.sources

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=_builder /app/target/release/zero2prod zero2prod
#COPY --from=_builder /app/bunyan bunyan
COPY configuration configuration
ENV APP_ENVIRONMENT production
#ENTRYPOINT ["/bin/sh", "-c", "./zero2prod | ./bunyan"]
ENTRYPOINT ["/bin/sh", "-c", "./zero2prod"]

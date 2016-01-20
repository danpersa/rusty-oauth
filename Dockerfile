FROM zalando/rusty-oauth-build-base

ADD . /rusty-oauth
WORKDIR /rusty-oauth
RUN cargo build --release --target x86_64-unknown-linux-musl

WORKDIR /rusty-oauth/target/x86_64-unknown-linux-musl/release

ADD Dockerfile.final /rusty-oauth/target/x86_64-unknown-linux-musl/release/Dockerfile

CMD docker build -t danpersa/rusty-oauth:latest .

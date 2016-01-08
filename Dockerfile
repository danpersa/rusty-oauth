FROM andrewd/rust-musl

RUN apt-get update && \
    BUILD_DEPENDENCIES="\
        curl" && \
    DEBIAN_FRONTEND=noninteractive apt-get install -yy gcc $BUILD_DEPENDENCIES

RUN curl -sSL -O https://get.docker.com/builds/Linux/x86_64/docker-1.9.1 && \
    chmod +x docker-1.9.1 && \
    mv docker-1.9.1 /usr/local/bin/docker

ADD . /rusty-oauth
WORKDIR /rusty-oauth
RUN cargo build --release --target x86_64-unknown-linux-musl

WORKDIR /rusty-oauth/target/x86_64-unknown-linux-musl/release

ADD Dockerfile.final /rusty-oauth/target/x86_64-unknown-linux-musl/release/Dockerfile

CMD docker build -t danpersa/rusty-oauth:latest .

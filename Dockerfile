FROM rust:1.68.0-buster

RUN useradd -m -u 1000 -s /bin/bash rust
RUN apt-get update && apt-get install -y vim \
    && rustup component add rustfmt

USER rust
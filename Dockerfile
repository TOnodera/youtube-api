FROM rust:1.68.0-buster

RUN useradd -m -u 1000 -s /bin/bash rust
RUN apt-get update && apt-get install -y vim pkg-config libssl-dev \
    && rustup component add rustfmt

# install node
RUN curl -sL https://deb.nodesource.com/setup_16.x | bash - \
    && apt-get install -y nodejs


USER rust
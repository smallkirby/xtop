FROM rust
SHELL ["/bin/bash", "-c"]

RUN groupadd -r rusty && useradd -r -g rusty rusty
WORKDIR /home/rusty
COPY . .

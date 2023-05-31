FROM devkitpro/devkita64
ENV PATH=$DEVKITPRO/devkitA64/bin:$PATH

# Install GCC for the CC link
RUN sudo apt-get update
RUN sudo apt-get install -y build-essential
RUN sudo apt-get install -y clang gcc-multilib

# Install Rust

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH=/root/.cargo/bin:$PATH
RUN rustup install nightly
RUN rustup default nightly
RUN rustup component add rust-src
RUN cargo install xargo

RUN cargo install cargo-nx --git https://github.com/aarch64-switch-rs/cargo-nx

VOLUME /data
WORKDIR /data
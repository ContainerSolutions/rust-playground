FROM --platform=${BUILDPLATFORM:-linux/amd64} rust:latest as builder

RUN rustup default nightly

WORKDIR /usr/src/rq

COPY Cargo.toml Cargo.toml

RUN mkdir src/ \ 
    && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo build --release  

FROM --platform=${BUILDPLATFORM:-linux/amd64} debian:buster-slim
ENV RUST_LOG=info
COPY --from=builder /usr/src/rq/target/release/rq /usr/local/bin/rq
CMD ["rq"]

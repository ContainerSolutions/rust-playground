FROM --platform=${BUILDPLATFORM:-linux/amd64} rust:latest as builder

RUN rustup default nightly

WORKDIR /usr/src/rq

COPY Cargo.toml Cargo.lock ./

RUN mkdir src/ \ 
    && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo install --path .

FROM --platform=${BUILDPLATFORM:-linux/amd64} debian:buster-slim
ENV RUST_LOG=info
COPY --from=builder /usr/local/cargo/bin/rq /usr/local/bin/rq
CMD ["rq"]

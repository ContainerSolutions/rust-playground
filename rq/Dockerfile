FROM --platform=${BUILDPLATFORM:-linux/amd64} rust:1.52.1 as builder
WORKDIR /usr/src/rq

COPY Cargo.toml Cargo.lock ./

RUN mkdir src/ \ 
    && echo "fn main() { println!("cache dependencies"); }" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo install --path .

FROM --platform=${BUILDPLATFORM:-linux/amd64} debian:buster-slim
ENV RUST_LOG=info
EXPOSE 8000
COPY --from=builder /usr/local/cargo/bin/rq /usr/local/bin/rq
CMD ["rq"]

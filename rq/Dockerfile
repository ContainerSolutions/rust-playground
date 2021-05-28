FROM --platform=${BUILDPLATFORM:-linux/amd64} rust:1.52.1 as builder
WORKDIR /usr/src/rq

COPY Cargo.toml Cargo.lock ./

RUN mkdir src/ \ 
    && echo 'fn main() { println!("build.rs"); }' > src/main.rs
RUN cargo build --release

RUN rm -fr src/ target/ /usr/local/cargo/bin/rq
COPY ./src ./src
RUN cargo install --path .

FROM --platform=${BUILDPLATFORM:-linux/amd64} debian:buster-slim
ENV RUST_LOG=info
EXPOSE 8000
COPY --from=builder /usr/local/cargo/bin/rq /usr/local/bin/rq
CMD ["rq"]

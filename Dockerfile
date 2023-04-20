FROM --platform=$BUILDPLATFORM rust:alpine as builder

WORKDIR /usr/src

RUN USER=root cargo new chatgpt

COPY Cargo.toml Cargo.lock /usr/src/chatgpt/

WORKDIR /usr/src/chatgpt

RUN apk add musl-dev openssl openssl-dev pkgconfig && cargo build --release

COPY src /usr/src/chatgpt/src/

RUN cargo build  --release

FROM --platform=$TARGETPLATFORM alpine:3.16.0 AS runtime

WORKDIR /usr/local/chatgpt/

COPY --from=builder /usr/src/chatgpt/target/release/chatgpt /usr/local/chatgpt/

CMD ["/usr/local/chatgpt/chatgpt"]

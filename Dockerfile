FROM clux/muslrust as build

# Make sure we can compile for libmusl
RUN rustup target add x86_64-unknown-linux-musl

# create a new empty shell project to cache dependencies
RUN USER=root cargo new --bin /app
WORKDIR /app
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release --target x86_64-unknown-linux-musl

# Delete fake source and build artifacts, add real code and rebuild
RUN rm src/*.rs
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/imap_junk_guardian*
COPY ./src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a slim final image
FROM alpine

RUN apk add ca-certificates openssl

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/imap-junk-guardian .

ENTRYPOINT ["./imap-junk-guardian"]

FROM rust:1.37 as build

# create a new empty shell project to cache deoendencies
RUN USER=root cargo new --bin app
WORKDIR /app
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

# Delete fake source and build artifacts, add real code and rebuild
RUN rm src/*.rs
RUN rm ./target/release/deps/imap_junk_guardian*
COPY ./src ./src
RUN cargo build --release

# Create a slim final image
FROM debian:buster-slim

RUN apt-get update && \
    apt-get install -y libssl1.1 ca-certificates

COPY --from=build /app/target/release/imap-junk-guardian .

CMD ["./imap-junk-guardian"]
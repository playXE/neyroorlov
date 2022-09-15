FROM rust:1.62 as build

WORKDIR /app
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=build /app/target/release/neuroorlov /usr/local/bin/neuroorlov
USER nobody
CMD [ "/usr/local/bin/neuroorlov" ]

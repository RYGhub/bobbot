FROM rust:1.59 AS files
WORKDIR /usr/src/bobbot
COPY . .

FROM files AS build
RUN cargo install --path .

FROM debian:buster AS system
RUN apt-get update
RUN apt-get install -y libssl1.1 ca-certificates
RUN rm -rf /var/lib/apt/lists/*
COPY --from=install /usr/local/cargo/bin/bobbot /usr/local/bin/bobbot

FROM system AS entrypoint
ENTRYPOINT ["bobbot"]
CMD []

FROM entrypoint AS final

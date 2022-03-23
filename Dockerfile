FROM rust:1.59 AS files
WORKDIR /usr/src/bobbot
COPY . .

FROM files AS install
RUN cargo install --path .

FROM install AS environment
ENV "RUST_LOG" "bobbot=debug"

FROM environment AS entrypoint
ENTRYPOINT [ "bobbot" ]
CMD []

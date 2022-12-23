FROM rust:1.59 AS files
WORKDIR /usr/src/bobbot
COPY . .

FROM files AS install
RUN cargo install --path .

FROM environment AS entrypoint
ENV RUST_LOG="bobbot"
ENTRYPOINT [ "bobbot" ]
CMD []

FROM entrypoint AS final

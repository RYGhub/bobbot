FROM --platform=${BUILDPLATFORM} rust:1.69-bullseye AS builder
ARG BUILDPLATFORM
ARG TARGETPLATFORM

RUN apt-get update && \
    apt-get upgrade --assume-yes && \
    apt-get install --assume-yes libpq5 libpq-dev

RUN \
    mkdir .cargo && \
    echo '[net]' >> .cargo/config.toml && \
    echo 'git-fetch-with-cli = true' >> .cargo/config.toml && \
    echo >> .cargo/config.toml && \
    if [ "${BUILDPLATFORM}" != "${TARGETPLATFORM}" ]; then \
        if [ "${TARGETPLATFORM}" = "linux/amd64" ]; then \
            dpkg --add-architecture amd64; \
            apt-get install --assume-yes gcc-x86-64-linux-gnu libpq5:amd64 libpq-dev:amd64; \
            echo '[target.x86_64-unknown-linux-gnu]' >> .cargo/config.toml; \
            echo 'linker = "x86-64-linux-gnu-gcc"' >> .cargo/config.toml; \
            echo >> .cargo/config.toml; \
        fi && \
        if [ "${TARGETPLATFORM}" = "linux/arm64" ]; then \
            dpkg --add-architecture arm64; \
            apt-get install --assume-yes gcc-aarch64-linux-gnu libpq5:arm64 libpq-dev:arm64; \
            echo '[target.aarch64-unknown-linux-gnu]' >> .cargo/config.toml; \
            echo 'linker = "aarch64-linux-gnu-gcc"' >> .cargo/config.toml; \
            echo >> .cargo/config.toml; \
        fi && \
        if [ "${TARGETPLATFORM}" = "linux/arm/v7" ]; then \
            dpkg --add-architecture armhf; \
            apt-get install --assume-yes gcc-arm-linux-gnueabihf libpq5:armhf libpq-dev:armhf; \
            echo '[target.armv7-unknown-linux-gnueabihf]' >> .cargo/config.toml; \
            echo 'linker = "arm-linux-gnueabihf-gcc"' >> .cargo/config.toml; \
            echo >> .cargo/config.toml; \
        fi \
    fi

RUN \
    if [ "${TARGETPLATFORM}" = "linux/amd64" ]; then \
        RUSTTARGET=x86_64-unknown-linux-gnu; \
    fi && \
    if [ "${TARGETPLATFORM}" = "linux/arm64" ]; then \
        RUSTTARGET=aarch64-unknown-linux-gnu; \
    fi && \
    if [ "${TARGETPLATFORM}" = "linux/arm/v7" ]; then \
        RUSTTARGET=armv7-unknown-linux-gnueabihf; \
    fi && \
    rustup target add ${RUSTTARGET}

WORKDIR /usr/src/bobbot/
COPY ./ ./

RUN \
    if [ "${TARGETPLATFORM}" = "linux/amd64" ]; then \
        RUSTTARGET=x86_64-unknown-linux-gnu; \
        export TARGET_CC=/usr/bin/aarch64-unknown-linux-gnu-gcc; \
        export TARGET_AR=/usr/bin/aarch64-unknown-linux-gnu-ar; \
    fi && \
    if [ "${TARGETPLATFORM}" = "linux/arm64" ]; then \
        RUSTTARGET=aarch64-unknown-linux-gnu; \
        export TARGET_CC=/usr/bin/aarch64-unknown-linux-gnu-gcc; \
        export TARGET_AR=/usr/bin/aarch64-unknown-linux-gnu-ar; \
    fi && \
    if [ "${TARGETPLATFORM}" = "linux/arm/v7" ]; then \
        RUSTTARGET=armv7-unknown-linux-gnueabihf; \
        export TARGET_CC=/usr/bin/aarch64-unknown-linux-gnu-gcc; \
        export TARGET_AR=/usr/bin/aarch64-unknown-linux-gnu-ar; \
    fi && \
    cargo build --all-features --bins --release --target=${RUSTTARGET}

#############################################################################

FROM --platform=${TARGETPLATFORM} rust:1.69-slim-bullseye AS final

RUN apt-get update && \
    apt-get upgrade --assume-yes && \
    apt-get install --assume-yes libpq5

WORKDIR /usr/src/bobbot/
COPY --from=builder \
    /usr/src/bobbot/target/*/release/bobbot \
    /usr/bin/

ENTRYPOINT ["bobbot"]
CMD []

LABEL org.opencontainers.image.title="Bob Bot"
LABEL org.opencontainers.image.description="Discord bot for creating and destroying temporary voice channels"
LABEL org.opencontainers.image.licenses="AGPL-3.0-or-later"
LABEL org.opencontainers.image.url="https://github.com/RYGhub/bobbot"
LABEL org.opencontainers.image.authors="Stefano Pigozzi <me@steffo.eu>"
ENV RUST_LOG "warn,bobbot=info"

# SPDX-License-Identifier: GPL-3.0-or-later
#
# Katna Server's container image. Build from the repository root:
#   docker build -f server/katna-server/Dockerfile -t katna-server .

FROM docker.io/library/rust:1-bookworm AS build
WORKDIR /src
COPY . .
# rust-toolchain.toml picks the toolchain; only this package is built.
RUN cargo build --release --locked -p katna-server \
    && cp target/release/katna-server /katna-server

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=build /katna-server /usr/local/bin/katna-server
ENV KATNA_SERVER_LISTEN=0.0.0.0:8080
EXPOSE 8080
USER nonroot
ENTRYPOINT ["/usr/local/bin/katna-server"]

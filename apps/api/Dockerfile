# syntax=docker/dockerfile:1
FROM public.ecr.aws/docker/library/rust:1.98 AS base

FROM base AS dev
RUN <<EOT
apt-get update
apt-get install -y curl pkg-config libssl-dev
cargo install cargo-watch cargo-llvm-cov llvm-tools-preview
rustup component add clippy rustfmt
EOT

FROM base AS builder
WORKDIR /apps/api
COPY . .
RUN cargo build --release

# --- Runtime Stage ---
FROM public.ecr.aws/debian/debian:stable-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && update-ca-certificates
COPY --from=builder /apps/api/target/release/api .
EXPOSE 3000
CMD ["./api"]
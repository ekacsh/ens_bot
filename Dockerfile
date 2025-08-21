# =========================
# 1. Builder stage
# =========================
FROM rust:1.88-alpine3.20 AS builder

# Install needed build tools
# Install necessary build dependencies
RUN apk add --no-cache \
    build-base \
    musl-dev \
    libc-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static \
    openssl

# Create appuser to match runtime stage
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Workdir
WORKDIR /app

# First copy Cargo files to leverage Docker cache
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy real source and build
COPY . .
RUN cargo build --release

# =========================
# 2. Runtime stage
# =========================
FROM alpine:3.20 AS runtime

# Create non-root user (same uid/gid as in builder to preserve ownership)
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

WORKDIR /app

# Copy only the built binary
COPY --from=builder /app/target/release/ens_bot /app/ens_bot

# Ensure correct ownership and minimal permissions
RUN chown appuser:appgroup /app/ens_bot && \
    chmod 500 /app/ens_bot

USER appuser

ENTRYPOINT ["/app/ens_bot"]

HEALTHCHECK --interval=30s --timeout=5s \
  CMD pgrep ens_bot || exit 1
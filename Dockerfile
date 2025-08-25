
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends procps && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user (same uid/gid as in builder to preserve ownership)
RUN addgroup --system appgroup && adduser --system --ingroup appgroup appuser

WORKDIR /app

# Copy only the built binary
COPY target/*/release/ens_bot /app/ens_bot

# Ensure correct ownership and minimal permissions
RUN chown appuser:appgroup /app/ens_bot && \
    chmod 500 /app/ens_bot

USER appuser

ENTRYPOINT ["/app/ens_bot"]

HEALTHCHECK --interval=30s --timeout=5s \
  CMD pgrep ens_bot || exit 1

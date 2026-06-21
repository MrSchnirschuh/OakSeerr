# Build stage: Rust backend
FROM rust:1.81-slim-bookworm AS backend-builder
WORKDIR /app
COPY backend/ .
RUN apt-get update && apt-get install -y pkg-config libsqlite3-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

# Build stage: Next.js frontend
FROM node:22-slim AS frontend-builder
WORKDIR /app
COPY frontend/ .
RUN npm ci
RUN npm run build

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libsqlite3-0 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=backend-builder /app/target/release/oakseerr /app/oakseerr

# Copy frontend build
COPY --from=frontend-builder /app/out /app/frontend/out
COPY --from=frontend-builder /app/public /app/frontend/public
COPY --from=frontend-builder /app/package.json /app/frontend/package.json

# Create data directories
RUN mkdir -p /app/config /app/cache

# Create non-root user matching PUID/PGID defaults
RUN groupadd -g 1000 oakseerr && \
    useradd -u 1000 -g oakseerr -m -s /bin/bash oakseerr && \
    chown -R oakseerr:oakseerr /app

USER oakseerr

EXPOSE 5055

ENV DATABASE_URL=sqlite:///app/config/oakseerr.db?mode=rwc
ENV LISTEN_ADDR=0.0.0.0:5055
ENV OAKSEERR_FRONTEND_PATH=/app/frontend/out

CMD ["/app/oakseerr"]

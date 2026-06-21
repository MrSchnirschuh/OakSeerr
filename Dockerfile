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
    && rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=backend-builder /app/target/release/oakseerr /app/oakseerr

# Copy frontend build
COPY --from=frontend-builder /app/.next /app/frontend/.next
COPY --from=frontend-builder /app/public /app/frontend/public
COPY --from=frontend-builder /app/node_modules /app/frontend/node_modules
COPY --from=frontend-builder /app/package.json /app/frontend/package.json

# Create data directory
RUN mkdir -p /app/data

EXPOSE 5055

ENV DATABASE_URL=sqlite:///app/data/oakseerr.db?mode=rwc
ENV LISTEN_ADDR=0.0.0.0:5055

CMD ["/app/oakseerr"]

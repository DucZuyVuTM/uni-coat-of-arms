# syntax=docker/dockerfile:1

# --------------------------- <Builder_Stage> ---------------------------
FROM rust:1.90-alpine AS builder

# Install build tools and wget for downloading image
RUN apk add --no-cache musl-dev gcc binutils wget

WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock* ./

# Copy code
COPY src ./src

# Build app with cache
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --target x86_64-unknown-linux-musl && \
    strip /app/target/x86_64-unknown-linux-musl/release/uni-coat-of-arms && \
    cp /app/target/x86_64-unknown-linux-musl/release/uni-coat-of-arms /app/uni-coat-of-arms

# Download image
RUN mkdir -p /app/static && \
    wget -O /app/static/MIREA_Gerb_Colour.png \
    https://www.mirea.ru/upload/medialibrary/80f/MIREA_Gerb_Colour.png

# --------------------------- <Runtime_Stage> ---------------------------
FROM scratch

# Metadata: author and group
LABEL author="Ву Дык Зуй" \
      group="ИКБО-10-23"

# WHEN IMAGE IS USED AS THE BASE OF ANOTHER IMAGE, this command will be run:
ONBUILD RUN echo "Сборка и запуск произведены. Автор: Ву Дык Зуй"

# Copy built binary file only
COPY --from=builder /app/uni-coat-of-arms /app

# Copy static dir with image inside
COPY --from=builder /app/static /static

# Default env
ENV DATABASE_URL="postgres://user:password@db:5432/mydb"

# Non-root run
USER 1000:1000

# Expose app port
EXPOSE 8080

# Run with Entrypoint
ENTRYPOINT ["/app"]

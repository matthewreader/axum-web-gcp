# Use the official Rust image as the build environment
FROM rust:1.80 AS builder

# Set the working directory inside the container
WORKDIR /app

# Update package lists and install required packages
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev lld clang libc6

# Copy the source code into the container
COPY . .

# Set environment variables for the build
ENV SQLX_OFFLINE=true
ENV APP_ENVIRONMENT=production

# Build the application
RUN cargo build --release

# Use a minimal base image for the runtime environment
FROM gcr.io/distroless/cc-debian12 AS runtime

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/axum-web-gcp axum-web-gcp

# Set environment variables for the runtime
ENV APP_ENVIRONMENT=production

# Expose the port that the application listens on
EXPOSE 8080

# Run the application
ENTRYPOINT ["./axum-web-gcp"]

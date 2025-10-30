# Build stage - install Nix and build the application
FROM debian:bookworm-slim AS builder

# Install dependencies for Nix installer
RUN apt-get update && apt-get install -y \
    curl \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

# Install Nix package manager
RUN curl -L https://nixos.org/nix/install | sh -s -- --daemon

# Enable Nix in the current shell and configure flakes
RUN echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

# Set up the environment for the Nix Flake
WORKDIR /app
COPY flake.nix flake.lock ./
COPY Cargo.toml Cargo.lock ./
COPY igait-backend/Cargo.toml ./igait-backend/
COPY igait-lib/Cargo.toml ./igait-lib/
COPY igait-pipeline/Cargo.toml ./igait-pipeline/

# Build the project
COPY igait-backend ./igait-backend
COPY igait-lib ./igait-lib
COPY igait-pipeline ./igait-pipeline

# Source Nix environment and build
RUN . /root/.nix-profile/etc/profile.d/nix.sh && nix build .#igait-backend

# Runtime stage - slim with Nix for runtime dependencies
FROM debian:bookworm-slim

# Install dependencies for Nix
RUN apt-get update && apt-get install -y \
    curl \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

# Install Nix package manager
RUN curl -L https://nixos.org/nix/install | sh -s -- --daemon

# Enable flakes
RUN echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

# Copy the Nix store from builder (contains all runtime dependencies)
COPY --from=builder /nix /nix

# Copy the built application
COPY --from=builder /app/result /app/result

# Set up runtime volumes
VOLUME /data
VOLUME /root/.ssh

# Expose the port
EXPOSE 3000

# Run the binary (Nix will provide runtime dependencies)
CMD ["/app/result/bin/igait-backend"]

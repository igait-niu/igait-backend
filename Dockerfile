FROM nixos/nix:2.18.3 AS builder

# Set up the environment for Nix Flakes
RUN nix-channel --update
RUN echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

# Set up the environment for the Nix Flake
WORKDIR /app
COPY flake.nix flake.lock ./
COPY Cargo.toml Cargo.lock ./
COPY igait-backend/Cargo.toml ./igait-backend/
COPY igait-lib/Cargo.toml ./igait-lib/

# Cache the dependencies
RUN nix develop .#igait-backend

# Import the work directory and build
COPY igait-backend ./igait-backend
COPY igait-lib ./igait-lib
RUN nix build .#igait-backend

# Run the binary
VOLUME /data
VOLUME /root/.ssh
CMD ["/app/result/bin/igait-backend"]
EXPOSE 3000

# Reminder: Command to run this image with terminal is `docker run -it <image> /bin/bash`

# This Dockerfile is for testing running the cli in a typical build agent
# Use Amazon Linux 2023 as the base image
FROM amazonlinux:2023

# Create a new directory for the project
WORKDIR /usr/src/cdevent

# Copy ./target/x86_64-unknown-linux-gnu/release/cdevent to the current directory
COPY ./target/x86_64-unknown-linux-gnu/release/cdevent .

# Set the entrypoint to the built binary
ENTRYPOINT ["./cdevent"]

# 1. cross build --release --target x86_64-unknown-linux-gnu
# 2. docker buildx build --tag cdevent:latest --platform linux/amd64 .
# 3. docker run --rm -it --platform linux/amd64 cdevent
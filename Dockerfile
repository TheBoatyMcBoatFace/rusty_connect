# Use a Rust image with pre-installed dependencies
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy the dependencies manifest file separately
COPY Cargo.toml Cargo.lock ./

# Install dependencies
RUN cargo build --release

# Copy the rest of the files
COPY . .

# Build the application
RUN cargo install --path .

# Set environment variables
ENV API_KEY=ChangeMe
ENV GOOGLE_PROJECT_ID=YourGoogleCloudProjectID
ENV GOOGLE_DATASET=YourDatasetName
ENV A11Y_URL=A11yWatchURL
ENV A11Y_JWT=A11yWatchAPIKey
ENV GOOGLE_APPLICATION_CREDENTIALS=YourGoogleCloudJsonCreds

# Expose port 8000
EXPOSE 8000

# Set the command to run when the container starts
CMD ["./target/release/main"]

FROM scratch

# Copy the build target
COPY target/x86_64-unknown-linux-musl/release/hope-backend /

# Copy the static files
COPY target/deploy /static

# Run the application by default
ENTRYPOINT ["/hope-backend"]
FROM docker.pkg.github.com/creator-rs/creator/android

# Install Cargo APK
RUN cargo install --git https://github.com/rust-windowing/android-ndk-rs --bin cargo-apk 

# Make directory for user code
RUN mkdir -p /src
WORKDIR /src

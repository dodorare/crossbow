FROM androidsdk/android-30
LABEL org.opencontainers.image.source https://github.com/dodorare/crossbow

RUN apt update \
    && apt install -yq unzip wget cmake build-essential

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install rustup targets for android
RUN rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android
# Install crossbundle cli
RUN cargo install --git=https://github.com/dodorare/crossbow --branch=main crossbundle

# Install Android NDK
RUN cd /usr/local \
    && wget -q http://dl.google.com/android/repository/android-ndk-r22-beta1-linux-x86_64.zip \
    && unzip -q android-ndk-r22-beta1-linux-x86_64.zip \
    && rm android-ndk-r22-beta1-linux-x86_64.zip
ENV ANDROID_NDK_ROOT /usr/local/android-ndk-r22-beta1

# Make directory for user code
RUN mkdir -p /src
WORKDIR /src
ENTRYPOINT ["crossbundle"]

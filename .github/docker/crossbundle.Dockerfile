FROM rust:stretch
LABEL org.opencontainers.image.source https://github.com/dodorare/crossbow

RUN apt-get update && \
    apt-get install -yq openjdk-8-jre unzip wget cmake && \
    rustup target add armv7-linux-androideabi \
    aarch64-linux-android i686-linux-android x86_64-linux-android

# Install Android SDK
ENV ANDROID_SDK_ROOT /opt/android-sdk-linux
RUN mkdir ${ANDROID_SDK_ROOT} && \
    cd ${ANDROID_SDK_ROOT} && \
    wget -q https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip && \
    unzip -q sdk-tools-linux-4333796.zip && \
    rm sdk-tools-linux-4333796.zip && \
    chown -R root:root /opt
RUN yes | ${ANDROID_SDK_ROOT}/tools/bin/sdkmanager "platform-tools" | grep -v = || true
RUN yes | ${ANDROID_SDK_ROOT}/tools/bin/sdkmanager "platforms;android-29" | grep -v = || true
RUN yes | ${ANDROID_SDK_ROOT}/tools/bin/sdkmanager "build-tools;29.0.0" | grep -v = || true
RUN ${ANDROID_SDK_ROOT}/tools/bin/sdkmanager --update | grep -v = || true

# Install Android NDK
RUN cd /usr/local && \
    wget -q http://dl.google.com/android/repository/android-ndk-r22-beta1-linux-x86_64.zip && \
    unzip -q android-ndk-r22-beta1-linux-x86_64.zip && \
    rm android-ndk-r22-beta1-linux-x86_64.zip
ENV ANDROID_NDK_ROOT /usr/local/android-ndk-r22-beta1

# Install crossbundle cli
RUN cargo install --git=https://github.com/dodorare/crossbow crossbundle

# Make directory for user code
RUN mkdir -p /src
WORKDIR /src

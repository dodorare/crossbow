FROM openjdk:8-jdk-slim-bullseye
LABEL org.opencontainers.image.source https://github.com/dodorare/crossbow

RUN apt update \
    && apt install -yq curl unzip wget cmake build-essential pkg-config libssl-dev libssl1.1

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install rustup targets for android
RUN rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android
# Install crossbundle cli
RUN cargo install --git=https://github.com/dodorare/crossbow --branch=main crossbundle

# Install Android SDK
ENV ANDROID_SDK_ROOT /opt/android-sdk-linux
RUN mkdir ${ANDROID_SDK_ROOT} \
    && cd ${ANDROID_SDK_ROOT} \
    && wget -q https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip \
    && unzip -q sdk-tools-linux-4333796.zip \
    && rm sdk-tools-linux-4333796.zip \
    && chown -R root:root /opt
RUN yes | ${ANDROID_SDK_ROOT}/tools/bin/sdkmanager "platform-tools" | grep -v = || true
RUN yes | ${ANDROID_SDK_ROOT}/tools/bin/sdkmanager "platforms;android-30" | grep -v = || true
RUN yes | ${ANDROID_SDK_ROOT}/tools/bin/sdkmanager "build-tools;29.0.0" | grep -v = || true
RUN ${ANDROID_SDK_ROOT}/tools/bin/sdkmanager --update | grep -v = || true

# Install Android NDK
RUN cd /usr/local \
    && wget -q http://dl.google.com/android/repository/android-ndk-r22-beta1-linux-x86_64.zip \
    && unzip -q android-ndk-r22-beta1-linux-x86_64.zip \
    && rm android-ndk-r22-beta1-linux-x86_64.zip
ENV ANDROID_NDK_ROOT /usr/local/android-ndk-r22-beta1

# Install bundletool
RUN wget -q https://github.com/google/bundletool/releases/download/1.8.2/bundletool-all-1.8.2.jar \
    && mv bundletool-all-1.8.2.jar ${ANDROID_SDK_ROOT}/bundletool-all-1.8.2.jar
ENV BUNDLETOOL_PATH=${ANDROID_SDK_ROOT}/bundletool-all-1.8.2.jar

# Make directory for user code
RUN mkdir -p /src
WORKDIR /src
ENTRYPOINT ["crossbundle"]

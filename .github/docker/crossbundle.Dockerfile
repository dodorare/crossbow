FROM ubuntu:24.04 AS builder

ARG RUST_VERSION=1.97.1
ARG TARGETARCH

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install --yes --no-install-recommends \
        build-essential \
        ca-certificates \
        cmake \
        curl \
        libssl-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 --silent --show-error --fail \
        https://sh.rustup.rs \
        | sh -s -- -y --profile minimal --default-toolchain "${RUST_VERSION}"

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add aarch64-linux-android x86_64-linux-android

WORKDIR /build
COPY . .
RUN --mount=type=cache,id=crossbundle-registry,target=/root/.cargo/registry,sharing=locked \
    --mount=type=cache,id=crossbundle-target-${TARGETARCH},target=/build/target,sharing=locked \
    CARGO_TARGET_DIR=/build/target \
    cargo install --path crossbundle/cli --locked --root /opt/crossbundle


FROM ubuntu:24.04

LABEL org.opencontainers.image.source="https://github.com/dodorare/crossbow"

ARG ANDROID_COMMAND_LINE_TOOLS_VERSION=15859902
ARG ANDROID_API_LEVEL=31
ARG ANDROID_BUILD_TOOLS_VERSION=30.0.3
ARG ANDROID_NDK_VERSION=27.3.13750724
ARG BUNDLETOOL_VERSION=1.18.3
ARG GRADLE_VERSION=7.4

ENV ANDROID_SDK_ROOT=/opt/android-sdk \
    ANDROID_NDK_ROOT=/opt/android-sdk/ndk/${ANDROID_NDK_VERSION} \
    BUNDLETOOL_PATH=/opt/android-sdk/bundletool.jar \
    GRADLE_HOME=/opt/gradle
ENV PATH="${ANDROID_SDK_ROOT}/cmdline-tools/latest/bin:${ANDROID_SDK_ROOT}/platform-tools:${GRADLE_HOME}/bin:${PATH}"

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install --yes --no-install-recommends \
        build-essential \
        ca-certificates \
        cmake \
        curl \
        libssl-dev \
        openjdk-17-jdk-headless \
        pkg-config \
        unzip \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p "${ANDROID_SDK_ROOT}/cmdline-tools" \
    && curl --fail --location --silent --show-error \
        "https://dl.google.com/android/repository/commandlinetools-linux-${ANDROID_COMMAND_LINE_TOOLS_VERSION}_latest.zip" \
        --output /tmp/command-line-tools.zip \
    && unzip -q /tmp/command-line-tools.zip -d "${ANDROID_SDK_ROOT}/cmdline-tools" \
    && mv "${ANDROID_SDK_ROOT}/cmdline-tools/cmdline-tools" "${ANDROID_SDK_ROOT}/cmdline-tools/latest" \
    && rm /tmp/command-line-tools.zip

RUN yes | sdkmanager --licenses >/dev/null \
    && sdkmanager \
        "build-tools;${ANDROID_BUILD_TOOLS_VERSION}" \
        "ndk;${ANDROID_NDK_VERSION}" \
        "platforms;android-${ANDROID_API_LEVEL}" \
        "platform-tools"

RUN curl --fail --location --silent --show-error \
        "https://github.com/google/bundletool/releases/download/${BUNDLETOOL_VERSION}/bundletool-all-${BUNDLETOOL_VERSION}.jar" \
        --output "${BUNDLETOOL_PATH}"

RUN curl --fail --location --silent --show-error \
        "https://services.gradle.org/distributions/gradle-${GRADLE_VERSION}-bin.zip" \
        --output /tmp/gradle.zip \
    && unzip -q /tmp/gradle.zip -d /opt \
    && mv "/opt/gradle-${GRADLE_VERSION}" "${GRADLE_HOME}" \
    && rm /tmp/gradle.zip

COPY --from=builder /opt/crossbundle/bin/crossbundle /usr/local/bin/crossbundle
COPY --from=builder /root/.cargo/bin /root/.cargo/bin
COPY --from=builder /root/.rustup /root/.rustup

ENV PATH="/root/.cargo/bin:${PATH}"

ENTRYPOINT ["crossbundle"]

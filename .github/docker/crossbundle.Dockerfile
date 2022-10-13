FROM androidsdk/android-31
LABEL org.opencontainers.image.source https://github.com/dodorare/crossbow

RUN apt update -yq && apt upgrade -yq \
    && apt install -yq curl unzip wget cmake build-essential pkg-config libssl-dev libssl1.1

# Workaround. Reinstall rustup to avoid problems with compile 
RUN rustup uninstall stable && rustup update nightly && rustup update stable
RUN rustup target add aarch64-linux-android x86_64-linux-android 

# Install Android NDK
RUN ulimit -c unlimited
RUN yes | ${ANDROID_SDK_ROOT}/cmdline-tools/tools/bin/sdkmanager "ndk;23.1.7779620"
RUN yes | ${ANDROID_SDK_ROOT}/cmdline-tools/tools/bin/sdkmanager --update
ENV ANDROID_NDK_ROOT=${ANDROID_SDK_ROOT}/ndk/23.1.7779620

# Install bundletool
RUN wget -q https://github.com/google/bundletool/releases/download/1.8.2/bundletool-all-1.8.2.jar
ENV BUNDLETOOL_PATH=${ANDROID_SDK_ROOT}/bundletool-all-1.8.2.jar

RUN wget https://services.gradle.org/distributions/gradle-7.4-all.zip \
    && unzip -q gradle-7.4-all.zip \
    && rm gradle-7.4-all.zip \
    && mv gradle-7.4 ${ANDROID_SDK_ROOT}/gradle \
    && chown -R root:root ${ANDROID_SDK_ROOT}/gradle
ENV GRADLE_HOME=${ANDROID_SDK_ROOT}/gradle/bin
ENV PATH=$GRADLE_HOME:${PATH}

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=/root/.cargo/bin:${PATH}

# Install crossbundle cli
RUN mkdir -p /src
WORKDIR /src
COPY . .
RUN cd crossbundle/cli && cargo install --path=. && rm -rf /src/*

ENTRYPOINT ["crossbundle"]

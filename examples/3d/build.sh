#!/bin/bash

# Exit this script immediatelly if any of the commands fails
set -e

PROJECT_NAME=threed

# The product of this script. This is the actual app bundle!
BUNDLE_DIR=${PROJECT_NAME}.app

# Check for --device flag
if [ "$1" = "--device" ]; then
  BUILDING_FOR_DEVICE=true
fi

# Print the current target architecture
if [ "${BUILDING_FOR_DEVICE}" = true ]; then
  echo 👍 Bulding ${PROJECT_NAME} for device
else
  echo 👍 Bulding ${PROJECT_NAME} for simulator
fi

echo

#############################################################
echo → Step 1: Prepare Working Folders
#############################################################

# Delete existing folders from previous builds
rm -rf ${BUNDLE_DIR}

mkdir -p ${BUNDLE_DIR}
echo ✅ Create ${BUNDLE_DIR} folder

echo

#############################################################
echo → Step 2: Compile Rust code
#############################################################

if [ "${BUILDING_FOR_DEVICE}" = true ]; then
	cargo lipo --targets aarch64-apple-ios
else
	cargo lipo --targets x86_64-apple-ios
  # cargo rustc --lib --target x86_64-apple-ios -- --crate-type='rlib,staticlib'
fi

#############################################################
echo → Step 3: Compile Objective-C Files
#############################################################

# Target architecture we want to build for
TARGET=""

# Path to the SDK we want to use for compiling
SDK_PATH=""

if [ "${BUILDING_FOR_DEVICE}" = true ]; then
  # Building for device
  TARGET=arm64-apple-ios12.0
  SDK_PATH=$(xcrun --show-sdk-path --sdk iphoneos)

  # The folder inside the app bundle where we
  # will copy all required dylibs
  FRAMEWORKS_DIR=Frameworks

  # Set additional flags for the compiler
  OTHER_FLAGS="-Xlinker -rpath -Xlinker @executable_path/${FRAMEWORKS_DIR}"
else
  # Building for simulator
  TARGET=x86_64-apple-ios12.0-simulator
  SDK_PATH=$(xcrun --show-sdk-path --sdk iphonesimulator)
fi

clang -x objective-c ios-src/main.m \
  -L../../target/universal/debug \
  -lthreedlib -lc++abi -lc++ \
  -framework Security -framework Metal -framework UIKit -framework CoreFoundation \
  -isysroot ${SDK_PATH} \
  -target ${TARGET} \
  ${OTHER_FLAGS} \
  -o ${BUNDLE_DIR}/${PROJECT_NAME}

echo ✅ Compile Objective-C source files

echo

#############################################################
echo → Step 4: Process and Copy Info.plist
#############################################################

# The location of the original Info.plist file
ORIGINAL_INFO_PLIST=ios-src/Info.plist

# The location of the processed Info.plist in the app bundle
PROCESSED_INFO_PLIST=${BUNDLE_DIR}/Info.plist

# The bundle identifier of the resulting app
APP_BUNDLE_IDENTIFIER=com.rust.${PROJECT_NAME}

DEVELOPMENT_LANGUAGE=en

cp ${ORIGINAL_INFO_PLIST} ${PROCESSED_INFO_PLIST}
echo ✅ Copy ${ORIGINAL_INFO_PLIST} to ${PROCESSED_INFO_PLIST}

echo ❎ Copy Icon to bundle. TODO
cp res/mipmap-hdpi/ic_launcher.png ${BUNDLE_DIR}/Icon.png

echo ❎ Copy assets to bundle. TODO
cp -r assets/ ${BUNDLE_DIR}/assets/

# A command line tool for dealing with plists
PLIST_BUDDY=/usr/libexec/PlistBuddy

# Set the correct name of the executable file we created at step 2
${PLIST_BUDDY} -c "Set :CFBundleExecutable ${PROJECT_NAME}" ${PROCESSED_INFO_PLIST}
echo ✅ Set CFBundleExecutable to ${PROJECT_NAME}

# Set a valid bundle indentifier
${PLIST_BUDDY} -c "Set :CFBundleIdentifier ${APP_BUNDLE_IDENTIFIER}" ${PROCESSED_INFO_PLIST}
echo ✅ Set CFBundleIdentifier to ${APP_BUNDLE_IDENTIFIER}

# Set a valid bundle development region
${PLIST_BUDDY} -c "Set :CFBundleDevelopmentRegion ${DEVELOPMENT_LANGUAGE}" ${PROCESSED_INFO_PLIST}
echo ✅ Set CFBundleDevelopmentRegion to ${DEVELOPMENT_LANGUAGE}

# Set the proper bundle name
${PLIST_BUDDY} -c "Set :CFBundleName ${PROJECT_NAME}" ${PROCESSED_INFO_PLIST}
echo ✅ Set CFBundleName to ${PROJECT_NAME}

echo

#############################################################
if [ "${BUILDING_FOR_DEVICE}" != true ]; then
  # If we build for simulator, we can exit the scrip here
  echo 🎉 Building ${PROJECT_NAME} for simulator successfully finished! 🎉
  exit 0
fi

#!/bin/bash

# Exit this script immediatelly if any of the commands fails
set -e

if [ "$PROJECT_NAME" = "" ]; then
  PROJECT_NAME=explorer
fi

# The product of this script. This is the actual app bundle!
BUNDLE_DIR=../../target/apple/${PROJECT_NAME}.app

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

#############################################################
echo → Step 1: Prepare Working Folders
#############################################################

# Delete existing folders from previous builds
rm -rf ${BUNDLE_DIR}

mkdir -p ${BUNDLE_DIR}
echo ✅ Create ${BUNDLE_DIR} folder

#############################################################
echo → Step 2: Compile Rust code
#############################################################

# Target architecture we want to build for
TARGET=""
# Path to the SDK we want to use for compiling
SDK_PATH=""

if [ "${BUILDING_FOR_DEVICE}" = true ]; then
  # Building for device
  # TARGET=arm64-apple-ios12.0
  TARGET=aarch64-apple-ios
  SDK_PATH=$(xcrun --show-sdk-path --sdk iphoneos)

  # The folder inside the app bundle where we
  # will copy all required dylibs
  FRAMEWORKS_DIR=Frameworks

  # Set additional flags for the compiler
  OTHER_FLAGS="-Xlinker -rpath -Xlinker @executable_path/${FRAMEWORKS_DIR}"
else
  # Building for simulator
  # TARGET=x86_64-apple-ios12.0-simulator
  TARGET=x86_64-apple-ios
  SDK_PATH=$(xcrun --show-sdk-path --sdk iphonesimulator)
fi

cargo rustc --bin ${PROJECT_NAME} --release --target ${TARGET} -- --crate-type=staticlib
cp ../../target/${TARGET}/release/${PROJECT_NAME} ${BUNDLE_DIR}/${PROJECT_NAME}

echo ✅ Compile Rust code

#############################################################
echo → Step 3: Process and Copy Info.plist
#############################################################

# The location of the processed Info.plist in the app bundle
PROCESSED_INFO_PLIST=${BUNDLE_DIR}/Info.plist
# The bundle identifier of the resulting app
APP_BUNDLE_IDENTIFIER=com.enfipy.${PROJECT_NAME}
# The bundle developer language
DEVELOPMENT_LANGUAGE=en

echo ❎ Copy Icon to bundle. TODO
cp res/mipmap-hdpi/ic_launcher.png ${BUNDLE_DIR}/Icon.png

echo ❎ Copy assets to bundle. TODO
cp -r assets/ ${BUNDLE_DIR}/assets/

echo """<?xml version=\"1.0\" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
	<key>CFBundleDevelopmentRegion</key>
	<string>${DEVELOPMENT_LANGUAGE}</string>
	<key>CFBundleExecutable</key>
	<string>${PROJECT_NAME}</string>
	<key>CFBundleIdentifier</key>
	<string>${APP_BUNDLE_IDENTIFIER}</string>
	<key>CFBundleInfoDictionaryVersion</key>
	<string>6.0</string>
	<key>CFBundleName</key>
	<string>${PROJECT_NAME}</string>
	<key>CFBundlePackageType</key>
	<string>APPL</string>
	<key>CFBundleShortVersionString</key>
	<string>1.0</string>
	<key>CFBundleVersion</key>
	<string>1</string>
	<key>UILaunchStoryboardName</key>
	<string>LaunchScreen</string>
	<key>UIRequiresFullScreen</key>
	<false/>
	<key>UISupportedInterfaceOrientations</key>
	<array>
		<string>UIInterfaceOrientationPortrait</string>
		<string>UIInterfaceOrientationLandscapeLeft</string>
		<string>UIInterfaceOrientationLandscapeRight</string>
		<string>UIInterfaceOrientationPortraitUpsideDown</string>
	</array>
</dict>
</plist>""" &> ${PROCESSED_INFO_PLIST}

echo

#############################################################
if [ "${BUILDING_FOR_DEVICE}" != true ]; then
  # If we build for simulator, we can exit the scrip here
  echo 🎉 Building ${PROJECT_NAME} for simulator successfully finished! 🎉
  exit 0
fi

#############################################################
echo → Step 4: Code Signing
#############################################################

export CODESIGN_ALLOCATE=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/codesign_allocate
codesign --force --sign - --timestamp=none ${BUNDLE_DIR}/

# The name of the provisioning file to use
# ⚠️ YOU NEED TO CHANGE THIS TO YOUR PROFILE ️️⚠️
PROVISIONING_PROFILE_NAME=1fdea7b0-9231-4b31-a511-cdd0724d8128.mobileprovision

# The location of the provisioning file inside the app bundle
EMBEDDED_PROVISIONING_PROFILE=${BUNDLE_DIR}/embedded.mobileprovision

cp ~/Library/MobileDevice/Provisioning\ Profiles/${PROVISIONING_PROFILE_NAME} ${EMBEDDED_PROVISIONING_PROFILE}
echo ✅ Copy provisioning profile ${PROVISIONING_PROFILE_NAME} to ${EMBEDDED_PROVISIONING_PROFILE}

# The team identifier of your signing identity
# ⚠️ YOU NEED TO CHANGE THIS TO YOUR ID ️️⚠️
TEAM_IDENTIFIER=V4SUV789T7

# The location if the .xcent file
XCENT_FILE=${BUNDLE_DIR}/${PROJECT_NAME}.xcent

/usr/libexec/PlistBuddy -c "Add :application-identifier string ${TEAM_IDENTIFIER}.${APP_BUNDLE_IDENTIFIER}" ${XCENT_FILE}
/usr/libexec/PlistBuddy -c "Add :com.apple.developer.team-identifier string ${TEAM_IDENTIFIER}" ${XCENT_FILE}

echo ✅ Create ${XCENT_FILE}

# The id of the identity used for signing
IDENTITY=A89832B9716C3CC6FC5AEE30F4CA728CA79044C9

# Sign the binary
codesign \
  --force \
  --timestamp=none \
  --sign ${IDENTITY} \
  ${BUNDLE_DIR}/${PROJECT_NAME}

# Sign the bundle itself
codesign \
  --force \
  --timestamp=none \
  --entitlements ${XCENT_FILE} \
  --sign ${IDENTITY} \
  ${BUNDLE_DIR}

echo ✅ Codesign ${BUNDLE_DIR}

echo

#############################################################
echo 🎉 Building ${PROJECT_NAME} for device successfully finished! 🎉
exit 0
#############################################################

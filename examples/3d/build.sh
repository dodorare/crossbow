#!/bin/bash

# Exit this script immediatelly if any of the commands fails
set -e

if [ "$PROJECT_NAME" = "" ]; then
  PROJECT_NAME=threed
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
echo → Step 4: Process and Copy Info.plist
#############################################################

# The location of the processed Info.plist in the app bundle
PROCESSED_INFO_PLIST=${BUNDLE_DIR}/Info.plist
# The bundle identifier of the resulting app
APP_BUNDLE_IDENTIFIER=com.rust.${PROJECT_NAME}
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

# #############################################################
# echo → Step 5: Copy Swift Runtime Libraries
# #############################################################

# # The folder where the Swift runtime libs live on the computer
# SWIFT_LIBS_SRC_DIR=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/iphoneos

# # The folder where we want to copy them in the app bundle
# SWIFT_LIBS_DEST_DIR=${BUNDLE_DIR}/${FRAMEWORKS_DIR}

# # The list of all libs we want to copy
# RUNTIME_LIBS=( libswiftCore.dylib libswiftCoreFoundation.dylib libswiftCoreGraphics.dylib libswiftCoreImage.dylib libswiftDarwin.dylib libswiftDispatch.dylib libswiftFoundation.dylib libswiftMetal.dylib libswiftObjectiveC.dylib libswiftQuartzCore.dylib libswiftSwiftOnoneSupport.dylib libswiftUIKit.dylib libswiftos.dylib )

# mkdir -p ${BUNDLE_DIR}/${FRAMEWORKS_DIR}
# echo ✅ Create ${SWIFT_LIBS_DEST_DIR} folder

# for library_name in "${RUNTIME_LIBS[@]}"; do
#   # Copy the library
#   cp ${SWIFT_LIBS_SRC_DIR}/$library_name ${SWIFT_LIBS_DEST_DIR}/
#   echo ✅ Copy $library_name to ${SWIFT_LIBS_DEST_DIR}
# done

# echo

# #############################################################
# echo → Step 6: Code Signing
# #############################################################

# export CODESIGN_ALLOCATE=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/codesign_allocate
# codesign --force --sign - --timestamp=none ${PROJECT_NAME}.app

# # The name of the provisioning file to use
# # ⚠️ YOU NEED TO CHANGE THIS TO YOUR PROFILE ️️⚠️
# PROVISIONING_PROFILE_NAME=23a6e9d9-ad3c-4574-832c-be6eb9d51b8c.mobileprovision

# # The location of the provisioning file inside the app bundle
# EMBEDDED_PROVISIONING_PROFILE=${BUNDLE_DIR}/embedded.mobileprovision

# cp ~/Library/MobileDevice/Provisioning\ Profiles/${PROVISIONING_PROFILE_NAME} ${EMBEDDED_PROVISIONING_PROFILE}
# echo ✅ Copy provisioning profile ${PROVISIONING_PROFILE_NAME} to ${EMBEDDED_PROVISIONING_PROFILE}


# # The team identifier of your signing identity
# # ⚠️ YOU NEED TO CHANGE THIS TO YOUR ID ️️⚠️
# TEAM_IDENTIFIER=X53G3KMVA6

# # The location if the .xcent file
# XCENT_FILE=${TEMP_DIR}/${PROJECT_NAME}.xcent

# ${PLIST_BUDDY} -c "Add :application-identifier string ${TEAM_IDENTIFIER}.${APP_BUNDLE_IDENTIFIER}" ${XCENT_FILE}
# ${PLIST_BUDDY} -c "Add :com.apple.developer.team-identifier string ${TEAM_IDENTIFIER}" ${XCENT_FILE}

# echo ✅ Create ${XCENT_FILE}


# # The id of the identity used for signing
# IDENTITY=E8C36646D64DA3566CB93E918D2F0B7558E78BAA

# # Sign all libraries in the bundle
# for lib in ${SWIFT_LIBS_DEST_DIR}/*; do
#   # Sign
#   codesign \
#     --force \
#     --timestamp=none \
#     --sign ${IDENTITY} \
#     ${lib}
#   echo ✅ Codesign ${lib}
# done

# # Sign the bundle itself
# codesign \
#   --force \
#   --timestamp=none \
#   --entitlements ${XCENT_FILE} \
#   --sign ${IDENTITY} \
#   ${BUNDLE_DIR}

# echo ✅ Codesign ${BUNDLE_DIR}

# echo

# #############################################################
# echo 🎉 Building ${PROJECT_NAME} for device successfully finished! 🎉
# exit 0
# #############################################################

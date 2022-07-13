# This file is intended to be run from Github Actions publish.yml.

[[ -z "$RELEASE_FLOW_TARGET" ]] && {
    echo "Env var RELEASE_FLOW_TARGET is empty";
    exit 1;
}

rustup target add $RELEASE_FLOW_TARGET

cargo build --manifest-path crossbundle/cli/Cargo.toml \
--release --all-features --target $RELEASE_FLOW_TARGET

OUTPUT_NAME="crossbundle-${RELEASE_FLOW_TARGET}"

if [[ "$OSTYPE" =~ ^windows ]]; then
    mkdir $OUTPUT_NAME
    dir target
    powershell copy-item -path target/$RELEASE_FLOW_TARGET/release/crossbundle.exe -destination $OUTPUT_NAME
    powershell copy-item -path README.md -destination $OUTPUT_NAME
    powershell copy-item -path LICENSE -destination $OUTPUT_NAME
    dir $OUTPUT_NAME
    powershell Compress-Archive -Path $OUTPUT_NAME/* -DestinationPath $OUTPUT_NAME.zip
else
    mkdir $OUTPUT_NAME
    cp target/$RELEASE_FLOW_TARGET/release/crossbundle $OUTPUT_NAME/
    cp README.md LICENSE* $OUTPUT_NAME/
    zip -r $OUTPUT_NAME.zip $OUTPUT_NAME
fi

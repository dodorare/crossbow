# This file is intended to be run from Github Actions publish.yml.

# Legends say that the order of the crates should be kept.
crates=(
    platform/android
    plugins/admob-android
    crossbundle/tools
    crossbundle/cli
)
for crate in "${crates[@]}"
do
    echo "Publishing ${crate}"
    cargo publish --manifest-path=$crate/Cargo.toml
    sleep 40
done
sleep 40
cargo publish

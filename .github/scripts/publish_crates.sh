#!/bin/bash
# This file is intended to be run from Github Actions publish.yml.

# Legends say that the order of the crates should be kept.
crates=(
    ./platform/android
    ./platform/ios
    ./
    ./crossbundle/tools
    ./crossbundle/cli
    ./plugins/admob-android
    ./plugins/play-games-services
)
for crate in "${crates[@]}"
do
    echo "Publishing ${crate}"
    pushd $crate
    cargo publish
    sleep 40
    popd
done

# Testing guide

## How to run tests in crossbow

In `crossbow`, we have unit tests, integration tests, and examples. To run tests, you will need to set up an **Android** and **Apple** environment (you can find more information in [installation category](../install/README.md)).

If you want to run tests for our `crossbundle` crate, you can make it by the following steps: download this repository, proceed to the `crossbundle/tools` directory, and run `cargo test`. It will run all tests for the `crossbundle-tools` crate that is primarily used by `crossbundle`.

## Dependency updates

Crossbow commits the workspace `Cargo.lock` because the workspace ships the `crossbundle` executable. Required CI, release, and installation checks use `--locked` so that a commit is always tested with its reviewed dependency graph.

Dependabot checks for Cargo updates every week and opens pull requests that update the manifests and lockfile. Minor and patch updates are grouped; major updates remain separate so their compatibility can be reviewed independently.

The `Latest compatible dependencies` workflow provides an additional early-warning check every week and on demand. It generates a new lockfile from the manifests, checks the complete workspace, and runs the Android test suites against that fresh resolution. This scheduled workflow does not replace or gate the reproducible checks run on pull requests.

## In case of issues

Feel free to open [Github Issues](https://github.com/dodorare/crossbow/issues/new/choose) - we will be happy to fix or review them.

# Crossbundle update command

Crossbundle has an inner command that allows to check the version used by the user and compare it with the version in `crates.io`.
To check the latest version of crossbundle project in `crates.io` use:

```sh
crossbundle update --check
```

If the version found in `crates.io` is newer than used now you can enter the command below:

```sh
crossbundle update --update
```

You also can just specify `--update` flag with [crossbundle build](command-build.md) or [crossbundle run](command-run.md). For example:

```sh
crossbundle build android --update
```

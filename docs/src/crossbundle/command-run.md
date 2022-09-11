# Crossbundle run command

## Crossbundle run gradle

Crossbow default run process requires installed Gradle on your PC.

To create a project go to the example you want to build and use the command below. The command belongs to macroquad engine examples building:

```sh
crossbundle run android

# To specify custom export gradle directory
crossbundle run android --export-path=./gen/
```

By default run directory is `target/android/<project_name>/gradle`. But you can specify your own build directory via `--export-path=<OUT_PATH>` flag.

## Crossbundle run native AAB/APK

If you don't want to use gradle you can specify it in strategy native-apk:

```sh
crossbundle run android -s=native-apk
# or do you need AAB:
crossbundle run android -s=native-aab
```

To find out available commands specify the -h flag.

```sh
crossbundle run android -h
```

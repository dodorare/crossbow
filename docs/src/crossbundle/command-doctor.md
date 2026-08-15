# Crossbundle doctor command

`crossbundle doctor` diagnoses every platform enabled when Crossbundle was compiled. It
does not search for a project, download tools, install components, generate files, or
compile code. Project checks run only when an explicit `--project` path is supplied.

```sh
crossbundle doctor
crossbundle doctor --platform android
crossbundle doctor --platform apple
crossbundle doctor --platform android,apple
crossbundle doctor --platform android --platform apple
crossbundle doctor --project ./Cargo.toml --platform apple
```

Platform names are typed and case-sensitive. Repeated or comma-delimited values are
deduplicated and reported in canonical `android`, `apple` order. Requesting an unknown
platform, or a known platform excluded from that Crossbundle build, is an invocation
error.

Common host and project checks run once. Each selected platform then runs its own checks.
On Linux and Windows, Apple-only tooling checks are `skip` with an explanation. These
skips do not fail the report, including under `--strict`.

## Compatibility and strict mode

Compatibility ranges live in `crossbundle/tools/Cargo.toml` under
`package.metadata.crossbundle.compatibility`:

- A preferred version passes.
- Another supported version passes with a note.
- An unsupported or unrecognized version warns normally and fails with `--strict`.
- A check that is irrelevant to the host or operation is skipped. Strict mode never
  promotes `skip` to `fail`.

Crossbundle currently checks the discovered Xcode and Apple SDK versions but does not
impose an artificial Xcode or SDK version range. Missing required Apple tools on macOS
still fail.

## Coverage

Common checks cover Cargo, rustc, the selected Cargo package, and typed Crossbow metadata.
With `--project`, package selection reuses the build path's read-only
`cargo locate-project --message-format plain` query; pass a package directory rather than a
virtual-workspace root.
Android checks cover Java, jarsigner, Gradle, Android SDK platforms and build tools, NDK,
adb, bundletool, configured Android targets, assets, resources, manifests, and local
Gradle plugins.

Apple checks cover the host OS, full Xcode installation, active developer directory,
Xcode version, Command Line Tools, `xcodebuild`, `xcrun`, `simctl`, iPhoneOS and
iPhoneSimulator SDKs, relevant installed Rust targets, and signing relevance. Project
checks use the same typed metadata and Info.plist model as Apple builds to validate bundle
metadata, bundle identifiers, deployment targets, Rust targets, assets, resources, icons,
platform-specific plugin compatibility, and signing configuration applicability. Signing
values and command output that may identify credentials are never included in reports.
Configured Android Gradle plugins are reported as inapplicable to Apple; ordinary Cargo
dependencies are not guessed to be plugins. The typed Apple project model has no
project-level signing fields: `project.apple.signing` warns when a device target requires
build-time signing arguments and skips simulator-only projects where signing is irrelevant.

## JSON contract

`--json` writes one JSON document to stdout; human diagnostics and errors go to stderr.
Schema version 1 contains:

```json
{
  "schema_version": 1,
  "command": "doctor",
  "scope": "host",
  "strict": false,
  "platforms": ["android", "apple"],
  "status": "pass",
  "summary": { "pass": 0, "warn": 0, "fail": 0, "skip": 0 },
  "checks": []
}
```

Check states are `pass`, `warn`, `fail`, or `skip`. Checks and platforms have deterministic
ordering, and `summary` is the exact aggregation of check states.

## Stable check IDs

The schema-v1 registry is:

- Common host: `host.rust.cargo`, `host.rust.rustc`
- Android host: `host.java.runtime`, `host.java.jarsigner`, `host.gradle`,
  `android.sdk.root`, `android.sdk.platform`, `android.sdk.build_tools`, `android.ndk`,
  `android.adb`, `android.bundletool`
- Apple host: `apple.host.os`, `apple.xcode.installation`, `apple.xcode.version`,
  `apple.xcode.developer_dir`, `apple.xcode.command_line_tools`,
  `apple.tool.xcodebuild`, `apple.tool.xcrun`, `apple.tool.simctl`,
  `apple.sdk.iphoneos`, `apple.sdk.iphonesimulator`, `apple.signing.identity`, and
  `apple.rust.target.<triple>`
- Common project: `project.cargo.manifest`, `project.cargo.package`,
  `project.crossbow.metadata`
- Android project: `project.android.assets`, `project.android.resources`,
  `project.android.icon`, `project.android.manifest`, `project.android.targets`,
  `project.android.rust_targets`, `project.android.plugins`,
  `project.android.target_sdk`, `project.android.min_sdk`
- Apple project: `project.apple.metadata`, `project.apple.bundle_identifier`,
  `project.apple.deployment_target`, `project.apple.target.<triple>`,
  `project.apple.assets`, `project.apple.icon`, `project.apple.signing`, and
  `project.apple.plugin.<normalized-name>`

Dynamic target suffixes are canonical Rust triples. Plugin suffixes are normalized by
lowercasing ASCII text, replacing each run of non-alphanumeric characters with one `-`,
trimming leading and trailing separators, and using `unnamed` if nothing remains.

## Exit codes

- `0`: no failed checks; warnings and skips are allowed
- `1`: one or more checks failed
- `2`: invalid invocation, a requested platform is not compiled in, or report
  serialization failed

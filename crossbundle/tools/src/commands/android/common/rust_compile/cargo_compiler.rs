use crate::{commands::android::cargo_env_target_cfg, error::*, types::*};
use std::path::Path;

pub fn native_rust_compile(
    build_target: AndroidTarget,
    target_dir: &Path,
    target_sdk_version: u32,
    ndk: &AndroidNdk,
) -> Result<()> {
    let mut cargo = std::process::Command::new("cargo");
    let triple = build_target.rust_triple();
    let clang_target = format!(
        "--target={}{}",
        build_target.ndk_llvm_triple(),
        target_sdk_version
    );
    let ar = ndk.toolchain_bin("ar", build_target)?;

    cargo.env(format!("AR_{}", triple), &ar);
    cargo.env(cargo_env_target_cfg("AR", triple), &ar);

    // Read initial RUSTFLAGS
    let mut rustflags = match std::env::var("CARGO_ENCODED_RUSTFLAGS") {
        Ok(val) => val,
        Err(std::env::VarError::NotPresent) => "".to_string(),
        Err(std::env::VarError::NotUnicode(_)) => {
            panic!("RUSTFLAGS environment variable contains non-unicode characters")
        }
    };

    let (clang, clang_pp) = ndk.clang(build_target, target_sdk_version)?;

    // Configure cross-compiler for `cc` crate
    // https://github.com/rust-lang/cc-rs#external-configuration-via-environment-variables
    cargo.env(format!("CC_{}", triple), &clang);
    cargo.env(format!("CFLAGS_{}", triple), &clang_target);
    cargo.env(format!("CXX_{}", triple), &clang_pp);
    cargo.env(format!("CXXFLAGS_{}", triple), &clang_target);

    // Configure LINKER for `rustc`
    // https://doc.rust-lang.org/beta/cargo/reference/environment-variables.html#configuration-environment-variables
    cargo.env(cargo_env_target_cfg("LINKER", triple), &clang);
    if !rustflags.is_empty() {
        rustflags.push('\x1f');
    }

    rustflags.push_str("-Clink-arg=");
    rustflags.push_str(&clang_target);

    let ar = ndk.toolchain_bin("ar", build_target)?;
    cargo.env(format!("AR_{}", triple), &ar);
    cargo.env(cargo_env_target_cfg("AR", triple), &ar);

    // Workaround for https://github.com/rust-windowing/android-ndk-rs/issues/149:
    // Rust (1.56 as of writing) still requires libgcc during linking, but this does
    // not ship with the NDK anymore since NDK r23 beta 3.
    // See https://github.com/rust-lang/rust/pull/85806 for a discussion on why libgcc
    // is still required even after replacing it with libunwind in the source.
    // XXX: Add an upper-bound on the Rust version whenever this is not necessary anymore.
    if ndk.build_tag() > 7272597 {
        let link_dir = target_dir.join("link-libraries");
        std::fs::create_dir_all(&link_dir).map_err(|_| Error::PathNotFound(link_dir.clone()))?;
        let libgcc = link_dir.join("libgcc.a");
        std::fs::write(&libgcc, "INPUT(-lunwind)")
            .map_err(|_| Error::PathNotFound(link_dir.clone()))?;

        // cdylibs in transitive dependencies still get built and also need this
        // workaround linker flag, yet arguments passed to `cargo rustc` are only
        // forwarded to the final compiler invocation rendering our workaround ineffective.
        // The cargo page documenting this discrepancy (https://doc.rust-lang.org/cargo/commands/cargo-rustc.html)
        // suggests to resort to RUSTFLAGS.
        // Note that `rustflags` will never be empty because of an unconditional `.push_str` above,
        // so we can safely start with appending \x1f here.
        rustflags.push_str("\x1f-L\x1f");
        rustflags.push_str(link_dir.to_str().expect("Target dir must be valid UTF-8"));
    }
    cargo.env("CARGO_ENCODED_RUSTFLAGS", rustflags);
    cargo.arg("rustc");
    cargo.arg("--lib");
    cargo.arg("--target").arg(triple);
    cargo.output_err(true)?;

    Ok(())
}

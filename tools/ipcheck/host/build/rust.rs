use std::path::Path;

pub fn build_new() -> std::io::Result<String> {
    build_bin("../impls/rust_new", "../artifacts/rust_new", "ipcheck_new")
}

pub fn build_current() -> std::io::Result<String> {
    build_bin(
        "../impls/rust_current",
        "../artifacts/rust_current",
        "ipcheck_current",
    )
}

fn build_bin(
    src_path: impl AsRef<Path>,
    artifact_path: impl AsRef<Path>,
    bin: &str,
) -> std::io::Result<String> {
    let src_path = src_path.as_ref();
    let artifact_path = artifact_path.as_ref();

    if !std::path::Path::new(artifact_path).exists() {
        std::fs::create_dir(artifact_path).expect("failed to create Rust artifacts dir");
    }

    std::fs::copy(src_path.join("main.rs"), artifact_path.join("main.rs"))?;
    std::fs::copy(
        src_path.join("Cargo.toml"),
        artifact_path.join("Cargo.toml"),
    )?;

    let output = std::process::Command::new("cargo")
        .args(&[
            "+nightly",
            "build",
            "--release",
            "-Z",
            "unstable-options",
            "--out-dir",
            ".",
        ])
        .current_dir(artifact_path)
        .output()?;

    if output.status.success() {
        Ok(artifact_path.join(bin).to_string_lossy().into_owned())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

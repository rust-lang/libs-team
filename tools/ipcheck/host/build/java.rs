pub fn build() -> std::io::Result<&'static str> {
    if !std::path::Path::new("../artifacts/java").exists() {
        std::fs::create_dir("../artifacts/java").expect("failed to create Java artifacts dir");
    }

    std::fs::copy(
        "../impls/java/IpCheck.java",
        "../artifacts/java/IpCheck.java",
    )?;

    let output = std::process::Command::new("java").args(&["--version"]).output()?;

    if output.status.success() {
        Ok("java ../artifacts/java/IpCheck.java")
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

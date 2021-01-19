pub fn build() -> std::io::Result<&'static str> {
    if !std::path::Path::new("../artifacts/python").exists() {
        std::fs::create_dir("../artifacts/python").expect("failed to create Python artifacts dir");
    }

    std::fs::copy(
        "../impls/python/ipcheck.py",
        "../artifacts/python/ipcheck.py",
    )?;

    let output = std::process::Command::new("python").args(&["--version"]).output()?;

    if output.status.success() {
        Ok("python ../artifacts/python/ipcheck.py")
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

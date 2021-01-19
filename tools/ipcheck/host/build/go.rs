pub fn build() -> std::io::Result<&'static str> {
    if !std::path::Path::new("../artifacts/go").exists() {
        std::fs::create_dir("../artifacts/go").expect("failed to create Go artifacts dir");
    }

    let output = std::process::Command::new("go")
        .args(&["build", "-o", "../../artifacts/go", "ipcheck.go"])
        .current_dir("../impls/go")
        .output()?;

    if output.status.success() {
        Ok("../artifacts/go/ipcheck")
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

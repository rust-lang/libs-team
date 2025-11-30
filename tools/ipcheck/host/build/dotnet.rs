pub fn build() -> std::io::Result<&'static str> {
    if !std::path::Path::new("../artifacts/dotnet").exists() {
        std::fs::create_dir("../artifacts/dotnet").expect("failed to create .NET artifacts dir");
    }

    std::fs::copy(
        "../impls/dotnet/IPCheck.cs",
        "../artifacts/dotnet/IPCheck.cs",
    )?;
    std::fs::copy(
        "../impls/dotnet/IPCheck.csproj",
        "../artifacts/dotnet/IPCheck.csproj",
    )?;

    let output = std::process::Command::new("dotnet")
        .args(&["--version"])
        .output()?;

    if output.status.success() {
        Ok("dotnet run --project ../artifacts/dotnet/IpCheck.csproj")
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

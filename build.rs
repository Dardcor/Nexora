#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon.ico");
    res.set("ProductName", "Nexora");
    res.set("FileDescription", "Nexora");
    res.set("OriginalFilename", "nexora.exe");
    res.set("InternalName", "Nexora");
    res.set("CompanyName", "Dardcor");
    let _ = res.compile();
}

#[cfg(not(windows))]
fn main() {}

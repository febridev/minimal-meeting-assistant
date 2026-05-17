fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/bridge.swift");
    
    let mut command = std::process::Command::new("swiftc");
    command
        .arg("-emit-library")
        .arg("-static")
        .arg("-module-name")
        .arg("meeting_assistant_swift")
        .arg("-o")
        .arg(format!("{}/libmeeting_assistant_swift.a", std::env::var("OUT_DIR").unwrap()))
        .arg("src/bridge.swift");

    let status = command.status().unwrap();
    if !status.success() {
        panic!("Failed to compile Swift code");
    }

    println!("cargo:rustc-link-search=native={}", std::env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=meeting_assistant_swift");
    
    tauri_build::build()
}

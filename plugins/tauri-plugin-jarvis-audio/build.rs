fn main() {
    // Build script for tauri-plugin-jarvis-audio
    // This is needed for Tauri plugin builds
    
    #[cfg(target_os = "android")]
    {
        println!("cargo:rustc-cfg=android");
    }
}

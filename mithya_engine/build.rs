// build.rs
fn main() {
    // Tell Cargo to rerun this build script if it changes
    println!("cargo:rerun-if-changed=build.rs");
    
    #[cfg(windows)]
    {
        // SDL2 requires these Windows system libraries
        println!("cargo:rustc-link-lib=advapi32");  // This is the missing one!
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=imm32");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=oleaut32");
        println!("cargo:rustc-link-lib=version");
        println!("cargo:rustc-link-lib=uuid");
        println!("cargo:rustc-link-lib=dinput8");
        println!("cargo:rustc-link-lib=dxguid");
        println!("cargo:rustc-link-lib=setupapi");
        
        // OpenGL if you're using it
        println!("cargo:rustc-link-lib=opengl32");
    }
}
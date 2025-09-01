use std::{
    env,
    path::{Path, PathBuf},
};

fn restore() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_path = Path::new(".");
    let qpm_path = PathBuf::from(env::var("QPM_PATH").unwrap_or_else(|_| "qpm".into()));

    // change if qpm.shared.json modified
    println!(
        "cargo:rerun-if-changed={}",
        manifest_path.join("qpm.json").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        manifest_path.join("qpm.shared.json").display()
    );


    let mut cmd = std::process::Command::new(qpm_path);
    cmd.current_dir(manifest_path)
        .arg("restore")
        // .arg("--quiet")
        .status()
        .map_err(|e| format!("Failed to run qpm: {}", e))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let include_dir = manifest_path.join("extern").join("includes");
    let lib_path = manifest_path.join("extern").join("libs");

    // run qpm restore
    restore().expect("Failed to restore dependencies");

    // cbindgen::Builder::new()
    //   .with_crate(&manifest_path)
    //   .generate()
    //   .expect("Unable to generate bindings")
    //   .write_to_file("include/bindings.h");

    // // The bindgen::Builder is the main entry point
    // // to bindgen, and lets you build up options for
    // // the resulting bindings.
    // let bindings = bindgen::Builder::default()
    //     // The input header we would like to generate
    //     // bindings for.
    //     .header("wrapper.h")
    //     // Tell cargo to invalidate the built crate whenever any of the
    //     // included header files changed.
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //     // Finish the builder and generate the bindings.
    //     .generate()
    //     // Unwrap the Result and panic on failure.
    //     .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("quest_compat.rs"))
    //     .expect("Couldn't write bindings!");

    build_cpp(include_dir, lib_path);
Ok(())
}

fn build_cpp(include_dir: PathBuf, lib_path: PathBuf) {
    // only compile in android linux AARCH64
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let quest = target_os == "android" && target_arch == "aarch64";
    if !quest  {
        return;
    } 

    println!("cargo:rustc-link-search={}", lib_path.display());
    
    cc::Build::new()
        .cpp(true) // Switch to C++ library compilation.
        .file("cpp/quest_compat.cpp")
        .cpp_link_stdlib("c++_static") // use libstdc++
        .flag_if_supported("-std=gnu++20")
        .flag_if_supported("-frtti")
        .flag_if_supported("-fexceptions")
        .flag_if_supported("-fdeclspec")
        .flag_if_supported("-Wno-invalid-offsetof")
        .flag("-DUNITY_2021")
        .flag("-DUNITY_2022")
        .flag("-DHAS_CODEGEN")
        .flag("-DNEED_UNSAFE_CSHARP")
        .flag("-DQUEST")
        .flag("-DFMT_HEADER_ONLY")
        // system include
        .flag(format!(
            "-isystem{}",
            include_dir // fmt/fmt/include
                .join("fmt")
                .join("fmt")
                .join("include")
                .display()
        ))
        .flag(format!(
            "-isystem{}",
            include_dir // libil2cpp/il2cpp/libil2cpp
                .join("libil2cpp")
                .join("il2cpp")
                .join("libil2cpp")
                .display()
        ))
        .flag(format!(
            "-isystem{}",
            include_dir // baselib include
                .join("libil2cpp")
                .join("il2cpp")
                .join("external")
                .join("baselib")
                .join("Include")
                .display()
        ))
        .flag(format!(
            "-isystem{}",
            include_dir // baselib android include
                .join("libil2cpp")
                .join("il2cpp")
                .join("external")
                .join("baselib")
                .join("Platforms")
                .join("Android")
                .join("Include")
                .display()
        ))
        .include(include_dir.join("bs-cordl").join("include"))
        .include(include_dir)
        .compile("quest_compat");
    }

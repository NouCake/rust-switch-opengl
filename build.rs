use std::{env, path::PathBuf};

use bindgen::CargoCallbacks;

fn main() {
    let devkitpro_path = PathBuf::from(env::var("DEVKITPRO").expect("DevKitPro needs to be installed"));
    let libdir_path = devkitpro_path.join("portlibs/switch");
    let headers_path_str = "wrapper.h";

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libdir_path.join("lib").to_str().unwrap());
    println!("cargo:rustc-link-search={}", devkitpro_path.join("libnx/lib").to_str().unwrap());
    println!("cargo:rustc-link-lib=glad");
    println!("cargo:rustc-link-lib=EGL");
    println!("cargo:rustc-link-lib=glapi");
    println!("cargo:rustc-link-lib=drm_nouveau");

    println!("cargo:rerun-if-changed={}", headers_path_str);
    let hmm = libdir_path.join("include");
    let hm = format!("-I{}", hmm.to_str().unwrap());

    let bindings = bindgen::Builder::default()
        .clang_arg(hm)
        .header(headers_path_str)
        .use_core()
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}

use bindgen::builder;
use std::env;
use std::path::PathBuf;

fn main() {
    let header = "native/mxchipWNET.h";
    let out_path = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search={}", out_path);
    println!("cargo:rustc-link-lib=dylib=mxchipWNet_3162");
    println!("cargo:rerun-if-changed={}", header);

    std::fs::copy(
        "native/mxchipWNet_3162.lib",
        format!("{}/libmxchipWNet_3162.a", out_path),
    )
    .unwrap();

    let bindings = builder()
        .header(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_comments(true)
        .clang_arg("-nostdlib")
        .size_t_is_usize(true)
        .ctypes_prefix("crate::ffi")
        .use_core()
        .generate()
        .unwrap();

    bindings
        .write_to_file(PathBuf::from(out_path).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

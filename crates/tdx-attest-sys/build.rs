use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=tdx_attest");
    let output_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen::Builder::default()
        .header("bindings.h")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(output_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

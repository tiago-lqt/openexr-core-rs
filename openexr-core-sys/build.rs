use anyhow::Result;
use std::path::PathBuf;
use std::{env, path::Path};

fn main() -> Result<()> {
    let openexr_root = env::var("OPENEXR_ROOT").expect("'OPENEXR_ROOT' envvar is not defined");

    let openexr_lib_path = Path::new(&openexr_root).join("lib");
    let openexr_include_path = Path::new(&openexr_root).join("include").join("OpenEXR");

    // Tell cargo to tell rustc to link the OpenEXRCore .lib
    println!("cargo:rustc-link-search={}", openexr_lib_path.display());

    println!("cargo:rustc-link-lib=OpenEXRCore-3_1");
    // println!("cargo:rustc-link-lib=static={}", "OpenEXRCore-2_5_static");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-F{}", openexr_include_path.display()))
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // generate newtypes for enums
        .default_enum_style(bindgen::EnumVariation::NewType { is_bitfield: false })
        // we'll do these manually
        // .blacklist_type("exr_attr_chromaticities_t")
        // .blacklist_type("exr_attr_keycode_t")
        // .blacklist_type("exr_attr_rational_t")
        // .blacklist_type("exr_attr_timecode_t")
        // .blacklist_type("exr_attr_tiledesc_t")
        // .blacklist_type("exr_channel_decode_info_t")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}

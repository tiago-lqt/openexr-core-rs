use anyhow::Result;
use std::path::PathBuf;
use std::{env, path::Path};

fn main() -> Result<()> {
    let openexr_root = env::var("OPENEXR_ROOT").expect("'OPENEXR_ROOT' envvar is not defined");
    let openexr_lib = Path::new(&openexr_root).join("lib");
    let openexr_include = Path::new(&openexr_root).join("include");

    let openexr_lib_name = "OpenEXRCore-3_1";

    println!("cargo:rustc-link-search={}", openexr_lib.display());
    println!("cargo:rustc-link-lib={}", openexr_lib_name);
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("src/openexr_core_wrapper.h")
        .clang_arg(format!("-I{}", openexr_include.display()))
        .clang_arg(format!("-I{}/OpenEXR", openexr_include.display()))
        .size_t_is_usize(true)
        .raw_line("use crate::*;")
        .default_enum_style(bindgen::EnumVariation::NewType { is_bitfield: false })
        .allowlist_recursively(false)
        .allowlist_function("exr_.**")
        .allowlist_type("exr_.*")
        .allowlist_type("_priv_exr_.*")
        .allowlist_type("_exr_.*")
        .allowlist_type("transcoding_pipeline_buffer_id")
        .allowlist_var("EXR_CONTEXT_.*")
        .allowlist_var("OPENEXR_VERSION_.*")
        .new_type_alias("exr_result_t")
        .blocklist_type("exr_attr_v2i_t")
        .blocklist_type("exr_attr_v2f_t")
        .blocklist_type("exr_attr_v2d_t")
        .blocklist_type("exr_attr_v3i_t")
        .blocklist_type("exr_attr_v3f_t")
        .blocklist_type("exr_attr_v3d_t")
        .rustified_enum("exr_error_code_t")
        .newtype_enum("exr_default_write_mode")
        .newtype_enum("exr_attr_list_access_mode")
        .newtype_enum("exr_storage_t")
        .newtype_enum("exr_compression_t")
        .newtype_enum("exr_envmap_t")
        .newtype_enum("exr_lineorder_t")
        .newtype_enum("exr_tile_level_mode_t")
        .newtype_enum("exr_tile_round_mode_t")
        .newtype_enum("exr_pixel_type_t")
        .rustfmt_bindings(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_path = PathBuf::from(manifest_dir).join("src/openexr_ffi.rs");

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");

    Ok(())
}

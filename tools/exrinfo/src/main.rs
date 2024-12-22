use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Parser;

use exr::attributes::Attribute;
use openexr_core::{
    self as exr,
    contexts::{
        initializer::ContextInitializer, reader::ReaderContext, traits::Context,
    },
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    files: Vec<String>,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[arg(short, long, default_value_t = false)]
    all_metadata: bool,

    #[arg(short, long, default_value_t = false)]
    strict: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    for file in args.files {
        let path = dunce::canonicalize(PathBuf::from(&file))?;

        process_file(&path, args.verbose, args.strict, args.all_metadata)?;
    }

    Ok(())
}

fn process_file(
    path: &Path,
    verbose: bool,
    strict: bool,
    all_metadata: bool,
) -> Result<()> {
    println!("{}", path.as_os_str().to_string_lossy());

    let mut init = ContextInitializer::default();

    if strict {
        init = init.with_strict_header();
    }

    if !verbose {
        init = init.with_silent_header_parse();
    }

    let reader = exr::read_with_init(path, init)?;

    print_header_info(reader, strict || all_metadata)?;

    Ok(())
}

fn print_header_info(reader: ReaderContext, verbose: bool) -> Result<()> {
    println!(
        "File '{}':",
        reader.file_name()?.as_os_str().to_string_lossy()
    );

    if verbose {
        println!("Flags:");
        if reader.is_singlepart_tiled()? {
            println!("singletile")
        }

        if reader.has_longnames()? {
            println!("longnames")
        } else {
            println!("shortnames")
        }

        if reader.has_nonimage_data()? {
            println!("deep")
        }

        if reader.is_multipart()? {
            println!("multipart")
        }

        println!(" parts: {}", reader.num_parts()?);
    }

    // for part in reader.parts() {
    //     print_part_info(part, is_multiplart, verbose)?;
    // }

    Ok(())
}

// fn print_part_info(part: PartReader, is_multipart: bool, verbose: bool) -> Result<()> {
//     let part_name = part.name()?.unwrap_or_else(|| "<single>".to_string());

//     if verbose || is_multipart || part.has_name()? {
//         println!(" part {}: {}", part.index()?, part_name);
//     }

//     if verbose {
//         for attribute in part.attributes()? {
//             println!("  ");
//             print_attr(attribute, verbose);
//         }

//         return Ok(());
//     }

//     if part.has_part_type()? {
//         println!("  ");
//         print_attr(part.part_type()?, verbose);
//     }
//     println!("  ");
//     print_attr(part.compression()?, verbose);

//     if part.has_tiles()? {
//         print_attr(part.tiles()?, verbose);
//     }
//     print_attr(part.displayWindow()?, verbose);
//     print_attr(part.dataWindow()?, verbose);
//     print_attr(part.channels()?, verbose);

//     if part.has_tiles()? {
//         println!(
//             "  tiled image has levels: x {} y {}",
//             part.num_tile_levels_x()?,
//             part.num_tile_levels_y()
//         );
//         println!("    x tile count:");
//         for l in 0..part.num_tile_levels_x() {
//             println!(
//                 " {} (sz {})",
//                 part.tile_level_tile_count_x(l),
//                 part.tile_level_tile_size_x(l)
//             );
//         }
//         println!("    y tile count:");

//         for l in 0..part.num_tile_levels_y() {
//             println!(
//                 " {} (sz {})",
//                 part.tile_level_tile_count_y(l),
//                 part.tile_level_tile_size_y(l)
//             );
//         }
//     }

//     Ok(())
// }

fn print_attr(_attr: Attribute, _verbose: bool) -> Result<()> {
    todo!()
}

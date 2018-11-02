//! This program is mainly intended for generating the dumps that are compiled in to
//! syntect, not as a helpful example for beginners.
//! Although it is a valid example for serializing syntaxes, you probably won't need
//! to do this yourself unless you want to cache your own compiled grammars.
//!
//! The standard command to generate the syntax dumps (including metadata) is:
//!
//! ```sh
//! cargo run --features="metadata" --example=gendata -- \
//! synpack testdata/Packages \
//! assets/default_newlines.packdump assets/default_nonewlines.packdump
//! assets/default_metadata.packdump testdata/DefaultPackage
//! ```
extern crate syntect;
use syntect::parsing::SyntaxSetBuilder;
use syntect::highlighting::ThemeSet;
use syntect::dumps::*;
use std::env;

fn usage_and_exit() -> ! {
    println!("USAGE: gendata synpack source-dir \
              newlines.packdump nonewlines.packdump \
              [metadata.packdump] [metadata extra-source-dir]\n       \
              gendata themepack source-dir themepack.themedump");
    ::std::process::exit(2);
}

fn main() {

    let mut a = env::args().skip(1);
    match (a.next(), a.next(), a.next(), a.next(), a.next(), a.next()) {
        (Some(ref cmd),
         Some(ref package_dir),
         Some(ref packpath_newlines),
         Some(ref packpath_nonewlines),
         ref _option_metapath,
         ref _option_metasource,
         ) if cmd == "synpack" => {
            let mut builder = SyntaxSetBuilder::new();
            builder.add_plain_text_syntax();
            builder.add_from_folder(package_dir, true).unwrap();
            let ss = builder.build();
            dump_to_file(&ss, packpath_newlines).unwrap();

            let mut builder_nonewlines = SyntaxSetBuilder::new();
            builder_nonewlines.add_plain_text_syntax();
            builder_nonewlines.add_from_folder(package_dir, false).unwrap();

            #[cfg(feature = "metadata")]
            {
                if let Some(metasource) = _option_metasource {
                    builder_nonewlines.add_from_folder(metasource, false).unwrap();
                }
            }

            let ss_nonewlines = builder_nonewlines.build();
            dump_to_file(&ss_nonewlines, packpath_nonewlines).unwrap();

            #[cfg(feature = "metadata")]
            {
                if let Some(metapath) = _option_metapath {
                    dump_to_file(&ss_nonewlines.metadata(), metapath).unwrap();
                }
            }

        }
        (Some(ref s), Some(ref theme_dir), Some(ref packpath), ..) if s == "themepack" => {
            let ts = ThemeSet::load_from_folder(theme_dir).unwrap();
            dump_to_file(&ts, packpath).unwrap();
        }
        _ => usage_and_exit(),
    }
}

// https://hackmd.io/@oli-obk/SyP5kZBmu?print-pdf#/
// https://smallcultfollowing.com/babysteps/blog/2020/04/09/libraryification/
// https://github.com/rust-lang/rust-analyzer/issues/12926
// https://doc.rust-lang.org/beta/nightly-rustc/

#![feature(rustc_private)]
#![deny(rustc::internal)]
extern crate rustc_codegen_ssa;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_metadata;
extern crate rustc_span;
extern crate rustc_target;

use rustc_codegen_ssa::back::metadata::DefaultMetadataLoader;
use rustc_metadata::locator;
use rustc_span::edition::Edition;
use rustc_target::spec::Target;
use rustc_target::spec::TargetTriple;
use std::io;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let path = std::env::args().nth(1).expect("no path given");
    //    let path = Path::new("/Volumes/Build/linux-build-llvm/rust/libmacros.so");
    let path = Path::new(&path);
    let triple = TargetTriple::from_triple("x86_64-apple-darwin");
    let target = Target::expect_builtin(&triple);
    let mut stdout = io::stdout();

    // https://stackoverflow.com/questions/75561888/cannot-access-a-scoped-thread-local-variable-without-calling-set-first
    rustc_span::create_session_globals_then(Edition::Edition2018, || {
        let _ = locator::list_file_metadata(&target, path, &DefaultMetadataLoader, &mut stdout);
    });

    //
    // https://doc.rust-lang.org/beta/nightly-rustc/rustc_metadata/locator/fn.list_file_metadata.html
    //
    // pub fn list_file_metadata(
    //     target: &Target,
    //     path: &Path,
    //     metadata_loader: &dyn MetadataLoader,
    //     out: &mut dyn Write
    // ) -> IoResult<()>
}

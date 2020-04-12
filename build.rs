extern crate bindgen;

use std::env;
use std::path::PathBuf;

//place to dump the bind gen process
const FILEPATH_CODE: &'static str = "src/os/";


fn main() {
    //env::set_var("LIBCLANG_PATH", "/usr/local/llvm90/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        //clang args
        .clang_arg("-O2")
        .clang_arg("-pipe")
        .clang_arg("-fno-strict-aliasing")
        .clang_arg("-Werror")
        .clang_arg("-D_KERNEL")
        .clang_arg("-DKLD_MODULE")
        .clang_arg("-nostdinc")
        .clang_arg("-I.")
        .clang_arg("-I/usr/src/sys")
        .clang_arg("-fno-common")
        .clang_arg("-fno-omit-frame-pointer")
        .clang_arg("-mno-omit-leaf-frame-pointer")
        .clang_arg("-MD")
        .clang_arg("-mcmodel=kernel")
        .clang_arg("-mno-red-zone")
        .clang_arg("-mno-mmx")
        .clang_arg("-mno-sse")
        .clang_arg("-msoft-float")
        .clang_arg("-fno-asynchronous-unwind-tables")
        .clang_arg("-ffreestanding")
        .clang_arg("-fwrapv")
        .clang_arg("-fstack-protector")
        .clang_arg("-Wall")
        .clang_arg("-Wredundant-decls")
        .clang_arg("-Wnested-externs")
        .clang_arg("-Wstrict-prototypes")
        .clang_arg("-Wmissing-prototypes")
        .clang_arg("-Wpointer-arith")
        .clang_arg("-Winline")
        .clang_arg("-Wcast-qual")
        .clang_arg("-Wundef")
        .clang_arg("-Wno-pointer-sign")
        .clang_arg("-D__printf__=__freebsd_kprintf__")
        .clang_arg("-Wmissing-include-dirs")
        .clang_arg("-fdiagnostics-show-option")
        .clang_arg("-Wno-unknown-pragmas")
        .clang_arg("-Wno-error-tautological-compare")
        .clang_arg("-Wno-error-empty-body")
        .clang_arg("-mno-aes")
        .clang_arg("-mno-avx")
        .clang_arg("-std=iso9899:1999")
        .clang_arg("sysroot=usr/include/sys")
        .clang_arg("-I/usr/include/sys/")
        .clang_arg("-target")
        .clang_arg("aarch64-unknown-freebsd")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(FILEPATH_CODE);
    // bindings
    //     .write_to_file(out_path)
    //     .expect("Couldn't write bindings!");
    let out_path = PathBuf::from(FILEPATH_CODE);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

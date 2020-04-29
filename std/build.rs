extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::io::Write;


//place to dump the bind gen process
const FILEPATH_CODE: &'static str = "src/os/";


const FILENAME_HEADER: &'static str = "src/os/kernel-include.rs";


const CLANG_HEADER_REQUIRED: [&'static str; 4] = [
	"usr/include/sys/types.h",
	"usr/include/sys/module.h",
	"sys/param.h",
	"sys/malloc.h",
];

fn main() {

    //for(key,value) in env::vars(){println!("{}: {}", key, value);}
    //env::set_var("LIBCLANG_PATH", "/usr/local/llvm90/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");
    //println!("cargo:rustc-link-search=/usr/include/sys/");
    //println!("cargo:rustc-link-lib=param.h");
    //println!("cargo:rustc-link-lib=malloc.h");
    //println!("cargo:rustc-link-lib=types.h");
    //println!("cargo:rustc-link-lib=module.h");




    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    //let filepath_header = FILENAME_HEADER.clone();

 //    match std::fs::File::create(filepath_header.clone()) {
	// 	Ok(mut file) => {
	// 		// Generate include lines for all requested headers
	// 		//for clang_file in clang_files.iter() {
	// 		//	writeln!(file, "#include <{}>", clang_file).unwrap();
	// 		//}
			
	// 		// Generate include lines for headers required by linux-std itself
	// 		for clang_file in CLANG_HEADER_REQUIRED.iter() {
	// 			writeln!(file, "#include <{}>", clang_file).unwrap();
	// 		}
	// 	},
	// 	Err(error) => {
	// 		panic!("Failed to open file \"{}\": {}", filepath_header, error);
	// 	}
	// }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .emit_builtins()
         

        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        
        //clang args
        //need these because clang can't find some header files
        //.clang_arg("sysroot=usr/include/")
        //.clang_arg("-I/usr/include/")
   
        .derive_debug(false)
        .opaque_type("timex")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .rustfmt_bindings(true)
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
        .write_to_file(out_path.join("bindings1.rs"))
        .expect("Couldn't write bindings!");
}

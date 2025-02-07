use std::env;

fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("bindgens")
        .generate_csharp_file("../dotnet/NativeMethods.g.cs")
        .unwrap();

        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        // For Generating C Bindings:
        //cbindgen::Builder::new()
        //  .with_crate(crate_dir)
        //  .generate()
        //  .expect("Unable to generate bindings")
        //  .write_to_file("../c/bindings.h");

        // For Generating C++ Bindings:
        //cbindgen::Builder::new()
        //  .with_crate(crate_dir)
        //  .generate()
        //  .expect("Unable to generate bindings")
        //  .write_to_file("../cpp/bindings.hpp");

}

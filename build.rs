use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("src/bridge/bridge.cpp")
        .cpp(true)
        .warnings(false)
        .flag("-std=c++11")
        .compile("bridge");

    add_search_path();

    println!("cargo:rustc-link-lib=xtptraderapi");
    println!("cargo:rustc-link-lib=xtpquoteapi");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/wrapper.hpp");
    println!("cargo:rerun-if-changed=src/bridge/bridge.hpp");
    println!("cargo:rerun-if-changed=src/bridge/bridge.cpp");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.hpp")
        /* // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks)) */
        .ignore_methods()
        .rustified_enum(".*")
        .blacklist_item("XTP_SIDE_TYPE")
        .blacklist_item("XTP_POSITION_EFFECT_TYPE")
        .blacklist_item("TXTPTradeTypeType")
        .blacklist_item("TXTPOrderTypeType")
        .blacklist_function("TraderSpiStub_Rust.*")
        .blacklist_function("QuoteSpiStub_Rust.*")
        /* .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        }) */
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(target_os = "windows"))]
fn add_search_path() {
    for path in std::env::var("LD_LIBRARY_PATH")
        .unwrap_or_else(|_| "".to_string())
        .split(":")
    {
        println!("cargo:rustc-link-search={}", path);
    }
}

#[cfg(target_os = "windows")]
fn add_search_path() {
    for path in std::env::var("PATH")
        .unwrap_or_else(|_| "".to_string())
        .split(":")
    {
        println!("cargo:rustc-link-search={}", path);
    }
}

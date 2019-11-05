extern crate pkg_config;
#[cfg(target_env = "msvc")]
extern crate vcpkg;

extern crate bindgen;

use cmake::Config;
use std::env;
use std::path::PathBuf;

#[cfg(not(target_env = "msvc"))]
fn lin(mongoc_version: &str) {
    use std::path::Path;
    use std::process::Command;

    if pkg_config::Config::new()
        .atleast_version(mongoc_version)
        .statik(true)
        .probe("libmongoc-1.0")
        .is_err()
    {
        let out_dir = env::var("OUT_DIR").expect("No out dir");
        //let out_dir = format!("{}/{}", out_dir_var, mongoc_version);
        let driver_src_path = format!("mongo-c-driver-{}", mongoc_version);
        println!("out {}", &out_dir);

        let libmongoc_path = Path::new(&out_dir).join("lib/libmongoc-1.0.a");
        if !libmongoc_path.exists() {
            // Download and extract driver archive
            let url = format!(
                    "https://github.com/mongodb/mongo-c-driver/releases/download/{}/mongo-c-driver-{}.tar.gz",
                    mongoc_version,
                    mongoc_version
                );
            assert!(Command::new("curl")
                .arg("-O") // Save to disk
                .arg("-L") // Follow redirects
                .arg(url)
                .status()
                .expect("Could not run curl")
                .success());

            let archive_name = format!("mongo-c-driver-{}.tar.gz", mongoc_version);
            assert!(Command::new("tar")
                .arg("xzf")
                .arg(&archive_name)
                .status()
                .expect("Could not run tar")
                .success());

            let dst = Config::new(&dbg!(driver_src_path))
                .define("ENABLE_STATIC", "AUTO")
                .define("ENABLE_AUTOMATIC_INIT_AND_CLEANUP", "OFF")
                .define("CMAKE_BUILD_TYPE", "Release")
                .build();

            println!("cargo:rustc-link-search=native={}", dst.display());
        }

        println!("!!!out {}", &out_dir);

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        bindgen::Builder::default()
            .header(format!("{}/include/libmongoc-1.0/mongoc.h", &out_dir))
            .clang_arg(format!("-I{}/include/libmongoc-1.0", &out_dir))
            .clang_arg(format!("-I{}/include/libbson-1.0", &out_dir))
            .whitelist_function("bson_.*")
            .whitelist_function("mongoc_.*")
            .whitelist_type("mongoc_error_.*")
            .whitelist_var("BSON_ERROR.*")
            .whitelist_var("MONGOC.*")
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings");

        // Output to Cargo
        println!("cargo:rustc-link-search=native={}/lib", &out_dir);
        println!("cargo:rustc-link-lib=dylib=bson-1.0");
        println!("cargo:rustc-link-lib=dylib=mongoc-1.0");
    }
}

#[cfg(target_env = "msvc")]
fn win(_mongoc_version: &str) {
    use vcpkg;

    let mongo_lib = "mongoc-1.0";
    let bson_lib = "bson-1.0";

    if vcpkg::Config::new()
        .emit_includes(true)
        .probe("mongo-c-driver")
        .is_ok()
    {
        // Output to Cargo
        println!("cargo:rustc-link-lib={}", bson_lib);
        println!("cargo:rustc-link-lib={}", mongo_lib);
    } else {
        if let Ok(bson_dir_lib) = env::var("MONGO_LIB") {
            if let Ok(mongo_dir_lib) = env::var("BSON_LIB") {
                println!("cargo:rustc-link-search=native={}", bson_dir_lib);
                println!("cargo:rustc-link-lib=dylib={}", bson_lib);
                println!("cargo:rustc-link-search=native={}", mongo_dir_lib);
                println!("cargo:rustc-link-lib=dylib={}", mongo_lib);
            } else {
                panic!("please define BSON_LIB to {}.lib, \n for example set BSON_LIB=C:\\vcpkg\\packages\\libbson_x64-windows\\lib", bson_lib);
            }
        } else {
            panic!("please define MONGO_LIB to {}.lib, \n for example set MONGO_LIB=C:\\vcpkg\\packages\\mongo-c-driver_x64-windows\\lib", mongo_lib);
        }
    }
}

fn main() {
    let mongoc_version = env!("CARGO_PKG_VERSION")
        .split('-')
        .next()
        .expect("Crate version is not valid");

    #[cfg(target_env = "msvc")]
    win(mongoc_version);
    #[cfg(not(target_env = "msvc"))]
    lin(mongoc_version);
}

use std::env;
use std::path::PathBuf;

fn main() {
    #[allow(unused_variables)]
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let is_windows = env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows";

    if is_windows {
        println!("cargo:rustc-link-lib=user32");
    }

    #[cfg(feature = "injectee")]
    {
        println!("cargo:rerun-if-changed=src/postjectee.c");
        println!("cargo:rerun-if-changed=src/postjectee.h");
        println!("cargo:rerun-if-changed=cmake/postject/postject-api.h");
        cc::Build::new()
            .file("src/postjectee.c")
            .flag_if_supported("-Wno-unused-parameter")
            .compile("postjectee");
        if is_windows {
            println!("cargo:rerun-if-changed=src/dummy_win_res.rc");
            embed_resource::compile("src/dummy_win_res.rc", embed_resource::NONE);
        }
    }

    #[cfg(feature = "injector")]
    {
        println!("cargo:rerun-if-changed=cmake/src/postjector.h");
        println!("cargo:rerun-if-changed=cmake/src/postjector.cpp");
        println!("cargo:rerun-if-changed=cmake/CMakeLists.txt");

        let dst = cmake::Config::new("cmake").build();

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );
        println!("cargo:rustc-link-lib=static=postjector");
        println!("cargo:rustc-link-lib=static=postject");
        println!("cargo:rustc-link-lib=static=LIEF");
    }
}

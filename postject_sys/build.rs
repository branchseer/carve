use std::env;
use std::path::PathBuf;

fn main() {
    #[allow(unused_variables)]
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    #[cfg(feature = "injectee")]
    {
        println!("cargo:rerun-if-changed=src/postjectee.c");
        println!("cargo:rerun-if-changed=src/postjectee.h");
        println!("cargo:rerun-if-changed=cmake/postject/postject-api.h");
        cc::Build::new()
            .file("src/postjectee.c")
            .flag_if_supported("-Wno-unused-parameter")
            .compile("postjectee");

        let api_bindings = bindgen::builder()
            .header("src/postjectee.h")
            .allowlist_function("postjectee_.*")
            .generate()
            .unwrap();
        api_bindings
            .write_to_file(out_path.join("postjectee_bindings.rs"))
            .unwrap();
    }

    #[cfg(feature = "injector")]
    {
        println!("cargo:rerun-if-changed=cmake/src/postjector.h");
        println!("cargo:rerun-if-changed=cmake/src/postjector.cpp");

        let dst = cmake::Config::new("cmake")
            .build_target("postjector")
            .build();
        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("build").display()
        );
        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("build/postject").display()
        );
        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("build/postject/vendor/lief").display()
        );
        println!("cargo:rustc-link-lib=static=postjector");
        println!("cargo:rustc-link-lib=static=postject");
        println!("cargo:rustc-link-lib=static=LIEF");

        let postjector_bindings = bindgen::builder()
            .header("cmake/src/postjector.h")
            .prepend_enum_name(false)
            .allowlist_var("POSTJECTOR_.*")
            .allowlist_function("postjector_.*")
            .generate()
            .unwrap();
        postjector_bindings
            .write_to_file(out_path.join("postjector_bindings.rs"))
            .unwrap();
    }
}
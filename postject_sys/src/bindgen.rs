#![cfg(test)]

use bindgen::Bindings;
use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

struct OnDiskBindings {
    path: PathBuf,
    bindings: Bindings,
}
impl OnDiskBindings {
    fn full_path(&self) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(&self.path)
    }
    fn write(&self) {
        self.bindings.write_to_file(self.full_path()).unwrap()
    }
    fn assert(&self) {
        let actual = fs::read_to_string(self.full_path()).unwrap();
        let expected = self.bindings.to_string();
        pretty_assertions::assert_str_eq!(actual, expected);
    }
}

fn postjector_bindings() -> OnDiskBindings {
    OnDiskBindings {
        bindings: bindgen::builder()
            .formatter(bindgen::Formatter::Prettyplease)
            .header(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/cmake/src/postjector.h"
            ))
            .prepend_enum_name(false)
            .allowlist_var("POSTJECTOR_.*")
            .allowlist_function("postjector_.*")
            .raw_line("#![allow(non_camel_case_types)]")
            .layout_tests(false)
            .generate()
            .unwrap(),
        path: PathBuf::from("src/postjector.rs"),
    }
}

#[test]
fn test_postjector_bindings() {
    postjector_bindings().assert()
}

#[test]
#[ignore]
fn write_postjector_bindings() {
    postjector_bindings().write()
}


fn postjectee_bindings() -> OnDiskBindings {
    OnDiskBindings {
        bindings: bindgen::builder()
        .formatter(bindgen::Formatter::Prettyplease)
        .header("src/postjectee.h")
        .allowlist_function("postjectee_.*")
        .layout_tests(false)
        .generate()
        .unwrap(),
        path: PathBuf::from("src/postjectee.rs"),
    }
}
#[test]
fn test_postjectee_bindings() {
    postjectee_bindings().assert()
}

#[test]
#[ignore]
fn write_postjectee_bindings() {
    postjectee_bindings().write()
}

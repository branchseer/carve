#![cfg(test)]

use bindgen::Builder;
use std::{
    fs,
    path::{Path, PathBuf},
};

struct OnDiskBindings {
    path: PathBuf,
    binding_builder: Builder,
}
impl OnDiskBindings {
    fn full_path(&self) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(&self.path)
    }
    fn source(&self) -> String {
        const TARGETS: &[&str] = &[
            "x86_64-unkownn-linux-gnu",
            "x86_64-pc-windows-gnu",
            "x86_64-pc-windows-msvc",
            "i686-pc-windows-gnu",
            "i686-pc-windows-msvc",
            "x86_64-apple-darwin",
            "arm64-apple-darwin",
            "x86_64-apple-ios",
            "arm64-apple-ios",
        ];
        let mut sources = TARGETS.iter().copied().map(|target| {
            let source = self.binding_builder
                .clone()
                .clang_args([&format!("--target={}", target)])
                .generate()
                .unwrap()
                .to_string();
            (target, source)
        });
        let baseline = sources.next().unwrap();
        for (target, source) in sources {
            pretty_assertions::assert_str_eq!(&baseline.1, &source, "bindings are different between target {} and {}", baseline.0, target);
        }
        return baseline.1
    }
    fn write(&self) {
        fs::write(self.full_path(), self.source()).unwrap();
    }
    fn assert(&self) {
        let actual = fs::read_to_string(self.full_path())
            .unwrap()
            .replace('\r', ""); // Git on Windows might insert CR at line endings
        let expected = self.source();
        pretty_assertions::assert_str_eq!(actual, expected);
    }
}

fn postjector_bindings() -> OnDiskBindings {
    OnDiskBindings {
        binding_builder: bindgen::builder()
            .formatter(bindgen::Formatter::Prettyplease)
            .header(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/cmake/src/postjector.h"
            ))
            .prepend_enum_name(false)
            .allowlist_var("POSTJECTOR_.*")
            .allowlist_function("postjector_.*")
            .raw_line("#![allow(non_camel_case_types)]")
            .layout_tests(false),
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
        binding_builder: bindgen::builder()
            .formatter(bindgen::Formatter::Prettyplease)
            .header("src/postjectee.h")
            .allowlist_function("postjectee_.*")
            .layout_tests(false),
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

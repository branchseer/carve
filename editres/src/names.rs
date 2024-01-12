use std::{ffi::CString, num::NonZeroU16};

/*
http://www.skyfree.org/linux/references/ELF_Format.pdf#page=21
Section names with a dot (.) prefix are reserved for the system....
Applications may use names without the prefix to avoid
conflicts with system sections.
*/
pub(crate) fn elf_section_name(id: NonZeroU16) -> CString {
    CString::new(format!("editres{id}")).unwrap()
}

/*
https://developer.apple.com/library/archive/documentation/Performance/Conceptual/CodeFootprint/Articles/MachOOverview.html
The convention for segment names is to use all-uppercase letters preceded by double underscores (for example, __TEXT);
the convention for section names is to use all-lowercase letters preceded by double underscores (for example, __text).
*/
pub(crate) fn macho_segment_name(id: NonZeroU16) -> CString {
    // https://github.com/nodejs/postject/issues/1
    // Mutiple sections under the same segment causes a crash.
    // As a workaround, a segment is added for each resource.
    CString::new(format!("__EDITRES{id}")).unwrap()
}
pub(crate) fn macho_section_name(id: NonZeroU16) -> CString {
    CString::new(format!("__editres{id}")).unwrap()
}

/*
https://github.com/nodejs/postject/blob/3c4f2080ee56025716c3add0f6c03b16e2af54ff/src/api.js#L94
PE resource names appear to only work if uppercase
*/
pub(crate) fn pe_resource_name(id: NonZeroU16) -> CString {
    CString::new(format!("EDITRES{id}")).unwrap()
}

#[doc(hidden)]
pub const SENTINEL_SUFFIX: &str = "EDITRES_5C129DBE873743999080563ED17CEA33";

use crate::names::{elf_section_name, macho_section_name, macho_segment_name, pe_resource_name};
use std::{
    num::NonZeroU16,
    ptr::{self, null},
    slice,
};

use postject_sys::postjectee::{postject_options, postjectee_find_resource};

#[doc(hidden)]
pub use const_format::concatcp;

#[macro_export]
macro_rules! resource {
    ($name: literal) => {{
        const NAME_LEN: usize = $name.len();
        static SENTINEL: &[u8] = $crate::injectee::concatcp!(
            "\0\0",
            $name,
            '|',
            NAME_LEN,
            $crate::names::SENTINEL_SUFFIX
        )
        .as_bytes();
        unsafe { $crate::injectee::get_resource_from_sentinel(SENTINEL.as_ptr()) }
    }};
}

unsafe fn find_resource(id: NonZeroU16) -> Option<&'static [u8]> {
    let mut size = 0usize;
    let res_head = postjectee_find_resource(
        null(), // name is ignored now that we specific detailed names in options
        &mut size,
        &postject_options {
            elf_section_name: elf_section_name(id).as_ptr(),
            macho_framework_name: null(),
            macho_section_name: macho_section_name(id).as_ptr(),
            macho_segment_name: macho_segment_name(id).as_ptr(),
            pe_resource_name: pe_resource_name(id).as_ptr(),
        },
    );
    if res_head.is_null() {
        None
    } else {
        Some(slice::from_raw_parts(res_head.cast(), size))
    }
}

unsafe fn get_resource_id(sentinel_head: *const u8) -> Option<NonZeroU16> {
    let id_bytes = ptr::read_volatile::<[u8; 2]>(sentinel_head.cast());
    let id = u16::from_be_bytes(id_bytes);
    NonZeroU16::new(id)
}

#[doc(hidden)]
pub unsafe fn get_resource_from_sentinel(sentinel_head: *const u8) -> Option<&'static [u8]> {
    let id = get_resource_id(sentinel_head)?;
    Some(find_resource(id).unwrap_or_else(|| panic!("Failed to locate resource {}", id)))
}

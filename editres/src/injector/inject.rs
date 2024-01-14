use super::list::{list_for_injection, ResourceEntry};
use crate::names::{elf_section_name, macho_section_name, macho_segment_name, pe_resource_name};
use anyhow::Context;
use postject_sys::postjector::{
    postjector_inject, PostjectorBuffer, POSTJECTOR_INJECT_ALREADY_EXISTS, POSTJECTOR_INJECT_ERROR,
    POSTJECTOR_INJECT_SUCCESS, POSTJECTOR_INJECT_UNKNOWN_EXECUTABLE_FORMAT,
    postjector_owned_buffer_data,
    postjector_owned_buffer_free, PostjectorOwnedBuffer,
};
use std::num::NonZeroUsize;
use std::ops::Deref;
use std::{mem::size_of, num::NonZeroU16};

fn adhoc_sign(executable_buffer: &[u8]) -> Result<Vec<u8>, apple_codesign::AppleCodesignError> {
    use apple_codesign::{MachOSigner, SettingsScope, SigningSettings};
    let signer = MachOSigner::new(executable_buffer)?;
    let mut signed = Vec::<u8>::with_capacity(executable_buffer.len());
    let mut settings = SigningSettings::default();
    settings.set_binary_identifier(SettingsScope::Main, "-");
    signer.write_signed_binary(&settings, &mut signed)?;
    Ok(signed)
}

struct OwnedPostInjectorBuffer(PostjectorOwnedBuffer);
impl OwnedPostInjectorBuffer {
    pub unsafe fn new(raw: PostjectorOwnedBuffer) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self(raw))
        }
    }
}
impl Drop for OwnedPostInjectorBuffer {
    fn drop(&mut self) {
        unsafe { postjector_owned_buffer_free(self.0) }
    }
}
impl Deref for OwnedPostInjectorBuffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let data = postjector_owned_buffer_data(self.0);
            std::slice::from_raw_parts(data.head, data.size)
        }
    }
}

pub fn inject(
    executable_buffer: &mut Vec<u8>,
    resource_name: &str,
    resource_data: &[u8],
) -> anyhow::Result<()> {
    let resources =
        list_for_injection(&executable_buffer).context("Invalid sentiel format in executable")?;
    let resource_vec = resources
        .iter()
        .map(|(name, entry)| (*name, entry))
        .collect::<Vec<(&str, &ResourceEntry)>>();
    let resource_position = resource_vec
        .iter()
        .position(|(name, _)| *name == resource_name)
        .context("Resource not found")?;
    let (_, ResourceEntry::None { id_positions }) = resource_vec[resource_position] else {
        anyhow::bail!("Resource {} already exists", resource_name);
    };

    let resource_id = NonZeroUsize::new(resource_position + 1).unwrap();
    let resource_id =
        NonZeroU16::try_from(resource_id).context("Resource count exceeds the limit")?;

    let id_bytes = resource_id.get().to_be_bytes();

    let id_positions = id_positions.clone();
    drop(resources);
    for id_position in id_positions {
        let id_slice = &mut executable_buffer[id_position..(id_position + size_of::<u16>())];
        id_slice.copy_from_slice(&id_bytes);
    }
    unsafe {
        let inject_result = postjector_inject(
            PostjectorBuffer {
                head: executable_buffer.as_mut_ptr(),
                size: executable_buffer.len(),
            },
            PostjectorBuffer {
                head: resource_data.as_ptr() as *mut u8,
                size: resource_data.len(),
            },
            elf_section_name(resource_id).as_ptr(),
            macho_segment_name(resource_id).as_ptr(),
            macho_section_name(resource_id).as_ptr(),
            pe_resource_name(resource_id).as_ptr(),
            0,
        );
        if inject_result.type_ == POSTJECTOR_INJECT_ALREADY_EXISTS {
            anyhow::bail!("Resource {resource_name} has already been set before");
        } else if inject_result.type_ == POSTJECTOR_INJECT_ERROR {
            if let Some(error) = OwnedPostInjectorBuffer::new(inject_result.data) {
                anyhow::bail!("Inject failed: {}", String::from_utf8_lossy(&error));
            } else {
                anyhow::bail!("Inject failed");
            }
        } else if inject_result.type_ == POSTJECTOR_INJECT_UNKNOWN_EXECUTABLE_FORMAT {
            anyhow::bail!("Invalid executable format");
        } else if inject_result.type_ == POSTJECTOR_INJECT_SUCCESS {
            let output = OwnedPostInjectorBuffer::new(inject_result.data).unwrap();
            if inject_result.is_macho != 0 {
                *executable_buffer = adhoc_sign(&output)?;
            } else {
                executable_buffer.clear();
                executable_buffer.extend_from_slice(&output);
            }
            Ok(())
        } else {
            panic!("Unexpcted injection error {}", inject_result.type_)
        }
    }
}

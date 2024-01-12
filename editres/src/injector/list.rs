use crate::names::SENTINEL_SUFFIX;
use anyhow::Context;
use std::collections::btree_map::Entry;
use std::mem::size_of;
use std::num::NonZeroU16;
use std::str::from_utf8;

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ResourceEntry {
    Exists { id: NonZeroU16 },
    None { id_positions: Vec<usize> },
}

pub(crate) fn list_for_injection<'a>(
    executable_buffer: &'a [u8],
) -> anyhow::Result<BTreeMap<&'a str, ResourceEntry>> {
    let mut resources = BTreeMap::<&'a str, ResourceEntry>::new();
    let sentiel_indices = executable_buffer
        .windows(SENTINEL_SUFFIX.len())
        .enumerate()
        .filter_map(|(idx, window)| {
            if window == SENTINEL_SUFFIX.as_bytes() {
                Some(idx)
            } else {
                None
            }
        });
    for sentiel_pos in sentiel_indices {
        let name_split_pos = executable_buffer[..sentiel_pos]
            .iter()
            .rposition(|byte| *byte == b'|')
            .context("Not name len found before the sentinel")?;
        let name_len_str = from_utf8(&executable_buffer[name_split_pos + 1..sentiel_pos])
            .context("name len is not utf8")?;
        let name_len: usize = name_len_str.parse().context("name len is not a number")?;
        let name_pos = name_split_pos - name_len;
        let name =
            from_utf8(&executable_buffer[name_pos..name_split_pos]).context("name is not utf8")?;
        let id_pos = name_pos - size_of::<u16>();
        let id_bytes: [u8; size_of::<u16>()] =
            executable_buffer[id_pos..name_pos].try_into().unwrap();
        let id = u16::from_be_bytes(id_bytes);
        match (resources.entry(name), NonZeroU16::new(id)) {
            (Entry::Vacant(vacant), id) => {
                vacant.insert(if let Some(id) = id {
                    ResourceEntry::Exists { id }
                } else {
                    ResourceEntry::None {
                        id_positions: vec![id_pos],
                    }
                });
            }
            (Entry::Occupied(mut occupied), id) => match (occupied.get_mut(), id) {
                (ResourceEntry::Exists { id: existing_id }, id) => {
                    if id != Some(*existing_id) {
                        anyhow::bail!("ids aren't the same ({existing_id} != {id:?}) between different sentiels of resource {name}");
                    }
                }
                (ResourceEntry::None { .. }, Some(id)) => {
                    anyhow::bail!("ids aren't the same (None != {id}) between different sentiels of resource {name}");
                }
                (ResourceEntry::None { id_positions }, None) => {
                    id_positions.push(id_pos);
                }
            },
        }
    }
    Ok(resources)
}

pub fn list(executable_buffer: &[u8]) -> anyhow::Result<Vec<(String, bool)>> {
    let resources = list_for_injection(&executable_buffer)?;

    Ok(resources
        .into_iter()
        .map(|(name, entry)| {
            (
                name.to_string(),
                matches!(entry, ResourceEntry::Exists { .. }),
            )
        })
        .collect())
}

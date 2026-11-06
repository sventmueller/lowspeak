//! Removes the free space AudioToolbox reserves in front of the audio data of an M4A file.

use anyhow::{Context as _, Result, ensure};

/// `moov`/`trak`/`mdia`/`minf`/`stbl`; anything deeper is not an M4A from AudioToolbox.
const MAX_DEPTH: usize = 5;

struct Atom {
    start: usize,
    header: usize,
    size: usize,
    kind: [u8; 4],
}

fn read_u32(data: &[u8], at: usize) -> Result<u32> {
    let bytes = data.get(at..at + 4).context("truncated MP4 atom")?;
    Ok(u32::from_be_bytes(bytes.try_into()?))
}

fn read_u64(data: &[u8], at: usize) -> Result<u64> {
    let bytes = data.get(at..at + 8).context("truncated MP4 atom")?;
    Ok(u64::from_be_bytes(bytes.try_into()?))
}

fn atoms(data: &[u8], start: usize, end: usize) -> Result<Vec<Atom>> {
    let mut atoms = Vec::new();
    let mut offset = start;
    while offset + 8 <= end {
        let size32 = read_u32(data, offset)?;
        let kind: [u8; 4] = data[offset + 4..offset + 8].try_into()?;
        let (header, size) = match size32 {
            0 => (8, end - offset),
            1 => (16, usize::try_from(read_u64(data, offset + 8)?)?),
            size => (8, usize::try_from(size)?),
        };
        ensure!(
            size >= header
                && offset
                    .checked_add(size)
                    .is_some_and(|atom_end| atom_end <= end),
            "invalid MP4 atom size"
        );
        atoms.push(Atom {
            start: offset,
            header,
            size,
            kind,
        });
        offset += size;
    }
    Ok(atoms)
}

fn is_padding(atom: &Atom) -> bool {
    &atom.kind == b"free" || &atom.kind == b"skip"
}

/// Positions of the `stco`/`co64` chunk offset tables inside `moov`.
fn chunk_offset_tables(
    data: &[u8],
    container: &Atom,
    depth: usize,
    tables: &mut Vec<(usize, bool)>,
) -> Result<()> {
    ensure!(depth <= MAX_DEPTH, "MP4 containers nested too deeply");
    for atom in atoms(
        data,
        container.start + container.header,
        container.start + container.size,
    )? {
        match &atom.kind {
            b"trak" | b"mdia" | b"minf" | b"stbl" => {
                chunk_offset_tables(data, &atom, depth + 1, tables)?;
            }
            b"stco" => tables.push((atom.start + atom.header, false)),
            b"co64" => tables.push((atom.start + atom.header, true)),
            _ => {}
        }
    }
    Ok(())
}

/// Drops padding atoms before `mdat` and moves the chunk offsets accordingly.
pub fn without_padding(data: &[u8]) -> Result<Vec<u8>> {
    let top = atoms(data, 0, data.len())?;
    let mdat = top
        .iter()
        .position(|atom| &atom.kind == b"mdat")
        .context("M4A without audio data")?;
    let removed: usize = top[..mdat]
        .iter()
        .filter(|atom| is_padding(atom))
        .map(|atom| atom.size)
        .sum();
    if removed == 0 {
        return Ok(data.to_vec());
    }
    let mut out = Vec::with_capacity(data.len() - removed);
    for (index, atom) in top.iter().enumerate() {
        if index < mdat && is_padding(atom) {
            continue;
        }
        out.extend_from_slice(&data[atom.start..atom.start + atom.size]);
    }

    let top = atoms(&out, 0, out.len())?;
    let moov = top
        .iter()
        .find(|atom| &atom.kind == b"moov")
        .context("M4A without moov atom")?;
    let mut tables = Vec::new();
    chunk_offset_tables(&out, moov, 1, &mut tables)?;
    let removed_wide = u64::try_from(removed)?;
    let removed_narrow = u32::try_from(removed)?;
    for (table, wide) in tables {
        // Full-box header (version and flags), then the entry count.
        let count = usize::try_from(read_u32(&out, table + 4)?)?;
        let entries = table + 8;
        for index in 0..count {
            if wide {
                let at = entries + index * 8;
                let offset = read_u64(&out, at)?
                    .checked_sub(removed_wide)
                    .context("chunk offset before padding")?;
                out[at..at + 8].copy_from_slice(&offset.to_be_bytes());
            } else {
                let at = entries + index * 4;
                let offset = read_u32(&out, at)?
                    .checked_sub(removed_narrow)
                    .context("chunk offset before padding")?;
                out[at..at + 4].copy_from_slice(&offset.to_be_bytes());
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    fn atom(kind: &[u8; 4], body: &[u8]) -> Vec<u8> {
        let size = u32::try_from(body.len() + 8).unwrap();
        [&size.to_be_bytes()[..], kind, body].concat()
    }

    /// An M4A layout as AudioToolbox writes it: `ftyp`, padding, `moov` with a chunk offset table,
    /// then `mdat` with the chunks.
    fn m4a(padding: usize, chunks: &[Vec<u8>], wide: bool) -> Vec<u8> {
        let ftyp = atom(b"ftyp", b"M4A \0\0\0\0");
        let free = atom(b"free", &vec![0; padding]);
        let entry = if wide { 8 } else { 4 };
        let table_size = 8 + 8 + chunks.len() * entry;
        let moov_size = 5 * 8 + table_size;
        let mut offset = ftyp.len() + free.len() + moov_size + 8;
        let mut table = vec![0, 0, 0, 0];
        table.extend(u32::try_from(chunks.len()).unwrap().to_be_bytes());
        for chunk in chunks {
            if wide {
                table.extend(u64::try_from(offset).unwrap().to_be_bytes());
            } else {
                table.extend(u32::try_from(offset).unwrap().to_be_bytes());
            }
            offset += chunk.len();
        }
        let stco = atom(if wide { b"co64" } else { b"stco" }, &table);
        let moov = atom(
            b"moov",
            &atom(
                b"trak",
                &atom(b"mdia", &atom(b"minf", &atom(b"stbl", &stco))),
            ),
        );
        assert_eq!(moov.len(), moov_size);
        [ftyp, free, moov, atom(b"mdat", &chunks.concat())].concat()
    }

    fn chunk_offsets(file: &[u8]) -> Vec<usize> {
        let top = atoms(file, 0, file.len()).unwrap();
        let moov = top.iter().find(|atom| &atom.kind == b"moov").unwrap();
        let mut tables = Vec::new();
        chunk_offset_tables(file, moov, 1, &mut tables).unwrap();
        let (table, wide) = tables[0];
        let count = usize::try_from(read_u32(file, table + 4).unwrap()).unwrap();
        (0..count)
            .map(|index| {
                let at = table + 8 + index * if wide { 8 } else { 4 };
                if wide {
                    usize::try_from(read_u64(file, at).unwrap()).unwrap()
                } else {
                    usize::try_from(read_u32(file, at).unwrap()).unwrap()
                }
            })
            .collect()
    }

    proptest! {
        #[test]
        fn removing_padding_keeps_every_chunk_reachable(
            padding in 0usize..4_000,
            chunks in prop::collection::vec(prop::collection::vec(any::<u8>(), 1..64), 1..8),
            wide in any::<bool>(),
        ) {
            let file = m4a(padding, &chunks, wide);
            let out = without_padding(&file).unwrap();
            prop_assert_eq!(out.len(), file.len() - padding - 8);
            for (offset, chunk) in chunk_offsets(&out).into_iter().zip(&chunks) {
                prop_assert_eq!(&out[offset..offset + chunk.len()], &chunk[..]);
            }
        }

        #[test]
        fn damaged_files_are_rejected_without_panicking(
            padding in 0usize..64,
            flips in prop::collection::vec((any::<prop::sample::Index>(), any::<u8>()), 1..8),
        ) {
            let mut file = m4a(padding, &[vec![1, 2, 3], vec![4, 5]], false);
            for (index, value) in flips {
                let at = index.index(file.len());
                file[at] = value;
            }
            let _ = without_padding(&file);
        }

        #[test]
        fn arbitrary_bytes_do_not_panic(bytes in prop::collection::vec(any::<u8>(), 0..512)) {
            let _ = without_padding(&bytes);
        }
    }

    #[test]
    fn a_huge_extended_size_is_rejected() {
        let mut file = atom(b"ftyp", b"M4A ");
        file.extend(1u32.to_be_bytes());
        file.extend(b"free");
        file.extend((u64::MAX - 4).to_be_bytes());
        assert!(without_padding(&file).is_err());
    }
}

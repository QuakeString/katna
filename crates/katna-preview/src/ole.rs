// SPDX-License-Identifier: GPL-3.0-or-later

//! OLE compound files, the container of Office 97–2003 documents: a small
//! file system of named streams. Word keeps its text in "WordDocument",
//! PowerPoint in "PowerPoint Document".

use std::io::{Cursor, Read};

use cfb::CompoundFile;

/// No stream larger than this is read.
const MAX_STREAM_BYTES: u64 = 256 * 1024 * 1024;

/// The first bytes of every compound file.
const MAGIC: [u8; 8] = [0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1];

pub(crate) type Ole = CompoundFile<Cursor<Vec<u8>>>;

pub(crate) fn is_ole(bytes: &[u8]) -> bool {
    bytes.starts_with(&MAGIC)
}

pub(crate) fn open(bytes: Vec<u8>) -> Option<Ole> {
    if !is_ole(&bytes) {
        return None;
    }
    CompoundFile::open(Cursor::new(bytes)).ok()
}

/// Stream `name` of the root storage, if it is there.
pub(crate) fn stream(ole: &mut Ole, name: &str) -> Option<Vec<u8>> {
    let stream = ole.open_stream(format!("/{name}")).ok()?;
    let mut out = Vec::new();
    stream.take(MAX_STREAM_BYTES).read_to_end(&mut out).ok()?;
    Some(out)
}

pub(crate) fn u8_at(bytes: &[u8], at: usize) -> Option<u8> {
    bytes.get(at).copied()
}

pub(crate) fn u16_at(bytes: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(bytes.get(at..at + 2)?.try_into().ok()?))
}

pub(crate) fn u32_at(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

/// Windows-1252 text, as older Office files store 8-bit characters.
pub(crate) fn cp1252(byte: u8) -> char {
    const HIGH: [u16; 32] = [
        0x20ac, 0x81, 0x201a, 0x0192, 0x201e, 0x2026, 0x2020, 0x2021, 0x02c6, 0x2030, 0x0160,
        0x2039, 0x0152, 0x8d, 0x017d, 0x8f, 0x90, 0x2018, 0x2019, 0x201c, 0x201d, 0x2022, 0x2013,
        0x2014, 0x02dc, 0x2122, 0x0161, 0x203a, 0x0153, 0x9d, 0x017e, 0x0178,
    ];
    match byte {
        0x80..=0x9f => char::from_u32(u32::from(HIGH[usize::from(byte - 0x80)])).unwrap_or('?'),
        _ => char::from(byte),
    }
}

/// Symbol-font characters (U+F000 to U+F0FF) that Office uses for
/// bullets, as ordinary characters.
pub(crate) fn symbol(c: char) -> char {
    match u32::from(c) {
        0xf0a7 | 0xf06e => '▪',
        0xf0d8 | 0xf0e0 => '➢',
        0xf0fc => '✓',
        0xf06f => '◦',
        0xf000..=0xf0ff => '•',
        _ => c,
    }
}

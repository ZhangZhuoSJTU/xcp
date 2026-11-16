/*
 * Copyright © 2018, Steve Smith <tarkasteve@gmail.com>
 *
 * Licensed under the Business Source License 1.1 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License in the COPYING file at the
 * root of this repository.
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 */

mod common;
mod errors;

use std::{fs, ops::Range};

use cfg_if::cfg_if;
use rustix::fs::FileTypeExt;

cfg_if! {
    if #[cfg(all(target_os = "linux", feature = "use_linux"))] {
        mod linux;
        use linux as backend;
    } else {
        mod fallback;
        use fallback as backend;
    }
}
pub use backend::{
    copy_file_bytes,
    copy_file_offset,
    copy_node,
    copy_sparse,
    file_physical_size,
    file_block_count,
    file_device_id,
    file_group,
    file_inode,
    file_owner,
    link_count,
    map_extents,
    next_sparse_segments,
    preferred_io_size,
    probably_sparse,
    reflink,
};
pub use common::{
    allocate_file,
    copy_file,
    copy_owner,
    copy_permissions,
    copy_timestamps,
    is_same_file,
    merge_extents,
    sync,
};
pub use errors::Error;

/// Flag whether the current OS support
/// [xattrs](https://man7.org/linux/man-pages/man7/xattr.7.html).
pub const XATTR_SUPPORTED: bool = {
    // NOTE: The xattr crate has a SUPPORTED_PLATFORM flag, however it
    // allows NetBSD, which fails for us, so we stick to platforms we've
    // tested.
    cfg_if! {
        if #[cfg(any(target_os = "linux", target_os = "freebsd"))] {
            true
        } else {
            false
        }
    }
};

/// Enum mapping for various *nix file types. Mapped from
/// [std::fs::FileType] and [rustix::fs::FileTypeExt].
#[derive(Debug)]
pub enum FileType {
    File,
    Dir,
    Symlink,
    Socket,
    Fifo,
    Char,
    Block,
    Other
}

impl From<fs::FileType> for FileType {
    fn from(ft: fs::FileType) -> Self {
        if ft.is_dir() {
            FileType::Dir
        } else if ft.is_file() {
            FileType::File
        } else if ft.is_symlink() {
            FileType::Symlink
        } else if ft.is_socket() {
            FileType::Socket
        } else if ft.is_fifo() {
            FileType::Fifo
        } else if ft.is_char_device() {
            FileType::Char
        } else if ft.is_block_device() {
            FileType::Block
        } else {
            FileType::Other
        }
    }
}

/// Struct representing a file extent metadata.
#[derive(Debug, PartialEq)]
pub struct Extent {
    /// Extent logical start
    pub start: u64,
    /// Extent logical end
    pub end: u64,
    /// Whether extent is shared between multiple file. This generally
    /// only applies to reflinked files on filesystems that support
    /// CoW.
    pub shared: bool,
}

impl From<Extent> for Range<u64> {
    fn from(e: Extent) -> Self {
        e.start..e.end
    }
}

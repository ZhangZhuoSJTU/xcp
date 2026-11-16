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

use std::fs::File;
use std::path::Path;

use log::warn;

use crate::Extent;
use crate::common::{copy_bytes_uspace, copy_range_uspace};
use crate::errors::{Result, Error};

pub fn copy_file_bytes(infd: &File, outfd: &File, bytes: u64) -> Result<usize> {
    copy_bytes_uspace(infd, outfd, bytes as usize)
}

pub fn copy_file_offset(infd: &File, outfd: &File, bytes: u64, off: i64) -> Result<usize> {
    copy_range_uspace(infd, outfd, bytes as usize, off as usize)
}

// No sparse file handling by default, needs to be implemented
// per-OS. This effectively disables the following operations.
pub fn probably_sparse(_fd: &File) -> Result<bool> {
    Ok(false)
}

pub fn file_physical_size(fd: &File) -> Result<u64> {
    Ok(fd.metadata()?.len())
}

pub fn preferred_io_size(_fd: &File) -> Result<u64> {
    Ok(4096)
}

pub fn file_block_count(fd: &File) -> Result<u64> {
    Ok(fd.metadata()?.len() / 512)
}

pub fn file_device_id(_fd: &File) -> Result<u64> {
    Ok(0)
}

pub fn file_inode(_fd: &File) -> Result<u64> {
    Ok(0)
}

pub fn link_count(_fd: &File) -> Result<u64> {
    Ok(1)
}

pub fn file_owner(_fd: &File) -> Result<u32> {
    Ok(0)
}

pub fn file_group(_fd: &File) -> Result<u32> {
    Ok(0)
}

pub fn map_extents(_fd: &File) -> Result<Option<Vec<Extent>>> {
    // FIXME: Implement for *BSD with lseek?
    Ok(None)
}

pub fn next_sparse_segments(_infd: &File, _outfd: &File, _pos: u64) -> Result<(u64, u64)> {
    // FIXME: Implement for *BSD with lseek?
    Err(Error::UnsupportedOperation {})
}

pub fn copy_sparse(infd: &File, outfd: &File) -> Result<u64> {
    let len = infd.metadata()?.len();
    copy_file_bytes(&infd, &outfd, len)
        .map(|i| i as u64)
}

pub fn copy_node(src: &Path, _dest: &Path) -> Result<()> {
    // FreeBSD `cp` just warns about this, so do the same here.
    warn!("Socket copy not supported by this OS: {}", src.to_string_lossy());
    Ok(())
}

pub fn reflink(_infd: &File, _outfd: &File) -> Result<bool> {
    Ok(false)
}

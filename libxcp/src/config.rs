/*
 * Copyright © 2024, Steve Smith <tarkasteve@gmail.com>
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

//! Driver configuration support.

use std::result;
use std::str::FromStr;

use crate::errors::XcpError;

/// Enum defining configuration options for handling
/// [reflinks](https://btrfs.readthedocs.io/en/latest/Reflink.html). [FromStr]
/// is supported.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Reflink {
    /// Attempt to reflink and fallback to a copy if it is not
    /// possible.
    #[default]
    Auto,
    /// Always attempt a reflink; return an error if not supported.
    Always,
    /// Always perform a full data copy. Note: when using Linux
    /// accelerated copy operations (the default when available) the
    /// kernel may choose to reflink rather than perform a fully copy
    /// regardless of this setting.
    Never,
}

// String conversion helper as a convenience for command-line parsing.
impl FromStr for Reflink {
    type Err = XcpError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "always" => Ok(Reflink::Always),
            "auto" => Ok(Reflink::Auto),
            "never" => Ok(Reflink::Never),
            _ => Err(XcpError::InvalidArguments(format!("Unexpected value for 'reflink': {s}"))),
        }
    }
}

/// Enum defining configuration options for handling backups of
/// overwritten files. [FromStr] is supported.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Backup {
    /// Do not create backups.
    None,
    /// Create a numbered backup if a previous backup exists.
    Auto,
    /// Create numbered backups. Numbered backups follow the semantics
    /// of `cp` numbered backups (e.g. `file.txt.~123~`).
    Numbered,
}

impl FromStr for Backup {
    type Err = XcpError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" | "off" => Ok(Backup::None),
            "auto" => Ok(Backup::Auto),
            "numbered" => Ok(Backup::Numbered),
            _ => Err(XcpError::InvalidArguments(format!("Unexpected value for 'backup': {s}"))),
        }
    }
}

/// A structure defining the runtime options for copy-drivers. This
/// would normally be passed to `load_driver()`.
#[derive(Clone, Debug)]
pub struct Config {
    /// Number of parallel workers. 0 means use the number of logical
    /// CPUs (the default).
    pub workers: usize,

    /// Block size for operations. Defaults to the full file size. Use
    /// a smaller value for finer-grained feedback.
    pub block_size: u64,

    /// Use .gitignore if present.
    ///
    /// NOTE: This is fairly basic at the moment, and only honours a
    /// .gitignore in the directory root for each source directory;
    /// global or sub-directory ignores are skipped. Default is
    /// `false`.
    pub gitignore: bool,

    /// Do not overwrite existing files. Default is `false`.
    pub no_clobber: bool,

    /// Do not copy the file permissions. Default is `false`.
    pub no_perms: bool,

    /// Do not copy the file permissions. Default is `false`.
    pub no_timestamps: bool,

    /// Copy ownership.
    ///
    /// Whether to copy ownship (user/group).  This option requires
    /// root permissions or appropriate capabilities; if the attempt
    /// to copy ownership fails a warning is issued but the operation
    /// continues.
    pub ownership: bool,

    /// Dereference symlinks. Default is `false`.
    pub dereference: bool,

    /// Target should not be a directory.
    ///
    /// Analogous to cp's no-target-directory. Expected behavior is that when
    /// copying a directory to another directory, instead of creating a sub-folder
    /// in target, overwrite target. Default is 'false`.
    pub no_target_directory: bool,

    /// Sync each file to disk after writing. Default is `false`.
    pub fsync: bool,

    /// Reflink options.
    ///
    /// Whether and how to use reflinks. 'auto' (the default) will
    /// attempt to reflink and fallback to a copy if it is not
    /// possible, 'always' will return an error if it cannot reflink,
    /// and 'never' will always perform a full data copy.
    pub reflink: Reflink,

    /// Backup options
    ///
    /// Whether to create backups of overwritten files. Current
    /// options are `None` or 'Numbered'. Numbered backups follow the
    /// semantics of `cp` numbered backups
    /// (e.g. `file.txt.~123~`). Default is `None`.
    pub backup: Backup,
}

impl Config {
    pub(crate) fn num_workers(&self) -> usize {
        if self.workers == 0 {
            num_cpus::get()
        } else {
            self.workers
        }
    }

    /// Returns a buffer size scaled to the file length and worker count.
    /// Avoids oversized allocations when copying many small files.
    pub fn scaled_buf_size(&self, len: u64) -> u64 {
        len.min(self.block_size) / self.num_workers() as u64
    }

    /// Select dispatch block size for extent-parallel sparse copying.
    /// Maps the file's data-density ratio to one of eight granularity
    /// tiers so that sparser files get finer blocks (better hole-skip
    /// parallelism) while denser files get coarser blocks (less
    /// scheduling overhead).
    ///
    /// Only meaningful for files confirmed sparse by `probably_sparse`.
    pub fn extent_block_size(&self, file_len: u64, physical: u64) -> u64 {
        const TIERS: [u64; 8] = [
            1 << 16, 1 << 17, 1 << 18, 1 << 19,
            1 << 20, 1 << 21, 1 << 22, 1 << 23,
        ];
        let tier = (physical / (file_len >> 3).max(1)) as usize;
        TIERS[tier]
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            workers: num_cpus::get(),
            block_size: u64::MAX,
            gitignore: false,
            no_clobber: false,
            no_perms: false,
            no_timestamps: false,
            ownership: false,
            dereference: false,
            no_target_directory: false,
            fsync: false,
            reflink: Reflink::Auto,
            backup: Backup::None,
        }
    }
}

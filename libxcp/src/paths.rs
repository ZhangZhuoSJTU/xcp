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

use std::path::Path;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use log::info;
use walkdir::DirEntry;

use crate::config::Config;
use crate::errors::Result;

/// Parse a git ignore file.
pub fn parse_ignore(source: &Path, config: &Config) -> Result<Option<Gitignore>> {
    let gitignore = if config.gitignore {
        let gifile = source.join(".gitignore");
        info!("Using .gitignore file {gifile:?}");
        let mut builder = GitignoreBuilder::new(source);
        builder.add(&gifile);
        let ignore = builder.build()?;
        Some(ignore)
    } else {
        None
    };
    Ok(gitignore)
}

/// Filter to return whether a given file should be ignored by a
/// filter file.
pub fn ignore_filter(entry: &DirEntry, ignore: &Option<Gitignore>) -> bool {
    match ignore {
        None => true,
        Some(gi) => {
            let path = entry.path();
            let m = gi.matched(path, path.is_dir());
            !m.is_ignore()
        }
    }
}

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

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid source: {0}")]
    InvalidSource(&'static str),

    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    OSError(#[from] rustix::io::Errno),

    #[error("Unsupported operation; this function should never be called on this OS.")]
    UnsupportedOperation,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

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

//! Custom error types.

use std::path::PathBuf;

pub use anyhow::Result;

#[derive(Debug, thiserror::Error)]
pub enum XcpError {
    #[error("Error during copy: {0}")]
    CopyError(String),

    #[error("Destination Exists: {0}, {1}")]
    DestinationExists(&'static str, PathBuf),

    #[error("Early shutdown: {0}")]
    EarlyShutdown(&'static str),

    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    #[error("Invalid destination: {0}")]
    InvalidDestination(&'static str),

    #[error("Invalid source: {0}")]
    InvalidSource(&'static str),

    #[error("Failed to reflink file and 'always' was specified: {0}")]
    ReflinkFailed(String),

    #[error("Unknown driver: {0}")]
    UnknownDriver(String),

    #[error("Unknown file-type: {0}")]
    UnknownFileType(PathBuf),

    #[error("Unsupported OS")]
    UnsupportedOS(&'static str),
}

/*
 * Copyright © 2018-2019, Steve Smith <tarkasteve@gmail.com>
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

//! Support for pluggable copy drivers.
//!
//! Two drivers are currently supported:
//! * `parfile`: Parallelise copying at the file level. This can improve
//!   speed on modern NVME devices, but can bottleneck on larger files.
//! * `parblock`: Parallelise copying at the block level. Block-size is
//!   configurable. This can have better performance for large files,
//!   but has a higher overhead.
//!
//! Drivers are configured with the [Config] struct. A convenience
//! function [load_driver()] is provided to load a dynamic-dispatched
//! instance of each driver.
//!
//! # Example
//!
//! See the example in top-level module.

pub mod parfile;
#[cfg(feature = "parblock")]
pub mod parblock;

use std::path::{Path, PathBuf};
use std::result;
use std::str::FromStr;
use std::sync::Arc;

use crate::config::Config;
use crate::errors::{Result, XcpError};
use crate::feedback::StatusUpdater;

/// The trait specifying driver operations; drivers should implement
/// this.
pub trait CopyDriver {
    /// Recursively copy a set of directories or files to a
    /// destination. `dest` can be a file if a single file is provided
    /// the source.  `StatusUpdater.send()` will be called with
    /// `StatusUpdate` objects depending on the driver configuration.
    /// `copy()` itself will block until all work is complete, so
    /// should be run in a thread if real-time updates are required.
    fn copy(&self, sources: Vec<PathBuf>, dest: &Path, stats: Arc<dyn StatusUpdater>) -> Result<()>;
}

/// An enum specifing the driver to use. This is just a helper for
/// applications to use with [load_driver()]. [FromStr] is implemented
/// to help with this.
#[derive(Debug, Clone, Copy)]
pub enum Drivers {
    ParFile,
    #[cfg(feature = "parblock")]
    ParBlock,
}

// String conversion helper as a convenience for command-line parsing.
impl FromStr for Drivers {
    type Err = XcpError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "parfile" => Ok(Drivers::ParFile),
            #[cfg(feature = "parblock")]
            "parblock" => Ok(Drivers::ParBlock),
            _ => Err(XcpError::UnknownDriver(s.to_owned())),
        }
    }
}

/// Load and configure the given driver.
pub fn load_driver(driver: Drivers, config: &Arc<Config>) -> Result<Box<dyn CopyDriver + Send>> {
    let driver_impl: Box<dyn CopyDriver + Send> = match driver {
        Drivers::ParFile => Box::new(parfile::Driver::new(config.clone())?),
        #[cfg(feature = "parblock")]
        Drivers::ParBlock => Box::new(parblock::Driver::new(config.clone())?),
    };

    Ok(driver_impl)
}

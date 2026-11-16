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

//! Support for runtime feedback of copy progress.
//!

//! Users of `libxcp` can implement the [StatusUpdater] trait and pass
//! an instance to the driver, usually using `load_driver()`. Two
//! implementations are provided:
//!
//! * [NoopUpdater]
//! * [ChannelUpdater]

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use crossbeam_channel as cbc;

use crate::config::Config;
use crate::errors::{Result, XcpError};

/// A struct representing an updated status.
#[derive(Debug)]
pub enum StatusUpdate {
    /// An update representing a successful copy of bytes between
    /// files.
    Copied(u64),
    /// An update representing that this number of bytes will need to be copied.
    Size(u64),
    /// An error during a copy operation.
    Error(XcpError)
}

pub trait StatusUpdater: Sync + Send {
    fn send(&self, update: StatusUpdate) -> Result<()>;
}

/// An implementation of [StatusUpdater] which will return
/// [StatusUpdate] objects via a channel. On copy completion the
/// channel will be closed, allowing the caller to iterator over
/// returned updates. See the top-level module for an example of
/// usage.
pub struct ChannelUpdater {
    chan_tx: cbc::Sender<StatusUpdate>,
    chan_rx: cbc::Receiver<StatusUpdate>,
    config: Arc<Config>,
    sent: AtomicU64,
}

impl ChannelUpdater {
    /// Create a new ChannelUpdater, including the channels.
    pub fn new(config: &Arc<Config>) -> ChannelUpdater {
        let (chan_tx, chan_rx) = cbc::unbounded();
        ChannelUpdater {
            chan_tx,
            chan_rx,
            config: config.clone(),
            sent: AtomicU64::new(0),
        }
    }

    /// Retrieve a clone of the receive end of the update
    /// channel. Note: As ChannelUpdater is consumed by the driver
    /// call you should call this before then; e.g:
    ///
    ///     # use std::sync::Arc;
    ///     use libxcp::config::Config;
    ///     use libxcp::feedback::{ChannelUpdater, StatusUpdater};
    ///
    ///     let config = Arc::new(Config::default());
    ///     let updater = ChannelUpdater::new(&config);
    ///     let stat_rx = updater.rx_channel();
    ///     let stats: Arc<dyn StatusUpdater> = Arc::new(updater);
    pub fn rx_channel(&self) -> cbc::Receiver<StatusUpdate> {
        self.chan_rx.clone()
    }
}

impl StatusUpdater for ChannelUpdater {
    // Wrapper around channel-send that groups updates together
    fn send(&self, update: StatusUpdate) -> Result<()> {
        if let StatusUpdate::Copied(bytes) = update {
            // Avoid saturating the queue with small writes
            let bsize = self.config.block_size;
            let prev_written = self.sent.fetch_add(bytes, Ordering::Relaxed);
            if ((prev_written + bytes) / bsize) > (prev_written / bsize) {
                self.chan_tx.send(update)?;
            }
        } else {
            self.chan_tx.send(update)?;
        }
        Ok(())
    }
}

/// A null updater for when no feedback is required.
pub struct NoopUpdater;

impl StatusUpdater for NoopUpdater {
    fn send(&self, _update: StatusUpdate) -> Result<()> {
        Ok(())
    }
}

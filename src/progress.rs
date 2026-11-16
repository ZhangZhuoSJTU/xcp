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

use crate::options::Opts;

use libxcp::errors::Result;
use terminal_size::Width;

struct NoopBar;

struct VisualBar {
    bar: indicatif::ProgressBar,
}

pub trait ProgressBar {
    #[allow(unused)]
    fn set_size(&self, size: u64);
    fn inc_size(&self, size: u64);
    fn inc(&self, size: u64);
    fn end(&self);
}


impl ProgressBar for NoopBar {
    fn set_size(&self, _size: u64) {
    }
    fn inc_size(&self, _size: u64) {
    }
    fn inc(&self, _size: u64) {
    }
    fn end(&self) {
    }
}

impl ProgressBar for VisualBar {
    fn set_size(&self, size: u64) {
        self.bar.set_length(size);
    }

    fn inc_size(&self, size: u64) {
        self.bar.inc_length(size);
    }

    fn inc(&self, size: u64) {
        self.bar.inc(size);
    }

    fn end(&self) {
        self.bar.finish();
    }
}

impl VisualBar {
    fn new(size: u64) -> Result<Self> {
        let bar = indicatif::ProgressBar::new(size).with_style(
            indicatif::ProgressStyle::default_bar()
                .template(
		    match terminal_size::terminal_size() {
			Some((Width(width), _)) if width < 160 => "[{wide_bar:.cyan/blue}]\n{bytes:>11} / {total_bytes:<11} | {percent:>3}% | {bytes_per_sec:^13} | {eta_precise} remaining",
			_ => "[{wide_bar:.cyan/blue}] {bytes:>11} / {total_bytes:<11} | {percent:>3}% | {bytes_per_sec:^13} | {eta_precise} remaining",
		    }
		)?
                .progress_chars("#>-"),
        );
        Ok(Self { bar })
    }
}

pub fn create_bar(opts: &Opts, size: u64) -> Result<Box<dyn ProgressBar>> {
    if opts.no_progress {
        Ok(Box::new(NoopBar {}))
    } else {
        Ok(Box::new(VisualBar::new(size)?))
    }
}

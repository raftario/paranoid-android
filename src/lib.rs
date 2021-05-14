//! Integration layer between [`tracing`] and Android logs.
//!
//! ## Usage
//!
//! ```rust
//! use tracing_subscriber::fmt::format::FmtSpan;
//! use tracing_subscriber::filter::EnvFilter;
//! use tracing_subscriber::prelude::*;
//!
//! // Use the crate name as the log tag
//! let tag = env!("CARGO_PKG_NAME");
//!
//! // Use the uppercased tag followed by `_LOG` as the filtering environment variable
//! let env = format!("{}_LOG", tag.to_uppercase());
//!
//! let subscriber = tracing_android::subscriber(tag) // Create a new subscriber
//!     .with_span_events(FmtSpan::CLOSE) // log events indicating the time spent in spans when they are closed
//!     .collector() // convert the subscriber into a collector to compose it with the wider `tracing` ecosystem
//!     .with(EnvFilter::from_env(env)) // filter logs based on the contents of an environment variable
//!     .init(); // register the collector globally and start logging !
//! ```

#![warn(rust_2018_idioms, missing_debug_implementations, missing_docs)]

mod logging;
mod subscriber;
mod writer;

#[cfg(feature = "reexport")]
pub use tracing;
#[cfg(feature = "reexport")]
pub use tracing_subscriber;

use tracing_core::Collect;
use tracing_subscriber::registry::LookupSpan;

pub use self::{
    logging::Buffer,
    subscriber::{Collector, Subscriber},
};

/// Returns a new [`Subscriber`] with the given tag and the default
/// configuration.
pub fn subscriber<C>(tag: impl AsRef<[u8]>) -> Subscriber<C>
where
    C: Collect + for<'a> LookupSpan<'a>,
{
    Subscriber::new(tag)
}

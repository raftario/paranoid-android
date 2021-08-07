//! Integration layer between `tracing` and Android logs.
//!
//! This crate provides a [`MakeWriter`](tracing_subscriber::fmt::MakeWriter)
//! and a [`Layer`](tracing_subscriber::Layer) suitable for writing Android
//! logs.
//!
//! ## Usage
//!
//! ```rust
//! tracing_android::init(env!("CARGO_PKG_NAME"));
//! ```
//!
//! or in with custom options and combined with other layers
//!
//! ```rust
//! # let other_layer = tracing_android::layer("other");
//! #
//! use tracing_subscriber::Registry;
//! use tracing_subscriber::fmt::FmtSpan;
//! use tracing_subscriber::prelude::*;
//!
//! let android_layer = tracing_android::layer(env!("CARGO_PKG_NAME"))
//!     .with_span_events(FmtSpan::CLOSE)
//!     .with_thread_names(true);
//!
//! let registry = Registry::default()
//!     .with(android_layer)
//!     .with(other_layer);
//!
//! # registry.init();
//! ```
//!
//! ## Cargo features
//!
//! * `api-30`: Enables support for Android API level 30 and source location
//!   information
//! * `json`: Enables support for the JSON log format
//! * `log`: Enables support for the JSON log format

#![warn(rust_2018_idioms, missing_debug_implementations, missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_root_url = "https://raftario.github.io/tracing-android/tracing-android")]

mod layer;
mod logging;
mod writer;

use tracing_subscriber::registry;

pub use self::{
    layer::{Layer, Subscriber},
    logging::Buffer,
    writer::{AndroidLogMakeWriter, AndroidLogWriter},
};

/// Returns a new [formatting layer](Layer) that can be
/// [composed](tracing_subscriber::Layer) with other layers to construct a
/// [`Subscriber`](tracing_core::Subscriber).
///
/// This is a shorthand for the equivalent [`Layer::new`] function.
pub fn layer<S>(tag: impl ToString) -> Layer<S>
where
    S: tracing_core::Subscriber + for<'a> registry::LookupSpan<'a>,
{
    Layer::new(tag)
}

/// Creates a [`Subscriber`] with the given tag and attempts to set it as the
/// [global default subscriber] in the current scope, panicking if this fails.
///
/// This is shorthand for
///
/// ```rust
/// tracing_android::layer(tag).init()
/// ```
///
/// [global default subscriber]: https://docs.rs/tracing/0.1/tracing/dispatcher/index.html#setting-the-default-subscriber
pub fn init(tag: impl ToString) {
    layer(tag).init();
}

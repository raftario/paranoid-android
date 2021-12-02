#![cfg(target_os = "android")]

//! Integration layer between `tracing` and Android logs.
//!
//! This crate provides a [`MakeWriter`](tracing_subscriber::fmt::MakeWriter) suitable for writing Android logs.
//!
//! It is designed as an integration with the [`fmt`](tracing_subscriber::fmt) subscriber from `tracing-subscriber`
//! and as such inherits all of its features and customization options.
//!
//! ## Usage
//!
//! ```rust
//! tracing_android::init(env!("CARGO_PKG_NAME"));
//! ```
//!
//! or with custom options and combined with other layers
//!
//! ```rust
//! # let other_layer = tracing_android::layer("other");
//! #
//! use tracing_subscriber::filter::LevelFilter;
//! use tracing_subscriber::fmt::FmtSpan;
//! use tracing_subscriber::prelude::*;
//!
//! let android_layer = tracing_android::layer(env!("CARGO_PKG_NAME"))
//!     .with_span_events(FmtSpan::CLOSE)
//!     .with_thread_names(true)
//!     .with_filter(LevelFilter::DEBUG);
//!
//! tracing_subcriber::registry()
//!     .with(android_layer)
//!     .with(other_layer)
//!     .init();
//! ```
//!
//! ## Cargo features
//!
//! * `api-30`: Enables support for Android API level 30 and source location information

#![warn(rust_2018_idioms, missing_debug_implementations, missing_docs)]

mod layer;
mod logging;
mod writer;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

pub use self::{
    layer::{layer, with_buffer, Layer},
    logging::Buffer,
    writer::{AndroidLogMakeWriter, AndroidLogWriter},
};

/// Creates a [`Subscriber`](tracing_core::Subscriber) with the given tag
/// and attempts to set it as the [global default subscriber] in the current scope, panicking if this fails.
///
/// [global default subscriber]: https://docs.rs/tracing/0.1/tracing/dispatcher/index.html#setting-the-default-subscriber
pub fn init(tag: impl ToString) {
    Registry::default().with(layer(tag)).init();
}

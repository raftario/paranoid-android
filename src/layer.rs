use tracing_core::Subscriber;
use tracing_subscriber::{
    fmt::{
        self,
        format::{self, Format},
    },
    registry::LookupSpan,
};

use crate::{AndroidLogMakeWriter, Buffer};

/// A [`Layer`](tracing_subscriber::Layer) that writes formatted representations of `tracing` events as Android logs.
pub type Layer<S, N = format::DefaultFields, E = format::Full> =
    fmt::Layer<S, N, format::Format<E, ()>, AndroidLogMakeWriter>;

/// Returns a new [formatting layer](Layer) with the given tag,
/// which can be [composed](tracing_subscriber::Layer) with other layers to construct a [`Subscriber`].
pub fn layer<S>(tag: impl ToString) -> Layer<S>
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    fmt::Layer::new()
        .event_format(default_format())
        .with_writer(AndroidLogMakeWriter::new(tag.to_string()))
}

/// Returns a new [formatting layer](Layer) with the given tag and using the given [Android log buffer](Buffer),
/// which can be [composed](tracing_subscriber::Layer) with other layers to construct a [`Subscriber`].
pub fn with_buffer<S>(tag: impl ToString, buffer: Buffer) -> Layer<S>
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    fmt::Layer::new()
        .event_format(default_format())
        .with_writer(AndroidLogMakeWriter::with_buffer(tag.to_string(), buffer))
}

fn default_format() -> Format<format::Full, ()> {
    Format::default().with_level(false).without_time()
}

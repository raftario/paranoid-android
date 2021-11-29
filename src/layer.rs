use tracing_subscriber::{
    fmt::{self, format},
    layer::{self, SubscriberExt as _},
    registry,
    util::SubscriberInitExt as _,
};

use crate::{AndroidLogMakeWriter, Buffer};

/// A [`Layer`](tracing_subscriber::Layer) that writes formatted representations
/// of `tracing` events as Android logs.
#[derive(Debug)]
pub struct Layer<S, N = format::DefaultFields, E = format::Full>(
    fmt::Layer<S, N, format::Format<E, ()>, AndroidLogMakeWriter>,
);

/// A [`Subscriber`](tracing_core::Subscriber) that writes formatted
/// representations of `tracing` events as Android logs.
///
/// This consists of a [`Layer`] wrapped in a [`Registry`](registry::Registry).
pub type Subscriber<N, E> = layer::Layered<
    fmt::Layer<registry::Registry, N, format::Format<E, ()>, AndroidLogMakeWriter>,
    registry::Registry,
>;

impl<S> Layer<S>
where
    S: tracing_core::Subscriber + for<'a> registry::LookupSpan<'a>,
{
    /// Returns a new [`Layer`] with the given tag.
    pub fn new(tag: impl ToString) -> Self {
        Self(
            fmt::Layer::new()
                .event_format(Self::default_format())
                .with_writer(AndroidLogMakeWriter::new(tag.to_string())),
        )
    }

    /// Returns the inner `tracing_suscriber`'s [`Layer`](tracing_subscriber::layer::Layer), in
    /// case wrapping it into a [`Registry`](registry::Registry) is undesired.
    pub fn inner(self) -> impl layer::Layer<S> {
        self.0
    }

    /// Returns a new [`Layer`] with the given tag and using the given [Android
    /// log buffer](Buffer).
    pub fn with_buffer(tag: impl ToString, buffer: Buffer) -> Self {
        Self(
            fmt::Layer::new()
                .event_format(Self::default_format())
                .with_writer(AndroidLogMakeWriter::with_buffer(tag.to_string(), buffer)),
        )
    }

    fn default_format() -> format::Format<format::Full, ()> {
        format::Format::default().with_level(false).without_time()
    }
}

impl<N, E> Layer<registry::Registry, N, E>
where
    fmt::Layer<registry::Registry, N, format::Format<E, ()>, AndroidLogMakeWriter>:
        layer::Layer<registry::Registry>,
{
    /// Returns a [`Subscriber`](tracing_core::Subscriber) by wrapping `self` in
    /// a [`Registry`](registry::Registry).
    pub fn subscriber(self) -> Subscriber<N, E> {
        let Self(inner) = self;
        registry::Registry::default().with(inner)
    }

    /// Converts `self` into a [`Subscriber`](tracing_core::Subscriber) and
    /// attempts to set it as the [global default subscriber] in the current
    /// scope, panicking if this fails.
    ///
    /// # Panics
    /// This method panics if a global default subscriber has already been set,
    /// or if a `log` logger has already been set (when the "log" feature is
    /// enabled).
    ///
    /// # `log` compatibility
    /// If the "log" feature flag is enabled, this will also attempt to
    /// initialize a [`log`] compatibility layer. This allows the subscriber to
    /// consume `log::Record`s as though they were `tracing` `Event`s.
    ///
    /// [global default subscriber]: https://docs.rs/tracing/0.1/tracing/dispatcher/index.html#setting-the-default-subscriber
    /// [`log`]: https://crates.io/log
    pub fn init(self)
    where
        Subscriber<N, E>: Into<tracing_core::Dispatch>,
    {
        self.subscriber().init()
    }
}

impl<S, N, E> Layer<S, N, E>
where
    N: for<'writer> fmt::FormatFields<'writer> + 'static,
{
    /// See [`tracing_subscriber` documentation](fmt::Layer::with_span_events)
    pub fn with_span_events(self, kind: format::FmtSpan) -> Self {
        let Self(inner) = self;
        let inner = inner.with_span_events(kind);
        Self(inner)
    }

    /// See [`tracing_subscriber` documentation](fmt::Layer::with_target)
    pub fn with_target(self, display_target: bool) -> Self {
        let Self(inner) = self;
        let inner = inner.with_target(display_target);
        Self(inner)
    }

    /// See [`tracing_subscriber` documentation](fmt::Layer::with_thread_ids)
    pub fn with_thread_ids(self, display_thread_ids: bool) -> Self {
        let Self(inner) = self;
        let inner = inner.with_thread_ids(display_thread_ids);
        Self(inner)
    }

    /// See [`tracing_subscriber` documentation](fmt::Layer::with_thread_names)
    pub fn with_thread_names(self, display_thread_names: bool) -> Self {
        let Self(inner) = self;
        let inner = inner.with_thread_names(display_thread_names);
        Self(inner)
    }

    /// See [`tracing_subscriber` documentation](fmt::Layer::compact)
    pub fn compact(self) -> Layer<S, N, format::Compact> {
        let Self(inner) = self;
        let inner = inner.compact();
        Layer(inner)
    }

    /// See [`tracing_subscriber` documentation](fmt::Layer::json)
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    pub fn json(self) -> Layer<S, format::JsonFields, format::Json> {
        let Self(inner) = self;
        let inner = inner.json();
        Layer(inner)
    }
}

#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
impl<S> Layer<S, format::JsonFields, format::Json> {
    /// See [`tracing_subscriber` documentation](fmt::Layer::flatten_event)
    pub fn flatten_event(self, flatten_event: bool) -> Self {
        let Self(inner) = self;
        let inner = inner.flatten_event(flatten_event);
        Self(inner)
    }

    /// See [`tracing_subscriber` documentation](fmt::Layer::with_current_span)
    pub fn with_current_span(self, display_current_span: bool) -> Self {
        let Self(inner) = self;
        let inner = inner.with_current_span(display_current_span);
        Self(inner)
    }

    /// See [`tracing_subscriber` documentation](fmt::Layer::with_span_list)
    pub fn with_span_list(self, display_span_list: bool) -> Self {
        let Self(inner) = self;
        let inner = inner.with_span_list(display_span_list);
        Self(inner)
    }
}

impl<S, N, E> Layer<S, N, E> {
    /// See [`tracing_subscriber` documentation](fmt::Layer::fmt_fields)
    pub fn fmt_fields<N2>(self, fmt_fields: N2) -> Layer<S, N2, E>
    where
        N2: for<'writer> fmt::FormatFields<'writer> + 'static,
    {
        let Self(inner) = self;
        let inner = inner.fmt_fields(fmt_fields);
        Layer(inner)
    }
}

use std::{any::TypeId, ptr::NonNull};

use tracing_core::{
    span::{Attributes, Id, Record},
    Collect, Event,
};
use tracing_subscriber::{
    fmt::{
        self,
        format::{self, FmtSpan},
        FormatEvent, FormatFields,
    },
    registry::LookupSpan,
    subscribe::{CollectExt, Context, Layered},
    util::SubscriberInitExt,
    Registry, Subscribe,
};

use crate::{logging::Buffer, writer::AndroidLogMakeWriter};

/// A [`Subscriber`] that logs formatted representations of `tracing` events to
/// the Android log.
///
/// ## Examples
///
/// Constructing a subscriber with the default configuration:
///
/// ```rust
/// Subscriber::new(env!("CARGO_PKG_NAME")).init();
/// ```
///
/// Overriding the subscriber's behavior:
///
/// ```rust
/// let subscriber = Subscriber::new(env!("CARGO_PKG_NAME"))
///    .with_thread_names(true) // include thread names when logging
///    .with_target(false); // don't include event targets when logging
///
/// subscriber.init();
/// ```
///
/// [`Subscriber`]: tracing_subscriber::Subscribe
#[derive(Debug)]
pub struct Subscriber<C, F = format::DefaultFields, E = format::Full> {
    fmt: fmt::Subscriber<C, F, format::Format<E, ()>, AndroidLogMakeWriter>,
}

/// Type alias for the [`Collector`] composed by layering a [`Subscriber`] with
/// a [`Registry`].
///
/// [`Collector`]: tracing_core::Collect
pub type Collector<F, E> = Layered<Subscriber<Registry, F, E>, Registry>;

impl<C> Subscriber<C>
where
    C: Collect + for<'a> LookupSpan<'a>,
{
    /// Returns a new [`Subscriber`] with the given tag and the default
    /// configuration.
    pub fn new(tag: impl AsRef<[u8]>) -> Self {
        Self {
            fmt: fmt::Subscriber::default()
                .event_format(Self::default_format())
                .with_writer(AndroidLogMakeWriter::new(tag)),
        }
    }

    /// Returns a new [`Subscriber`] with the given tag and the default
    /// configuration, logging to the provided [`Buffer`]. This is only
    /// effective on Android 11+.
    pub fn with_buffer(tag: impl AsRef<[u8]>, buffer: Buffer) -> Self {
        Self {
            fmt: fmt::Subscriber::default()
                .event_format(Self::default_format())
                .with_writer(AndroidLogMakeWriter::with_buffer(tag, buffer)),
        }
    }

    fn default_format() -> format::Format<format::Full, ()> {
        format::Format::default().with_level(false).without_time()
    }
}

impl<F, E> Subscriber<Registry, F, E>
where
    Subscriber<Registry, F, E>: Subscribe<Registry> + Send + Sync,
{
    /// Attempts to set `self` as the [global default
    /// collector](tracing::dispatch#setting-the-default-collector) in the
    /// current scope, panicking if this fails, by first layering it with a
    /// [`Registry`].
    ///
    /// This method panics if a global default collector has already been set.
    pub fn init(self) {
        self.collector().init()
    }
}

impl<F, E> Subscriber<Registry, F, E>
where
    Subscriber<Registry, F, E>: Subscribe<Registry>,
{
    /// Converts this [`Subscriber`] into a [`Collector`] by layering it with a
    /// [`Registry`].
    ///
    /// [`Subscriber`]: tracing_subscriber::Subscribe
    /// [`Collector`]: tracing_core::Collect
    pub fn collector(self) -> Collector<F, E> {
        Registry::default().with(self)
    }
}

impl<C, F, E> Subscriber<C, F, E>
where
    F: for<'writer> FormatFields<'writer> + 'static,
{
    /// Do not emit timestamps with spans.
    pub fn without_time(self) -> Self {
        Self {
            fmt: self.fmt.without_time(),
        }
    }

    /// Configures how synthesized events are emitted at points in the [span
    /// lifecycle][lifecycle].
    ///
    /// The following options are available:
    ///
    /// - `FmtSpan::NONE`: No events will be synthesized when spans are created,
    ///   entered, exited, or closed. Data from spans will still be included as
    ///   the context for formatted events. This is the default.
    /// - `FmtSpan::NEW`: An event will be synthesized when spans are created.
    /// - `FmtSpan::ENTER`: An event will be synthesized when spans are entered.
    /// - `FmtSpan::EXIT`: An event will be synthesized when spans are exited.
    /// - `FmtSpan::CLOSE`: An event will be synthesized when a span closes. If
    ///   [timestamps are enabled][time] for this formatter, the generated event
    ///   will contain fields with the span's _busy time_ (the total time for
    ///   which it was entered) and _idle time_ (the total time that the span
    ///   existed but was not entered).
    /// - `FmtSpan::ACTIVE`: Events will be synthesized when spans are entered
    ///   or exited.
    /// - `FmtSpan::FULL`: Events will be synthesized whenever a span is
    ///   created, entered, exited, or closed. If timestamps are enabled, the
    ///   close event will contain the span's busy and idle time, as described
    ///   above.
    ///
    /// Note that the generated events will only be part of the log output by
    /// this formatter; they will not be recorded by other `Collector`s or by
    /// `Subscriber`s added to this subscriber.
    ///
    /// [lifecycle]: mod@tracing::span#the-span-lifecycle
    /// [time]: Subscriber::without_time()
    pub fn with_span_events(self, kind: FmtSpan) -> Self {
        Self {
            fmt: self.fmt.with_span_events(kind),
        }
    }

    /// Sets whether or not an event's target is displayed.
    pub fn with_target(self, display_target: bool) -> Self {
        Self {
            fmt: self.fmt.with_target(display_target),
        }
    }

    /// Sets whether or not the [thread ID] of the current thread is displayed
    /// when formatting events
    ///
    /// [thread ID]: std::thread::ThreadId
    pub fn with_thread_ids(self, display_thread_ids: bool) -> Self {
        Self {
            fmt: self.fmt.with_thread_ids(display_thread_ids),
        }
    }

    /// Sets whether or not the [name] of the current thread is displayed
    /// when formatting events
    ///
    /// [name]: std::thread#naming-threads
    pub fn with_thread_names(self, display_thread_names: bool) -> Self {
        Self {
            fmt: self.fmt.with_thread_names(display_thread_names),
        }
    }

    /// Sets the subscriber being built to use a [less verbose
    /// formatter](format::Compact).
    pub fn compact(self) -> Subscriber<C, F, format::Compact> {
        Subscriber {
            fmt: self.fmt.compact(),
        }
    }

    /// Sets the subscriber being built to use a [JSON formatter](format::Json).
    ///
    /// The full format includes fields from all entered spans.
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    pub fn json(self) -> Subscriber<C, format::JsonFields, format::Json> {
        Subscriber {
            fmt: self.fmt.json(),
        }
    }
}

#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
impl<C> Subscriber<C, format::JsonFields, format::Json> {
    /// Sets the JSON subscriber being built to flatten event metadata.
    ///
    /// See [`format::Json`]
    pub fn flatten_event(self, flatten_event: bool) -> Self {
        Self {
            fmt: self.fmt.flatten_event(flatten_event),
        }
    }

    /// Sets whether or not the formatter will include the current span in
    /// formatted events.
    ///
    /// See [`format::Json`]
    pub fn with_current_span(self, display_current_span: bool) -> Self {
        Self {
            fmt: self.fmt.with_current_span(display_current_span),
        }
    }

    /// Sets whether or not the formatter will include a list (from root to
    /// leaf) of all currently entered spans in formatted events.
    ///
    /// See [`format::Json`]
    pub fn with_span_list(self, display_span_list: bool) -> Self {
        Self {
            fmt: self.fmt.with_span_list(display_span_list),
        }
    }
}

impl<C, F, E> Subscribe<C> for Subscriber<C, F, E>
where
    C: Collect + for<'a> LookupSpan<'a>,
    F: for<'writer> FormatFields<'writer> + 'static,
    format::Format<E, ()>: FormatEvent<C, F> + 'static,
{
    #[inline]
    fn new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, C>) {
        self.fmt.new_span(attrs, id, ctx)
    }

    #[inline]
    fn on_record(&self, span: &Id, values: &Record<'_>, ctx: Context<'_, C>) {
        self.fmt.on_record(span, values, ctx)
    }

    #[inline]
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, C>) {
        self.fmt.on_event(event, ctx)
    }

    #[inline]
    fn on_enter(&self, id: &Id, ctx: Context<'_, C>) {
        self.fmt.on_enter(id, ctx)
    }

    #[inline]
    fn on_exit(&self, id: &Id, ctx: Context<'_, C>) {
        self.fmt.on_exit(id, ctx)
    }

    #[inline]
    fn on_close(&self, id: Id, ctx: Context<'_, C>) {
        self.fmt.on_close(id, ctx)
    }

    unsafe fn downcast_raw(&self, id: TypeId) -> Option<NonNull<()>> {
        if id == TypeId::of::<Self>() {
            Some(NonNull::from(self).cast())
        } else {
            self.fmt.downcast_raw(id)
        }
    }
}

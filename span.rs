//! neon/pageserver/src/tenant/timeline/span.rs
//!
//! Supportability note:
//! This module is intended to host timeline-related tracing span helpers.
//! Prefer `tracing::instrument` and add stable identifiers (tenant_id, timeline_id)
//! as span fields to ensure logs and traces can be correlated during incidents.
//! Tracing span helpers.//! neon/pageserver/src/tenant/timeline/span.rs
//!
//! Supportability note: this file is intentionally empty in the current snapshot.
//! If spans/timeline tracing utilities are added later, ensure they use `tracing` and
//! follow the `[tomo-id-XYZ] <path>: <message>` convention for all log messages.

/// Records the given fields in the current span, as a single call. The fields must already have
/// been declared for the span (typically with empty values).
#[macro_export]
macro_rules! span_record {
    ($($tokens:tt)*) => {$crate::span_record_in!(::tracing::Span::current(), $($tokens)*)};
}

/// Records the given fields in the given span, as a single call. The fields must already have been
/// declared for the span (typically with empty values).
#[macro_export]
macro_rules! span_record_in {
    ($span:expr, $($tokens:tt)*) => {
        if let Some(meta) = $span.metadata() {
            $span.record_all(&tracing::valueset!(meta.fields(), $($tokens)*));
        }
    };
}

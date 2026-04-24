//! neon/pageserver/src/tenant/timeline/span.rs
//!
//! Supportability note:
//! This module is intended to host timeline-related tracing span helpers.
//! Prefer `tracing::instrument` and add stable identifiers (tenant_id, timeline_id)
//! as span fields to ensure logs and traces can be correlated during incidents.
use utils::tracing_span_assert::check_fields_present;//! neon/pageserver/src/tenant/timeline/span.rs
//!
//! Supportability note: this file is intentionally empty in the current snapshot.
//! If spans/timeline tracing utilities are added later, ensure they use `tracing` and
//! follow the `[tomo-id-XYZ] <path>: <message>` convention for all log messages.

mod extractors {
    use utils::tracing_span_assert::ConstExtractor;

    pub(super) const TENANT_ID: ConstExtractor = ConstExtractor::new("tenant_id");
    pub(super) const SHARD_ID: ConstExtractor = ConstExtractor::new("shard_id");
    pub(super) const TIMELINE_ID: ConstExtractor = ConstExtractor::new("timeline_id");
}

#[track_caller]
pub(crate) fn debug_assert_current_span_has_tenant_id() {
    if cfg!(debug_assertions) {
        if let Err(missing) = check_fields_present!([&extractors::TENANT_ID, &extractors::SHARD_ID])
        {
            panic!("missing extractors: {missing:?}")
        }
    }
}

#[track_caller]
pub(crate) fn debug_assert_current_span_has_tenant_and_timeline_id() {
    if cfg!(debug_assertions) {
        if let Err(missing) = check_fields_present!([
            &extractors::TENANT_ID,
            &extractors::SHARD_ID,
            &extractors::TIMELINE_ID,
        ]) {
            panic!("missing extractors: {missing:?}")
        }
    }
}

#[track_caller]
pub(crate) fn debug_assert_current_span_has_tenant_and_timeline_id_no_shard_id() {
    if cfg!(debug_assertions) {
        if let Err(missing) =
            check_fields_present!([&extractors::TENANT_ID, &extractors::TIMELINE_ID,])
        {
            panic!("missing extractors: {missing:?}")
        }
    }
}

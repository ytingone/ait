//! Catalog and native session-list reads use separate budgets within shared transport response admission.

/// All externally admitted commands retain a global permit until execution completes.
pub(super) const COMMAND_CAPACITY: usize = 64;
/// Catalog responses retain a permit through delivery or response-owner cancellation.
pub(super) const CATALOG_RESPONSE_CAPACITY: usize = 64;
/// Listings have a command sublimit within the global budget and a separate response pool.
/// Commands include queued and executing requests; responses remain bounded through delivery.
pub(super) const LISTING_CAPACITY: usize = 16;
/// Accepted listings wait asynchronously for execution capacity, without occupying catalog slots.
pub(super) const LISTING_CONCURRENCY: usize = 4;

/// Select discovery and session enumeration without admitting session lifecycle mutations here.
pub(super) fn handles(method: &str) -> bool {
    matches!(
        method,
        "provider.available.list.request"
            | "provider.models.list.request"
            | "provider.modes.list.request"
            | "provider.features.list.request"
            | "provider.snapshot.get.request"
            | "provider.snapshot.refresh.request"
            | "provider.sessions.recent.list.request"
    )
}

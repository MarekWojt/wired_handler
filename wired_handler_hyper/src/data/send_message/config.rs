use std::{sync::OnceLock, time::Duration};

use thiserror::Error;

/// Cap for `GLOBAL_MAX_PARALLEL_SENDS` to prevent accidentally setting it too high
const MAX_GLOBAL_MAX_PARALLEL_SENDS: usize = 10_000;

/// Default value for `GLOBAL_SEND_TIMEOUT_MS`
const DEFAULT_TIMEOUT_MS: u32 = 5000;
/// Default value for `GLOBAL_MAX_PARALLEL_SENDS`
const DEFAULT_MAX_PARALLEL_SENDS: usize = 250;

/// Global send timeout for sending websocket messages
static GLOBAL_SEND_TIMEOUT_MS: OnceLock<u32> = OnceLock::new();
/// Global maximum parallel websocket message sends
static GLOBAL_MAX_PARALLEL_SENDS: OnceLock<usize> = OnceLock::new();

#[derive(Debug, Error)]
pub enum SetGlobalSendTimeoutError {
    #[error("The given duration is too long, {0:?} > {max:?}", max = Duration::from_millis(u32::MAX as u64))]
    TooLarge(Duration),
    #[error("The global send timeout has already been set")]
    AlreadySet(Duration),
}

#[derive(Debug, Error)]
pub enum SetGlobalMaxParallelSendsError {
    #[error("The given quantity of max parallel sends is too long, {0:?} > {max:?}. If this is intended, use the \"high-max-parallel-sends\" feature flag", max = MAX_GLOBAL_MAX_PARALLEL_SENDS)]
    TooLarge(usize),
    #[error("The global max parallel sends have already been set")]
    AlreadySet(usize),
}

/// Sets the timeout for websocket message sends
///
/// # Errors
/// if the given duration in milliseconds is larger than a maximum sized `u32` (~49.7 days)
pub fn set_global_send_timeout(duration: Duration) -> Result<(), SetGlobalSendTimeoutError> {
    let millis = duration.as_millis();

    if millis > u32::MAX as u128 {
        return Err(SetGlobalSendTimeoutError::TooLarge(duration));
    }

    tracing::debug!("Global send timeout set to {millis}");

    GLOBAL_SEND_TIMEOUT_MS
        .set(millis as u32)
        .map_err(|_| SetGlobalSendTimeoutError::AlreadySet(duration))
}

/// Returns the currently set timeout for websocket message sends
pub fn global_send_timeout() -> Duration {
    Duration::from_millis(*GLOBAL_SEND_TIMEOUT_MS.get_or_init(|| DEFAULT_TIMEOUT_MS) as u64)
}

fn do_set_global_max_parallel_sends(
    max_parallel_sends: usize,
) -> Result<(), SetGlobalMaxParallelSendsError> {
    tracing::debug!("Global max parallel sends set to {max_parallel_sends}");
    GLOBAL_MAX_PARALLEL_SENDS
        .set(max_parallel_sends)
        .map_err(|_| SetGlobalMaxParallelSendsError::AlreadySet(max_parallel_sends))
}

/// Sets the count of maximum parallel websocket message sends, can only be done once
///
/// Returns an `Err` containing the failed value if the value already set
///
/// Capped at `10,000`, if you need more, use the `"high-max-parallel-sends"` feature
#[cfg(not(feature = "high-max-parallel-sends"))]
pub fn set_global_max_parallel_sends(
    max_parallel_sends: usize,
) -> Result<(), SetGlobalMaxParallelSendsError> {
    if max_parallel_sends > MAX_GLOBAL_MAX_PARALLEL_SENDS {
        return Err(SetGlobalMaxParallelSendsError::TooLarge(max_parallel_sends));
    }

    do_set_global_max_parallel_sends(max_parallel_sends)
}

/// Sets the count of maximum parallel websocket message sends, can only be done once
///
/// Returns an `Err` containing the failed value if the value already set
///
/// **Uncapped, use with care**
#[cfg(feature = "high-max-parallel-sends")]
pub fn set_global_max_parallel_sends(
    max_parallel_sends: usize,
) -> Result<(), SetGlobalMaxParallelSendsError> {
    do_set_global_max_parallel_sends(max_parallel_sends)
}

/// Returns the currently set maximum parallel websocket message sends
pub fn global_max_parallel_sends() -> usize {
    *GLOBAL_MAX_PARALLEL_SENDS.get_or_init(|| DEFAULT_MAX_PARALLEL_SENDS)
}

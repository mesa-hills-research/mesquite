//! Portable monotonic time. `None` is C's null clock, not a wall-clock epoch.
use std::time::{Duration, Instant};
pub(crate) type Clock = Option<Instant>;
pub(crate) type DurationMicros = u64;
pub(crate) const fn duration_from_micros(micros: u64) -> DurationMicros {
    micros
}
pub(crate) const fn duration_to_micros(duration: DurationMicros) -> u64 {
    duration
}
pub(crate) const fn clock_null() -> Clock {
    None
}
pub(crate) fn clock_now() -> Clock {
    Some(Instant::now())
}
pub(crate) fn clock_after(base: Clock, duration: DurationMicros) -> Clock {
    base.map(|b| b + Duration::from_micros(duration))
}
pub(crate) fn clock_is_null(clock: Clock) -> bool {
    clock.is_none()
}
pub(crate) fn clock_is_gt(clock: Clock, other: Clock) -> bool {
    clock > other
}

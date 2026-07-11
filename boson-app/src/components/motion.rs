use leptos::prelude::*;
use orbital_motion::{MotionCurve, MotionDuration, PresenceMotion, SlideFrom};

/// Slide-in preset for run error MessageBar reveal (short, readable immediately).
pub fn boson_error_reveal_motion() -> Signal<PresenceMotion> {
    Signal::from(
        PresenceMotion::slide(SlideFrom::Bottom)
            .with_duration(MotionDuration::UltraFast)
            .with_curve(MotionCurve::DecelerateMid),
    )
}

/// Fade preset for dashboard KPI stagger enter.
pub fn boson_kpi_enter_motion() -> Signal<PresenceMotion> {
    Signal::from(PresenceMotion::fade().with_duration(MotionDuration::Normal))
}

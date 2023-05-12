//! Telemetry settings.

#[cfg(feature = "tracing")]
use super::tracing::settings::TracingSettings;

#[cfg(feature = "logging")]
use super::log::settings::LoggingSettings;

#[cfg(feature = "settings")]
use crate::settings::settings;

/// Telemetry settings.
#[cfg_attr(feature = "settings", settings(crate_path = "crate"))]
#[cfg_attr(not(feature = "settings"), derive(Clone, Default, Debug))]
pub struct TelemetrySettings {
    /// Distributed tracing settings
    #[cfg(feature = "tracing")]
    pub tracing: TracingSettings,

    /// Logging settings.
    #[cfg(feature = "logging")]
    pub logging: LoggingSettings,
}

fn _assert_traits_implemented_for_all_features() {
    fn assert<S: std::fmt::Debug + Clone + Default>() {}

    assert::<TelemetrySettings>();
}

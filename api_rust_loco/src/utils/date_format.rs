//! Locale-aware date formatting utilities.
//!
//! All date formatting in the application should go through these helpers so
//! that the output automatically respects the user's locale (set via
//! `rust_i18n::set_locale()` or the `Accept-Language` header).
//!
//! # Usage
//!
//! ```ignore
//! use crate::utils::date_format;
//!
//! let formatted = date_format::format_date(chrono::Utc::now());
//! ```

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};

/// Returns the date-only format pattern for the current locale.
///
/// | Locale  | Pattern       | Example            |
/// |---------|---------------|--------------------|
/// | `en`    | `%m/%d/%Y`    | 06/12/2026         |
/// | `pt-BR` | `%d/%m/%Y`    | 12/06/2026         |
/// | `es`    | `%d/%m/%Y`    | 12/06/2026         |
/// | other   | `%Y-%m-%d`    | 2026-06-12         |
fn date_only_pattern() -> &'static str {
    match &*rust_i18n::locale() {
        "en" => "%m/%d/%Y",
        "pt-BR" | "es" => "%d/%m/%Y",
        _ => "%Y-%m-%d",
    }
}

/// Returns the date+time format pattern for the current locale.
fn datetime_pattern() -> &'static str {
    match &*rust_i18n::locale() {
        "en" => "%m/%d/%Y %I:%M %p",
        "pt-BR" | "es" => "%d/%m/%Y %H:%M",
        _ => "%Y-%m-%d %H:%M",
    }
}

/// Returns the long date pattern suitable for email headers.
///
/// | Locale  | Pattern              | Example                  |
/// |---------|----------------------|--------------------------|
/// | `en`    | `%B %d, %Y`          | June 12, 2026            |
/// | `pt-BR` | `%d de %B de %Y`     | 12 de junho de 2026      |
/// | `es`    | `%d de %B de %Y`     | 12 de junio de 2026      |
fn long_date_pattern() -> &'static str {
    match &*rust_i18n::locale() {
        "en" => "%B %d, %Y",
        "pt-BR" | "es" => "%d de %B de %Y",
        _ => "%B %d, %Y",
    }
}

/// Returns the short month/day pattern for chart axes (e.g. `%d/%m` or `%m/%d`).
fn short_date_pattern() -> &'static str {
    match &*rust_i18n::locale() {
        "en" => "%m/%d",
        "pt-BR" | "es" => "%d/%m",
        _ => "%m-%d",
    }
}

// ---------------------------------------------------------------------------
// Public helpers
// ---------------------------------------------------------------------------

/// Formats a `DateTime<FixedOffset>` as a date-only string (e.g. `12/06/2026`).
pub fn format_date(dt: DateTime<FixedOffset>) -> String {
    dt.format(date_only_pattern()).to_string()
}

/// Formats a `NaiveDateTime` as a date-only string.
pub fn format_date_naive(dt: NaiveDateTime) -> String {
    dt.format(date_only_pattern()).to_string()
}

/// Formats a `NaiveDate` as a date-only string.
pub fn format_date_short(dt: NaiveDate) -> String {
    dt.format(date_only_pattern()).to_string()
}

/// Formats a `NaiveDateTime` as a date+time string (e.g. `12/06/2026 14:30`).
pub fn format_datetime_naive(dt: NaiveDateTime) -> String {
    dt.format(datetime_pattern()).to_string()
}

/// Formats a `DateTime<FixedOffset>` as a date+time string.
pub fn format_datetime_tz(dt: DateTime<FixedOffset>) -> String {
    dt.format(datetime_pattern()).to_string()
}

/// Formats a `DateTime<FixedOffset>` as a long date string for emails.
pub fn format_date_long(dt: DateTime<FixedOffset>) -> String {
    dt.format(long_date_pattern()).to_string()
}

/// Formats a `DateTime<FixedOffset>` as a short month/day string for charts.
pub fn format_date_short_chart(dt: DateTime<FixedOffset>) -> String {
    dt.format(short_date_pattern()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn formats_date_in_pt_br() {
        rust_i18n::set_locale("pt-BR");
        let dt = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2026, 6, 12, 0, 0, 0)
            .unwrap();
        assert_eq!(format_date(dt), "12/06/2026");
    }

    #[test]
    fn formats_date_in_en() {
        rust_i18n::set_locale("en");
        let dt = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2026, 6, 12, 0, 0, 0)
            .unwrap();
        assert_eq!(format_date(dt), "06/12/2026");
    }

    #[test]
    fn formats_datetime_in_pt_br() {
        rust_i18n::set_locale("pt-BR");
        let dt = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2026, 6, 12, 14, 30, 0)
            .unwrap();
        assert_eq!(format_datetime_tz(dt), "12/06/2026 14:30");
    }

    #[test]
    fn formats_datetime_in_en() {
        rust_i18n::set_locale("en");
        let dt = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2026, 6, 12, 14, 30, 0)
            .unwrap();
        assert_eq!(format_datetime_tz(dt), "06/12/2026 02:30 PM");
    }

    #[test]
    fn formats_long_date_in_pt_br() {
        rust_i18n::set_locale("pt-BR");
        let dt = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2026, 6, 12, 0, 0, 0)
            .unwrap();
        assert_eq!(format_date_long(dt), "12 de junho de 2026");
    }

    #[test]
    fn formats_long_date_in_en() {
        rust_i18n::set_locale("en");
        let dt = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2026, 6, 12, 0, 0, 0)
            .unwrap();
        assert_eq!(format_date_long(dt), "June 12, 2026");
    }

    #[test]
    fn formats_short_chart_date_in_pt_br() {
        rust_i18n::set_locale("pt-BR");
        let dt = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2026, 6, 12, 0, 0, 0)
            .unwrap();
        assert_eq!(format_date_short_chart(dt), "12/06");
    }

    #[test]
    fn formats_short_chart_date_in_en() {
        rust_i18n::set_locale("en");
        let dt = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2026, 6, 12, 0, 0, 0)
            .unwrap();
        assert_eq!(format_date_short_chart(dt), "06/12");
    }
}

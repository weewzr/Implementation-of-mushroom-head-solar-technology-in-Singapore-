//! Canonical weather/irradiance ingestion foundation.
//!
//! Strict parsing/QC for the project's canonical CSV interchange format.
//! This layer reports defects; it does not impute, repair, or silently coerce
//! source observations.

use std::fmt;

pub const REQUIRED_HEADER: [&str; 7] = [
    "timestamp",
    "ghi_w_m2",
    "dhi_w_m2",
    "dni_w_m2",
    "ambient_temperature_c",
    "wind_speed_m_s",
    "wind_direction_deg",
];

#[derive(Debug, Clone, PartialEq)]
pub struct WeatherRecord {
    pub timestamp: String,
    /// Absolute UTC Unix time in seconds, parsed from the explicit-offset timestamp.
    pub timestamp_utc_s: i64,
    pub ghi_w_m2: f64,
    pub dhi_w_m2: f64,
    pub dni_w_m2: f64,
    pub ambient_temperature_c: f64,
    pub wind_speed_m_s: f64,
    pub wind_direction_deg: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QcSummary {
    pub expected_samples: Option<usize>,
    pub parsed_samples: usize,
    pub issue_count: usize,
    pub duplicate_timestamps: usize,
    pub nonmonotonic_timestamps: usize,
    pub negative_irradiance_values: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QcIssue {
    pub row: usize,
    pub field: &'static str,
    pub message: String,
}

impl fmt::Display for QcIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "row {} field {}: {}", self.row, self.field, self.message)
    }
}

fn parse_finite(value: &str, row: usize, field: &'static str) -> Result<f64, QcIssue> {
    let parsed = value.parse::<f64>().map_err(|_| QcIssue {
        row,
        field,
        message: format!("expected numeric value, got {value:?}"),
    })?;
    if !parsed.is_finite() {
        return Err(QcIssue {
            row,
            field,
            message: "non-finite value".to_string(),
        });
    }
    Ok(parsed)
}

fn days_from_civil(year: i64, month: u32, day: u32) -> Option<i64> {
    if !(1..=12).contains(&month) || day == 0 {
        return None;
    }
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let month_days = [31_u32, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if day > month_days[(month - 1) as usize] {
        return None;
    }
    let y = year - i64::from(month <= 2);
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = month as i64 + if month > 2 { -3 } else { 9 };
    let doy = (153 * mp + 2) / 5 + day as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146097 + doe - 719468)
}

fn parse_fixed_digits(s: &str, start: usize, len: usize) -> Option<i64> {
    s.get(start..start + len)?.parse::<i64>().ok()
}

/// Parse the canonical ISO-8601 subset YYYY-MM-DDTHH:MM:SS(Z|+HH:MM|-HH:MM)
/// to an absolute UTC Unix timestamp. Fractional seconds are intentionally not
/// accepted in schema v1; provider-specific inputs must be normalized upstream.
fn parse_timestamp_utc_s(timestamp: &str) -> Option<i64> {
    if timestamp.len() < 20
        || timestamp.as_bytes().get(4) != Some(&b'-')
        || timestamp.as_bytes().get(7) != Some(&b'-')
        || timestamp.as_bytes().get(10) != Some(&b'T')
        || timestamp.as_bytes().get(13) != Some(&b':')
        || timestamp.as_bytes().get(16) != Some(&b':')
    {
        return None;
    }
    let year = parse_fixed_digits(timestamp, 0, 4)?;
    let month = parse_fixed_digits(timestamp, 5, 2)? as u32;
    let day = parse_fixed_digits(timestamp, 8, 2)? as u32;
    let hour = parse_fixed_digits(timestamp, 11, 2)?;
    let minute = parse_fixed_digits(timestamp, 14, 2)?;
    let second = parse_fixed_digits(timestamp, 17, 2)?;
    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    let offset_s = if timestamp.len() == 20 && timestamp.ends_with('Z') {
        0
    } else if timestamp.len() == 25
        && matches!(timestamp.as_bytes().get(19), Some(b'+') | Some(b'-'))
        && timestamp.as_bytes().get(22) == Some(&b':')
    {
        let oh = parse_fixed_digits(timestamp, 20, 2)?;
        let om = parse_fixed_digits(timestamp, 23, 2)?;
        if oh > 23 || om > 59 {
            return None;
        }
        let magnitude = oh * 3600 + om * 60;
        if timestamp.as_bytes()[19] == b'+' { magnitude } else { -magnitude }
    } else {
        return None;
    };
    let days = days_from_civil(year, month, day)?;
    Some(days * 86400 + hour * 3600 + minute * 60 + second - offset_s)
}

fn has_explicit_offset(timestamp: &str) -> bool {
    if timestamp.ends_with('Z') {
        return true;
    }
    let Some(t_pos) = timestamp.find('T') else {
        return false;
    };
    timestamp[t_pos + 1..]
        .char_indices()
        .any(|(i, c)| i >= 5 && (c == '+' || c == '-'))
}

fn validate_record(record: &WeatherRecord, row: usize, issues: &mut Vec<QcIssue>) {
    if !has_explicit_offset(&record.timestamp) {
        issues.push(QcIssue {
            row,
            field: "timestamp",
            message: "timestamp must carry Z or an explicit UTC offset".to_string(),
        });
    }
    for (field, value) in [
        ("ghi_w_m2", record.ghi_w_m2),
        ("dhi_w_m2", record.dhi_w_m2),
        ("dni_w_m2", record.dni_w_m2),
    ] {
        if value < 0.0 {
            issues.push(QcIssue {
                row,
                field,
                message: "negative irradiance requires source/QC interpretation".to_string(),
            });
        }
    }
    if record.wind_speed_m_s < 0.0 {
        issues.push(QcIssue {
            row,
            field: "wind_speed_m_s",
            message: "wind speed cannot be negative".to_string(),
        });
    }
    if !(0.0..360.0).contains(&record.wind_direction_deg) {
        issues.push(QcIssue {
            row,
            field: "wind_direction_deg",
            message: "expected degrees clockwise from north in [0, 360)".to_string(),
        });
    }
}

/// Parse the canonical seven-column CSV fixture/interchange form.
///
/// Provider-specific formats must be normalized by a separately documented
/// preprocessing step. Timestamp ordering below is intentionally only a
/// normalized-string foundation: absolute-time parsing is still required before
/// this module can satisfy the full canonical time-series contract.
pub fn parse_canonical_csv(input: &str) -> Result<(Vec<WeatherRecord>, Vec<QcIssue>), String> {
    let mut lines = input.lines().filter(|line| !line.trim().is_empty());
    let header = lines.next().ok_or_else(|| "missing CSV header".to_string())?;
    let columns: Vec<&str> = header.split(',').map(str::trim).collect();
    if columns != REQUIRED_HEADER {
        return Err(format!(
            "canonical header mismatch; expected {}",
            REQUIRED_HEADER.join(",")
        ));
    }

    let mut records = Vec::new();
    let mut issues = Vec::new();

    for (index, line) in lines.enumerate() {
        let row = index + 2;
        let values: Vec<&str> = line.split(',').map(str::trim).collect();
        if values.len() != REQUIRED_HEADER.len() {
            issues.push(QcIssue {
                row,
                field: "row",
                message: format!(
                    "expected {} columns, found {}",
                    REQUIRED_HEADER.len(),
                    values.len()
                ),
            });
            continue;
        }

        let parsed = (|| -> Result<WeatherRecord, QcIssue> {
            Ok(WeatherRecord {
                timestamp: values[0].to_string(),
                timestamp_utc_s: parse_timestamp_utc_s(values[0]).ok_or_else(|| QcIssue {
                    row,
                    field: "timestamp",
                    message: "expected canonical ISO-8601 timestamp with explicit offset".to_string(),
                })?,
                ghi_w_m2: parse_finite(values[1], row, "ghi_w_m2")?,
                dhi_w_m2: parse_finite(values[2], row, "dhi_w_m2")?,
                dni_w_m2: parse_finite(values[3], row, "dni_w_m2")?,
                ambient_temperature_c: parse_finite(values[4], row, "ambient_temperature_c")?,
                wind_speed_m_s: parse_finite(values[5], row, "wind_speed_m_s")?,
                wind_direction_deg: parse_finite(values[6], row, "wind_direction_deg")?,
            })
        })();

        match parsed {
            Ok(record) => {
                validate_record(&record, row, &mut issues);
                records.push(record);
            }
            Err(issue) => issues.push(issue),
        }
    }

    for pair in records.windows(2) {
        if pair[0].timestamp_utc_s == pair[1].timestamp_utc_s {
            issues.push(QcIssue {
                row: 0,
                field: "timestamp",
                message: format!("duplicate timestamp {}", pair[0].timestamp),
            });
        } else if pair[0].timestamp_utc_s > pair[1].timestamp_utc_s {
            issues.push(QcIssue {
                row: 0,
                field: "timestamp",
                message: format!(
                    "non-monotonic absolute timestamp sequence: {} then {}",
                    pair[0].timestamp, pair[1].timestamp
                ),
            });
        }
    }

    Ok((records, issues))
}

pub fn summarize_qc(
    records: &[WeatherRecord],
    issues: &[QcIssue],
    expected_samples: Option<usize>,
) -> QcSummary {
    QcSummary {
        expected_samples,
        parsed_samples: records.len(),
        issue_count: issues.len(),
        duplicate_timestamps: issues
            .iter()
            .filter(|issue| issue.message.contains("duplicate timestamp"))
            .count(),
        nonmonotonic_timestamps: issues
            .iter()
            .filter(|issue| issue.message.contains("non-monotonic"))
            .count(),
        negative_irradiance_values: issues
            .iter()
            .filter(|issue| {
                matches!(issue.field, "ghi_w_m2" | "dhi_w_m2" | "dni_w_m2")
                    && issue.message.contains("negative irradiance")
            })
            .count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "timestamp,ghi_w_m2,dhi_w_m2,dni_w_m2,ambient_temperature_c,wind_speed_m_s,wind_direction_deg";

    #[test]
    fn accepts_minimal_valid_fixture() {
        let csv = format!("{HEADER}\n2026-01-01T12:00:00+08:00,800,120,700,31.2,2.4,180\n");
        let (records, issues) = parse_canonical_csv(&csv).unwrap();
        assert_eq!(records.len(), 1);
        assert!(issues.is_empty());
    }

    #[test]
    fn reports_negative_irradiance_without_silent_repair() {
        let csv = format!("{HEADER}\n2026-01-01T12:00:00+08:00,-1,120,700,31.2,2.4,180\n");
        let (_, issues) = parse_canonical_csv(&csv).unwrap();
        assert!(issues.iter().any(|issue| issue.field == "ghi_w_m2"));
    }

    #[test]
    fn reports_missing_timezone_offset() {
        let csv = format!("{HEADER}\n2026-01-01T12:00:00,800,120,700,31.2,2.4,180\n");
        let (_, issues) = parse_canonical_csv(&csv).unwrap();
        assert!(issues.iter().any(|issue| issue.field == "timestamp"));
    }

    #[test]
    fn reports_duplicate_timestamp() {
        let csv = format!("{HEADER}\n2026-01-01T12:00:00+08:00,800,120,700,31.2,2.4,180\n2026-01-01T12:00:00+08:00,810,121,701,31.3,2.5,181\n");
        let (_, issues) = parse_canonical_csv(&csv).unwrap();
        assert!(issues.iter().any(|issue| issue.message.contains("duplicate timestamp")));
    }

    #[test]
    fn equivalent_offsets_map_to_same_absolute_time() {
        let a = parse_timestamp_utc_s("2026-01-01T12:00:00+08:00").unwrap();
        let b = parse_timestamp_utc_s("2026-01-01T04:00:00Z").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn absolute_time_detects_order_across_offsets() {
        let csv = format!("{HEADER}\n2026-01-01T12:00:00+08:00,800,120,700,31.2,2.4,180\n2026-01-01T03:59:59Z,810,121,701,31.3,2.5,181\n");
        let (_, issues) = parse_canonical_csv(&csv).unwrap();
        assert!(issues.iter().any(|issue| issue.message.contains("non-monotonic absolute")));
    }

    #[test]
    fn qc_summary_counts_without_imputation() {
        let csv = format!("{HEADER}\n2026-01-01T12:00:00+08:00,-1,120,700,31.2,2.4,180\n2026-01-01T12:00:00+08:00,810,121,701,31.3,2.5,181\n");
        let (records, issues) = parse_canonical_csv(&csv).unwrap();
        let summary = summarize_qc(&records, &issues, Some(2));
        assert_eq!(summary.expected_samples, Some(2));
        assert_eq!(summary.parsed_samples, 2);
        assert_eq!(summary.negative_irradiance_values, 1);
        assert_eq!(summary.duplicate_timestamps, 1);
    }
}

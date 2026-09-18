//! Canonical weather/irradiance ingestion foundation.
//!
//! This module deliberately performs strict, dependency-free validation of the
//! project's canonical CSV interchange format. It does not impute, repair, or
//! silently coerce source data. Provider-specific acquisition/preprocessing
//! belongs upstream and must remain traceable through the acquisition manifest.

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
    pub ghi_w_m2: f64,
    pub dhi_w_m2: f64,
    pub dni_w_m2: f64,
    pub ambient_temperature_c: f64,
    pub wind_speed_m_s: f64,
    pub wind_direction_deg: f64,
    #[test]
    fn qc_summary_counts_without_imputation() {
        let csv = format!(
            "{HEADER}\n2026-01-01T12:00:00+08:00,-1,120,700,31.2,2.4,180\n2026-01-01T12:00:00+08:00,810,121,701,31.3,2.5,181\n"
        );
        let (records, issues) = parse_canonical_csv(&csv).unwrap();
        let summary = summarize_qc(&records, &issues, Some(2));
        assert_eq!(summary.expected_samples, Some(2));
        assert_eq!(summary.parsed_samples, 2);
        assert_eq!(summary.negative_irradiance_values, 1);
        assert_eq!(summary.duplicate_timestamps, 1);
    }
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
/// Quoted commas are intentionally unsupported: provider-specific CSV formats
/// must be normalized by a separately documented preprocessing step rather than
/// weakening the canonical contract.
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
                ghi_w_m2: parse_finite(values[1], row, "ghi_w_m2")?,
                dhi_w_m2: parse_finite(values[2], row, "dhi_w_m2")?,
                dni_w_m2: parse_finite(values[3], row, "dni_w_m2")?,
                ambient_temperature_c: parse_finite(
                    values[4],
                    row,
                    "ambient_temperature_c",
                )?,
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
        if pair[0].timestamp == pair[1].timestamp {
            issues.push(QcIssue {
                row: 0,
                field: "timestamp",
                message: format!("duplicate timestamp {}", pair[0].timestamp),
            });
        } else if pair[0].timestamp > pair[1].timestamp {
            issues.push(QcIssue {
                row: 0,
                field: "timestamp",
                message: format!(
                    "non-monotonic timestamp sequence: {} then {}",
                    pair[0].timestamp, pair[1].timestamp
                ),
            });
        }
    }

    Ok((records, issues))
}

/// Build a compact QC summary without changing or imputing any observations.
///
/// `expected_samples` is deliberately caller-supplied because the canonical
/// source manifest, not this parser, owns the coverage and interval convention.
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
            .filter(|issue| issue.message.contains("non-monotonic timestamp"))
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
        let csv = format!(
            "{HEADER}\n2026-01-01T12:00:00+08:00,800,120,700,31.2,2.4,180\n"
        );
        let (records, issues) = parse_canonical_csv(&csv).unwrap();
        assert_eq!(records.len(), 1);
        assert!(issues.is_empty());
    }

    #[test]
    fn reports_negative_irradiance_without_silent_repair() {
        let csv = format!(
            "{HEADER}\n2026-01-01T12:00:00+08:00,-1,120,700,31.2,2.4,180\n"
        );
        let (_, issues) = parse_canonical_csv(&csv).unwrap();
        assert!(issues.iter().any(|issue| issue.field == "ghi_w_m2"));
    }

    #[test]
    fn reports_missing_timezone_offset() {
        let csv = format!(
            "{HEADER}\n2026-01-01T12:00:00,800,120,700,31.2,2.4,180\n"
        );
        let (_, issues) = parse_canonical_csv(&csv).unwrap();
        assert!(issues.iter().any(|issue| issue.field == "timestamp"));
    }

    #[test]
    fn reports_duplicate_timestamp() {
        let csv = format!(
            "{HEADER}\n2026-01-01T12:00:00+08:00,800,120,700,31.2,2.4,180\n2026-01-01T12:00:00+08:00,810,121,701,31.3,2.5,181\n"
        );
        let (_, issues) = parse_canonical_csv(&csv).unwrap();
        assert!(issues
            .iter()
            .any(|issue| issue.message.contains("duplicate timestamp")));
    }
}

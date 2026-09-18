//! Canonical weather/irradiance ingestion foundation.
//!
//! Strict parsing/QC for the project's canonical CSV interchange format.
//! This layer reports defects; it does not impute, repair, or silently coerce
//! source observations.

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct DatasetMetadata {
    pub provider: String,
    pub product_name: String,
    pub source_identifier: String,
    pub retrieval_date: String,
    pub coverage_start: String,
    pub coverage_end: String,
    pub native_sampling_interval_s: i64,
    pub source_timezone: String,
    pub timestamp_semantics: String,
    pub station_id: String,
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub elevation_m: Option<f64>,
    pub coordinate_datum: Option<String>,
    pub licence_name: String,
    pub permission_reference: String,
    pub raw_redistribution: String,
    pub derived_output_redistribution: String,
    pub source_checksum: Option<String>,
    pub immutable_source_id: Option<String>,
    pub provider_quality_flags_available: bool,
}

impl DatasetMetadata {
    /// Validate the minimum typed manifest/site/provenance contract without
    /// inventing unknown metadata. Empty required strings remain explicit errors.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        for (name, value) in [
            ("provider", self.provider.as_str()),
            ("product_name", self.product_name.as_str()),
            ("source_identifier", self.source_identifier.as_str()),
            ("retrieval_date", self.retrieval_date.as_str()),
            ("coverage_start", self.coverage_start.as_str()),
            ("coverage_end", self.coverage_end.as_str()),
            ("source_timezone", self.source_timezone.as_str()),
            ("timestamp_semantics", self.timestamp_semantics.as_str()),
            ("station_id", self.station_id.as_str()),
            ("licence_name", self.licence_name.as_str()),
            ("permission_reference", self.permission_reference.as_str()),
            ("raw_redistribution", self.raw_redistribution.as_str()),
            ("derived_output_redistribution", self.derived_output_redistribution.as_str()),
        ] {
            if value.trim().is_empty() {
                errors.push(format!("{name} is required"));
            }
        }
        if self.native_sampling_interval_s <= 0 {
            errors.push("native_sampling_interval_s must be positive".to_string());
        }
        if !(-90.0..=90.0).contains(&self.latitude_deg) {
            errors.push("latitude_deg must be in [-90, 90]".to_string());
        }
        if !(-180.0..=180.0).contains(&self.longitude_deg) {
            errors.push("longitude_deg must be in [-180, 180]".to_string());
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

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
    pub relative_humidity_percent: Option<f64>,
    pub air_pressure_pa: Option<f64>,
    /// Provider-supplied QC/status token preserved verbatim when available.
    pub provider_quality_flag: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QcSummary {
    pub expected_samples: Option<usize>,
    pub parsed_samples: usize,
    pub issue_count: usize,
    /// Rows rejected during parsing/schema conversion; distinct from temporal gaps.
    pub rejected_rows: usize,
    pub duplicate_timestamps: usize,
    pub nonmonotonic_timestamps: usize,
    pub negative_irradiance_values: usize,
    /// Missing samples inferred only from positive timestamp gaps that are exact
    /// multiples of the declared interval. This never includes rejected rows.
    pub temporal_gap_missing_samples: Option<usize>,
    pub longest_gap_s: Option<i64>,
    pub interval_change_count: usize,
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
    if let Some(rh) = record.relative_humidity_percent {
        if !(0.0..=100.0).contains(&rh) {
            issues.push(QcIssue {
                row,
                field: "relative_humidity_percent",
                message: "expected relative humidity in [0, 100] percent".to_string(),
            });
        }
    }
    if let Some(pressure) = record.air_pressure_pa {
        if pressure <= 0.0 {
            issues.push(QcIssue {
                row,
                field: "air_pressure_pa",
                message: "air pressure must be positive".to_string(),
            });
        }
    }
}

/// Parse the canonical CSV fixture/interchange form.
///
/// The seven required columns must appear first in canonical order. Schema-v1
/// optional relative humidity, air pressure and provider quality flag columns may
/// follow in that order. Provider flags are preserved verbatim and are not
/// interpreted or discarded by the canonical ingestion layer.
///
/// Provider-specific formats must be normalized by a separately documented
/// preprocessing step. The strict schema-v1 timestamp subset is normalized to
/// absolute UTC seconds here; broader provider-format normalization, manifest
/// linkage and the remaining QC contract are still separate foundation work.
pub fn parse_canonical_csv(input: &str) -> Result<(Vec<WeatherRecord>, Vec<QcIssue>), String> {
    let mut lines = input.lines().filter(|line| !line.trim().is_empty());
    let header = lines.next().ok_or_else(|| "missing CSV header".to_string())?;
    let columns: Vec<&str> = header.split(',').map(str::trim).collect();
    let required_prefix_ok = columns.len() >= REQUIRED_HEADER.len()
        && columns[..REQUIRED_HEADER.len()] == REQUIRED_HEADER;
    let optional_columns = &columns[REQUIRED_HEADER.len()..];
    let optional_ok = matches!(
        optional_columns,
        []
            | ["relative_humidity_percent"]
            | ["relative_humidity_percent", "air_pressure_pa"]
            | ["relative_humidity_percent", "air_pressure_pa", "provider_quality_flag"]
    );
    if !required_prefix_ok || !optional_ok {
        return Err(format!(
            "canonical header mismatch; expected {} with optional trailing relative_humidity_percent,air_pressure_pa,provider_quality_flag",
            REQUIRED_HEADER.join(",")
        ));
    }

    let mut records = Vec::new();
    let mut issues = Vec::new();

    for (index, line) in lines.enumerate() {
        let row = index + 2;
        let values: Vec<&str> = line.split(',').map(str::trim).collect();
        if values.len() != columns.len() {
            issues.push(QcIssue {
                row,
                field: "row",
                message: format!(
                    "expected {} columns, found {}",
                    columns.len(),
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
                relative_humidity_percent: if columns.len() >= 8 {
                    Some(parse_finite(values[7], row, "relative_humidity_percent")?)
                } else {
                    None
                },
                air_pressure_pa: if columns.len() >= 9 {
                    Some(parse_finite(values[8], row, "air_pressure_pa")?)
                } else {
                    None
                },
                provider_quality_flag: if columns.len() >= 10 {
                    Some(values[9].to_string())
                } else {
                    None
                },
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

    let mut first_seen: HashMap<i64, usize> = HashMap::new();
    for (index, record) in records.iter().enumerate() {
        let row = index + 2;
        if let Some(first_row) = first_seen.insert(record.timestamp_utc_s, row) {
            issues.push(QcIssue {
                row,
                field: "timestamp",
                message: format!(
                    "duplicate absolute timestamp {} (first seen at row {first_row})",
                    record.timestamp
                ),
            });
        }
    }

    for pair in records.windows(2) {
        if pair[0].timestamp_utc_s > pair[1].timestamp_utc_s {
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

/// Dataset-level temporal QC against a declared native interval.
///
/// The interval is a source/manifest property. This routine reports departures
/// and never inserts or removes observations.
pub fn check_declared_interval(
    records: &[WeatherRecord],
    declared_interval_s: i64,
) -> Result<Vec<QcIssue>, String> {
    if declared_interval_s <= 0 {
        return Err("declared interval must be positive".to_string());
    }
    let mut issues = Vec::new();
    for pair in records.windows(2) {
        let dt = pair[1].timestamp_utc_s - pair[0].timestamp_utc_s;
        if dt > 0 && dt != declared_interval_s {
            let missing = if dt > declared_interval_s && dt % declared_interval_s == 0 {
                dt / declared_interval_s - 1
            } else {
                0
            };
            issues.push(QcIssue {
                row: 0,
                field: "timestamp",
                message: format!(
                    "sampling interval change: observed {dt} s, declared {declared_interval_s} s, implied missing samples {missing}"
                ),
            });
        }
    }
    Ok(issues)
}

pub fn summarize_qc(
    records: &[WeatherRecord],
    issues: &[QcIssue],
    declared_interval_s: Option<i64>,
) -> QcSummary {
    let positive_gaps: Vec<i64> = records
        .windows(2)
        .map(|pair| pair[1].timestamp_utc_s - pair[0].timestamp_utc_s)
        .filter(|dt| *dt > 0)
        .collect();
    let longest_gap_s = positive_gaps.iter().copied().max();
    let temporal_gap_missing_samples = declared_interval_s.and_then(|interval| {
        if interval <= 0 {
            return None;
        }
        Some(
            positive_gaps
                .iter()
                .filter(|dt| **dt > interval && **dt % interval == 0)
                .map(|dt| (*dt / interval - 1) as usize)
                .sum(),
        )
    });
    QcSummary {
        expected_samples: None,
        parsed_samples: records.len(),
        issue_count: issues.len(),
        rejected_rows: issues.iter().filter(|issue| issue.field == "row").count(),
        duplicate_timestamps: issues
            .iter()
            .filter(|issue| issue.message.contains("duplicate absolute timestamp"))
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
        temporal_gap_missing_samples,
        longest_gap_s,
        interval_change_count: issues
            .iter()
            .filter(|issue| issue.message.contains("sampling interval change"))
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
    fn accepts_schema_v1_optional_meteorology_columns() {
        let header = format!("{HEADER},relative_humidity_percent,air_pressure_pa");
        let csv = format!("{header}\n2026-01-01T12:00:00+08:00,800,120,700,31.2,2.4,180,78.5,100800\n");
        let (records, issues) = parse_canonical_csv(&csv).unwrap();
        assert!(issues.is_empty());
        assert_eq!(records[0].relative_humidity_percent, Some(78.5));
        assert_eq!(records[0].air_pressure_pa, Some(100800.0));
    }

    #[test]
    fn reports_invalid_optional_meteorology_without_repair() {
        let header = format!("{HEADER},relative_humidity_percent,air_pressure_pa");
        let csv = format!("{header}\n2026-01-01T12:00:00+08:00,800,120,700,31.2,2.4,180,101,-1\n");
        let (_, issues) = parse_canonical_csv(&csv).unwrap();
        assert!(issues.iter().any(|issue| issue.field == "relative_humidity_percent"));
        assert!(issues.iter().any(|issue| issue.field == "air_pressure_pa"));
    }

    #[test]
    fn preserves_provider_quality_flag_verbatim() {
        let header = format!("{HEADER},relative_humidity_percent,air_pressure_pa,provider_quality_flag");
        let csv = format!("{header}\n2026-01-01T12:00:00+08:00,800,120,700,31.2,2.4,180,78.5,100800,SUSPECT_CLOCK\n");
        let (records, issues) = parse_canonical_csv(&csv).unwrap();
        assert!(issues.is_empty());
        assert_eq!(records[0].provider_quality_flag.as_deref(), Some("SUSPECT_CLOCK"));
    }

    #[test]
    fn typed_dataset_metadata_rejects_missing_provenance() {
        let metadata = DatasetMetadata {
            provider: String::new(),
            product_name: "example".to_string(),
            source_identifier: "source".to_string(),
            retrieval_date: "2026-09-18".to_string(),
            coverage_start: "2026-01-01".to_string(),
            coverage_end: "2026-12-31".to_string(),
            native_sampling_interval_s: 60,
            source_timezone: "+08:00".to_string(),
            timestamp_semantics: "interval_end".to_string(),
            station_id: "station".to_string(),
            latitude_deg: 1.3,
            longitude_deg: 103.8,
            elevation_m: None,
            coordinate_datum: None,
            licence_name: "example".to_string(),
            permission_reference: "ref".to_string(),
            raw_redistribution: "unknown".to_string(),
            derived_output_redistribution: "unknown".to_string(),
            source_checksum: None,
            immutable_source_id: None,
            provider_quality_flags_available: true,
        };
        let errors = metadata.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("provider is required")));
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
        assert!(issues.iter().any(|issue| issue.message.contains("duplicate absolute timestamp")));
    }

    #[test]
    fn reports_nonadjacent_duplicate_absolute_timestamp() {
        let csv = format!("{HEADER}\n2026-01-01T12:00:00+08:00,800,120,700,31.2,2.4,180\n2026-01-01T12:01:00+08:00,810,121,701,31.3,2.5,181\n2026-01-01T04:00:00Z,820,122,702,31.4,2.6,182\n");
        let (_, issues) = parse_canonical_csv(&csv).unwrap();
        assert!(issues.iter().any(|issue| issue.message.contains("duplicate absolute timestamp")));
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
    fn declared_interval_reports_gap_without_imputation() {
        let csv = format!("{HEADER}\n2026-01-01T12:00:00+08:00,800,120,700,31.2,2.4,180\n2026-01-01T12:02:00+08:00,810,121,701,31.3,2.5,181\n");
        let (records, _) = parse_canonical_csv(&csv).unwrap();
        let issues = check_declared_interval(&records, 60).unwrap();
        assert!(issues.iter().any(|issue| issue.message.contains("implied missing samples 1")));
        let summary = summarize_qc(&records, &issues, Some(60));
        assert_eq!(summary.temporal_gap_missing_samples, Some(1));
        assert_eq!(summary.longest_gap_s, Some(120));
        assert_eq!(summary.interval_change_count, 1);
    }

    #[test]
    fn qc_summary_counts_without_imputation() {
        let csv = format!("{HEADER}\n2026-01-01T12:00:00+08:00,-1,120,700,31.2,2.4,180\n2026-01-01T12:00:00+08:00,810,121,701,31.3,2.5,181\n");
        let (records, issues) = parse_canonical_csv(&csv).unwrap();
        let summary = summarize_qc(&records, &issues, None);
        assert_eq!(summary.expected_samples, None);
        assert_eq!(summary.parsed_samples, 2);
        assert_eq!(summary.negative_irradiance_values, 1);
        assert_eq!(summary.duplicate_timestamps, 1);
        assert_eq!(summary.temporal_gap_missing_samples, None);
    }

    #[test]
    fn rejected_rows_are_not_counted_as_temporal_missing_samples() {
        let csv = format!("{HEADER}\n2026-01-01T12:00:00+08:00,800,120,700,31.2,2.4,180\nbad,row\n2026-01-01T12:01:00+08:00,810,121,701,31.3,2.5,181\n");
        let (records, issues) = parse_canonical_csv(&csv).unwrap();
        let summary = summarize_qc(&records, &issues, Some(60));
        assert_eq!(summary.rejected_rows, 1);
        assert_eq!(summary.temporal_gap_missing_samples, Some(0));
    }
}

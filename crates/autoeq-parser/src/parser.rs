use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterType {
    Peaking,
    LowShelf,
    HighShelf,
    LowPass,
    HighPass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametricFilter {
    pub enabled: bool,
    pub filter_type: FilterType,
    pub frequency_hz: f64,
    pub gain_db: f64,
    pub q: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoEqProfile {
    pub preamp_db: f64,
    pub filters: Vec<ParametricFilter>,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid preamp line: {0}")]
    InvalidPreamp(String),
    #[error("Invalid filter line {line_num}: {detail}")]
    InvalidFilter { line_num: usize, detail: String },
    #[error("Unknown filter type: {0}")]
    UnknownFilterType(String),
}

impl AutoEqProfile {
    pub fn parse(content: &str) -> Result<Self, ParseError> {
        let mut preamp_db = 0.0;
        let mut filters = Vec::new();

        for (i, line) in content.lines().enumerate() {
            let line_num = i + 1;
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("Preamp:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(db) = parts[1].parse::<f64>() {
                        preamp_db = db;
                    } else {
                        return Err(ParseError::InvalidPreamp(line.to_string()));
                    }
                } else {
                    return Err(ParseError::InvalidPreamp(line.to_string()));
                }
                continue;
            }

            if line.starts_with("Filter") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 4 {
                    return Err(ParseError::InvalidFilter { line_num, detail: "Too few tokens".to_string() });
                }

                let enabled = match parts[2] {
                    "ON" => true,
                    "OFF" => false,
                    _ => return Err(ParseError::InvalidFilter { line_num, detail: "Expected ON/OFF".to_string() }),
                };

                let filter_type = match parts[3] {
                    "PK" | "PEQ" => FilterType::Peaking,
                    "LSC" | "LS" => FilterType::LowShelf,
                    "HSC" | "HS" => FilterType::HighShelf,
                    "LP" => FilterType::LowPass,
                    "HP" => FilterType::HighPass,
                    other => return Err(ParseError::UnknownFilterType(other.to_string())),
                };

                let mut frequency_hz = 0.0;
                let mut gain_db = 0.0;
                let mut q = 0.707; // default for LP/HP if missing

                // Find Fc, Gain, Q
                let mut j = 4;
                while j < parts.len() {
                    match parts[j] {
                        "Fc" => {
                            if j + 1 < parts.len() {
                                frequency_hz = parts[j+1].parse::<f64>().map_err(|_| ParseError::InvalidFilter { line_num, detail: "Invalid Fc".to_string() })?;
                            }
                        }
                        "Gain" => {
                            if j + 1 < parts.len() {
                                gain_db = parts[j+1].parse::<f64>().map_err(|_| ParseError::InvalidFilter { line_num, detail: "Invalid Gain".to_string() })?;
                            }
                        }
                        "Q" => {
                            if j + 1 < parts.len() {
                                q = parts[j+1].parse::<f64>().map_err(|_| ParseError::InvalidFilter { line_num, detail: "Invalid Q".to_string() })?;
                            }
                        }
                        _ => {}
                    }
                    j += 1;
                }

                filters.push(ParametricFilter {
                    enabled,
                    filter_type,
                    frequency_hz,
                    gain_db,
                    q,
                });
            }
        }

        Ok(Self {
            preamp_db,
            filters,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_complete() {
        let content = "\
Preamp: -6.2 dB
Filter 1: ON PK Fc 105 Hz Gain -2.3 dB Q 1.41
Filter 2: ON LSC Fc 105 Hz Gain 7.0 dB Q 0.71
Filter 3: ON HSC Fc 8000 Hz Gain 2.1 dB Q 0.71
Filter 4: OFF PK Fc 2400 Hz Gain -3.5 dB Q 2.80
";
        let profile = AutoEqProfile::parse(content).unwrap();
        assert_eq!(profile.preamp_db, -6.2);
        assert_eq!(profile.filters.len(), 4);
        assert_eq!(profile.filters[0].filter_type, FilterType::Peaking);
        assert_eq!(profile.filters[0].frequency_hz, 105.0);
        assert!(profile.filters[0].enabled);
        
        assert_eq!(profile.filters[1].filter_type, FilterType::LowShelf);
        assert_eq!(profile.filters[2].filter_type, FilterType::HighShelf);
        
        assert_eq!(profile.filters[3].filter_type, FilterType::Peaking);
        assert!(!profile.filters[3].enabled);
    }

    #[test]
    fn test_parse_disabled_filters() {
        let content = "Filter 1: OFF PK Fc 100 Hz Gain 0.0 dB Q 1.0\n";
        let profile = AutoEqProfile::parse(content).unwrap();
        assert!(!profile.filters[0].enabled);
    }

    #[test]
    fn test_parse_only_preamp() {
        let content = "Preamp: -4.5 dB\n";
        let profile = AutoEqProfile::parse(content).unwrap();
        assert_eq!(profile.preamp_db, -4.5);
        assert!(profile.filters.is_empty());
    }

    #[test]
    fn test_parse_empty() {
        let profile = AutoEqProfile::parse("").unwrap();
        assert_eq!(profile.preamp_db, 0.0);
        assert!(profile.filters.is_empty());
    }

    #[test]
    fn test_parse_comments_and_blank_lines() {
        let content = "\n# Some comment\nPreamp: -1.0 dB\n\n# Filter\nFilter 1: ON PK Fc 1000 Hz Gain -1.0 dB Q 1.0";
        let profile = AutoEqProfile::parse(content).unwrap();
        assert_eq!(profile.preamp_db, -1.0);
        assert_eq!(profile.filters.len(), 1);
    }

    #[test]
    fn test_parse_unknown_filter_type() {
        let content = "Filter 1: ON UNKNOWN Fc 1000 Hz Gain -1.0 dB Q 1.0";
        assert!(matches!(AutoEqProfile::parse(content), Err(ParseError::UnknownFilterType(_))));
    }

    #[test]
    fn test_missing_q_for_lp_hp() {
        let content = "Filter 1: ON LP Fc 1000 Hz";
        let profile = AutoEqProfile::parse(content).unwrap();
        assert_eq!(profile.filters[0].q, 0.707);
    }
}

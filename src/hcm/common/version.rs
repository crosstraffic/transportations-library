//! HCM edition selection.
//!
//! The library implements the 7th Edition of the Highway Capacity Manual. In November 2025 the
//! Transportation Research Board published Edition 7.1, which does not supersede the whole manual:
//! it replaces four chapters (13, 14, 27, and 28) with new merge, diverge, and weaving
//! methodologies developed under NCHRP Research Report 1038. Every other chapter is unchanged.
//!
//! [`HcmVersion`] lets a caller pick which edition a calculation follows, the way a documentation
//! site lets a reader pick a language version. The two editions produce different numbers for the
//! same segment, so the choice is an explicit input rather than something inferred. Selecting an
//! edition is meaningful only for a chapter the edition actually changed; see
//! [`HcmVersion::changes_chapter`].

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// The chapters Edition 7.1 replaces. Chapters 13 and 14 carry the core methodologies; 27 and 28
/// are their supplements (example problems and computational detail).
pub const V7_1_REPLACED_CHAPTERS: [u8; 4] = [13, 14, 27, 28];

/// Which edition of the HCM a calculation follows.
///
/// The default is [`HcmVersion::V7`]. Existing callers keep the numbers they had before Edition 7.1
/// existed, and moving a segment to the new methodology is a deliberate, visible act.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum HcmVersion {
    /// HCM 7th Edition (2022).
    #[default]
    #[serde(rename = "7")]
    V7,
    /// HCM Edition 7.1 (November 2025), replacement Chapters 13, 14, 27, and 28.
    #[serde(rename = "7.1")]
    V7_1,
}

impl HcmVersion {
    /// The newest edition the library implements.
    pub const LATEST: HcmVersion = HcmVersion::V7_1;

    /// Every selectable edition, oldest first. Useful for building a version picker.
    pub const ALL: [HcmVersion; 2] = [HcmVersion::V7, HcmVersion::V7_1];

    /// Whether this edition changed the methodology of the given HCM chapter relative to the 7th
    /// Edition. Selecting [`HcmVersion::V7_1`] for any other chapter is a no-op, because Edition
    /// 7.1 left that chapter alone.
    pub fn changes_chapter(&self, chapter: u8) -> bool {
        match self {
            HcmVersion::V7 => false,
            HcmVersion::V7_1 => V7_1_REPLACED_CHAPTERS.contains(&chapter),
        }
    }

    /// The edition label as it appears in the manual's page footers ("7" or "7.1").
    pub fn label(&self) -> &'static str {
        match self {
            HcmVersion::V7 => "7",
            HcmVersion::V7_1 => "7.1",
        }
    }
}

impl fmt::Display for HcmVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

impl FromStr for HcmVersion {
    type Err = String;

    /// Parses an edition label. Accepts the forms a caller is likely to type: `7`, `7.0`, `v7`,
    /// `HCM7`, `7.1`, `v7.1`, `HCM 7.1`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cleaned: String = s
            .trim()
            .to_ascii_lowercase()
            .replace("hcm", "")
            .replace(' ', "")
            .replace('_', "")
            .trim_start_matches('v')
            .to_string();
        match cleaned.as_str() {
            "7" | "7.0" => Ok(HcmVersion::V7),
            "7.1" => Ok(HcmVersion::V7_1),
            _ => Err(format!(
                "unknown HCM version {s:?}; expected one of \"7\" or \"7.1\""
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_the_seventh_edition() {
        assert_eq!(HcmVersion::default(), HcmVersion::V7);
    }

    #[test]
    fn parses_the_forms_a_caller_would_type() {
        for s in ["7", "7.0", "v7", "HCM7", " hcm 7 "] {
            assert_eq!(s.parse::<HcmVersion>().unwrap(), HcmVersion::V7, "{s}");
        }
        for s in ["7.1", "v7.1", "HCM 7.1", "hcm_7.1"] {
            assert_eq!(s.parse::<HcmVersion>().unwrap(), HcmVersion::V7_1, "{s}");
        }
        assert!("6".parse::<HcmVersion>().is_err());
        assert!("7.2".parse::<HcmVersion>().is_err());
    }

    #[test]
    fn round_trips_through_display_and_serde() {
        for v in HcmVersion::ALL {
            assert_eq!(v.to_string().parse::<HcmVersion>().unwrap(), v);
            let json = serde_json::to_string(&v).unwrap();
            assert_eq!(serde_json::from_str::<HcmVersion>(&json).unwrap(), v);
        }
        // The serde form is the manual's own label, not the Rust variant name, so a JSON
        // config reads as `"version": "7.1"`.
        assert_eq!(serde_json::to_string(&HcmVersion::V7_1).unwrap(), "\"7.1\"");
    }

    #[test]
    fn only_the_four_replaced_chapters_change() {
        for ch in 1..=38u8 {
            assert!(!HcmVersion::V7.changes_chapter(ch), "ch {ch}");
            assert_eq!(
                HcmVersion::V7_1.changes_chapter(ch),
                matches!(ch, 13 | 14 | 27 | 28),
                "ch {ch}"
            );
        }
    }
}

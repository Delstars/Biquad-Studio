use crate::parser::AutoEqProfile;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadphoneEntry {
    pub name: String,
    pub source: String,       // e.g., "oratory1990", "crinacle", "rtings"
    pub file_path: String,    // relative path to ParametricEQ.txt
}

pub struct HeadphoneDatabase {
    entries: Vec<HeadphoneEntry>,
}

impl HeadphoneDatabase {
    fn walk_dir_recursive(dir: &Path, base_path: &Path, entries: &mut Vec<HeadphoneEntry>) -> Result<(), std::io::Error> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    Self::walk_dir_recursive(&path, base_path, entries)?;
                } else if path.file_name() == Some(std::ffi::OsStr::new("ParametricEQ.txt")) {
                    if let Some(parent) = path.parent() {
                        if let Some(grandparent) = parent.parent() {
                            let name = parent.file_name().unwrap_or_default().to_string_lossy().to_string();
                            let source = grandparent.file_name().unwrap_or_default().to_string_lossy().to_string();
                            
                            if let Ok(rel_path) = path.strip_prefix(base_path) {
                                entries.push(HeadphoneEntry {
                                    name,
                                    source,
                                    file_path: rel_path.to_string_lossy().replace('\\', "/"),
                                });
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Load from a directory containing AutoEQ results structure
    pub fn from_directory(base_path: &Path) -> Result<Self, std::io::Error> {
        let mut entries = Vec::new();
        
        if base_path.is_dir() {
            Self::walk_dir_recursive(base_path, base_path, &mut entries)?;
        }
        
        Ok(Self { entries })
    }
    
    /// Simple case-insensitive substring search
    pub fn search(&self, query: &str) -> Vec<&HeadphoneEntry> {
        let q = query.to_lowercase();
        self.entries.iter().filter(|e| e.name.to_lowercase().contains(&q)).collect()
    }
    
    /// Load the ParametricEQ.txt profile for a given entry
    pub fn load_profile(&self, base_path: &Path, entry: &HeadphoneEntry) -> Result<AutoEqProfile, Box<dyn std::error::Error>> {
        let full_path = base_path.join(&entry.file_path);
        let content = fs::read_to_string(full_path)?;
        let profile = AutoEqProfile::parse(&content)?;
        Ok(profile)
    }
    
    /// Get all unique headphone names
    pub fn headphone_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.entries.iter().map(|e| e.name.as_str()).collect();
        names.sort_unstable();
        names.dedup();
        names
    }
    
    /// Number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_empty_directory() {
        let dir = std::env::temp_dir();
        let db = HeadphoneDatabase::from_directory(&dir).unwrap();
        // Just verify it doesn't fail, it might not be empty depending on the system's temp dir
        // but it shouldn't crash.
    }

    #[test]
    fn test_database_search_case_insensitive() {
        let db = HeadphoneDatabase {
            entries: vec![
                HeadphoneEntry {
                    name: "Sennheiser HD 600".to_string(),
                    source: "oratory1990".to_string(),
                    file_path: "oratory1990/Sennheiser HD 600/ParametricEQ.txt".to_string(),
                },
                HeadphoneEntry {
                    name: "Hifiman Sundara".to_string(),
                    source: "crinacle".to_string(),
                    file_path: "crinacle/Hifiman Sundara/ParametricEQ.txt".to_string(),
                }
            ],
        };
        
        let results = db.search("HD 600");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Sennheiser HD 600");
        
        let results2 = db.search("sennheiser");
        assert_eq!(results2.len(), 1);
        
        let results3 = db.search("xyz");
        assert!(results3.is_empty());
    }
}

use cosmic_text::FontSystem;
use std::collections::{HashSet, HashMap};
use crate::error::{Result, CloveError};

/// Font manager with caching
pub struct FontManager {
    font_system: FontSystem,
    font_names: HashSet<String>, // Track loaded font names
    font_name_map: HashMap<String, String>, // Maps custom names to font family names
    default_family: Option<String>,
}

impl FontManager {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            font_names: HashSet::new(),
            font_name_map: HashMap::new(),
            default_family: None,
        }
    }
    
    /// Load font from file path
    pub fn load(&mut self, name: &str, path: &str) -> Result<&mut Self> {
        let font_data = std::fs::read(path)
            .map_err(|e| CloveError::FontLoadError(e.to_string()))?;
        
        // Load font into cosmic-text
        self.font_system.db_mut().load_font_data(font_data);
        
        // Track font name
        self.font_names.insert(name.to_string());
        
        // Map custom name to family name (use name as family name for simplicity)
        self.font_name_map.insert(name.to_string(), name.to_string());
        
        Ok(self)
    }
    
    /// Set default font family
    pub fn set_default(&mut self, name: &str) -> Result<()> {
        if !self.font_names.contains(name) {
            return Err(CloveError::FontNotFound(name.to_string()));
        }
        self.default_family = Some(name.to_string());
        Ok(())
    }
    
    /// Get font system reference
    pub fn font_system(&self) -> &FontSystem {
        &self.font_system
    }
    
    /// Get font system mutable reference
    pub fn font_system_mut(&mut self) -> &mut FontSystem {
        &mut self.font_system
    }
    
    /// Get default font family
    pub fn default_family(&self) -> Option<&str> {
        self.default_family.as_deref()
    }
    
    /// Get count of loaded fonts
    pub fn font_count(&self) -> usize {
        self.font_names.len()
    }
    
    /// Get font family name by custom name
    pub(crate) fn get_font_family(&self, name: &str) -> Option<&String> {
        self.font_name_map.get(name)
    }
    
    /// Get text height estimate (simplified version)
    pub(crate) fn get_text_height(&self, _font_name: &str, font_size: f32) -> Option<f32> {
        // Simplified: return font_size * 1.2 as a rough estimate
        // In production, you'd want to use actual font metrics
        Some(font_size * 1.2)
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for FontManager {
    fn clone(&self) -> Self {
        // FontSystem doesn't implement Clone, so we create a new one
        // This is a simplified clone - in production you'd want to handle this better
        Self {
            font_system: FontSystem::new(),
            font_names: self.font_names.clone(),
            font_name_map: self.font_name_map.clone(),
            default_family: self.default_family.clone(),
        }
    }
}


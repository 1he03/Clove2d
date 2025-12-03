use crate::layer::Layer;
use crate::text::TextStyle;
use crate::geometry::Point;
use crate::error::Result;
use cosmic_text::{Buffer, Attrs, Family, Weight, Style as CosmicStyle, Shaping, SwashCache, CacheKey, SubpixelBin, CacheKeyFlags};
use tiny_skia::Pixmap;

/// Text builder for drawing text
pub struct TextBuilder<'a> {
    layer: &'a mut Layer,
    text: String,
    style: TextStyle,
    position: Point,
}

impl<'a> TextBuilder<'a> {
    pub fn new(layer: &'a mut Layer, text: &str, style: TextStyle, position: Point) -> Self {
        Self {
            layer,
            text: text.to_string(),
            style,
            position,
        }
    }
    
    pub fn style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }
    
    pub fn font_family(mut self, family: &str) -> Self {
        self.style = self.style.font_family(family);
        self
    }
    
    pub fn font_size(mut self, size: f32) -> Self {
        self.style = self.style.font_size(size);
        self
    }
    
    pub fn color(mut self, color: crate::color::Color) -> Self {
        self.style = self.style.color(color);
        self
    }
    
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = Point::new(x, y);
        self
    }
    
    pub fn align(mut self, align: crate::text::TextAlign) -> Self {
        self.style = self.style.align(align);
        self
    }
    
    pub fn width(mut self, width: crate::text::TextWidth) -> Self {
        self.style = self.style.width(width);
        self
    }
    
    pub fn font_weight(mut self, weight: crate::text::FontWeight) -> Self {
        self.style = self.style.font_weight(weight);
        self
    }
    
    /// Draw text - uses FontManager from Layer
    pub fn draw(self) -> Result<&'a mut Layer> {
        // Get FontManager from layer
        let mut font_manager_guard = self.layer.font_manager()
            .ok_or_else(|| {
                crate::error::CloveError::InvalidState("FontManager not set on Canvas".to_string())
            })?;
        
        let font_family = font_manager_guard.get_font_family(&self.style.font_family)
            .ok_or_else(|| {
                crate::error::CloveError::InvalidState(
                    format!("Font '{}' not found", self.style.font_family)
                )
            })?
            .clone();
        
        // Calculate buffer size based on TextWidth
        let (width, _height) = self.layer.dimensions();
        let buffer_width = match self.style.width {
            crate::text::TextWidth::None => None,
            crate::text::TextWidth::Max(w) => Some(w),
            crate::text::TextWidth::FullPage => Some(width as f32),
            crate::text::TextWidth::Layer => Some(self.layer.width.unwrap_or(width) as f32),
        };
        
        let line_count = self.text.matches('\n').count() + 1;
        let text_ascent = font_manager_guard.get_text_height(&self.style.font_family, self.style.font_size)
            .unwrap_or(self.style.font_size);
        let estimated_height = (line_count as f32 * self.style.line_height * self.style.font_size).max(text_ascent * 2.0);
        
        // Create buffer for text rendering
        let mut buffer = Buffer::new(
            font_manager_guard.font_system_mut(),
            cosmic_text::Metrics::new(self.style.font_size, self.style.line_height)
        );
        
        buffer.set_size(
            font_manager_guard.font_system_mut(),
            buffer_width,
            Some(estimated_height.max(1000.0)),
        );
        
        // Set text attributes
        let attrs = Attrs::new()
            .family(Family::Name(&font_family))
            .weight(match self.style.font_weight {
                crate::text::FontWeight::Normal => Weight::NORMAL,
                crate::text::FontWeight::Bold => Weight::BOLD,
                crate::text::FontWeight::Light => Weight::LIGHT,
            })
            .style(match self.style.font_style {
                crate::text::FontStyle::Normal => CosmicStyle::Normal,
                crate::text::FontStyle::Italic => CosmicStyle::Italic,
            })
            .stretch(cosmic_text::Stretch::Normal);
        
        // Detect text direction (simplified - check first character)
        let is_rtl = self.text.chars().next()
            .map(|c| {
                let code = c as u32;
                code >= 0x0600 && code <= 0x06FF // Arabic Unicode range
            })
            .unwrap_or(false);
        
        let shaping = if is_rtl {
            Shaping::Advanced
        } else {
            Shaping::Advanced
        };
        
        buffer.set_text(
            font_manager_guard.font_system_mut(),
            &self.text,
            &attrs,
            shaping,
            None,
        );
        
        buffer.shape_until_scroll(font_manager_guard.font_system_mut(), false);
        
        // Get text color
        let rgba = self.style.color.to_rgba();
        let text_r = rgba.r;
        let text_g = rgba.g;
        let text_b = rgba.b;
        let text_a = rgba.a;
        
        // Create swash cache
        let mut swash_cache = SwashCache::new();
        
        // Get layout runs
        let layout_runs: Vec<_> = buffer.layout_runs().collect();
        let text_ascent = font_manager_guard.get_text_height(&self.style.font_family, self.style.font_size)
            .unwrap_or(self.style.font_size);
        let line_height = self.style.line_height * self.style.font_size;
        
        // Calculate text bounds - use glyph positions relative to buffer (without self.position)
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        
        // Calculate width from glyph positions
        for run in layout_runs.iter() {
            for glyph in run.glyphs.iter() {
                let x = glyph.x;
                min_x = min_x.min(x);
                max_x = max_x.max(x + glyph.w);
            }
        }
        
        // Ensure valid bounds for width
        if min_x == f32::MAX {
            min_x = 0.0;
        }
        if max_x == f32::MIN {
            max_x = 0.0;
        }
        
        // Calculate height based on line count (like old code)
        let actual_height = if layout_runs.is_empty() {
            text_ascent
        } else {
            let last_line_index = (layout_runs.len() - 1) as f32;
            // Height = baseline_y of last line + text_ascent (for the last line itself)
            let baseline_y_last = text_ascent + (last_line_index * line_height);
            baseline_y_last + text_ascent
        };
        
        let text_width = (max_x - min_x).max(1.0).ceil() as u32;
        let text_height = actual_height.max(1.0).ceil() as u32;
        let actual_text_width = (max_x - min_x).max(1.0);
        
        // Create temporary pixmap for text (starts at 0,0)
        let mut text_pixmap = Pixmap::new(text_width, text_height)
            .ok_or_else(|| crate::error::CloveError::InvalidState("Failed to create text pixmap".to_string()))?;
        text_pixmap.fill(tiny_skia::Color::TRANSPARENT);
        
        // Render glyphs to temporary pixmap (relative to text_pixmap origin 0,0)
        for (line_index, run) in layout_runs.iter().enumerate() {
            let baseline_y = text_ascent + (line_index as f32 * line_height);
            
            for glyph in run.glyphs.iter() {
                // Position glyph relative to text_pixmap origin (0,0)
                let x = glyph.x - min_x;
                let y = baseline_y + glyph.y;
                
                // Get cache key
                let (_x_int, x_bin) = SubpixelBin::new(glyph.x);
                let (_y_int, y_bin) = SubpixelBin::new(glyph.y);
                let cache_key = CacheKey {
                    font_id: glyph.font_id,
                    glyph_id: glyph.glyph_id,
                    font_size_bits: glyph.font_size.to_bits(),
                    x_bin,
                    y_bin,
                    flags: CacheKeyFlags::empty(),
                    font_weight: attrs.weight,
                };
                
                // Get glyph image and draw it
                if let Some(image) = swash_cache.get_image(
                    font_manager_guard.font_system_mut(),
                    cache_key,
                ).as_ref() {
                    Self::draw_glyph_to_pixmap(
                        &mut text_pixmap,
                        image,
                        x,
                        y,
                        text_r,
                        text_g,
                        text_b,
                        text_a,
                    );
                }
            }
        }
        
        // Drop font_manager_guard before borrowing layer mutably
        drop(font_manager_guard);
        
        // Calculate container width for alignment
        let (layer_width, _layer_height) = self.layer.dimensions();
        let container_width = match self.style.width {
            crate::text::TextWidth::None => actual_text_width,
            crate::text::TextWidth::Max(w) => w,
            crate::text::TextWidth::FullPage => layer_width as f32,
            crate::text::TextWidth::Layer => self.layer.width.unwrap_or(layer_width) as f32,
        };
        
        // Calculate target position based on alignment
        let target_x = match self.style.align {
            crate::text::TextAlign::Left => self.position.x,
            crate::text::TextAlign::Center => self.position.x + (container_width - actual_text_width) / 2.0,
            crate::text::TextAlign::Right => self.position.x + container_width - actual_text_width,
        };
        let target_y = self.position.y;
        
        // Get pixmap from layer
        let mut pixmap = self.layer.get_pixmap_mut()?;
        
        // Draw text pixmap onto layer pixmap at calculated position
        let paint = tiny_skia::PixmapPaint::default();
        pixmap.draw_pixmap(
            target_x.max(0.0) as i32,
            target_y.max(0.0) as i32,
            text_pixmap.as_ref(),
            &paint,
            tiny_skia::Transform::identity(),
            None,
        );
        
        // Update layer content from pixmap
        self.layer.update_from_pixmap(&pixmap)?;
        
        Ok(self.layer)
    }
    
    /// Draw a glyph image to pixmap
    fn draw_glyph_to_pixmap(
        pixmap: &mut Pixmap,
        image: &cosmic_text::SwashImage,
        x: f32,
        y: f32,
        text_r: u8,
        text_g: u8,
        text_b: u8,
        text_a: u8,
    ) {
        let data = &image.data;
        let width = image.placement.width as usize;
        let height = image.placement.height as usize;
        let stride = width;
        
        let start_x = (x + image.placement.left as f32) as i32;
        let start_y = (y - image.placement.top as f32) as i32;
        
        let width_i32 = width as i32;
        let height_i32 = height as i32;
        let pixmap_width_i32 = pixmap.width() as i32;
        let pixmap_height_i32 = pixmap.height() as i32;
        let pixmap_stride = pixmap.width() as usize;
        
        if (start_x + width_i32 < 0) || (start_x >= pixmap_width_i32) ||
           (start_y + height_i32 < 0) || (start_y >= pixmap_height_i32) {
            return;
        }
        
        let text_r_f32 = text_r as f32;
        let text_g_f32 = text_g as f32;
        let text_b_f32 = text_b as f32;
        let text_a_f32 = text_a as f32;
        let inv_255 = 1.0 / 255.0;
        
        let pixmap_data = pixmap.data_mut();
        let pixmap_width = pixmap_width_i32;
        let pixmap_height = pixmap_height_i32;
        let pixmap_data_len = pixmap_data.len();
        
        for row in 0..height {
            let py = start_y + row as i32;
            if py < 0 || py >= pixmap_height {
                continue;
            }
            
            let py_u32 = py as u32;
            let row_base_index = (py_u32 as usize * pixmap_stride) * 4;
            
            for col in 0..width {
                let src_index = row * stride + col;
                if src_index >= data.len() {
                    continue;
                }
                
                let alpha = data[src_index];
                if alpha == 0 {
                    continue;
                }
                
                let px = start_x + col as i32;
                if px < 0 || px >= pixmap_width {
                    continue;
                }
                
                let px_u32 = px as u32;
                let col_offset = (px_u32 as usize) * 4;
                let index = row_base_index + col_offset;
                
                if index + 3 >= pixmap_data_len {
                    continue;
                }
                
                // Read background pixel
                let bg_a = pixmap_data[index + 3];
                
                let (r, g, b, a) = if bg_a > 0 {
                    // Background exists - blend
                    let bg_r = pixmap_data[index];
                    let bg_g = pixmap_data[index + 1];
                    let bg_b = pixmap_data[index + 2];
                    
                    // Unpremultiply background
                    let bg_a_f32 = bg_a as f32;
                    let inv_bg_a = 255.0 / bg_a_f32;
                    let bg_r_f32 = bg_r as f32 * inv_bg_a;
                    let bg_g_f32 = bg_g as f32 * inv_bg_a;
                    let bg_b_f32 = bg_b as f32 * inv_bg_a;
                    
                    // Blend with text color
                    let alpha_f = alpha as f32 * inv_255;
                    let inv_alpha_f = 1.0 - alpha_f;
                    let r = (text_r_f32 * alpha_f + bg_r_f32 * inv_alpha_f) as u8;
                    let g = (text_g_f32 * alpha_f + bg_g_f32 * inv_alpha_f) as u8;
                    let b = (text_b_f32 * alpha_f + bg_b_f32 * inv_alpha_f) as u8;
                    let a = (text_a_f32 * alpha_f + bg_a_f32 * inv_alpha_f) as u8;
                    
                    (r, g, b, a)
                } else {
                    // No background - just use text color
                    (text_r, text_g, text_b, alpha)
                };
                
                // Premultiply for tiny_skia
                let a_f32 = a as f32;
                let premult_factor = a_f32 * inv_255;
                
                // Write pixels
                pixmap_data[index] = (r as f32 * premult_factor) as u8;
                pixmap_data[index + 1] = (g as f32 * premult_factor) as u8;
                pixmap_data[index + 2] = (b as f32 * premult_factor) as u8;
                pixmap_data[index + 3] = a;
            }
        }
    }
}


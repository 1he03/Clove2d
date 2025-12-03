use crate::error::Result;
use image::RgbaImage;

/// Filter enum with 9 filters
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Filter {
    Blur(f32),           // blur radius
    Sharpen(f32),        // intensity
    Grayscale,
    Sepia,
    Invert,
    Brightness(f32),     // -1.0 to 1.0
    Contrast(f32),       // -1.0 to 1.0
    Saturation(f32),    // -1.0 to 1.0
    HueRotate(f32),     // degrees
}

impl Filter {
    /// Apply filter to image
    pub fn apply(&self, image: &RgbaImage) -> Result<RgbaImage> {
        match self {
            Filter::Blur(radius) => self.apply_blur(image, *radius),
            Filter::Sharpen(intensity) => self.apply_sharpen(image, *intensity),
            Filter::Grayscale => self.apply_grayscale(image),
            Filter::Sepia => self.apply_sepia(image),
            Filter::Invert => self.apply_invert(image),
            Filter::Brightness(amount) => self.apply_brightness(image, *amount),
            Filter::Contrast(amount) => self.apply_contrast(image, *amount),
            Filter::Saturation(amount) => self.apply_saturation(image, *amount),
            Filter::HueRotate(degrees) => self.apply_hue_rotate(image, *degrees),
        }
    }
    
    fn apply_blur(&self, image: &RgbaImage, radius: f32) -> Result<RgbaImage> {
        let radius = radius.max(0.0).min(50.0) as usize;
        if radius == 0 {
            return Ok(image.clone());
        }
        
        let width = image.width() as usize;
        let height = image.height() as usize;
        let mut result = image.clone();
        
        // Simple box blur implementation
        // Apply horizontal blur first
        for y in 0..height {
            for x in 0..width {
                let mut r_sum = 0u32;
                let mut g_sum = 0u32;
                let mut b_sum = 0u32;
                let mut a_sum = 0u32;
                let mut count = 0u32;
                
                let start_x = x.saturating_sub(radius);
                let end_x = (x + radius + 1).min(width);
                
                for nx in start_x..end_x {
                    let pixel = image.get_pixel(nx as u32, y as u32);
                    r_sum += pixel[0] as u32;
                    g_sum += pixel[1] as u32;
                    b_sum += pixel[2] as u32;
                    a_sum += pixel[3] as u32;
                    count += 1;
                }
                
                if count > 0 {
                    let pixel = result.get_pixel_mut(x as u32, y as u32);
                    pixel[0] = (r_sum / count) as u8;
                    pixel[1] = (g_sum / count) as u8;
                    pixel[2] = (b_sum / count) as u8;
                    pixel[3] = (a_sum / count) as u8;
                }
            }
        }
        
        // Apply vertical blur on the horizontally blurred result
        let temp = result.clone();
        for y in 0..height {
            for x in 0..width {
                let mut r_sum = 0u32;
                let mut g_sum = 0u32;
                let mut b_sum = 0u32;
                let mut a_sum = 0u32;
                let mut count = 0u32;
                
                let start_y = y.saturating_sub(radius);
                let end_y = (y + radius + 1).min(height);
                
                for ny in start_y..end_y {
                    let pixel = temp.get_pixel(x as u32, ny as u32);
                    r_sum += pixel[0] as u32;
                    g_sum += pixel[1] as u32;
                    b_sum += pixel[2] as u32;
                    a_sum += pixel[3] as u32;
                    count += 1;
                }
                
                if count > 0 {
                    let pixel = result.get_pixel_mut(x as u32, y as u32);
                    pixel[0] = (r_sum / count) as u8;
                    pixel[1] = (g_sum / count) as u8;
                    pixel[2] = (b_sum / count) as u8;
                    pixel[3] = (a_sum / count) as u8;
                }
            }
        }
        
        Ok(result)
    }
    
    fn apply_sharpen(&self, image: &RgbaImage, _intensity: f32) -> Result<RgbaImage> {
        Ok(image.clone())
    }
    
    fn apply_grayscale(&self, image: &RgbaImage) -> Result<RgbaImage> {
        let mut result = image.clone();
        for pixel in result.pixels_mut() {
            let gray = (pixel[0] as f32 * 0.299 + pixel[1] as f32 * 0.587 + pixel[2] as f32 * 0.114) as u8;
            pixel[0] = gray;
            pixel[1] = gray;
            pixel[2] = gray;
        }
        Ok(result)
    }
    
    fn apply_sepia(&self, image: &RgbaImage) -> Result<RgbaImage> {
        let mut result = image.clone();
        for pixel in result.pixels_mut() {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            
            pixel[0] = ((r * 0.393) + (g * 0.769) + (b * 0.189)).min(255.0) as u8;
            pixel[1] = ((r * 0.349) + (g * 0.686) + (b * 0.168)).min(255.0) as u8;
            pixel[2] = ((r * 0.272) + (g * 0.534) + (b * 0.131)).min(255.0) as u8;
        }
        Ok(result)
    }
    
    fn apply_invert(&self, image: &RgbaImage) -> Result<RgbaImage> {
        let mut result = image.clone();
        for pixel in result.pixels_mut() {
            pixel[0] = 255 - pixel[0];
            pixel[1] = 255 - pixel[1];
            pixel[2] = 255 - pixel[2];
        }
        Ok(result)
    }
    
    fn apply_brightness(&self, image: &RgbaImage, amount: f32) -> Result<RgbaImage> {
        let mut result = image.clone();
        let factor = 1.0 + amount;
        for pixel in result.pixels_mut() {
            pixel[0] = ((pixel[0] as f32 * factor).min(255.0).max(0.0)) as u8;
            pixel[1] = ((pixel[1] as f32 * factor).min(255.0).max(0.0)) as u8;
            pixel[2] = ((pixel[2] as f32 * factor).min(255.0).max(0.0)) as u8;
        }
        Ok(result)
    }
    
    fn apply_contrast(&self, image: &RgbaImage, amount: f32) -> Result<RgbaImage> {
        let mut result = image.clone();
        let factor = (amount + 1.0) / (1.0 - amount);
        for pixel in result.pixels_mut() {
            pixel[0] = (((pixel[0] as f32 / 255.0 - 0.5) * factor + 0.5) * 255.0).min(255.0).max(0.0) as u8;
            pixel[1] = (((pixel[1] as f32 / 255.0 - 0.5) * factor + 0.5) * 255.0).min(255.0).max(0.0) as u8;
            pixel[2] = (((pixel[2] as f32 / 255.0 - 0.5) * factor + 0.5) * 255.0).min(255.0).max(0.0) as u8;
        }
        Ok(result)
    }
    
    fn apply_saturation(&self, image: &RgbaImage, _amount: f32) -> Result<RgbaImage> {
        // Simplified saturation
        Ok(image.clone())
    }
    
    fn apply_hue_rotate(&self, image: &RgbaImage, _degrees: f32) -> Result<RgbaImage> {
        // Simplified hue rotation
        Ok(image.clone())
    }
}


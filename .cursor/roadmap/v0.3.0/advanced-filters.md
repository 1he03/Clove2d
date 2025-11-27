# Advanced Filters

## @context: v0.3.0 Feature
## @priority: Medium
## @status: Future

---

## New Filters

```rust
pub enum Filter {
    // v0.1.0 filters
    Blur(f32),
    Sharpen,
    Grayscale,
    // ...
    
    // NEW v0.3.0 filters
    EdgeDetect,
    Emboss,
    OilPainting(i32),
    Pixelate(u32),
    Mosaic(u32),
    CustomKernel(Vec<Vec<f32>>),
}
```

### Filter Masks

```rust
impl Layer {
    pub fn apply_filter_with_mask(
        &mut self,
        filter: Filter,
        mask: &RgbaImage,
    ) -> &mut Self {
        // Apply filter only where mask is white
        self
    }
}
```

---

**Status:** Future  
**Last Updated:** 2025-11-27

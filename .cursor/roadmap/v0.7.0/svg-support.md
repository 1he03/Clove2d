# SVG Support

## @context: v0.7.0 Feature
## @priority: High
## @status: Future

---

## SVG Import

```rust
impl Canvas {
    pub fn load_svg(&mut self, path: &str) -> Result<&mut Self> {
        let svg = SvgParser::parse_file(path)?;
        
        for element in svg.elements {
            self.add_svg_element(element)?;
        }
        
        Ok(self)
    }
}
```

## SVG Export

```rust
impl Canvas {
    pub fn save_svg(&self, path: &str) -> Result<()> {
        let svg = SvgBuilder::new()
            .size(self.width, self.height)
            .add_layers(&self.layers)
            .build();
        
        std::fs::write(path, svg.to_string())?;
        Ok(())
    }
}
```

---

**Dependencies:**
- `resvg` or `usvg` for SVG parsing
- `svg` crate for SVG generation

**Tasks:**
- [ ] Implement SVG parser
- [ ] Convert SVG to Clove2d
- [ ] Implement SVG export
- [ ] Handle transformations
- [ ] Write SVG tests
- [ ] Add examples

---

**Status:** Future  
**Last Updated:** 2025-11-27

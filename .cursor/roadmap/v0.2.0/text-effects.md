# Text Effects

## @context: v0.2.0 Feature
## @priority: Medium
## @status: Future

---

## Features

### 1. Text on Path

```rust
impl TextBuilder {
    pub fn follow_path(mut self, path: &Path) -> Self {
        self.path = Some(path.clone());
        self
    }
}
```

### 2. Text Outline & Glow

```rust
impl TextBuilder {
    pub fn outline(mut self, color: Color, width: f32) -> Self {
        self.outline = Some((color, width));
        self
    }
    
    pub fn glow(mut self, color: Color, radius: f32) -> Self {
        self.glow = Some((color, radius));
        self
    }
}
```

### 3. Enhanced Emoji

- Full color emoji rendering
- Emoji variant selectors
- Emoji sequence support

---

**Status:** Future  
**Last Updated:** 2025-11-27

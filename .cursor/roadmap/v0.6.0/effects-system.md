# Effects System

## @context: v0.6.0 Feature
## @priority: High
## @status: Future

---

## Layer Effects

```rust
pub enum LayerEffect {
    DropShadow {
        color: Color,
        offset: Point,
        blur: f32,
        spread: f32,
    },
    InnerShadow {
        color: Color,
        offset: Point,
        blur: f32,
    },
    OuterGlow {
        color: Color,
        size: f32,
    },
    InnerGlow {
        color: Color,
        size: f32,
    },
    Bevel {
        depth: f32,
        size: f32,
    },
}

impl Layer {
    pub fn add_effect(&mut self, effect: LayerEffect) -> &mut Self {
        self.effects.push(effect);
        self
    }
}
```

---

## Masking

```rust
impl Layer {
    pub fn set_clipping_mask(&mut self, mask: &Layer) -> &mut Self {
        self.clipping_mask = Some(mask.id);
        self
    }
    
    pub fn set_alpha_mask(&mut self, mask: &RgbaImage) -> &mut Self {
        self.alpha_mask = Some(mask.clone());
        self
    }
}
```

---

**Status:** Future  
**Last Updated:** 2025-11-27

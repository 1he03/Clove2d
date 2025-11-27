# Advanced Text Layout

## @context: v0.2.0 Feature
## @priority: High
## @status: Future

---

## Features

### 1. Text Justification

```rust
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,          // Basic (v0.1.0)
    JustifyAll,       // NEW: Justify last line too
    JustifyDistribute, // NEW: Distribute evenly
}
```

### 2. Hyphenation

```rust
pub struct HyphenationConfig {
    enabled: bool,
    language: String,
    min_word_length: usize,
}

impl TextBuilder {
    pub fn hyphenation(mut self, config: HyphenationConfig) -> Self {
        self.hyphenation = Some(config);
        self
    }
}
```

### 3. Advanced Line Breaking

- Knuth-Plass algorithm
- Widow/orphan control
- Keep-together rules
- Better word spacing

---

## Implementation

**Dependencies:**
- `hyphenation` crate for language-specific hyphenation
- `unicode-linebreak` for better line breaking

**Tasks:**
- [ ] Implement justification algorithms
- [ ] Add hyphenation support
- [ ] Improve line breaking
- [ ] Handle BiDi edge cases
- [ ] Write tests
- [ ] Add examples

---

**Status:** Future  
**Last Updated:** 2025-11-27

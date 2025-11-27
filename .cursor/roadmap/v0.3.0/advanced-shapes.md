# Advanced Shapes

## @context: v0.3.0 Feature
## @priority: High
## @status: Future

---

## Features

### 1. Custom Path Builder

```rust
pub struct PathBuilder {
    commands: Vec<PathCommand>,
}

impl PathBuilder {
    pub fn move_to(mut self, x: f32, y: f32) -> Self { /* ... */ }
    pub fn line_to(mut self, x: f32, y: f32) -> Self { /* ... */ }
    pub fn curve_to(mut self, cp1: Point, cp2: Point, end: Point) -> Self { /* ... */ }
    pub fn arc_to(mut self, /* ... */) -> Self { /* ... */ }
    pub fn close(mut self) -> Self { /* ... */ }
    pub fn build(self) -> Path { /* ... */ }
}
```

### 2. Boolean Operations

```rust
impl Shape {
    pub fn union(&self, other: &Shape) -> Shape { /* ... */ }
    pub fn intersect(&self, other: &Shape) -> Shape { /* ... */ }
    pub fn subtract(&self, other: &Shape) -> Shape { /* ... */ }
    pub fn xor(&self, other: &Shape) -> Shape { /* ... */ }
}
```

### 3. Shape Morphing

```rust
impl Shape {
    pub fn morph_to(&self, target: &Shape, progress: f32) -> Shape { /* ... */ }
}
```

---

**Status:** Future  
**Last Updated:** 2025-11-27

# Phase 3: Shapes System

## @context: v0.1.0 Phase 3
## @timeline: Week 5-6 (2 weeks)
## @priority: Critical
## @status: Planning
## @dependencies: Phase 2 (Canvas Core)
## @last-updated: 2025-11-27

---

## Table of Contents

1. [Overview](#overview)
2. [Objectives](#objectives)
3. [Shape Implementations](#shape-implementations)
4. [Builder Patterns](#builder-patterns)
5. [Testing Requirements](#testing-requirements)
6. [Definition of Done](#definition-of-done)

---

## Overview

Phase 3 implements the complete shapes system with 12 different shapes, each supporting fill, stroke, shadow, and transform operations.

### Shapes to Implement

1. Rectangle
2. Rounded Rectangle
3. Circle
4. Ellipse
5. Line
6. Polyline
7. Polygon
8. Triangle
9. Arc
10. Bezier Curve (Quadratic & Cubic)
11. Path (Custom)
12. Star

---

## Objectives

### Primary Goals

- ✅ Implement all 12 shapes
- ✅ Support fill, stroke, shadow
- ✅ Builder pattern for each shape
- ✅ High-quality anti-aliasing

### Success Metrics

- [ ] All shapes render correctly
- [ ] Draw 1000 shapes <100ms
- [ ] Anti-aliasing quality excellent
- [ ] Stroke widths accurate

---

## Shape Implementations

### 1. Rectangle

**File:** `src/shapes/rectangle.rs`

```rust
pub struct RectBuilder<'a> {
    layer: &'a mut Layer,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    fill: Option<Color>,
    stroke: Option<(Color, f32)>,
    shadow: Option<Shadow>,
    opacity: f32,
}

impl<'a> RectBuilder<'a> {
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }
    
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    
    pub fn fill(mut self, color: Color) -> Self {
        self.fill = Some(color);
        self
    }
    
    pub fn stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke = Some((color, width));
        self
    }
    
    pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.shadow = Some(Shadow { color, offset_x: x, offset_y: y, blur });
        self
    }
    
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
    
    pub fn draw(self) -> Result<&'a mut Layer> {
        // Draw rectangle using backend
        Ok(self.layer)
    }
}
```

**Tasks:**
- [ ] Implement RectBuilder
- [ ] Add all styling options
- [ ] Integrate with backend
- [ ] Write tests
- [ ] Add examples

---

### 2. Rounded Rectangle

**File:** `src/shapes/rounded_rect.rs`

```rust
pub struct RoundedRectBuilder<'a> {
    layer: &'a mut Layer,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    corner_radius: f32,
    fill: Option<Color>,
    stroke: Option<(Color, f32)>,
}

impl<'a> RoundedRectBuilder<'a> {
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }
    
    pub fn draw(self) -> Result<&'a mut Layer> {
        // Draw rounded rectangle
        Ok(self.layer)
    }
}
```

---

### 3. Circle

**File:** `src/shapes/circle.rs`

```rust
pub struct CircleBuilder<'a> {
    layer: &'a mut Layer,
    center_x: f32,
    center_y: f32,
    radius: f32,
    fill: Option<Color>,
    stroke: Option<(Color, f32)>,
}

impl<'a> CircleBuilder<'a> {
    pub fn center(mut self, x: f32, y: f32) -> Self {
        self.center_x = x;
        self.center_y = y;
        self
    }
    
    pub fn radius(mut self, r: f32) -> Self {
        self.radius = r;
        self
    }
    
    pub fn draw(self) -> Result<&'a mut Layer> {
        // Draw circle
        Ok(self.layer)
    }
}
```

---

### 4-12. Other Shapes

Similar implementation pattern for:
- Ellipse (cx, cy, rx, ry)
- Line (x1, y1, x2, y2)
- Polyline (points: Vec<Point>)
- Polygon (points: Vec<Point>)
- Triangle (p1, p2, p3)
- Arc (center, radius, start_angle, end_angle)
- Bezier (control points)
- Path (custom path commands)
- Star (center, points, inner_radius, outer_radius)

---

## Testing Requirements

### Visual Tests

```rust
#[test]
fn test_all_shapes() -> Result<()> {
    let mut canvas = Canvas::new(1200, 800)?;
    
    canvas.create_layer("shapes")?
        .draw_rect().position(50, 50).size(100, 100).fill(Color::RED).draw()?
        .draw_circle().center(250, 100).radius(50).fill(Color::BLUE).draw()?
        .draw_line().from(350, 50).to(450, 150).stroke(Color::BLACK, 2.0).draw()?;
    
    canvas.save("all_shapes.png")?;
    Ok(())
}
```

---

## Definition of Done

- [ ] All 12 shapes implemented
- [ ] All builders working
- [ ] All tests passing
- [ ] Documentation complete
- [ ] Examples added

---

**Phase:** 3/6  
**Status:** Planning  
**Duration:** 2 weeks  
**Last Updated:** 2025-11-27

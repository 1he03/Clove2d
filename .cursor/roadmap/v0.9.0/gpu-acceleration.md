# GPU Acceleration

## @context: v0.9.0 Feature
## @priority: Medium
## @status: Future

---

## GPU Backend

```rust
#[cfg(feature = "gpu")]
pub struct WgpuBackend {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Canvas {
    #[cfg(feature = "gpu")]
    pub fn with_gpu() -> CanvasBuilder {
        CanvasBuilder::new()
            .backend(BackendType::Gpu)
    }
}
```

## Feature Flag

```toml
[features]
default = ["cpu"]
cpu = []
gpu = ["wgpu"]
```

---

**Dependencies:**
- `wgpu` for GPU access

**Tasks:**
- [ ] Implement WgpuBackend
- [ ] Add GPU shaders
- [ ] Implement fallback
- [ ] Write GPU tests
- [ ] Benchmark GPU vs CPU
- [ ] Add examples

---

**Status:** Future  
**Last Updated:** 2025-11-27

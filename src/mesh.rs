use eframe::egui_wgpu::wgpu;
use egui::{pos2, Pos2, Rect};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: Pos2,
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];

    pub fn new(pos: Pos2) -> Self {
        Self { pos }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn make_triangle() -> Self {
        Self {
            vertices: vec![
                Vertex {
                    pos: pos2(0.0, 0.5),
                },
                Vertex {
                    pos: pos2(-0.5, -0.5),
                },
                Vertex {
                    pos: pos2(0.5, -0.5),
                },
            ],
            indices: vec![0, 1, 2],
        }
    }

    pub fn bounds(&self) -> Rect {
        let mut min = Pos2::new(f32::MAX, f32::MAX);
        let mut max = Pos2::new(f32::MIN, f32::MIN);

        for vertex in &self.vertices {
            min.x = min.x.min(vertex.pos.x);
            min.y = min.y.min(vertex.pos.y);
            max.x = max.x.max(vertex.pos.x);
            max.y = max.y.max(vertex.pos.y);
        }

        Rect::from_min_max(min, max)
    }
}

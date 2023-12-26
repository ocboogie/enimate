use crate::mesh::Vertex;
use crate::object::{Material, Object};
use crate::world::{ObjectId, World};
use eframe::wgpu::ColorTargetState;
use eframe::{
    egui_wgpu::{self, wgpu},
    wgpu::util::DeviceExt,
};
use egui::{Rect, Rgba};
use rand::Rng;
use std::collections::HashMap;

struct RendererResourceManager(pub HashMap<usize, RendererResources>);

pub struct Renderer {
    id: usize,
}

impl Renderer {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
        let wgpu_render_state = cc.wgpu_render_state.as_ref()?;

        let device = &wgpu_render_state.device;

        let id = rand::thread_rng().gen::<usize>();

        let resources = Self::init(device, Some(wgpu_render_state.target_format.into()));

        {
            let mut wgpu_renderer = wgpu_render_state.renderer.write();

            if let Some(manager) = wgpu_renderer
                .callback_resources
                .get_mut::<RendererResourceManager>()
            {
                manager.0.insert(id, resources);
            } else {
                let mut map = HashMap::new();
                map.insert(id, resources);

                wgpu_renderer
                    .callback_resources
                    .insert(RendererResourceManager(map));
            }
        }

        Some(Self { id })
    }

    fn init<'a>(
        device: &'a wgpu::Device,
        custom_targets: Option<ColorTargetState>,
    ) -> RendererResources {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // Since we don't know the screen size yet, we don't initialize the camera buffer yet
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Buffer"),
            size: 16,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let mesh_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Mesh Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &mesh_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &(if let Some(targets) = custom_targets {
                    [Some(targets)]
                } else {
                    // FIXME::::::
                    [None]
                }),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        RendererResources {
            pipeline,
            camera_bind_group,
            camera_buffer,
            loaded_meshes: HashMap::new(),
        }
    }
}

struct RendererCallback {
    id: usize,
    world: World,
    render_size: egui::Vec2,
}

impl RendererCallback {
    fn get_mut_resources<'a>(
        &self,
        cb_resources: &'a mut egui_wgpu::CallbackResources,
    ) -> &'a mut RendererResources {
        let resource_manager: &mut RendererResourceManager =
            cb_resources.get_mut().expect("No resource manager");
        resource_manager.0.get_mut(&self.id).unwrap()
    }

    fn get_resources<'a>(
        &self,
        cb_resources: &'a egui_wgpu::CallbackResources,
    ) -> &'a RendererResources {
        let resource_manager: &RendererResourceManager =
            cb_resources.get().expect("No resource manager");
        resource_manager.0.get(&self.id).unwrap()
    }
}

impl egui_wgpu::CallbackTrait for RendererCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _egui_encoder: &mut wgpu::CommandEncoder,
        cb_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources = self.get_mut_resources(cb_resources);

        resources.prepare(device, queue, &self.world, self.render_size);
        Vec::new()
    }

    fn paint<'a>(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'a>,
        cb_resources: &'a egui_wgpu::CallbackResources,
    ) {
        let resources = self.get_resources(cb_resources);
        resources.paint(render_pass, &self.world);
    }
}

impl Renderer {
    pub fn paint_at(&mut self, ui: &mut egui::Ui, rect: Rect, world: World) {
        ui.painter_at(rect)
            .add(egui_wgpu::Callback::new_paint_callback(
                rect,
                RendererCallback {
                    id: self.id,
                    world,
                    render_size: rect.size(),
                },
            ));
    }
}

// Same as Material but we convert the color to rgba
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct MaterialGpu {
    color: Rgba,
}

impl From<Material> for MaterialGpu {
    fn from(material: Material) -> Self {
        Self {
            color: Rgba::from(material.color),
        }
    }
}
struct LoadedMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub transform_buffer: wgpu::Buffer,
    pub material_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

struct RendererResources {
    pipeline: wgpu::RenderPipeline,
    camera_bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
    loaded_meshes: HashMap<ObjectId, LoadedMesh>,
}

impl RendererResources {
    fn load_mesh(&mut self, device: &wgpu::Device, id: &ObjectId, object: &Object) {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Vertex Buffer: {}", id)),
            contents: bytemuck::cast_slice(&object.mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Index Buffer: {}", id)),
            contents: bytemuck::cast_slice(&object.mesh.indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Transform Buffer: {}", id)),
            // No need for padding, since transform is 16 bytes aligned.
            contents: bytemuck::bytes_of(&object.transform),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });
        let material_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Material Buffer: {}", id)),
            // No need for padding, since material is 16 bytes aligned.
            contents: bytemuck::bytes_of(&MaterialGpu::from(object.material.clone())),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("Bind Group: {}", id)),
            layout: &self.pipeline.get_bind_group_layout(1),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: transform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: material_buffer.as_entire_binding(),
                },
            ],
        });

        self.loaded_meshes.insert(
            *id,
            LoadedMesh {
                vertex_buffer,
                index_buffer,
                transform_buffer,
                material_buffer,
                bind_group,
            },
        );
    }

    fn update_mesh(queue: &wgpu::Queue, object: &Object, loaded_mesh: &LoadedMesh) {
        // FIXME: Might have a bigger size than the vertex buffer, so
        //        just double check that the buffer can grow, which I don't
        //        think it can.
        queue.write_buffer(
            &loaded_mesh.vertex_buffer,
            0,
            bytemuck::cast_slice(&object.mesh.vertices),
        );
        queue.write_buffer(
            &loaded_mesh.index_buffer,
            0,
            bytemuck::cast_slice(&object.mesh.indices),
        );
        queue.write_buffer(
            &loaded_mesh.transform_buffer,
            0,
            bytemuck::bytes_of(&object.transform),
        );
        queue.write_buffer(
            &loaded_mesh.material_buffer,
            0,
            bytemuck::bytes_of(&MaterialGpu::from(object.material.clone())),
        );
    }

    fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        world: &World,
        render_size: egui::Vec2,
    ) {
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&vec![1.0 / render_size.x, 0.0, 0.0, 1.0 / render_size.y]),
        );

        // FIXME: Bad performance, since it is updating the entire buffer every
        //        frame. Should hashing be used to determine if the buffer
        //        needs to be updated?
        for (id, object) in &world.objects {
            if let Some(loaded_mesh) = self.loaded_meshes.get(id) {
                Self::update_mesh(queue, object, loaded_mesh);
            } else {
                self.load_mesh(device, id, object);
            }
        }
    }

    fn paint<'rp>(&'rp self, render_pass: &mut wgpu::RenderPass<'rp>, world: &World) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

        for (id, object) in &world.objects {
            let loaded_mesh = self.loaded_meshes.get(id).expect("Mesh not loaded");

            render_pass.set_vertex_buffer(0, loaded_mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                loaded_mesh.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            render_pass.set_bind_group(1, &loaded_mesh.bind_group, &[]);

            render_pass.draw_indexed(0..object.mesh.indices.len() as u32, 0, 0..1);
        }
    }
}

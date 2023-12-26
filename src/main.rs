use animation::{AnimateTransform, Parallel};
use egui::epaint::{CircleShape, Tessellator};
use renderer::Renderer;
use scene::Scene;
use world::World;

mod animation;
mod animation_ui;
mod mesh;
mod object;
mod renderer;
mod scene;
mod scene_builder;
mod world;

struct App {
    scene: Scene,
    renderer: Renderer,
    play: bool,
    current_time: f32,
    total_time: f32,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.stable_dt) as f32;

        if self.play && self.current_time < self.total_time {
            self.current_time += dt;
        }

        if self.current_time >= self.total_time {
            self.play = false;
            self.current_time = self.total_time;
        }

        self.scene.update(self.current_time / self.total_time);

        ctx.request_repaint();

        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.play = !self.play;
        }

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Animations");
            ui.separator();
            self.scene.animation.ui(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Time: {}", self.current_time));
            if self.play {
                if ui.button("Pause").clicked() {
                    self.play = false;
                }
            } else {
                if ui.button("Play").clicked() {
                    self.play = true;
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Slider::new(&mut self.current_time, 0.0..=self.total_time)
                        .clamp_to_range(true),
                );

                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    let rect = ui.available_rect_before_wrap();
                    let response = ui.allocate_rect(rect, egui::Sense::drag());

                    self.renderer.paint_at(ui, rect, self.scene.world.clone());
                });
            });
        });
    }
}

impl App {
    fn new<'a>(cc: &'a eframe::CreationContext<'a>, scene: Scene) -> Self {
        let renderer = Renderer::new(cc).unwrap();

        Self {
            scene,
            renderer,
            play: true,
            current_time: 0.0,
            total_time: 5.0,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut native_options = eframe::NativeOptions::default();
    native_options.renderer = eframe::Renderer::Wgpu;

    let mut tessellator = Tessellator::new(1.0, Default::default(), [1, 1], vec![]);
    let mut circle = egui::Mesh::default();
    tessellator.tessellate_circle(
        CircleShape::filled(egui::Pos2::ZERO, 1.0, egui::Color32::WHITE),
        &mut circle,
    );

    let circle = mesh::Mesh {
        vertices: circle
            .vertices
            .iter()
            .map(|v| mesh::Vertex::new(v.pos))
            .collect(),
        indices: circle.indices.clone(),
    };

    let mut world = World::default();

    let transform_1 = object::Transform {
        position: egui::Pos2::new(100.0, 0.0),
        rotation: 0.0,
        scale: 300.0,
    };
    world.objects.insert(
        0,
        object::Object {
            mesh: mesh::Mesh::make_triangle(),
            material: object::Material {
                color: egui::Color32::RED,
            },
            transform: transform_1,
        },
    );
    let transform_2 = object::Transform {
        position: egui::Pos2::new(100.0, 0.0),
        rotation: 0.0,
        scale: 100.0,
    };
    world.objects.insert(
        1,
        object::Object {
            mesh: circle,
            material: object::Material {
                color: egui::Color32::BLUE,
            },
            transform: transform_2,
        },
    );

    let animation = Box::new(Parallel::new(vec![
        Box::new(AnimateTransform::new(
            0,
            transform_1.with_rotation(0.0),
            transform_1.with_rotation(std::f32::consts::PI * 2.0),
        )),
        Box::new(AnimateTransform::new(
            1,
            transform_2
                .with_position(egui::Pos2::new(-300.0, 200.0))
                .with_scale(100.0),
            transform_2
                .with_position(egui::Pos2::new(-300.0, -200.0))
                .with_scale(200.0),
        )),
    ]));

    let scene = Scene::new(world, animation);

    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, scene))),
    )?;

    Ok(())
}

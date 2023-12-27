use builder::Builder;
use component::Component;
use egui::{
    epaint::{CircleShape, Tessellator},
    pos2, Color32, Vec2,
};
use lyon::path::Path;
use mesh::Mesh;
use motion::{AnimateTransform, Parallel};
use motion_ui::fixme;
use object::{Material, Model, Object, ObjectId, ObjectKind, Transform};
use renderer::Renderer;
use scene::Scene;
use scene_builder::SceneBuilder;
use world::ObjectTree;

mod builder;
mod component;
mod mesh;
mod motion;
mod motion_ui;
mod object;
mod renderer;
mod scene;
mod scene_builder;
mod utils;
mod world;

struct App {
    scene: Scene,
    renderer: Renderer,
    play: bool,
    current_time: f32,
    total_time: f32,
}

fn show_object(ui: &mut egui::Ui, objects: &ObjectTree, id: ObjectId) {
    let object = objects.get(&id).unwrap();

    ui.collapsing(format!("Object {}", id), |ui| {
        ui.label(format!(
            "Position: ({}, {})",
            object.transform.position.x, object.transform.position.y
        ));
        ui.label(format!("Scale: {}", object.transform.scale));
        ui.label(format!("Rotation: {}", object.transform.rotation));
        ui.label(format!(
            "Anchor: ({}, {})",
            object.transform.anchor.x, object.transform.anchor.y
        ));

        match object.object_kind {
            ObjectKind::Model(_) => {
                ui.label("Model");
            }
            ObjectKind::Group(ref children) => {
                ui.label("Group");
                for child in children {
                    show_object(ui, objects, *child);
                }
            }
        }
    });
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

        ctx.request_repaint();

        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            if self.current_time >= self.total_time {
                self.current_time = 0.0;
                self.play = true;
            } else {
                self.play = !self.play;
            }
        }

        let objects = self.scene.objects_at(self.current_time);

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Animations");
                ui.separator();
                let root = self.scene.root;
                fixme(ui, &mut self.scene, root);
                // self.scene.root_mut().ui(ui, &mut self.scene);
            });
        });

        egui::SidePanel::right("object_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Objects");
                ui.separator();
                show_object(ui, &objects, objects.root);
            });
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

                    self.renderer.paint_at(ui, rect, objects);
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
            total_time: 2.5,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut native_options = eframe::NativeOptions::default();
    native_options.renderer = eframe::Renderer::Wgpu;

    let mut scene_builder = SceneBuilder::new(5.0);

    scene_builder
        .line(
            pos2(0.0, 0.0),
            pos2(0.0, 100.0),
            10.0,
            Color32::GREEN.into(),
        )
        .add();

    let grid = scene_builder
        .build(Grid::new(10, 1.0, Vec2::new(100.0, 100.0), Color32::GREEN))
        .with_scale(10.0)
        .add();
    scene_builder.animate(grid, 0.5, |a| a.fade_in());

    let scene = scene_builder.finish();

    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, scene))),
    )?;

    Ok(())
}

struct Grid {
    lines: usize,
    width: f32,
    size: Vec2,
    color: Color32,
}

impl Grid {
    fn new(lines: usize, width: f32, size: Vec2, color: Color32) -> Self {
        Self {
            lines,
            width,
            size,
            color,
        }
    }
}

impl Component for Grid {
    fn build(&self, builder: &mut impl Builder) {
        let mut pbuilder = Path::builder();

        let horizontal_stride = self.size.x / self.lines as f32;
        for x in 0..=self.lines {
            pbuilder.begin(lyon::math::point(
                x as f32 * horizontal_stride - self.size.x / 2.0,
                -self.size.y / 2.0,
            ));
            pbuilder.line_to(lyon::math::point(
                x as f32 * horizontal_stride - self.size.x / 2.0,
                self.size.y / 2.0,
            ));
            pbuilder.close();
        }

        let vertical_stride = self.size.y / self.lines as f32;
        for y in 0..=self.lines {
            pbuilder.begin(lyon::math::point(
                -self.size.x / 2.0,
                y as f32 * vertical_stride - self.size.y / 2.0,
            ));
            pbuilder.line_to(lyon::math::point(
                self.size.x / 2.0,
                y as f32 * vertical_stride - self.size.y / 2.0,
            ));
            pbuilder.close();
        }

        let path = pbuilder.build();
        builder.path(path, self.width, self.color.into()).add();
    }
}

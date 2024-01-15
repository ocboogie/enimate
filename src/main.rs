use std::collections::HashMap;

// use component::Component;
use egui::{pos2, Color32, Stroke, Vec2};
use lyon::{math::point, path::Path};

use motion::{AddObject, Motion, MotionId};
// use building::{Builder, SceneBuilder};
use motion_ui::fixme;
use object::{
    FillMaterial, Material, Model, Object, ObjectId, ObjectKind, StrokeMaterial, Transform,
};
use object_tree::ObjectTree;
use renderer::Renderer;
use scene::Scene;

// mod building;
// mod component;
mod mesh;
mod motion;
mod motion_ui;
mod object;
mod object_tree;
mod renderer;
mod scene;
mod utils;
mod world;

struct App {
    current_scene: usize,
    scenes: Vec<(&'static str, Scene)>,
    renderer: Renderer,
    play: bool,
    current_time: f32,
}

impl App {
    fn scene(&self) -> &Scene {
        &self.scenes[self.current_scene].1
    }

    fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scenes[self.current_scene].1
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.stable_dt) as f32;

        if self.play && self.current_time < self.scene().length {
            self.current_time += dt;
        }

        if self.current_time >= self.scene().length {
            self.play = false;
            self.current_time = self.scene().length;
        }

        ctx.request_repaint();

        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            if self.current_time >= self.scene().length {
                self.current_time = 0.0;
                self.play = true;
            } else {
                self.play = !self.play;
            }
        }

        let current_time = self.current_time;
        let objects = self.scene_mut().objects_at(current_time);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::ComboBox::from_label("Scene")
                .selected_text(format!("{}", self.scenes[self.current_scene].0))
                .show_ui(ui, |ui| {
                    for (i, (name, _)) in self.scenes.iter().enumerate() {
                        if ui
                            .selectable_value(&mut self.current_scene, i, *name)
                            .clicked()
                        {
                            self.current_time = 0.0;
                            self.play = true;
                        }
                    }
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
                let length = self.scene().length;

                ui.add(
                    egui::Slider::new(&mut self.current_time, 0.0..=length).clamp_to_range(true),
                );

                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    let rect = ui.available_rect_before_wrap();
                    let _response = ui.allocate_rect(rect, egui::Sense::drag());

                    // let boxes = objects.bounding_boxes();

                    self.renderer.paint_at(ui, rect, objects);

                    // if false {
                    //     let bb_canvas = ui.painter_at(rect);
                    //     for (_id, bb) in boxes {
                    //         bb_canvas.rect_stroke(
                    //             bb.translate(rect.center().to_vec2()),
                    //             0.0,
                    //             Stroke::new(1.0, Color32::RED),
                    //         );
                    //     }
                    // }
                });
            });
        });
    }
}

impl App {
    fn new<'a>(cc: &'a eframe::CreationContext<'a>, scenes: Vec<(&'static str, Scene)>) -> Self {
        let renderer = Renderer::new(cc).unwrap();

        Self {
            current_scene: 0,
            scenes,
            renderer,
            play: true,
            current_time: 0.0,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut native_options = eframe::NativeOptions::default();
    native_options.renderer = eframe::Renderer::Wgpu;

    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, vec![("Path", path())]))),
    )?;

    Ok(())
}

fn path() -> Scene {
    let mut motions: HashMap<MotionId, Box<dyn Motion>> = HashMap::new();
    let root: MotionId = rand::random::<usize>();

    let mut builder = Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(1.0, 2.0));
    builder.line_to(point(2.0, 0.0));
    builder.line_to(point(1.0, 1.0));
    builder.close();
    let path = builder.build();

    motions.insert(
        root,
        Box::new(AddObject {
            object: Object {
                object_kind: ObjectKind::Model(Model {
                    path,
                    material: Material {
                        fill: Some(FillMaterial::new(Color32::RED)),
                        stroke: Some(StrokeMaterial::new(Color32::BLUE, 0.1)),
                    },
                }),
                transform: Transform::default().with_scale(100.0),
            },
            rooted: true,
            object_id: 123,
        }),
    );

    Scene::new(motions, root, 2.0)
}

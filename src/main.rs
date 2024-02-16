use std::collections::HashMap;

use building::{Alignment, Builder, SceneBuilder};
use dynamics::{DynamicTransform, DynamicValue};
// use component::Component;
use egui::{pos2, Color32, Pos2, Stroke, Vec2};
use lyon::{
    math::point,
    path::{traits::PathBuilder, Path, Winding},
};

use motion::{AddObject, Motion, MotionId, Parallel, Rotate, Sequence, SetTransform, SetVariable};
// use building::{Builder, SceneBuilder};
use motion_ui::fixme;
use object::{
    FillMaterial, Material, Model, Object, ObjectId, ObjectKind, StrokeMaterial, Transform,
};
use object_tree::ObjectTree;
use renderer::Renderer;
use scene::Scene;
use world::World;

mod building;
// mod component;
mod dynamics;
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

        egui::SidePanel::left("scene_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Scene Tree");
                ui.separator();
                let root = self.scene().root;
                fixme(ui, self.scene_mut(), root);
                // self.scene.root_mut().ui(ui, &mut self.scene);
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

                    let mut input = HashMap::new();
                    let pos = ctx.input(|i| i.pointer.hover_pos()).unwrap_or(Pos2::ZERO);

                    let pos = pos - rect.center().to_vec2();

                    input.insert(0, pos.x);
                    input.insert(1, pos.y);

                    let current_time = self.current_time;
                    let objects = self.scene_mut().render_with_input(current_time, input);

                    let boxes = objects.bounding_boxes();

                    self.renderer.paint_at(ui, rect, objects);

                    if true {
                        let bb_canvas = ui.painter_at(rect);
                        for (_id, bb) in boxes {
                            bb_canvas.rect_stroke(
                                bb.translate(rect.center().to_vec2()),
                                0.0,
                                Stroke::new(1.0, Color32::RED),
                            );
                        }
                    }
                });
            });
        });
    }
}

impl App {
    fn new<'a>(cc: &'a eframe::CreationContext<'a>, scenes: Vec<(&'static str, Scene)>) -> Self {
        let renderer = Renderer::new(cc).unwrap();

        Self {
            current_scene: scenes.len() - 1,
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
        Box::new(|cc| {
            Box::new(App::new(
                cc,
                vec![
                    ("Mouse input", mouse_input()),
                    ("Stroke", stroke()),
                    ("Building", building()),
                    ("Animations", animations()),
                    ("Alignment", alignment()),
                    // ("Variables", variables()),
                ],
            ))
        }),
    )?;

    Ok(())
}

fn stroke() -> Scene {
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

// fn variables() -> Scene {
//     let mut motions: HashMap<MotionId, Box<dyn Motion>> = HashMap::new();
//     let mut variables_subscriptions: HashMap<usize, Vec<MotionId>> = HashMap::new();
//
//     let mut builder = Path::builder();
//     builder.add_circle(point(0.0, 0.0), 10.0, Winding::Positive);
//     let path = builder.build();
//
//     let add: MotionId = rand::random::<usize>();
//     let circle_id: ObjectId = rand::random::<usize>();
//     motions.insert(
//         add,
//         Box::new(AddObject {
//             object: Object {
//                 object_kind: ObjectKind::Model(Model {
//                     path,
//                     material: FillMaterial::new(Color32::RED).into(),
//                 }),
//                 transform: Transform::default().with_scale(1.0),
//             },
//             rooted: true,
//             object_id: circle_id,
//         }),
//     );
//
//     let var = 3;
//
//     let mv: MotionId = rand::random::<usize>();
//     motions.insert(mv, Box::new(SetVariable { var }));
//
//     let root: MotionId = rand::random::<usize>();
//     motions.insert(
//         root,
//         Box::new(Parallel {
//             motions: vec![add, mv],
//         }),
//     );
//
//     let tracker: MotionId = rand::random::<usize>();
//     motions.insert(
//         tracker,
//         Box::new(Rotate {
//             around: pos2(0.0, 0.0).into(),
//             object_id: circle_id,
//             from: 0.0.into(),
//             to: 360.0.into(),
//         }),
//     );
//
//     variables_subscriptions.insert(var, vec![tracker]);
//
//     Scene::new(motions, root, variables_subscriptions, 2.0)
// }

fn mouse_input() -> Scene {
    let mut motions: HashMap<MotionId, Box<dyn Motion>> = HashMap::new();
    let mut variables_subscriptions: HashMap<usize, Vec<MotionId>> = HashMap::new();

    let mut builder = Path::builder();
    builder.add_circle(point(0.0, 0.0), 10.0, Winding::Positive);
    let path = builder.build();

    let add_object: MotionId = rand::random::<usize>();
    let circle_id: ObjectId = rand::random::<usize>();
    motions.insert(
        add_object,
        Box::new(AddObject {
            object: Object {
                object_kind: ObjectKind::Model(Model {
                    path,
                    material: FillMaterial::new(Color32::RED).into(),
                }),
                transform: Transform::default().with_scale(1.0),
            },
            rooted: true,
            object_id: circle_id,
        }),
    );

    let update_pos: MotionId = rand::random::<usize>();
    let mut transform: DynamicTransform = Transform::default().with_position(pos2(0.0, 0.0)).into();

    transform.position.x = DynamicValue::Variable(0);
    transform.position.y = DynamicValue::Variable(1);

    motions.insert(
        update_pos,
        Box::new(SetTransform {
            object_id: circle_id,
            transform,
        }),
    );

    let root = rand::random::<usize>();

    motions.insert(
        root,
        Box::new(Parallel {
            motions: vec![add_object, update_pos],
        }),
    );

    // variables_subscriptions.insert(0, vec![update_pos]);
    // variables_subscriptions.insert(1, vec![update_pos]);

    Scene::new(motions, root, 2.0)
}

pub fn building() -> Scene {
    let mut builder = SceneBuilder::new(5.0);

    builder
        .circle(10.0, FillMaterial::new(Color32::RED).into())
        .add();

    builder.finish()
}

fn animations() -> Scene {
    let mut scene_builder = SceneBuilder::new(5.0);

    scene_builder.parallel(|p| {
        for i in 0..9 {
            p.sequence(|s| {
                s.delay(0.1 * i as f32);
                s.rect(50.0, 50.0, FillMaterial::new(Color32::RED).into())
                    .with_position(pos2(
                        (i % 3) as f32 * 100.0 - 100.0,
                        (i / 3) as f32 * 100.0 - 100.0,
                    ))
                    .animate(0.3, |a| a.fade_in());
            });
        }
    });

    scene_builder.finish()
}

fn alignment() -> Scene {
    let mut b = SceneBuilder::new(5.0);

    let moving_rect = b
        .rect(50.0, 50.0, FillMaterial::new(Color32::RED).into())
        // .with_position(pos2(25.0, 25.0))
        .with_position(pos2(0.0, 50.0))
        .animate(0.3, |a| a.translate(pos2(0.0, -50.0)));

    b.delay(0.3);

    let moving_circle = b
        .circle(25.0, FillMaterial::new(Color32::BLUE).into())
        .animate(0.3, |a| a.move_to(Alignment::target(moving_rect).left()));
    // .with_position(pos2(-50.0, 50.0))

    b.finish()
}

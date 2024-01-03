use component::Component;
use egui::{pos2, Color32, Stroke, Vec2};
use lyon::path::Path;

use building::{Builder, SceneBuilder};
use motion_ui::fixme;
use object::{ObjectId, ObjectKind};
use object_tree::ObjectTree;
use renderer::Renderer;
use scene::Scene;

mod building;
mod component;
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

        egui::SidePanel::left("scene_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Scene Tree");
                ui.separator();
                let root = self.scene().root;
                fixme(ui, self.scene_mut(), root);
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
                let length = self.scene().length;

                ui.add(
                    egui::Slider::new(&mut self.current_time, 0.0..=length).clamp_to_range(true),
                );

                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    let rect = ui.available_rect_before_wrap();
                    let _response = ui.allocate_rect(rect, egui::Sense::drag());

                    let boxes = objects.bounding_boxes();

                    self.renderer.paint_at(ui, rect, objects);

                    if false {
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
        Box::new(|cc| {
            Box::new(App::new(
                cc,
                vec![("Alignment", alignment()), ("Animations", animations())],
            ))
        }),
    )?;

    Ok(())
}

fn alignment() -> Scene {
    let mut b = SceneBuilder::new(5.0);

    b.parallel(|p| {
        let moving_rect = p
            .rect(50.0, 50.0, Color32::RED.into())
            .with_position(pos2(0.0, 0.0))
            .animate(0.3, |a| a.fade_in());
    });

    b.finish()
}

fn animations() -> Scene {
    let mut scene_builder = SceneBuilder::new(5.0);

    scene_builder.parallel(|p| {
        for i in 0..9 {
            p.sequence(|s| {
                s.delay(0.1 * i as f32);
                s.rect(50.0, 50.0, Color32::RED.into())
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
    type Handle = ();
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

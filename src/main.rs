use animation::{Animation, MotionAnimation};
use builder::Builder;
use component::{Component, ComponentExt, Handle};
use dynamics::{Dynamic, OwnedDynamic};
use easing::Easing::{self, EaseInOut};
use egui::{pos2, Color32, Pos2, Stroke};
use group::{Group, GroupHandle};
use lyon::{math::point, path::Path};
use motion::{EmbededScene, FadeIn, Motion, Move};
use object::{FillMaterial, Material, Model, Object, ObjectId, StrokeMaterial, Transform};
use renderer::Renderer;
use scene::{Scene, SceneBuilder};
use shapes::{Circle, Line};
use spacing::Alignment;
use std::collections::HashMap;
use timing::{Concurrently, Sequence, Wait};
use typst::Typst;

use crate::renderer::UNIT_GRID_HEIGHT;

mod animation;
mod builder;
mod component;
mod dynamics;
mod easing;
mod group;
mod mesh;
mod motion;
mod object;
mod object_tree;
mod renderer;
mod scene;
mod shapes;
mod spacing;
mod timing;
mod trigger;
mod typst;
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

        if self.play && self.current_time < self.scene().length() {
            self.current_time += dt;
        }

        if self.current_time >= self.scene().length() {
            self.play = false;
            self.current_time = self.scene().length();
        }

        ctx.request_repaint();

        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            if self.current_time >= self.scene().length() {
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

        // egui::SidePanel::left("scene_panel").show(ctx, |ui| {
        //     egui::ScrollArea::vertical().show(ui, |ui| {
        //         ui.heading("Scene Tree");
        //         ui.separator();
        //         let root = self.scene().root;
        //         fixme(ui, self.scene_mut(), root);
        //         // self.scene.root_mut().ui(ui, &mut self.scene);
        //     });
        // });

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
                let length = self.scene().length();

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
                    let size = rect.size();
                    let objects =
                        self.scene_mut()
                            .render_with_input(current_time, (size.x, size.y), input);

                    let boxes = objects.bounding_boxes();

                    self.renderer.paint_at(ui, rect, objects);

                    if true {
                        let bb_canvas = ui.painter_at(rect);
                        for (_id, bb) in boxes {
                            bb_canvas.rect_stroke(
                                (bb * (size.y / UNIT_GRID_HEIGHT))
                                    .translate(rect.center().to_vec2()),
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
                    // ("Mouse input", mouse_input()),
                    ("Stroke", stroke()),
                    // ("Building", building()),
                    ("Animations", animations()),
                    ("Movement", movement()),
                    // ("Variables", variables()),
                    ("Scenes", embedded_scenes()),
                    ("Dynamic Alignment", dynamic_alignment()),
                    ("Render Grid", render_grid()),
                    ("Grid", grid()),
                    ("Animating Component Children", animate_component_children()),
                    ("Typst", typst_example()),
                    ("Dynamic path", dynamic_line()),
                ],
            ))
        }),
    )?;

    Ok(())
}

fn stroke() -> Scene {
    let mut builder = Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(1.0, 2.0));
    builder.line_to(point(2.0, 0.0));
    builder.line_to(point(1.0, 1.0));
    builder.close();
    let path = builder.build();

    let mut b = SceneBuilder::new();

    b.add::<Object>(
        Model::new(
            path,
            Material {
                stroke: Some(StrokeMaterial::new(Color32::RED, 0.1)),
                fill: Some(FillMaterial::new(Color32::WHITE)),
            },
        )
        .into(),
    );

    b.play(Wait.with_duration(5.0));

    b.finish()
}

fn animations() -> Scene {
    let mut b = SceneBuilder::new();

    let mut c = Concurrently(Vec::new());

    for i in 0..9 {
        let circle = b.add(
            Circle {
                radius: 0.5,
                material: FillMaterial::new(Color32::RED).into(),
            }
            .with_position(pos2((i % 3) as f32 * 1.0 - 1.0, (i / 3) as f32 * 1.0 - 1.0)),
        );

        let mut seq = Sequence::default();

        seq.add(Wait.with_duration(0.1 * i as f32));
        seq.add(MotionAnimation {
            duration: 0.3,
            motion: FadeIn {
                object_id: **circle,
            },
            easing: Easing::Linear,
        });
        c.add(seq);
    }

    b.play(c);
    b.play(Wait.with_duration(5.0));

    b.finish()
}

fn movement() -> Scene {
    let mut b = SceneBuilder::new();

    let circle_a = b.add(Circle {
        radius: 1.0,
        material: FillMaterial::new(Color32::RED).into(),
    });
    let circle_b = b.add(Circle {
        radius: 1.0,
        material: FillMaterial::new(Color32::BLUE).into(),
    });

    let mut c = Concurrently::default();

    c.add(
        circle_a
            .mv(pos2(-1.0, 2.0), pos2(1.0, 2.0))
            .with_duration(1.0)
            .with_easing(Easing::EaseInOut),
    );
    c.add(
        circle_b
            .mv(pos2(-1.0, -2.0), pos2(1.0, -2.0))
            .with_duration(1.0)
            .with_easing(Easing::Linear),
    );
    b.play(c);

    b.finish()
}

pub fn embedded_scenes() -> Scene {
    let mut b = SceneBuilder::new();

    let mut c = Concurrently::default();
    c.add(EmbededScene {
        scene: animations(),
        transform: OwnedDynamic::new(
            Transform::default()
                .with_position(pos2(-2.0, 0.0))
                .with_scale(0.5),
        ),
        speed: 1.0,
        object_id: rand::random::<usize>(),
        rooted: true,
    });
    c.add(EmbededScene {
        scene: movement(),
        transform: OwnedDynamic::new(
            Transform::default()
                .with_position(pos2(2.0, 0.0))
                .with_scale(0.5),
        ),
        speed: 1.0,
        object_id: rand::random::<usize>(),
        rooted: true,
    });

    b.play(c);

    b.finish()
}

fn dynamic_alignment() -> Scene {
    let mut b = SceneBuilder::new();

    let right_circle = b.add(
        Circle {
            radius: 0.5,
            material: FillMaterial::new(Color32::RED).into(),
        }
        .with_position(pos2(1.0, 1.0)),
    );
    let left_circle = b.add(
        Circle {
            radius: 0.5,
            material: FillMaterial::new(Color32::BLUE).into(),
        }
        .with_position(pos2(-1.0, 1.0)),
    );

    let mut c = Concurrently::default();

    c.add(
        right_circle
            .mv(Alignment::new(**right_circle).center(), pos2(1.0, -1.0))
            .with_duration(1.0)
            .with_easing(EaseInOut),
    );

    c.add(
        left_circle
            .mv(
                Alignment::new(**left_circle).center(),
                Alignment::new(**right_circle).left(),
            )
            .with_duration(1.0)
            .with_easing(EaseInOut),
    );

    b.play(c);

    b.finish()
}

fn grid() -> Scene {
    let mut b = SceneBuilder::new();

    b.add(Grid {
        rows: 10,
        cols: 10,
        width: 8.0,
        height: 8.0,
        material: StrokeMaterial::new(Color32::BLUE, 0.1).into(),
    });

    b.finish()
}

fn animate_component_children() -> Scene {
    let mut b = SceneBuilder::new();

    let grid_handle = b.add(Grid {
        rows: 10,
        cols: 10,
        width: 8.0,
        height: 8.0,
        material: StrokeMaterial::new(Color32::BLUE, 0.1).into(),
    });

    let mut c = Concurrently::default();

    c.add(
        grid_handle
            .vertical_lines
            .mv(pos2(0.0, -grid_handle.grid.height), pos2(0.0, 0.0))
            .with_duration(0.5)
            .with_easing(Easing::EaseInOut),
    );
    c.add(
        grid_handle
            .horizontal_lines
            .mv(pos2(-grid_handle.grid.width, 0.0), pos2(0.0, 0.0))
            .with_duration(0.5)
            .with_easing(Easing::EaseInOut),
    );

    b.play(c);

    b.finish()
}

struct Grid {
    rows: usize,
    cols: usize,
    width: f32,
    height: f32,
    material: Material,
}

struct GridHandle {
    grid: Grid,
    horizontal_lines: Handle<Group<Line>>,
    vertical_lines: Handle<Group<Line>>,
}

impl Component for Grid {
    type Handle = GridHandle;

    fn build<B: Builder>(self, builder: &mut B) -> GridHandle {
        let mut horizontal_lines = Group::new();
        let mut vertical_lines = Group::new();

        for i in 0..=self.rows {
            let x = -self.width / 2.0;
            let y = (i as f32 / self.rows as f32) * self.height - self.height / 2.0;

            horizontal_lines.add(Line {
                start: pos2(x, y),
                end: pos2(x + self.width, y),
                material: self.material.clone(),
            });
        }

        for i in 0..=self.cols {
            let x = (i as f32 / self.cols as f32) * self.width - self.width / 2.0;
            let y = -self.height / 2.0;

            let line = vertical_lines.add(Line {
                start: pos2(x, y),
                end: pos2(x, y + self.height),
                material: self.material.clone(),
            });
        }

        GridHandle {
            grid: self,
            horizontal_lines: builder.add(horizontal_lines),
            vertical_lines: builder.add(vertical_lines),
        }
    }
}

fn render_grid() -> Scene {
    let mut b = SceneBuilder::new();

    for y in 0..=8 {
        for x in 0..16 {
            let circle = b.add(
                Circle {
                    radius: 0.1,
                    material: FillMaterial::new(Color32::RED).into(),
                }
                .with_position(pos2(x as f32 - 8.0, y as f32 - 4.0)),
            );
        }
    }

    b.finish()
}

fn typst_example() -> Scene {
    let mut b = SceneBuilder::new();

    // b.add(Typst {
    //     text: r#"$e^(i pi)+1=0$"#.to_string(),
    //     material: FillMaterial::new(Color32::RED).into(),
    // });
    b.add(Typst {
        text: r#""area" = pi dot "radius"^2"#.to_string(),
        material: FillMaterial::new(Color32::RED).into(),
    });

    b.finish()
}

fn dynamic_line() -> Scene {
    let mut b = SceneBuilder::new();

    let line = b.add(Line {
        start: pos2(-1.0, 0.0),
        end: pos2(1.0, 0.0),
        material: StrokeMaterial::new(Color32::RED, 0.1).into(),
    });

    b.play(
        line.animate(Some(pos2(-1.0, 1.0)), Some(pos2(1.0, -1.0)))
            .with_duration(1.0),
    );

    b.finish()
}

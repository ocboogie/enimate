use animation::{AnimateTransform, Parallel};
use builder::Builder;
use component::Component;
use egui::{
    epaint::{CircleShape, Tessellator},
    pos2, Color32, Vec2,
};
use lyon::path::Path;
use object::{Material, ObjectId, ObjectKind, Transform};
use renderer::Renderer;
use scene::Scene;
use scene_builder::SceneBuilder;
use world::ObjectTree;

mod animation;
mod animation_ui;
mod builder;
mod component;
mod mesh;
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
    let object = objects.get_object(id).unwrap();

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

        self.scene.update(self.current_time / self.total_time);

        ctx.request_repaint();

        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.play = !self.play;
        }

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Animations");
                ui.separator();
                self.scene.animation.ui(ui);
            });
        });

        egui::SidePanel::right("object_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Objects");
                ui.separator();
                show_object(ui, &self.scene.world, self.scene.world.root);
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
            pos2(-100.0, 0.0),
            pos2(100.0, 0.0),
            10.0,
            Color32::RED.into(),
        )
        .with_position(pos2(10.0, 110.0))
        .center_anchor()
        .with_animations()
        .translate(pos2(0.0, -500.0))
        .rotate(std::f32::consts::PI / 2.0)
        .add();

    let space = 500.0;
    for i in 0..9 {
        scene_builder
            .circle(
                (space / 9 as f32) * i as f32 - space / 2.0,
                100.0,
                10.0,
                Color32::BLUE.into(),
            )
            .with_animations()
            .translate(pos2((space / 9 as f32) * i as f32 - space / 2.0, -500.0))
            .add();
    }

    scene_builder
        .line(
            pos2(-100.0, 0.0),
            pos2(100.0, 0.0),
            10.0,
            Color32::from_gray(128).into(),
        )
        .center_anchor()
        .with_animations()
        .rotate(std::f32::consts::PI)
        .add();

    let mut group = scene_builder.group();

    for i in 0..9 {
        group
            .build(Grid::new(
                10,
                2.5,
                Vec2::new(100.0, 100.0),
                Color32::from_gray(128),
            ))
            .with_position(pos2((i % 3) as f32 * 100.0, (i / 3) as f32 * 100.0))
            // .with_anchor(pos2(250.0, 250.0))
            // .with_position(pos2(-100.0, 0.0))
            .with_animations()
            .rotate(std::f32::consts::PI)
            .add();
    }

    group
        .finish()
        .with_anchor(pos2(100.0, 100.0))
        .with_scale(2.5)
        .with_animations()
        .rotate(std::f32::consts::PI / 2.0)
        .add();

    // scene_builder
    //     .circle(-200.0, 200.0, 100.0, Color32::RED.into())
    //     .add();
    // scene_builder
    //     .circle(200.0, -200.0, 100.0, Color32::BLUE.into())
    //     .with_animations()
    //     .translate(pos2(-300.0, 200.0))
    //     .add();

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
            // builder
            //     .line(
            //         pos2(0.0, 0.0),
            //         pos2(0.0, self.size.y),
            //         self.width,
            //         self.color.into(),
            //     )
            //     .with_animations()
            //     .translate(pos2(x as f32 * horizontal_stride, 0.0))
            //     .add();
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
            // builder
            //     .line(
            //         pos2(0.0, 0.0),
            //         pos2(self.size.x, 0.0),
            //         self.width,
            //         self.color.into(),
            //     )
            //     .with_animations()
            //     .translate(pos2(0.0, y as f32 * vertical_stride))
            //     .add();
        }

        let path = pbuilder.build();
        builder.path(path, self.width, self.color.into()).add();
    }
}

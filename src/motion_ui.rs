use crate::{
    motion::{
        AddObject, AnimateTransform, FadeIn, Keyframe, Motion, MotionId, NoOp, Parallel, Sequence,
        Trigger,
    },
    scene::Scene,
};

pub trait MotionUi {
    fn ui(&mut self, ui: &mut egui::Ui, scene: &mut Scene);
}

pub fn fixme(ui: &mut egui::Ui, scene: &mut Scene, motion_id: MotionId) {
    let mut motion = scene.motions.remove(&motion_id).unwrap();
    motion.ui(ui, scene);
    scene.motions.insert(motion_id, motion);
}

impl MotionUi for Parallel {
    fn ui(&mut self, ui: &mut egui::Ui, scene: &mut Scene) {
        ui.label("Parallel");
        ui.indent("Parallel", |ui| {
            for id in &mut self.motions {
                fixme(ui, scene, *id);
            }
        });
    }
}

impl MotionUi for Sequence {
    fn ui(&mut self, ui: &mut egui::Ui, scene: &mut Scene) {
        ui.label("Sequence");
        ui.indent("Sequence", |ui| {
            for (duration, id) in &mut self.motions {
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(duration, 0.0..=1.0).text("Duration"));
                    fixme(ui, scene, *id);
                });
            }
        });
    }
}

impl MotionUi for Keyframe {
    fn ui(&mut self, ui: &mut egui::Ui, scene: &mut Scene) {
        ui.label("Keyframe");
        ui.indent("Keyframe", |ui| {
            ui.add(egui::Slider::new(&mut self.from_min, 0.0..=500.0).text("From Min"));
            ui.add(egui::Slider::new(&mut self.from_max, 0.0..=500.0).text("From Max"));
            ui.add(egui::Slider::new(&mut self.to_min, 0.0..=500.0).text("To Min"));
            ui.add(egui::Slider::new(&mut self.to_max, 0.0..=500.0).text("To Max"));
            fixme(ui, scene, self.motion);
        });
    }
}

impl MotionUi for Trigger {
    fn ui(&mut self, ui: &mut egui::Ui, scene: &mut Scene) {
        ui.label("Trigger");
        ui.indent("Trigger", |ui| {
            ui.add(egui::Slider::new(&mut self.time, 0.0..=500.0).text("Time"));
            fixme(ui, scene, self.motion);
        });
    }
}

impl MotionUi for AddObject {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut Scene) {
        ui.label("Add Object");
        ui.indent("Add Object", |ui| {
            ui.label(format!("Object: {}", self.object_id));
        });
    }
}

impl MotionUi for AnimateTransform {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut Scene) {
        ui.label("Animate Transform");
        ui.indent("Animate Transform", |ui| {
            ui.add(
                egui::Slider::new(&mut self.from.position.x, -100.0..=100.0)
                    .clamp_to_range(false)
                    .text("From X"),
            );
            ui.add(
                egui::Slider::new(&mut self.from.position.y, -100.0..=100.0)
                    .clamp_to_range(false)
                    .text("From Y"),
            );
            ui.add(
                egui::Slider::new(&mut self.from.rotation, 0.0..=360.0)
                    .clamp_to_range(false)
                    .text("From Rotation"),
            );
            ui.add(
                egui::Slider::new(&mut self.from.scale, 0.0..=100.0)
                    .clamp_to_range(false)
                    .text("From Scale"),
            );

            ui.add(
                egui::Slider::new(&mut self.to.position.x, -100.0..=100.0)
                    .clamp_to_range(false)
                    .text("To X"),
            );
            ui.add(
                egui::Slider::new(&mut self.to.position.y, -100.0..=100.0)
                    .clamp_to_range(false)
                    .text("To Y"),
            );
            ui.add(
                egui::Slider::new(&mut self.to.rotation, 0.0..=360.0)
                    .clamp_to_range(false)
                    .text("To Rotation"),
            );
            ui.add(
                egui::Slider::new(&mut self.to.scale, 0.0..=100.0)
                    .clamp_to_range(false)
                    .text("To Scale"),
            );
        });
    }
}

impl MotionUi for NoOp {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut Scene) {
        ui.label("Noop");
    }
}

impl MotionUi for FadeIn {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut Scene) {
        ui.label("Fade In");
    }
}

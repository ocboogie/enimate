use crate::animation::{AnimateTransform, Animation, Keyframe, Noop, Parallel, Sequence};

pub trait AnimationUI {
    fn ui(&mut self, ui: &mut egui::Ui);
}

impl AnimationUI for Sequence {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Sequence");
        ui.indent("Sequence", |ui| {
            for animation in &mut self.animations {
                animation.ui(ui);
            }
        });
    }
}

impl AnimationUI for Parallel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Parallel");
        ui.indent("Parallel", |ui| {
            for animation in &mut self.animations {
                animation.ui(ui);
            }
        });
    }
}

impl AnimationUI for Keyframe {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Keyframe");
        ui.indent("Keyframe", |ui| {
            ui.add(egui::Slider::new(&mut self.from_min, 0.0..=1.0).text("From Min"));
            ui.add(egui::Slider::new(&mut self.from_max, 0.0..=1.0).text("From Max"));
            ui.add(egui::Slider::new(&mut self.to_min, 0.0..=1.0).text("To Min"));
            ui.add(egui::Slider::new(&mut self.to_max, 0.0..=1.0).text("To Max"));
            self.animation.ui(ui);
        });
    }
}

impl AnimationUI for AnimateTransform {
    fn ui(&mut self, ui: &mut egui::Ui) {
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

impl AnimationUI for Noop {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Noop");
    }
}

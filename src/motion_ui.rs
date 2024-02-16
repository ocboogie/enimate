use std::borrow::Cow;

use crate::{
    motion::{
        AddObject, AnimateTransform, FadeIn, Keyframe, Motion, MotionId, NoOp, Parallel, Rotate,
        Sequence, SetTransform, SetVariable, Trigger,
    },
    scene::Scene,
    world::World,
};

pub trait MotionUi {
    fn ui(&mut self, ui: &mut egui::Ui, scene: &mut Scene);
}

pub trait MotionGenericUi {
    fn label(&self) -> Cow<'static, str>;
    fn children(&self) -> &[MotionId] {
        &[]
    }
}

impl<M: MotionGenericUi> MotionUi for M {
    fn ui(&mut self, ui: &mut egui::Ui, scene: &mut Scene) {
        ui.label(self.label());
        let children = self.children();

        if children.is_empty() {
            return;
        }

        ui.indent(self.label(), |ui| {
            for id in children {
                fixme(ui, scene, *id);
            }
        });
    }
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
                ui.label(format!("Duration: {}", duration));
                fixme(ui, scene, *id);
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

impl MotionGenericUi for AnimateTransform {
    fn label(&self) -> Cow<'static, str> {
        format!("Animate Transform {}", self.object_id).into()
    }
}

// impl MotionUi for Transform {
//     fn ui(&mut self, ui: &mut egui::Ui, _: &mut Scene) {
//         ui.label(format!("Transform {}", self.object_id));
//         ui.indent("Transform", |ui| {
//             ui.add(
//                 egui::Slider::new(&mut self.transform.position.x, -100.0..=100.0)
//                     .clamp_to_range(false)
//                     .text("X"),
//             );
//             ui.add(
//                 egui::Slider::new(&mut self.transform.position.y, -100.0..=100.0)
//                     .clamp_to_range(false)
//                     .text("Y"),
//             );
//             ui.add(
//                 egui::Slider::new(&mut self.transform.rotation, 0.0..=360.0)
//                     .clamp_to_range(false)
//                     .text("Rotation"),
//             );
//             ui.add(
//                 egui::Slider::new(&mut self.transform.scale, 0.0..=100.0)
//                     .clamp_to_range(false)
//                     .text("Scale"),
//             );
//         });
//     }
// }

impl MotionUi for NoOp {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut Scene) {
        ui.label("Noop");
    }
}

impl MotionGenericUi for SetVariable {
    fn label(&self) -> Cow<'static, str> {
        format!("Set Variable {}", self.var).into()
    }
}

impl MotionGenericUi for SetTransform {
    fn label(&self) -> Cow<'static, str> {
        format!("Set Transofrm {}", self.object_id).into()
    }
}

impl MotionGenericUi for Rotate {
    fn label(&self) -> Cow<'static, str> {
        format!("Rotate {}", self.object_id).into()
    }
}

impl MotionUi for FadeIn {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut Scene) {
        ui.label("Fade In");
    }
}

use crate::{
    object::{
        Color, FillMaterial, Material, Model, Object, ObjectKind, Path, StrokeMaterial, Transform,
    },
    object_tree::ObjectTree,
    timing::Time,
};
use egui::pos2;
use lyon::{
    geom::point,
    path::{Path as LyonPath, Winding},
};
use steel::{
    rvals::FromSteelVal,
    steel_vm::{engine::Engine, register_fn::RegisterFn},
    SteelErr, SteelVal,
};
use steel_derive::Steel;

// #[derive(Clone, Debug, Steel)]
// struct Component {
//     builder: SteelVal,
//     props: Vec<SteelVal>,
//     transform: Transform,
// }

// impl Component {
//     fn new(builder: SteelVal, props: Vec<SteelVal>, transform: Transform) -> Self {
//         Self {
//             builder,
//             props,
//             transform,
//         }
//     }
// }

// #[derive(Clone, Debug, Steel)]
// struct ComponentTree(Vec<Component>);
//
// impl ComponentTree {
//     fn new() -> Self {
//         ComponentTree(Vec::new())
//     }
//
//     fn add(&mut self, component: Component) {
//         self.0.push(component);
//     }
//
//     fn build(&self, engine: &mut Engine) -> ObjectTree {
//         let mut tree = ObjectTree::new();
//
//         for component in &self.0 {
//             let object = Object::from_steelval(
//                 &engine
//                     .call_function_with_args(component.builder.clone(), component.props.clone())
//                     .unwrap(),
//             )
//             .unwrap();
//         }
//
//         tree
//     }
// }

fn draw_circle(radius: f32) -> Path {
    let mut path_builder = LyonPath::builder();
    path_builder.add_circle(point(0.0, 0.0), radius, Winding::Positive);

    path_builder.build().into()
}

pub struct Scene {
    engine: Engine,
    length: f32,
}

impl Scene {
    pub fn build(content: &str) -> Result<Self, SteelErr> {
        let mut engine = Engine::new();
        // let mut engine = Engine::new_sandboxed();

        engine.register_fn("draw-circle", draw_circle);
        engine.register_fn("model", Model::new);
        engine.register_fn("fill", |color: Color, model: Model| {
            // FIXME: We don't want this method to have side effects
            //        so we clone, but that clones the path which is expensive
            let mut new = model.clone();
            new.material.fill = Some(FillMaterial::new(color));
            new
        });
        engine.register_fn("stroke", |width: f32, color: Color, model: Model| {
            let mut new = model.clone();
            new.material.stroke = Some(StrokeMaterial::new(color, width));
            new
        });
        engine.register_fn(
            "object-model",
            |path: Path, fill: Option<Color>, stroke: Option<StrokeMaterial>| Object {
                object_kind: ObjectKind::Model(Model::new(
                    path,
                    Material {
                        fill: fill.map(Into::into),
                        stroke,
                    },
                )),
                transform: Transform::default(),
            },
        );
        engine.register_fn("object-group", Object::new_group);
        engine.register_fn("color", Color::new);
        engine.register_fn("object-tree", ObjectTree::from_objects);
        engine.register_fn("translate", |x: f32, y: f32, object: Object| {
            let mut new = object.clone();
            new.transform = new.transform.translate(pos2(x, y));
            new
        });
        engine.register_fn("transform-translate", |x: f32, y: f32| Transform {
            position: pos2(x, y),
            ..Default::default()
        });
        engine.register_fn("apply-transform", |object: Object, transform: Transform| {
            Object {
                object_kind: object.object_kind,
                transform: object.transform.and_then(&transform),
            }
        });
        engine.register_fn("transform-pos-x", |transform: Transform| {
            transform.position.x
        });
        engine.register_fn("transform-pos-y", |transform: Transform| {
            transform.position.y
        });
        engine.register_fn("transform-rot", |transform: Transform| transform.rotation);
        engine.register_fn("transform-scale", |transform: Transform| transform.scale);
        engine.register_fn("transform-anchor-x", |transform: Transform| {
            transform.anchor.x
        });
        engine.register_fn("transform-anchor-y", |transform: Transform| {
            transform.anchor.y
        });
        engine.register_fn(
            "transform",
            |pos_x: f32, pos_y: f32, rot: f32, scale: f32, anchor_x: f32, anchor_y: f32| {
                Transform {
                    position: pos2(pos_x, pos_y),
                    rotation: rot,
                    scale,
                    anchor: pos2(anchor_x, anchor_y),
                }
            },
        );
        engine.register_fn("id", || rand::random::<usize>());

        engine.run(content)?;

        let length = engine.extract("length")?;

        Ok(Self { engine, length })
    }

    pub fn render(&mut self, time: Time) -> Result<ObjectTree, SteelErr> {
        ObjectTree::from_steelval(
            &self
                .engine
                .call_function_by_name_with_args("main", vec![time.into()])
                .map_err(|err| {
                    err.emit_result("foo.scm", include_str!("../scenes/animations.scm"));
                    err
                })?,
        )
    }

    pub fn length(&self) -> f32 {
        self.length
    }
}

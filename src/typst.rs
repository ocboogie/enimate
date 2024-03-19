use crate::{
    builder::Builder,
    component::{Component, Handle},
    group::Group,
    object::{FillMaterial, Material, Object, ObjectId, StrokeMaterial},
    Transform,
};
use comemo::Prehashed;
use egui::{pos2, vec2, Pos2};
use lyon::{
    algorithms::aabb::bounding_box,
    geom::Point,
    lyon_tessellation::{FillTessellator, StrokeTessellator},
    path::Path,
};
use once_cell::sync::Lazy;
use typst::{
    diag::FileResult,
    eval::Tracer,
    foundations::{Bytes, Datetime, Smart},
    layout::{Abs, Margin, PageElem},
    syntax::{FileId, Source, VirtualPath},
    text::{Font, FontBook},
    Library, World,
};
use typst_svg::svg_merged;

/// How many Tpyst points are in a enimate unit.
const POINTS_PER_UNIT: f32 = 24.0;

static LIBRARY: Lazy<Prehashed<Library>> = Lazy::new(|| {
    let mut lib = Library::default();
    // lib.styles
    //     .set(PageElem::set_width(Smart::Custom(Abs::pt(240.0).into())));
    // lib.styles.set(PageElem::set_height(Smart::Auto));
    // lib.styles
    //     .set(PageElem::set_margin(Margin::splat(Some(Smart::Custom(
    //         Abs::pt(0.0).into(),
    //     )))));
    Prehashed::new(lib)
});

static FONTS: Lazy<(Prehashed<FontBook>, Vec<Font>)> = Lazy::new(|| {
    let fonts: Vec<_> = typst_assets::fonts()
        .flat_map(|data| Font::iter(Bytes::from_static(data)))
        .collect();
    let book = FontBook::from_fonts(&fonts);
    (Prehashed::new(book), fonts)
});

struct EnimateWorld(Source);

impl World for EnimateWorld {
    fn library(&self) -> &Prehashed<Library> {
        &LIBRARY
    }
    fn book(&self) -> &Prehashed<FontBook> {
        &FONTS.0
    }

    fn main(&self) -> Source {
        self.0.clone()
    }

    fn source(&self, _: FileId) -> FileResult<Source> {
        Ok(self.0.clone())
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        unimplemented!()
    }

    fn font(&self, index: usize) -> Option<Font> {
        Some(FONTS.1[index].clone())
    }

    fn today(&self, _: Option<i64>) -> Option<Datetime> {
        None
    }
}

pub struct Typst {
    pub text: String,
    pub material: Material,
}

impl Component for Typst {
    type Handle = ();

    fn build<B: Builder>(self, builder: &mut B) -> () {
        let id = FileId::new(None, VirtualPath::new("main.typ"));
        let source = Source::new(id, self.text);
        let world = EnimateWorld(source);

        let mut tracer = Tracer::new();

        let document = match typst::compile(&world, &mut tracer) {
            Ok(doc) => doc,
            Err(err) => {
                let msg = &err[0].message;
                panic!(
                    "while trying to compile:\n{}:\n\nerror: {}",
                    world.main().text(),
                    msg
                );
            }
        };

        let svg = svg_merged(&document, Abs::pt(0.0).into());

        let opt = usvg::Options::default();
        let rtree = usvg::Tree::from_data(svg.as_bytes(), &opt).unwrap();

        let mut group = Group::new();

        let rect = &rtree.svg_node().view_box.rect;
        let offset = vec2(
            (rect.x() + rect.width() / 2.0) as f32,
            (rect.y() + rect.height() / 2.0) as f32,
        );

        for node in rtree.root().descendants() {
            if let usvg::NodeKind::Path(ref p) = *node.borrow() {
                let flip_y = p.transform.d < 0.0;
                let path = convert_path(p, flip_y);

                let mut transform = convert_transform(&p.transform);

                transform.position = pos2(
                    transform.position.x / POINTS_PER_UNIT,
                    transform.position.y / POINTS_PER_UNIT,
                );
                transform.scale /= POINTS_PER_UNIT;

                group.add(Object::new_model(path, self.material.clone()).with_transform(transform));
            }
        }

        builder.add(group.with_transform(Transform {
            position: pos2(offset.x / POINTS_PER_UNIT, offset.y / POINTS_PER_UNIT),
            ..Default::default()
        }));
    }
}

fn convert_path(p: &usvg::Path, flip_y: bool) -> Path {
    let mut builder = Path::svg_builder();

    let flipper = if flip_y { -1.0 } else { 1.0 };

    // Taken from https://github.com/jpopesculian/lyon-usvg/blob/master/src/lib.rs#L79
    for segment in p.data.iter() {
        match *segment {
            usvg::PathSegment::MoveTo { x, y } => {
                builder.move_to(Point::new(x as f32, y as f32 * flipper));
            }
            usvg::PathSegment::LineTo { x, y } => {
                builder.line_to(Point::new(x as f32, y as f32 * flipper));
            }
            usvg::PathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                builder.cubic_bezier_to(
                    Point::new(x1 as f32, y1 as f32 * flipper),
                    Point::new(x2 as f32, y2 as f32 * flipper),
                    Point::new(x as f32, y as f32 * flipper),
                );
            }
            usvg::PathSegment::ClosePath => {
                builder.close();
            }
        }
    }
    builder.build()
}

fn convert_transform(t: &usvg::Transform) -> Transform {
    Transform {
        position: pos2(t.e as f32, t.f as f32),
        rotation: t.b.atan2(t.a) as f32,
        scale: (t.a * t.a + t.b * t.b).sqrt() as f32,
        anchor: Pos2::ZERO,
    }
}

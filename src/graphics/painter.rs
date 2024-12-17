use super::{color::Color, paint::Paint, text::Text};
use crate::math::Vec2;
use skia_safe::{Canvas, Path, Point, Rect, Vector};
use std::f32::consts::PI;

pub struct Painter<'a> {
    canvas: &'a Canvas,
}

impl Painter<'_> {
    pub fn new<'a>(canvas: &'a Canvas) -> Painter<'a> {
        Painter { canvas }
    }

    pub fn translate(&mut self, translation: Vec2, f: impl FnOnce(&mut Painter)) {
        if translation == Vec2::ZERO {
            f(self);
        } else {
            self.canvas.save();
            self.canvas
                .translate(Vector::new(translation.x, translation.y));
            f(self);
            self.canvas.restore();
        }
    }

    pub fn draw_rect(&mut self, position: Vec2, size: Vec2, paint: impl Into<Paint>) {
        let paint = paint.into().into();
        if let Some(paint) = paint {
            self.canvas.draw_rect(
                skia_safe::Rect::new(
                    position.x,
                    position.y,
                    position.x + size.x,
                    position.y + size.y,
                ),
                &paint,
            );
        }
    }

    pub fn draw_round_rect(
        &mut self,
        position: Vec2,
        size: Vec2,
        radius: f32,
        smoothing: f32,
        paint: impl Into<Paint>,
    ) {
        let paint: Option<skia_safe::Paint> = paint.into().into();
        if let Some(mut paint) = paint {
            paint.set_anti_alias(true);
            self.canvas.draw_path(
                &round_rect_path(size, radius, smoothing).with_offset((position.x, position.y)),
                &paint,
            );
        }
    }

    pub fn draw_paragraph(&mut self, text: &Text, position: Vec2, width: f32) {
        let mut paragraph = text.paragraph.borrow_mut();
        if paragraph.max_width() != width {
            paragraph.layout(width);
        }
        paragraph.paint(self.canvas, Point::new(position.x, position.y));
    }

    // Shadow example
    // pub fn test(&mut self, position: Vec2, size: Vec2, radius: f32, smoothing: f32) {
    //     let mut path = round_rect_path(size, radius, smoothing);
    //     path.offset((position.x, position.y));

    //     if let Some(mut paint) = Option::<skia_safe::Paint>::from(Paint::fill(Color::BLACK)) {
    //         paint.set_image_filter(drop_shadow(
    //             (10.0, 10.0),
    //             (5.0, 5.0),
    //             skia_safe::Color::BLUE,
    //             None,
    //             None,
    //             None,
    //         ));
    //         self.canvas.draw_path(&path, &paint);
    //     }

    //     if let Some(paint) = Paint::fill(Color::RED).into() {
    //         let mut points = vec![Point::new(0.0, 0.0); 128];
    //         let point_count = path.get_points(&mut points);
    //         println!("{}", point_count);
    //         for i in 0..point_count {
    //             self.canvas.draw_circle(points[i], 2.0, &paint);
    //         }
    //     }
    // }
}

impl From<Paint> for Option<skia_safe::Paint> {
    fn from(paint: Paint) -> Self {
        let mut sk_paint = skia_safe::Paint::default();
        match paint {
            Paint::Fill { color } => {
                if color.a == 0 {
                    return None;
                }
                sk_paint.set_color(color);
            }
            Paint::Stroke { width, color } => {
                if width == 0.0 || color.a == 0 {
                    return None;
                }
                sk_paint.set_stroke(true);
                sk_paint.set_color(color);
                sk_paint.set_stroke_width(width);
                sk_paint.set_stroke_miter(10.0);
            }
        }
        Some(sk_paint)
    }
}

impl From<Color> for skia_safe::Color {
    fn from(color: Color) -> Self {
        skia_safe::Color::from_argb(color.a, color.r, color.g, color.b)
    }
}

// Smoothing functions based on https://www.figma.com/blog/desperately-seeking-squircles/
struct Smoothing {
    contact: Vec2,
    intersection: f32,
}

impl Smoothing {
    fn new(smoothing: f32) -> Self {
        let angle = smoothing * PI / 4.0;
        let sin = angle.sin();
        let cos = angle.cos();
        let tan = sin / cos;

        // The contact where the bezier smoothing and arc meet
        let contact_x = 1.0 - sin;
        let contact_y = 1.0 - cos;
        let contact_slope = tan;
        let contact_slope_x_axis_intersection = contact_x + contact_y / contact_slope;

        Self {
            contact: Vec2::new(contact_x, contact_y),
            intersection: contact_slope_x_axis_intersection,
        }
    }
}

fn round_rect_path(size: Vec2, mut radius: f32, smoothing: f32) -> Path {
    radius = radius.min(size.x.min(size.y) / 2.0);

    let hs = (smoothing * radius).min(size.x / 2.0 - radius) / radius;
    let vs = (smoothing * radius).min(size.y / 2.0 - radius) / radius;

    let Smoothing {
        contact: hc,
        intersection: hi,
    } = Smoothing::new(hs);
    let Smoothing {
        contact: vc,
        intersection: vi,
    } = Smoothing::new(vs);

    let mut path = Path::new();

    // Top left
    path.move_to((0.0, radius * (1.0 + vs)));
    if vs > 0.0 {
        path.cubic_to(
            (0.0, radius * (vi * 0.67 + (1.0 + vs) * 0.33)),
            (0.0, radius * vi),
            (radius * vc.y, radius * vc.x),
        );
    }
    if hs + vs < 2.0 {
        path.arc_to(
            Rect::from_xywh(0.0, 0.0, radius * 2.0, radius * 2.0),
            225.0 - 45.0 * (1.0 - vs),
            45.0 * (2.0 - hs - vs),
            false,
        );
    }
    if hs > 0.0 {
        path.cubic_to(
            (radius * hi, 0.0),
            (radius * (hi * 0.67 + (1.0 + hs) * 0.33), 0.0),
            (radius * (1.0 + hs), 0.0),
        );
    }

    // Top right
    path.line_to((size.x - radius * (1.0 + hs), 0.0));
    if hs > 0.0 {
        path.cubic_to(
            (size.x - radius * (hi * 0.67 + (1.0 + hs) * 0.33), 0.0),
            (size.x - radius * hi, 0.0),
            (size.x - radius * hc.x, radius * hc.y),
        );
    }
    if hs + vs < 2.0 {
        path.arc_to(
            Rect::from_xywh(size.x - radius * 2.0, 0.0, radius * 2.0, radius * 2.0),
            315.0 - 45.0 * (1.0 - hs),
            45.0 * (2.0 - hs - vs),
            false,
        );
    }
    if vs > 0.0 {
        path.cubic_to(
            (size.x, radius * vi),
            (size.x, radius * (vi * 0.67 + (1.0 + vs) * 0.33)),
            (size.x, radius * (1.0 + vs)),
        );
    }

    // Bottom right
    path.line_to((size.x, size.y - radius * (1.0 + vs)));
    if vs > 0.0 {
        path.cubic_to(
            (size.x, size.y - radius * (vi * 0.67 + (1.0 + vs) * 0.33)),
            (size.x, size.y - radius * vi),
            (size.x - radius * vc.y, size.y - radius * vc.x),
        );
    }
    if hs + vs < 2.0 {
        path.arc_to(
            Rect::from_xywh(
                size.x - radius * 2.0,
                size.y - radius * 2.0,
                radius * 2.0,
                radius * 2.0,
            ),
            45.0 - 45.0 * (1.0 - vs),
            45.0 * (2.0 - hs - vs),
            false,
        );
    }
    if hs > 0.0 {
        path.cubic_to(
            (size.x - radius * hi, size.y),
            (size.x - radius * (hi * 0.67 + (1.0 + hs) * 0.33), size.y),
            (size.x - radius * (1.0 + hs), size.y),
        );
    }

    // Bottom left
    path.line_to((radius * (1.0 + hs), size.y));
    if hs > 0.0 {
        path.cubic_to(
            (radius * (hi * 0.67 + (1.0 + hs) * 0.33), size.y),
            (radius * hi, size.y),
            (radius * hc.x, size.y - radius * hc.y),
        );
    }
    if hs + vs < 2.0 {
        path.arc_to(
            Rect::from_xywh(0.0, size.y - radius * 2.0, radius * 2.0, radius * 2.0),
            135.0 - 45.0 * (1.0 - hs),
            45.0 * (2.0 - hs - vs),
            false,
        );
    }
    if vs > 0.0 {
        path.cubic_to(
            (0.0, size.y - radius * vi),
            (0.0, size.y - radius * (vi * 0.67 + (1.0 + vs) * 0.33)),
            (0.0, size.y - radius * (1.0 + vs)),
        );
    }

    path.close();
    path
}

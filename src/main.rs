mod core;
mod skia;
mod utils;
mod views;
mod winit;

use ::core::f32;
use ::winit::event_loop::EventLoop;
use core::{Binding, Context, View};
use itertools::Itertools;
use macroquad::{
    color::{BLACK, BLUE, GREEN, RED},
    math::Vec2,
};
use skia::SkiaRenderer;
use skia_safe::{
    textlayout::{
        FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextAlign, TextHeightBehavior,
        TextStyle,
    },
    Canvas, Color, Color4f, ConditionallySend, Font, FontMgr, Paint, Point, Rect, Sendable,
    TextBlob, Typeface,
};
use views::{
    column, label, row, spacer, Backgroundable, Borderable, Clickable, Component, ContentBuilder,
    Paddable,
};
use winit::WinitApp;
// use winit::{dpi::LogicalUnit, event_loop::EventLoop};

fn main() {
    dbg!(FontMgr::new().family_names().collect_vec());

    let event_loop = EventLoop::new().unwrap();
    event_loop
        .run_app(&mut WinitApp::new(
            |window| SkiaRenderer::new(window),
            render,
        ))
        .unwrap();
}

thread_local! {
    static FONT_COLLECTION: FontCollection = {
        let mut collection = FontCollection::new();
        collection.set_asset_font_manager(Some(FontMgr::new()));
        collection
    };
}

fn render(canvas: &Canvas) {
    canvas.clear(Color4f::new(1.0, 1.0, 1.0, 1.0));
    // canvas.draw_text_blob(
    //     TextBlob::from_str("Hello\nWorld", &Font::from_typeface(a.clone(), Some(12.0))).unwrap(),
    //     Point::new(100.0, 100.0),
    //     &Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None),
    // );
    let mut style = TextStyle::new();
    style.set_font_size(14.0);
    style.set_color(Color::from_argb(255, 0, 0, 0));
    style.set_font_families(&["Comic Sans MS"]);

    let mut par_style = ParagraphStyle::new();
    par_style.set_text_align(TextAlign::Justify);
    par_style.set_max_lines(4);
    par_style.set_text_height_behavior(TextHeightBehavior::DisableFirstAscent);
    par_style.set_ellipsis("...");

    let a = FONT_COLLECTION.with(|collection| collection.clone());
    let mut paragraph = ParagraphBuilder::new(&par_style, a)
        .push_style(&style)
        .add_text("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed non risus. Suspendisse lectus tortor, dignissim sit amet, adipiscing nec, ultricies sed, dolor.")
        .build();
    paragraph.layout(f32::INFINITY);
    paragraph.layout(550.0);

    // dbg!(paragraph.longest_line());
    // dbg!(paragraph.max_width());
    // dbg!(paragraph.max_intrinsic_width());
    // dbg!(paragraph.min_intrinsic_width());
    // paragraph.layout(paragraph.min_intrinsic_width());
    let w = paragraph.longest_line() * 0.0 + 550.0;
    let h = paragraph.height();
    // canvas.draw_rect(
    //     Rect::from_xywh(50.0, 50.0, w, h),
    //     &Paint::new(Color4f::new(0.7, 0.7, 0.7, 1.0), None),
    // );
    paragraph.paint(canvas, Point::new(50.0, 50.0));
    // println!("{}", paragraph.height());
    // canvas.draw_text_blob(
    //     paragraph,
    //     Point::new(100.0, 150.0),
    //     &Paint::new(Color4f::new(1.0, 1.0, 1.0, 1.0), None),
    // );
    // canvas.draw_text_blob(
    //     TextBlob::from_str("Hello\nWorld", &Font::from_typeface(a.clone(), Some(24.0))).unwrap(),
    //     Point::new(100.0, 210.0),
    //     &Paint::new(Color4f::new(1.0, 1.0, 1.0, 1.0), None),
    // );

    // canvas.clear(Color4f::new(0.0, 0.0, 0.0, 1.0));
    // for _ in 0..1000 {
    //     canvas.draw_rect(
    //         Rect::from_xywh(
    //             rand::random::<f32>() * 1400.0,
    //             rand::random::<f32>() * 1000.0,
    //             200.0,
    //             200.0,
    //         ),
    //         &Paint::new(
    //             Color4f::new(
    //                 rand::random::<f32>(),
    //                 rand::random::<f32>(),
    //                 rand::random::<f32>(),
    //                 rand::random::<f32>(),
    //             ),
    //             None,
    //         ),
    //     );
    // }
}

// fn window_conf() -> Conf {
//     Conf {
//         window_title: "Flux".to_owned(),
//         high_dpi: true,
//         ..Default::default()
//     }
// }

// #[macroquad::main(window_conf)]
// async fn main() {
//     let mut states = HashMap::new();
//     let mut state_dependencies = Bigraph::new();
//     let mut state_changes = HashSet::new();

//     let mut tree = ViewTree::new(Main.border(4.0, BLACK).padding_all(16.0));
//     let mut prev_screen_size = screen_size();
//     tree.update(
//         &mut Context::new(&mut states, &mut state_dependencies),
//         Id(0),
//     );

//     let mut updates = 0u32;
//     let mut elapsed: Duration = Duration::ZERO;

//     loop {
//         clear_background(WHITE);
//         tree.draw();

//         if screen_size() != prev_screen_size {
//             prev_screen_size = screen_size();
//             let start = Instant::now();
//             tree.update(
//                 &mut Context::new(&mut states, &mut state_dependencies),
//                 Id(0),
//             );
//             let delta = start.elapsed();
//             updates += 1;
//             elapsed += delta;
//             println!("Resize: {:?} - {:?}", delta, elapsed.checked_div(updates));
//         }

//         if is_mouse_button_pressed(MouseButton::Left) {
//             tree.interact(
//                 &mut ContextMut::new(&mut states, &mut state_changes),
//                 Interaction::Click(mouse_position().into()),
//             );
//             println!("{:?}", states);
//             // println!("{:?}", states.dependencies);
//             println!("{:?}", state_changes);

//             let start = Instant::now();
//             let mut update = HashSet::<Option<Id>>::new();
//             for i in state_changes.iter() {
//                 update.extend(state_dependencies.get_v_connections(*i));
//             }
//             state_changes.clear();

//             for i in update {
//                 if let Some(i) = i {
//                     tree.update(&mut Context::new(&mut states, &mut state_dependencies), i);
//                 }
//             }
//             let delta = start.elapsed();
//             updates += 1;
//             elapsed += delta;
//             println!("Update: {:?} - {:?}", delta, elapsed.checked_div(updates));
//         }

//         next_frame().await;
//     }
// }

#[derive(PartialEq)]
struct Main;

#[derive(PartialEq)]
struct Item {
    index: usize,
    size: Vec2,
}

struct MainState {
    items: Vec<Vec2>,
}

impl Component for Main {
    fn build(&self, ctx: &mut Context) -> impl View {
        let state = ctx.state(|| MainState {
            items: vec![
                Vec2::new(20.0, 20.0),
                // Vec2::new(50.0, 50.0),
                // Vec2::new(50.0, 50.0),
            ],
        });

        let count = ctx.state(|| 0);

        let length = ctx.get(state).items.len();
        let a = length + *ctx.get(count);

        return column![
            label("Test default")
                .padding_vertical(0.0)
                .padding_horizontal(8.0)
                .border(4.0, BLACK),
            label("Test 16, blue")
                .size(16.0)
                .color(BLUE)
                .padding_vertical(0.0)
                .padding_horizontal(8.0)
                .border(4.0, BLACK),
            spacer()
                .width(100.0)
                .height(100.0)
                .background(RED)
                .on_click(move |ctx| {
                    *ctx.get_mut(count) += 1;
                    let state = ctx.get_mut(state);
                    state.items.push(Vec2::new(20.0, 20.0));
                    // state.items[0].y += 10.0;
                }),
            spacer()
                .height(50.0)
                .width(20.0)
                .max_height(100.0)
                .min_height(20.0)
                .background(GREEN),
            spacer()
                .height(30.0)
                .width(20.0)
                .max_height(100.0)
                .min_height(30.0)
                .background(GREEN),
            column(ContentBuilder::from_items(0..1, {
                let items = &ctx.get(state).items;
                move |_| {
                    row(ContentBuilder::from_items(
                        items.iter().enumerate(),
                        |(index, item)| Item { index, size: *item },
                    ))
                    .spacing(10.0)
                }
            }))
            .spacing(10.0),
            label(format!("{}", a)),
            Text(count)
        ]
        .spacing(10.0)
        .padding_all(10.0);
    }
}

impl Component for Item {
    fn build(&self, _ctx: &mut Context) -> impl View {
        let index = self.index.to_owned();
        column![spacer()
            .width(self.size.x)
            .height(self.size.y)
            .max_width(if index == 0 || true {
                50.0 //f32::INFINITY
            } else {
                self.size.x
            })
            .border(4.0, BLACK)
            .background(BLUE)
            .on_click(move |_| println!("Clicked: {}", index))]
    }
}

#[derive(PartialEq)]
struct Text(Binding<usize>);

impl Component for Text {
    fn build(&self, ctx: &mut Context) -> impl View {
        let count = ctx.get(self.0);
        label(format!("Items: {}", *count))
            // .color(WHITE)
            .padding_vertical(0.0)
            .padding_horizontal(8.0)
            .border(4.0, BLACK)
    }
}

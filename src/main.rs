mod app;
mod core;
mod graphics;
mod utils;
mod views;

use ::core::f32;
use ::winit::event_loop::EventLoop;
use app::App;
use core::{Binding, Context, View};
use glam::Vec2;
use graphics::{Color, Painter, Paragraph, ParagraphStyle};
use itertools::Itertools;
use views::{
    column, label, row, spacer, Backgroundable, Borderable, Clickable, Component, ContentBuilder,
    Paddable,
};
// use winit::{dpi::LogicalUnit, event_loop::EventLoop};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut App::new(Main)).unwrap();
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
            label("Test\ndefaultg")
                .size(16.0)
                .padding_vertical(0.0)
                .padding_horizontal(8.0)
                .background(Color::BLUE),
            label("Test 16, blue")
                .size(12.0)
                .color(Color::BLUE)
                .padding_vertical(0.0)
                .padding_horizontal(8.0)
                .border(2.0, Color::BLACK),
            spacer()
                .width(100.0)
                .height(100.0)
                .background(Color::RED)
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
                .background(Color::GREEN),
            spacer()
                .height(30.0)
                .width(20.0)
                .max_height(100.0)
                .min_height(30.0)
                .background(Color::GREEN),
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
            .border(4.0, Color::BLACK)
            .background(Color::BLUE)
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
            .border(2.0, Color::BLACK)
    }
}

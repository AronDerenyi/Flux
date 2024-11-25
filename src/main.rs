mod core;
mod utils;
mod views;

use core::{Context, Interaction, View, ViewTree};
use macroquad::prelude::*;
use miniquad::window::screen_size;
use std::{
    any::Any,
    collections::HashMap,
    rc::Rc,
    time::{Duration, Instant},
};
use utils::id_vec::Id;
use views::{
    column, label, row, spacer, Backgroundable, Borderable, Clickable, Component, ContentBuilder,
    Paddable,
};

#[macroquad::main("Flux")]
async fn main() {
    let mut states = HashMap::<Id, Rc<dyn Any>>::new();
    let mut tree = ViewTree::new(Main.border(4.0, BLACK).padding_all(16.0));
    let mut prev_screen_size = screen_size();
    tree.update(&mut states, Id(0));

    let mut updates = 0u32;
    let mut elapsed: Duration = Duration::ZERO;

    loop {
        clear_background(WHITE);
        tree.draw();

        if screen_size() != prev_screen_size {
            prev_screen_size = screen_size();
            let start = Instant::now();
            tree.update(&mut states, Id(0));
            let delta = start.elapsed();
            updates += 1;
            elapsed += delta;
            println!("Update: {:?} - {:?}", delta, elapsed.checked_div(updates));
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            tree.interact(Interaction::Click(mouse_position().into()));
            let start = Instant::now();
            tree.update(&mut states, Id(2));
            let delta = start.elapsed();
            updates += 1;
            elapsed += delta;
            println!("Update: {:?} - {:?}", delta, elapsed.checked_div(updates));
        }

        next_frame().await;
    }
}

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
                .on_click({
                    let state = state.clone();
                    move || {
                        state.borrow_mut().items.push(Vec2::new(20.0, 20.0));
                        // state.borrow_mut().items[0].y += 10.0;
                    }
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
            column(ContentBuilder::from_items(0..10, {
                let state = state.clone();
                move |_| {
                    row(ContentBuilder::from_items(
                        state.borrow().items.iter().enumerate(),
                        |(index, item)| Item { index, size: *item },
                    ))
                    .spacing(10.0)
                }
            }))
            .spacing(10.0),
            label(format!("Items: {}", state.borrow().items.len()))
                .padding_vertical(0.0)
                .padding_horizontal(8.0)
                .border(4.0, BLACK),
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
            .on_click(move || println!("Clicked: {}", index))]
    }
}

mod core;
mod utils;
mod views;

use core::{App, ContentBuilder, Context, View};
use macroquad::prelude::*;
use utils::id_vec::Id;
use views::{row, spacer, Backgroundable, Borderable, Clickable, Component, Paddable};

#[macroquad::main("RustUI")]
async fn main() {
    let mut app = App::new(Main.border(4.0, BLACK).padding_all(16.0));
    app.update(Id(0));

    loop {
        clear_background(WHITE);

        app.draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            app.interact(mouse_position().into());
            app.update(Id(2));
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
            spacer(Vec2::new(100.0, 100.0)).background(RED).on_click({
                let state = state.clone();
                move || {
                    state.borrow_mut().items.push(Vec2::new(20.0, 20.0));
                    // state.borrow_mut().items[0].y += 10.0;
                }
            }),
            row(ContentBuilder::from_items(
                state.borrow().items.iter().enumerate(),
                |(index, item)| { Item { index, size: *item } }
            ))
            .spacing(10.0)
        ]
        .spacing(10.0)
        .padding_all(10.0);
    }
}

impl Component for Item {
    fn build(&self, _ctx: &mut Context) -> impl View {
        let index = self.index.to_owned();
        spacer(self.size)
            .border(4.0, BLACK)
            .background(BLUE)
            .on_click(move || println!("Clicked: {}", index))
    }
}

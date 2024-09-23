mod core;
mod views;

use core::{App, ContentBuilder, Context, View};
use macroquad::prelude::*;
use views::{Clickable, Column, Component, Paddable, Spacer};

#[macroquad::main("RustUI")]
async fn main() {
    let mut app = App::new(Main);
    let debug_spacer = Spacer::new(Vec2::new(10.0, 10.0))
        .padding_all(10.0)
        .padding_all(10.0)
        .padding_all(10.0);
    println!("spacer: {}", size_of_val(&debug_spacer));
    println!("ContentBuilder: {}", size_of::<ContentBuilder>());
    println!("Column: {}", size_of::<Column>());

    loop {
        clear_background(WHITE);

        app.build();
        app.draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            app.interact(mouse_position().into());
        }

        next_frame().await
    }
}

#[derive(Clone)]
struct Main;

struct MainState {
    spacing: f32,
    padding: f32,
}

impl Component for Main {
    fn build(&self, ctx: &mut Context) -> impl View {
        let state = ctx.state(|| MainState {
            spacing: 10.0,
            padding: 10.0,
        });

        let view = Column::new(ContentBuilder::from_slice([
            Box::new(Spacer::new(Vec2::new(100.0, 100.0)).on_click({
                let state = state.clone();
                move || {
                    state.borrow_mut().spacing += 10.0;
                }
            })),
            Box::new(Spacer::new(Vec2::new(100.0, 100.0)).on_click({
                let state = state.clone();
                move || {
                    state.borrow_mut().padding += 10.0;
                }
            })),
        ]))
        .spacing(state.borrow().spacing)
        .padding_all(state.borrow().padding);
        view
    }
}

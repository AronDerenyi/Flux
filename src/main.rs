mod core;
mod views;

use core::{App, ContentBuilder, Context, View};
use macroquad::prelude::*;
use views::{Clickable, Column, Component, Paddable, Row, Spacer};

#[macroquad::main("RustUI")]
async fn main() {
    let mut app = App::new(Main);
    app.build();
    app.print();

    // let debug_spacer = Spacer::new(Vec2::new(10.0, 10.0))
    //     .padding_all(10.0)
    //     .padding_all(10.0)
    //     .padding_all(10.0);
    // println!("spacer: {}", size_of_val(&debug_spacer));
    // println!("ContentBuilder: {}", size_of::<ContentBuilder>());
    // println!("Column: {}", size_of::<Column>());

    loop {
        clear_background(WHITE);

        app.draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            app.interact(mouse_position().into());
            app.build();
            app.print();
        }

        next_frame().await
    }
}

#[derive(Clone)]
struct Main;

struct MainState {
    items: Vec<Vec2>,
}

impl Component for Main {
    fn build(&self, ctx: &mut Context) -> impl View {
        let state = ctx.state(|| MainState { items: vec![] });

        let view = Column::new(content![
            Spacer::new(Vec2::new(100.0, 100.0)).on_click({
                let state = state.clone();
                move || {
                    state.borrow_mut().items.push(Vec2::new(100.0, 20.0));
                }
            }),
            Column::new(ContentBuilder::from_items(
                state.borrow().items.iter(),
                |item| { Spacer::new(*item) }
            ))
        ])
        .spacing(10.0)
        .padding_all(10.0);
        view
    }
}

mod component;
mod view;
mod view_tree;
mod views;

pub use component::Component;
use macroquad::prelude::*;
use view::ContentBuilder;
pub use view::View;
use view_tree::{Context, ViewTree};
use views::{Clickable, Column, Paddable, Spacer};

#[macroquad::main("RustUI")]
async fn main() {
    let mut tree = ViewTree::new(|| Main);
    tree.build();
    tree.print();

    let debug_spacer = Spacer::new(Vec2::new(10.0, 10.0))
        .padding_all(10.0)
        .padding_all(10.0)
        .padding_all(10.0);
    println!("spacer: {}", size_of_val(&debug_spacer));
    println!("ContentBuilder: {}", size_of::<ContentBuilder>());
    println!("Column: {}", size_of::<Column>());

    loop {
        clear_background(WHITE);

        tree.build();

        for node in tree.nodes.iter() {
            draw_rectangle_lines(
                node.layout.position.x,
                node.layout.position.y,
                node.layout.size.x,
                node.layout.size.y,
                2.0,
                RED,
            );
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            tree.interaction(mouse_position().into());
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

        Column::new(ContentBuilder::from_slice([
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
        .spacing(state.clone().borrow().spacing)
        .padding_all(state.clone().borrow().padding)
    }
}

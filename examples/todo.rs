use flux_ui::{
    app::App,
    column,
    core::{Binding, Context, View},
    graphics::Color,
    math::Vec2,
    views::*,
};

fn main() {
    App::run(Main);
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

        let count = ctx.state(|| 0);

        let length = ctx.get(state).items.len();
        let a = length + *ctx.get(count);

        return column![
            label("Test\ndefault")
                .size(16.0)
                .padding_vertical(0.0)
                .padding_horizontal(8.0)
                .background(0xE0E0E0),
            label("Test 16, blue")
                .size(12.0)
                .color(Color::BLUE)
                .padding_vertical(0.0)
                .padding_horizontal(8.0)
                .border(1.0, Color::BLACK),
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
            .border(1.0, Color::BLACK)
    }
}

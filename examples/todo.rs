use flux_ui::{
    app::App,
    column,
    core::{Binding, Context, View},
    graphics::Color,
    math::Vec2,
    row,
    views::*,
};

fn main() {
    App::run(ContentView);
}

#[derive(Clone, PartialEq)]
struct Todo {
    name: String,
    done: bool,
}

struct Todos {
    items: Vec<Todo>,
}

#[derive(PartialEq)]
struct ContentView;

impl Component for ContentView {
    fn build(&self, ctx: &mut Context) -> impl View {
        let selected = ctx.state(|| Option::<usize>::None);
        let todos = ctx.state(|| Todos {
            items: vec![
                Todo {
                    name: "First".into(),
                    done: false,
                },
                Todo {
                    name: "Second".into(),
                    done: false,
                },
                Todo {
                    name: "Third".into(),
                    done: false,
                },
                Todo {
                    name: "Fourth".into(),
                    done: false,
                },
            ],
        });

        ListView { selected, todos }.background(0xE0E0E0)
    }
}

#[derive(PartialEq)]
struct ListView {
    selected: Binding<Option<usize>>,
    todos: Binding<Todos>,
}

impl Component for ListView {
    fn build(&self, ctx: &mut Context) -> impl View {
        let selected_binding = self.selected;
        let todos_binding = self.todos;

        let selected = ctx.get(self.selected);
        column![
            column(ContentBuilder::from_items(
                ctx.get(self.todos).items.iter().enumerate(),
                |(index, item)| {
                    ListItemView {
                        index,
                        todos: todos_binding,
                        todo: item.clone(),
                        selected: selected.map_or(false, |selected| selected == index),
                    }
                    .on_click(move |ctx| *ctx.get_mut(selected_binding) = Some(index))
                },
            ))
            .spacing(2.0),
            spacer(),
            row![
                spacer().height(0.0),
                label("New item").size(16.0),
                spacer().height(0.0)
            ]
            .padding_all(16.0)
            .background(Color::WHITE)
            .on_click(move |ctx| {
                let items = &mut ctx.get_mut(todos_binding).items;
                items.push(Todo {
                    name: format!("Item {}", items.len() + 1),
                    done: false,
                })
            })
        ]
        .spacing(16.0)
        .padding_all(16.0)
    }
}

#[derive(PartialEq)]
struct ListItemView {
    index: usize,
    todos: Binding<Todos>,
    todo: Todo,
    selected: bool,
}

impl Component for ListItemView {
    fn build(&self, _ctx: &mut Context) -> impl View {
        let Self { index, todos, .. } = *self;

        row![
            label(&self.todo.name).size(16.0),
            spacer().height(0.0),
            spacer()
                .width(20.0)
                .height(20.0)
                .background(Color::RED)
                .on_click(move |ctx| {
                    ctx.get_mut(todos).items.remove(index);
                })
        ]
        .padding_all(16.0)
        .border(
            2.0,
            if self.selected {
                0x404040.into()
            } else {
                Color::TRANSPARENT
            },
        )
        .background(Color::WHITE)
    }
}

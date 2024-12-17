use flux_ui::prelude::*;

fn main() {
    App::run(
        WindowOptions {
            background: WindowBackground::Blurred,
            show_titlebar: false,
            ..Default::default()
        },
        ContentView,
    );
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

        col![
            spacer()
                .height(28.0)
                .background::<Color>((223, 223, 223, 200).into()),
            ListView { selected, todos }.background(Color::from_rgba(255, 255, 255, 200))
        ]
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
        col![
            col(ContentBuilder::from_items(
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
                spacer()
                    .width(100.0)
                    .height(100.0)
                    .background(BoxDecoration {
                        color: Some(Color::WHITE),
                        border: None,
                        radius: 32.0,
                        smoothing: 1.0,
                    })
                    .background(BoxDecoration {
                        color: Some(Color::BLACK),
                        border: None,
                        radius: 32.0,
                        smoothing: 0.0,
                    }),
                spacer()
                    .width(100.0)
                    .height(100.0)
                    .background(BoxDecoration {
                        color: Some(Color::BLACK),
                        border: None,
                        radius: 32.0,
                        smoothing: 0.0,
                    }),
                spacer()
                    .width(100.0)
                    .height(100.0)
                    .background(BoxDecoration {
                        color: Some(Color::BLACK),
                        border: None,
                        radius: 32.0,
                        smoothing: 0.6,
                    }),
                spacer()
                    .width(100.0)
                    .height(100.0)
                    .background(BoxDecoration {
                        color: Some(Color::BLACK),
                        border: None,
                        radius: 32.0,
                        smoothing: 1.0,
                    }),
                spacer().height(0.0)
            ]
            .spacing(8.0),
            spacer(),
            AddButton {
                todos: todos_binding
            }
        ]
        .spacing(16.0)
        .padding_all(16.0)
        .on_click(move |ctx| *ctx.get_mut(selected_binding) = None)
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
            label("Delete")
                .size(12.0)
                .color(Color::WHITE)
                .padding_bottom(1.0)
                .padding_axial(8.0, 4.0)
                .background(BoxDecoration {
                    color: Some(Color::RED),
                    border: None,
                    radius: 32.0,
                    smoothing: 0.6,
                })
                .on_click(move |ctx| {
                    ctx.get_mut(todos).items.remove(index);
                })
        ]
        .padding_all(16.0)
        .background(BoxDecoration {
            color: Some(Color::WHITE),
            border: if self.selected {
                Some(BorderDecoration {
                    width: 2.0,
                    color: 0x404040.into(),
                })
            } else {
                None
            },
            radius: 8.0,
            smoothing: 0.6,
        })
    }
}

#[derive(PartialEq)]
struct AddButton {
    todos: Binding<Todos>,
}

impl Component for AddButton {
    fn build(&self, ctx: &mut Context) -> impl View {
        let Self { todos } = *self;
        let color = ctx.state(|| Color::WHITE);

        row![
            spacer().height(0.0),
            label("New item").size(16.0),
            spacer().height(0.0)
        ]
        .padding_all(16.0)
        .background(BoxDecoration {
            color: Some(*ctx.get(color)),
            border: None,
            radius: 8.0,
            smoothing: 0.6,
        })
        .on_mouse(move |ctx, prev_state, state| {
            match state {
                MouseState::Idle => *ctx.get_mut(color) = Color::WHITE,
                MouseState::Hover => *ctx.get_mut(color) = 0xEFEFEF.into(),
                MouseState::Pressed => *ctx.get_mut(color) = 0xDFDFDF.into(),
            }
            if state == MouseState::Hover && prev_state == MouseState::Pressed {
                let items = &mut ctx.get_mut(todos).items;
                items.push(Todo {
                    name: format!("Item {}", items.len() + 1),
                    done: false,
                });
            }
        })
    }
}

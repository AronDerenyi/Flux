use crate::{
    core::{
        context::{Context, ContextMut, StateChanges, StateDependencies, States},
        interaction::Interaction,
        view::View,
        view_tree::ViewTree,
    },
    graphics::renderer::Renderer,
    utils::{bigraph::Bigraph, id_vec::Id},
};
use glam::Vec2;
use std::collections::{HashMap, HashSet};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalPosition, Size},
    event::{DeviceId, ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    platform::macos::WindowAttributesExtMacOS,
    window::{Theme, Window, WindowAttributes, WindowId},
};

pub struct WindowOptions {
    pub title: String,
    pub size: Vec2,
    pub background: WindowBackground,
    pub show_title: bool,
    pub show_buttons: bool,
    pub show_titlebar: bool,
}

pub enum WindowBackground {
    Opaque,
    Transparent,
    Blurred,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: "Flux".into(),
            size: Vec2::new(800.0, 600.0),
            background: WindowBackground::Opaque,
            show_title: true,
            show_buttons: true,
            show_titlebar: true,
        }
    }
}

pub struct App {
    window_options: WindowOptions,
    state: AppState,
    cursors: HashMap<DeviceId, Option<PhysicalPosition<f64>>>,
    tree: ViewTree,
    states: States,
    state_dependencies: StateDependencies,
    state_changes: StateChanges,
}

enum AppState {
    Uninitialized,
    Ok(Window, Renderer),
}

impl App {
    pub fn run(window_options: WindowOptions, root: impl View) {
        let mut states = HashMap::new();
        let mut state_dependencies = Bigraph::new();
        let tree = ViewTree::build_from(
            &mut Context::new(&mut states, &mut state_dependencies),
            Vec2::new(800.0, 600.0),
            root,
        );

        let mut app = Self {
            window_options,
            state: AppState::Uninitialized,
            cursors: HashMap::new(),
            tree,
            states,
            state_dependencies,
            state_changes: HashSet::new(),
        };

        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut app).unwrap();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let AppState::Uninitialized = self.state {
            let window_attributes = WindowAttributes::default()
                .with_theme(Some(Theme::Light))
                .with_title(self.window_options.title.clone())
                .with_inner_size(Size::new(LogicalSize {
                    width: self.window_options.size.x,
                    height: self.window_options.size.y,
                }))
                .with_blur(
                    if let WindowBackground::Blurred = self.window_options.background {
                        true
                    } else {
                        false
                    },
                )
                .with_transparent(
                    if let WindowBackground::Opaque = self.window_options.background {
                        false
                    } else {
                        true
                    },
                )
                .with_title_hidden(!self.window_options.show_title)
                .with_titlebar_buttons_hidden(!self.window_options.show_buttons)
                .with_titlebar_transparent(!self.window_options.show_titlebar)
                .with_fullsize_content_view(!self.window_options.show_titlebar);

            let window = event_loop.create_window(window_attributes).unwrap();
            let renderer = Renderer::new(&window);

            window.request_redraw();
            self.state = AppState::Ok(window, renderer);
        };
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let AppState::Ok(window, renderer) = &mut self.state else {
            return;
        };
        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                renderer.set_size(size.width, size.height);

                let size = size.to_logical(window.scale_factor());
                self.tree.resize(Vec2::new(size.width, size.height));

                window.request_redraw();
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                renderer.set_scale_factor(scale_factor);

                let size = window.inner_size().to_logical(scale_factor);
                self.tree.resize(Vec2::new(size.width, size.height));

                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                renderer.render(|painter| self.tree.draw(painter));
            }
            WindowEvent::CursorEntered { device_id } => {
                self.cursors.insert(device_id, None);
            }
            WindowEvent::CursorLeft { device_id } => {
                self.cursors.remove(&device_id);
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                if let Some(cursor_position) = self.cursors.get_mut(&device_id) {
                    *cursor_position = Some(position);
                }
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                let Some(position) = self
                    .cursors
                    .get(&device_id)
                    .and_then(Option::as_ref)
                    .map(|physical| physical.to_logical(window.scale_factor()))
                else {
                    return;
                };

                if state != ElementState::Pressed || button != MouseButton::Left {
                    return;
                }

                let mut context = ContextMut::new(&mut self.states, &mut self.state_changes);
                let interaction = Interaction::Click(Vec2::new(position.x, position.y));
                self.tree.interact(&mut context, interaction);

                let mut dirty_views = HashSet::<Id>::new();
                for state_key in self.state_changes.iter() {
                    dirty_views.extend(
                        self.state_dependencies
                            .get_v_connections(*state_key)
                            .iter()
                            .filter_map(|id| id.as_ref()),
                    );
                }
                self.state_changes.clear();

                if !dirty_views.is_empty() {
                    let mut context = Context::new(&mut self.states, &mut self.state_dependencies);
                    let size = window.inner_size().to_logical(window.scale_factor());
                    for id in dirty_views {
                        self.tree
                            .rebuild(&mut context, Vec2::new(size.width, size.height), id);
                    }

                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

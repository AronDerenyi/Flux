use crate::{
    core::{
        context::{StateChanges, StateDependencies, States},
        Context, View, ViewTree,
    },
    graphics::Renderer,
    utils::bigraph::Bigraph,
};
use glam::Vec2;
use std::collections::{HashMap, HashSet};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, Size},
    event::WindowEvent,
    window::{Window, WindowAttributes},
};

pub struct App {
    state: AppState,
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
    pub fn new(root: impl View) -> Self {
        let mut states = HashMap::new();
        let mut state_dependencies = Bigraph::new();
        let tree = ViewTree::build_from(
            &mut Context::new(&mut states, &mut state_dependencies),
            Vec2::new(800.0, 600.0),
            root,
        );

        Self {
            state: AppState::Uninitialized,
            tree,
            states,
            state_dependencies,
            state_changes: HashSet::new(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let AppState::Uninitialized = self.state {
            let mut window_attributes = WindowAttributes::default();
            window_attributes.title = "Flux".into();
            window_attributes.inner_size = Some(Size::new(LogicalSize {
                width: 800.0,
                height: 600.0,
            }));

            let window = event_loop.create_window(window_attributes).unwrap();
            let renderer = Renderer::new(&window);

            window.request_redraw();
            self.state = AppState::Ok(window, renderer);
        };
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
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
                window.request_redraw();
            }
            _ => {}
        }
    }
}

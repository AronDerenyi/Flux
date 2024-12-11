use super::WinitRenderer;
use skia_safe::Canvas;
use std::mem;
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, Size},
    event::WindowEvent,
    window::{Window, WindowAttributes},
};

pub struct WinitApp<R: WinitRenderer, B: FnOnce(&Window) -> R> {
    state: AppState<R, B>,
    render_function: fn(&Canvas),
}

enum AppState<R: WinitRenderer, B: FnOnce(&Window) -> R> {
    Building(B),
    Error(WinitError),
    Ok(Window, R),
}

impl<R: WinitRenderer, B: FnOnce(&Window) -> R> WinitApp<R, B> {
    pub fn new(renderer_builder: B, render_function: fn(&Canvas)) -> Self {
        Self {
            state: AppState::Building(renderer_builder),
            render_function,
        }
    }
}

enum WinitError {
    Uninitialized,
}

impl<R: WinitRenderer, B: FnOnce(&Window) -> R> ApplicationHandler for WinitApp<R, B> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.state = match mem::replace(&mut self.state, AppState::Error(WinitError::Uninitialized))
        {
            AppState::Building(builder) => {
                let mut window_attributes = WindowAttributes::default();
                window_attributes.title = "Flux".into();
                window_attributes.inner_size = Some(Size::new(LogicalSize {
                    width: 800.0,
                    height: 600.0,
                }));

                let window = event_loop.create_window(window_attributes).unwrap();
                let renderer = builder(&window);

                window.request_redraw();
                AppState::Ok(window, renderer)
            }
            state => state,
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
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                renderer.set_scale_factor(scale_factor);
            }
            WindowEvent::RedrawRequested => {
                renderer.render(|canvas| (self.render_function)(canvas));
                window.request_redraw();
            }
            _ => {}
        }
    }
}

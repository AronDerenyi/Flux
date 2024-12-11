use crate::winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, Size},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    raw_window_handle::HasWindowHandle,
    window::{Window, WindowAttributes, WindowId},
    WinitRenderer,
};
use cocoa::{appkit::NSView, base::id as cocoa_id};
use core_graphics_types::geometry::CGSize;
use foreign_types_shared::{ForeignType, ForeignTypeRef};
use metal_rs::{CommandQueue, Device, MTLPixelFormat, MetalLayer};
use objc::rc::autoreleasepool;
use objc::runtime::YES;
use skia_safe::{
    gpu::{self, backend_render_targets, mtl, DirectContext, SurfaceOrigin},
    scalar, Canvas, Color4f, ColorType, Paint, Point, Rect,
};

pub struct SkiaRenderer {
    layer: MetalLayer,
    queue: CommandQueue,
    skia: DirectContext,
    scale_factor: f64,
}

impl SkiaRenderer {
    pub fn new(window: &Window) -> Self {
        let window_handle = window
            .window_handle()
            .expect("Failed to retrieve a window handle");

        let raw_window_handle = window_handle.as_raw();

        let device = Device::system_default().expect("no device found");

        let metal_layer = {
            let draw_size = window.inner_size();
            let layer = MetalLayer::new();
            layer.set_device(&device);
            layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
            layer.set_presents_with_transaction(false);
            // Disabling this option allows Skia's Blend Mode to work.
            // More about: https://developer.apple.com/documentation/quartzcore/cametallayer/1478168-framebufferonly
            layer.set_framebuffer_only(false);

            unsafe {
                let view = match raw_window_handle {
                    raw_window_handle::RawWindowHandle::AppKit(appkit) => appkit.ns_view.as_ptr(),
                    _ => panic!("Wrong window handle type"),
                } as cocoa_id;
                view.setWantsLayer(YES);
                view.setLayer(layer.as_ref() as *const _ as _);
            }
            layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));
            layer
        };

        let command_queue = device.new_command_queue();

        let backend = unsafe {
            mtl::BackendContext::new(
                device.as_ptr() as mtl::Handle,
                command_queue.as_ptr() as mtl::Handle,
            )
        };

        let skia_context = gpu::direct_contexts::make_metal(&backend, None).unwrap();

        Self {
            layer: metal_layer,
            queue: command_queue,
            skia: skia_context,
            scale_factor: window.scale_factor(),
        }
    }
}

impl WinitRenderer for SkiaRenderer {
    fn set_size(&mut self, width: u32, height: u32) {
        self.layer
            .set_drawable_size(CGSize::new(width as f64, height as f64));
    }

    fn set_scale_factor(&mut self, scale_factor: f64) {
        self.scale_factor = scale_factor;
    }

    fn render(&mut self, f: impl FnOnce(&Canvas)) {
        let Some(drawable) = self.layer.next_drawable() else {
            return;
        };

        let (drawable_width, drawable_height) = {
            let size = self.layer.drawable_size();
            (size.width as scalar, size.height as scalar)
        };

        let mut surface = unsafe {
            let texture_info = mtl::TextureInfo::new(drawable.texture().as_ptr() as mtl::Handle);

            let backend_render_target = backend_render_targets::make_mtl(
                (drawable_width as i32, drawable_height as i32),
                &texture_info,
            );

            gpu::surfaces::wrap_backend_render_target(
                &mut self.skia,
                &backend_render_target,
                SurfaceOrigin::TopLeft,
                ColorType::BGRA8888,
                None,
                None,
            )
            .unwrap()
        };

        let canvas = surface.canvas();
        canvas.scale((self.scale_factor as f32, self.scale_factor as f32));
        f(canvas);

        self.skia.flush_and_submit();
        drop(surface);

        let command_buffer = self.queue.new_command_buffer();
        command_buffer.present_drawable(drawable);
        command_buffer.commit();
    }
}

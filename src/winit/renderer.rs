use skia_safe::Canvas;

pub trait WinitRenderer {
    fn set_size(&mut self, width: u32, height: u32);
    fn set_scale_factor(&mut self, scale_factor: f64);
    fn render(&mut self, f: impl FnOnce(&Canvas));
}

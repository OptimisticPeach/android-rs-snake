use graphics::*;

pub fn draw_pause<G: Graphics>(
    c: &Context,
    g: &mut G,
    winfo: &mut super::window_info::WindowInfoCache,
) {
    rectangle(
        [1., 1., 1., 0.5],
        [
            0.,
            0.,
            winfo.window_size.0 as f64,
            winfo.window_size.1 as f64,
        ],
        c.transform,
        g,
    );
}

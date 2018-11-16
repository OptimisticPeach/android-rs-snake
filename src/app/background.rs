use graphics::*;
use graphics::math::*;

const BACKGROUND_COLOR: [f32; 4] = [0.4, 0.38, 0.4, 1.];

pub fn background_draw<G: Graphics>(
    c: &Context,
    transform: Matrix2d,
    g: &mut G,
    winfo: &mut super::window_info::WindowInfoCache
) {
    let mut line = line::Line::new(BACKGROUND_COLOR, 2.);
    let grid = graphics::grid::Grid{
        cols: winfo.grid_size.0 as u32,
        rows: winfo.grid_size.1 as u32,
        units: 64.
    };
    grid.draw(&line, &c.draw_state, transform, g);

    rectangle(BACKGROUND_COLOR, [0., 0., winfo.gridoffsets.0, winfo.window_size.1 as f64], c.transform, g);
    rectangle(BACKGROUND_COLOR, [0., 0., winfo.window_size.0 as f64, winfo.gridoffsets.1], c.transform, g);
    rectangle(BACKGROUND_COLOR, [winfo.window_size.0 as f64 - winfo.gridoffsets.0, 0., winfo.gridoffsets.0, winfo.window_size.1 as f64], c.transform, g);
    rectangle(BACKGROUND_COLOR, [0., winfo.window_size.1 as f64 - (winfo.offsets.1 + winfo.margin as f64), winfo.window_size.0 as f64, winfo.offsets.1 + winfo.margin as f64], c.transform, g);
}

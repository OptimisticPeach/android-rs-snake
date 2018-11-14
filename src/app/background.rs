use graphics::*;

fn draw_grid<G: Graphics>(
    c: &Context,
    g: &mut G,
    width: u32,
    height: u32,
    header_y: u32,
    margin_y: u32,
    margin_x: u32,
    precalc: Option<((usize, usize), (f64, f64))>,
) -> Option<((f64, f64), (usize, usize))> {
    let rect = rectangle::Rectangle::new([0., 0., 0., 1.]);
    match precalc {
        Some(((num_x, num_y), (gridoffset_x, gridoffset_y))) => {
            for i in 0..num_x {
                for j in 0..num_y {
                    rect.draw(
                        rectangle::square(
                            gridoffset_x + (i * 64) as f64,
                            gridoffset_y + (j * 64) as f64,
                            60.,
                        ),
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }
            }
            None
        }
        None => {
            let height = height - (margin_y * 2) - header_y;
            let width = width - (margin_x * 2);
            let num_x = width / 64;
            let num_y = height / 64;
            let offset_x = (width as f64 - (num_x as f64 * 64.)) * 0.5;
            let offset_y = (height as f64 - (num_y as f64 * 64.)) * 0.5;

            for i in 0..num_x {
                for j in 0..num_y {
                    rect.draw(
                        rectangle::square(
                            margin_x as f64 + (i * 64) as f64 + offset_x,
                            header_y as f64 + margin_y as f64 + (j * 64) as f64 + offset_y,
                            60.,
                        ),
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }
            }
            Some(((offset_x, offset_y), (num_x as usize, num_y as usize)))
        }
    }
}

pub fn background_draw<G: Graphics>(
    c: &Context,
    g: &mut G,
    winfo: &mut super::window_info::WindowInfoCache,
    dirty: bool,
) {
    if dirty {
        let (offset, grid) = draw_grid(
            c,
            g,
            winfo.window_size.0 as u32,
            winfo.window_size.1 as u32,
            winfo.header as u32,
            winfo.margin as u32,
            winfo.margin as u32,
            None,
        )
        .unwrap();
        winfo.set_offsets(offset);
        winfo.grid_size = grid;
    } else {
        draw_grid(
            c,
            g,
            0,
            0,
            winfo.header as u32,
            winfo.margin as u32,
            winfo.margin as u32,
            Some((winfo.grid_size, winfo.gridoffsets)),
        );
    }
}

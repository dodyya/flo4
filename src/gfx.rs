use std::cmp::min;

use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

// const COLORS: [&str; 7] = [
//     "9a7bd2", "0000ff", "00ff00", "00ffff", "ff0000", "ff00ff", "ffff00",
// ];

use crate::board::Board;
use crate::board::Cell;
use hsv::{self, hsv_to_rgb};

pub const PIXEL_SCALE: u32 = 40;
const PPC: usize = 3;
pub const PIXELS_PER_CELL: u32 = PPC as u32;

pub struct Gfx {
    pub window: Window,
    pixels: Pixels,
    pub width: u32,
    pub height: u32,
}

impl Gfx {
    pub async fn new(cols: u32, rows: u32) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new();
        // physical window size = virtual size × scale; the web build uses a
        // smaller fixed canvas instead of the full native resolution.
        #[cfg(not(target_arch = "wasm32"))]
        let physical_size = PhysicalSize::new(
            cols * PIXEL_SCALE * PIXELS_PER_CELL,
            rows * PIXEL_SCALE * PIXELS_PER_CELL,
        );
        #[cfg(target_arch = "wasm32")]
        let physical_size = PhysicalSize::new(cols * 60, rows * 60);

        let window = WindowBuilder::new()
            .with_title("Flow Four")
            .with_inner_size(physical_size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();

        // On the web, attach winit's canvas to the host page.
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            let canvas = window.canvas();
            canvas.set_width(physical_size.width);
            canvas.set_height(physical_size.height);
            let doc = web_sys::window().unwrap().document().unwrap();
            let el = web_sys::Element::from(canvas);
            match doc.get_element_by_id("flo4-canvas-container") {
                Some(container) => container.append_child(&el).unwrap(),
                None => doc.body().unwrap().append_child(&el).unwrap(),
            };
        }

        // SurfaceTexture uses the physical (window) pixels,
        // but the 'logical' pixel buffer stays at width×height
        let surface_texture =
            SurfaceTexture::new(physical_size.width, physical_size.height, &window);

        let pixels = PixelsBuilder::new(
            cols * PIXELS_PER_CELL,
            rows * PIXELS_PER_CELL,
            surface_texture,
        )
        .build_async()
        .await
        .unwrap();

        (
            Gfx {
                window,
                pixels,
                width: cols * PIXELS_PER_CELL,
                height: rows * PIXELS_PER_CELL,
            },
            event_loop,
        )
    }

    pub fn render(&mut self) {
        self.pixels.render().unwrap();
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw();
    }

    pub fn display(&mut self, board: &Board) {
        let n_colors = board.num_colors() as u8;
        let frame = self.pixels.frame_mut();

        let W = self.width as usize;

        let white = [255, 255, 255, 255];
        let black = [0, 0, 0, 255];
        frame.fill(0);

        for i in 0..board.len() {
            let c = board[i];
            match c {
                Cell::Empty {} => {}
                _ => {
                    let color;
                    // if c.color() < 5 {
                    let hue = (c.color() as f64 / n_colors as f64);
                    let rgb = hsv_to_rgb(
                        (2.85419 * hue % 1.0) * 360.0,
                        1.0,
                        if hue > 0.5 { 0.8 } else { 1.0 },
                    );
                    color = [rgb.0, rgb.1, rgb.2, 255];
                    // } else {
                    // let mut decoded = [0; 3];
                    // hex::decode_to_slice(COLORS[(c.color()) as usize], &mut decoded)
                    //     .expect("Decoding failed");
                    // color = [decoded[0], decoded[1], decoded[2], 255];
                    // // }

                    let topleft = ((i * PPC) / W * PPC * W + (i * PPC) % W) * 4;

                    for i in 0..PPC - 2 {
                        frame[topleft + 4 + (W) * (i + 1) * 4
                            ..topleft + (W) * (i + 1) * 4 + 4 + 4 * (PPC - 2)]
                            .copy_from_slice(&color.repeat(PPC - 2));
                    }

                    let orientation = board.orientation(i);
                    if c.is_head() {
                        frame[topleft..topleft + 4 * PPC].copy_from_slice(&color.repeat(PPC));

                        for i in 0..PPC {
                            frame[topleft + (W) * i * 4..topleft + (W) * i * 4 + 4 * PPC]
                                .copy_from_slice(&color.repeat(PPC));
                        }

                        frame[topleft + (PPC / 2) * 4 + (W) * (PPC / 2) * 4
                            ..topleft + (PPC / 2) * 4 + (W) * (PPC / 2) * 4 + 4]
                            .copy_from_slice(&white);

                        // if orientation == 0 {
                        //     for i in 0..PPC {
                        //         frame[topleft + (W) * (i) * 4..topleft + (W) * (i) * 4 + 4 * (PPC)]
                        //             .copy_from_slice(&black.repeat(PPC));
                        //     }

                        //     frame[topleft + 4..topleft + 4 + 4 * (PPC - 2)]
                        //         .copy_from_slice(&color.repeat(PPC - 2));
                        //     for i in 0..PPC - 2 {
                        //         frame[topleft + (W) * (i + 1) * 4..topleft + (W) * (i + 1) * 4 + 4]
                        //             .copy_from_slice(&color);
                        //     }

                        //     frame[topleft + 4 + (W) * (PPC - 1) * 4
                        //         ..topleft + 4 + (W) * (PPC - 1) * 4 + 4 * (PPC - 2)]
                        //         .copy_from_slice(&color.repeat(PPC - 2));

                        //     for i in 0..PPC - 2 {
                        //         frame[topleft + 4 * (PPC - 1) + (W) * (i + 1) * 4
                        //             ..topleft + 4 * (PPC - 1) + (W) * (i + 1) * 4 + 4]
                        //             .copy_from_slice(&color);
                        //     }
                        // }
                    }

                    if orientation & 1 == 1 {
                        frame[topleft + 4..topleft + 4 + 4 * (PPC - 2)]
                            .copy_from_slice(&color.repeat(PPC - 2));
                    }

                    if orientation & 2 == 2 {
                        for i in 0..PPC - 2 {
                            frame[topleft + (W) * (i + 1) * 4..topleft + (W) * (i + 1) * 4 + 4]
                                .copy_from_slice(&color);
                        }
                    }

                    if orientation & 4 == 4 {
                        frame[topleft + 4 + (W) * (PPC - 1) * 4
                            ..topleft + 4 + (W) * (PPC - 1) * 4 + 4 * (PPC - 2)]
                            .copy_from_slice(&color.repeat(PPC - 2));
                    }

                    if orientation & 8 == 8 {
                        for i in 0..PPC - 2 {
                            frame[topleft + 4 * (PPC - 1) + (W) * (i + 1) * 4
                                ..topleft + 4 * (PPC - 1) + (W) * (i + 1) * 4 + 4]
                                .copy_from_slice(&color);
                        }
                    }
                }
            }
        }
    }

    pub fn success_display(&mut self, board: &Board) {
        let frame = self.pixels.frame_mut();

        let W = self.width as usize;

        let white = [255, 255, 255, 255];
        let black = [0, 0, 0, 255];
        frame.fill(0);

        for i in 0..board.len() {
            let c = board[i];
            match c {
                Cell::Empty {} => {}
                _ => {
                    let color = [255, 255, 255, 255];

                    let topleft = ((i * PPC) / W * PPC * W + (i * PPC) % W) * 4;

                    for i in 0..PPC - 2 {
                        frame[topleft + 4 + (W) * (i + 1) * 4
                            ..topleft + (W) * (i + 1) * 4 + 4 + 4 * (PPC - 2)]
                            .copy_from_slice(&color.repeat(PPC - 2));
                    }

                    let orientation = board.orientation(i);
                    if c.is_head() {
                        frame[topleft..topleft + 4 * PPC].copy_from_slice(&color.repeat(PPC));

                        for i in 0..PPC {
                            frame[topleft + (W) * i * 4..topleft + (W) * i * 4 + 4 * PPC]
                                .copy_from_slice(&color.repeat(PPC));
                        }

                        // frame[topleft + (PPC / 2) * 4 + (W) * (PPC / 2) * 4
                        //     ..topleft + (PPC / 2) * 4 + (W) * (PPC / 2) * 4 + 4]
                        //     .copy_from_slice(&white);

                        if orientation == 0 {
                            for i in 0..PPC {
                                frame[topleft + (W) * (i) * 4..topleft + (W) * (i) * 4 + 4 * (PPC)]
                                    .copy_from_slice(&black.repeat(PPC));
                            }

                            frame[topleft + 4..topleft + 4 + 4 * (PPC - 2)]
                                .copy_from_slice(&color.repeat(PPC - 2));
                            for i in 0..PPC - 2 {
                                frame[topleft + (W) * (i + 1) * 4..topleft + (W) * (i + 1) * 4 + 4]
                                    .copy_from_slice(&color);
                            }

                            frame[topleft + 4 + (W) * (PPC - 1) * 4
                                ..topleft + 4 + (W) * (PPC - 1) * 4 + 4 * (PPC - 2)]
                                .copy_from_slice(&color.repeat(PPC - 2));

                            for i in 0..PPC - 2 {
                                frame[topleft + 4 * (PPC - 1) + (W) * (i + 1) * 4
                                    ..topleft + 4 * (PPC - 1) + (W) * (i + 1) * 4 + 4]
                                    .copy_from_slice(&color);
                            }
                        }
                    }

                    if orientation & 1 == 1 {
                        frame[topleft + 4..topleft + 4 + 4 * (PPC - 2)]
                            .copy_from_slice(&color.repeat(PPC - 2));
                    }

                    if orientation & 2 == 2 {
                        for i in 0..PPC - 2 {
                            frame[topleft + (W) * (i + 1) * 4..topleft + (W) * (i + 1) * 4 + 4]
                                .copy_from_slice(&color);
                        }
                    }

                    if orientation & 4 == 4 {
                        frame[topleft + 4 + (W) * (PPC - 1) * 4
                            ..topleft + 4 + (W) * (PPC - 1) * 4 + 4 * (PPC - 2)]
                            .copy_from_slice(&color.repeat(PPC - 2));
                    }

                    if orientation & 8 == 8 {
                        for i in 0..PPC - 2 {
                            frame[topleft + 4 * (PPC - 1) + (W) * (i + 1) * 4
                                ..topleft + 4 * (PPC - 1) + (W) * (i + 1) * 4 + 4]
                                .copy_from_slice(&color);
                        }
                    }
                }
            }
        }
    }

    pub fn fail_display(&mut self, board: &Board) {
        let frame = self.pixels.frame_mut();

        let W = self.width as usize;

        let white = [255, 255, 255, 255];
        let black = [0, 0, 0, 255];
        frame.fill(0);

        for i in 0..board.len() {
            let c = board[i];
            match c {
                Cell::Empty {} => {}
                _ => {
                    let color = [128, 0, 0, 255];

                    let topleft = ((i * PPC) / W * PPC * W + (i * PPC) % W) * 4;

                    for i in 0..PPC - 2 {
                        frame[topleft + 4 + (W) * (i + 1) * 4
                            ..topleft + (W) * (i + 1) * 4 + 4 + 4 * (PPC - 2)]
                            .copy_from_slice(&color.repeat(PPC - 2));
                    }

                    let orientation = board.orientation(i);
                    if c.is_head() {
                        frame[topleft..topleft + 4 * PPC].copy_from_slice(&color.repeat(PPC));

                        for i in 0..PPC {
                            frame[topleft + (W) * i * 4..topleft + (W) * i * 4 + 4 * PPC]
                                .copy_from_slice(&color.repeat(PPC));
                        }

                        // frame[topleft + (PPC / 2) * 4 + (W) * (PPC / 2) * 4
                        //     ..topleft + (PPC / 2) * 4 + (W) * (PPC / 2) * 4 + 4]
                        //     .copy_from_slice(&white);

                        if orientation == 0 {
                            for i in 0..PPC {
                                frame[topleft + (W) * (i) * 4..topleft + (W) * (i) * 4 + 4 * (PPC)]
                                    .copy_from_slice(&black.repeat(PPC));
                            }

                            frame[topleft + 4..topleft + 4 + 4 * (PPC - 2)]
                                .copy_from_slice(&color.repeat(PPC - 2));
                            for i in 0..PPC - 2 {
                                frame[topleft + (W) * (i + 1) * 4..topleft + (W) * (i + 1) * 4 + 4]
                                    .copy_from_slice(&color);
                            }

                            frame[topleft + 4 + (W) * (PPC - 1) * 4
                                ..topleft + 4 + (W) * (PPC - 1) * 4 + 4 * (PPC - 2)]
                                .copy_from_slice(&color.repeat(PPC - 2));

                            for i in 0..PPC - 2 {
                                frame[topleft + 4 * (PPC - 1) + (W) * (i + 1) * 4
                                    ..topleft + 4 * (PPC - 1) + (W) * (i + 1) * 4 + 4]
                                    .copy_from_slice(&color);
                            }
                        }
                    }

                    if orientation & 1 == 1 {
                        frame[topleft + 4..topleft + 4 + 4 * (PPC - 2)]
                            .copy_from_slice(&color.repeat(PPC - 2));
                    }

                    if orientation & 2 == 2 {
                        for i in 0..PPC - 2 {
                            frame[topleft + (W) * (i + 1) * 4..topleft + (W) * (i + 1) * 4 + 4]
                                .copy_from_slice(&color);
                        }
                    }

                    if orientation & 4 == 4 {
                        frame[topleft + 4 + (W) * (PPC - 1) * 4
                            ..topleft + 4 + (W) * (PPC - 1) * 4 + 4 * (PPC - 2)]
                            .copy_from_slice(&color.repeat(PPC - 2));
                    }

                    if orientation & 8 == 8 {
                        for i in 0..PPC - 2 {
                            frame[topleft + 4 * (PPC - 1) + (W) * (i + 1) * 4
                                ..topleft + 4 * (PPC - 1) + (W) * (i + 1) * 4 + 4]
                                .copy_from_slice(&color);
                        }
                    }
                }
            }
        }
    }
}

fn _rst(frame: &mut [u8]) {
    let black = [0, 0, 0, 255].repeat(frame.len() / 4);
    frame.copy_from_slice(&black)
}

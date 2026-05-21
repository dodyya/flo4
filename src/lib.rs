#![allow(unused)]
#![allow(non_snake_case)]

mod board;
mod game;
mod gfx;
mod solver;
mod solver_stack;

use crate::board::Board;
use crate::game::Game;
use crate::solver::Solver;
use crate::solver_stack::SolverStack;
use std::cmp::min;

use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::ControlFlow,
};

const SOLVE_STEPS_PER_FRAME: u32 = 16;

// Puzzles for each grid size, bundled in so the web build needs no filesystem.
const PUZZLES_6: &str = include_str!("puzzles_6x6.txt");
const PUZZLES_9: &str = include_str!("puzzles_9x9.txt");
const PUZZLES_12: &str = include_str!("puzzles_12x12.txt");

#[derive(Clone, Copy)]
pub enum Mode {
    Play,
    Solve,
}

fn nth_puzzle(size: usize, n: u32) -> &'static str {
    let bundle = match size {
        6 => PUZZLES_6,
        12 => PUZZLES_12,
        _ => PUZZLES_9,
    };
    let all: Vec<&str> = bundle
        .split("\n@@@\n")
        .filter(|p| !p.trim().is_empty())
        .collect();
    all[((n.max(1) - 1) as usize) % all.len()].trim()
}

fn solver_for(size: usize, n: u32) -> SolverStack {
    let mut board = Board::load_board(nth_puzzle(size, n), size, size);
    board.strip();
    SolverStack::new(Solver::new(&board))
}

fn display_size(gfx: &gfx::Gfx) -> (f64, f64) {
    let s = gfx.window.inner_size();
    (s.width.max(1) as f64, s.height.max(1) as f64)
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    let query = web_sys::window()
        .and_then(|w| w.location().search().ok())
        .unwrap_or_default();
    let mode = if query.contains("solve") {
        Mode::Solve
    } else {
        Mode::Play
    };
    let size = if query.contains("res=6") {
        6
    } else if query.contains("res=12") {
        12
    } else {
        9
    };
    wasm_bindgen_futures::spawn_local(run(mode, size));
}

pub async fn run(mode: Mode, size: usize) {
    match mode {
        Mode::Play => run_play(size).await,
        Mode::Solve => run_solve(size).await,
    }
}

async fn run_play(size: usize) {
    let mut n = 1;
    let mut game = Game::new(nth_puzzle(size, n));
    let mut col = 0;
    let mut row = 0;
    let (mut gfx, event_loop) = gfx::Gfx::new(size as u32, size as u32).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Event::MainEventsCleared = event {
            gfx.display(game.get_board());
            gfx.render();

            if game.is_finished() {
                #[cfg(not(target_arch = "wasm32"))]
                println!("Level {} complete!", n);
                n += 1;
                #[cfg(not(target_arch = "wasm32"))]
                {
                    *control_flow = ControlFlow::WaitUntil(
                        std::time::Instant::now()
                            .checked_add(std::time::Duration::from_secs(3))
                            .unwrap(),
                    );
                }
                game = Game::new(nth_puzzle(size, n));
            }
        }

        if let Event::WindowEvent { event, .. } = &event {
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::MouseInput { state, button, .. } => match (state, button) {
                    (ElementState::Pressed, MouseButton::Left) => {
                        game.handle_mouse_press(row, col)
                    }
                    (ElementState::Released, MouseButton::Left) => {
                        game.handle_mouse_release()
                    }
                    (ElementState::Pressed, MouseButton::Right) => {
                        game.handle_right_click()
                    }
                    _ => {}
                },
                WindowEvent::CursorMoved { position, .. } => {
                    let (cols, rows) = {
                        let b = game.get_board();
                        (b.cols.max(1), b.rows.max(1))
                    };
                    let (w, h) = display_size(&gfx);
                    let new_col = min((position.x * cols as f64 / w) as usize, cols - 1);
                    let new_row = min((position.y * rows as f64 / h) as usize, rows - 1);
                    if new_row != row || new_col != col {
                        row = new_row;
                        col = new_col;
                        game.handle_mouse_move(row, col);
                    }
                }
                _ => {}
            }
        }
    });
}

async fn run_solve(size: usize) {
    let mut n = 1;
    let mut solver = solver_for(size, n);
    let mut hold = 0u32;
    let (mut gfx, event_loop) = gfx::Gfx::new(size as u32, size as u32).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Event::MainEventsCleared = event {
            if hold > 0 {
                hold -= 1;
                if hold == 0 {
                    n += 1;
                    solver = solver_for(size, n);
                }
            } else if solver.done() || solver.failed() {
                hold = 24;
            } else {
                for _ in 0..SOLVE_STEPS_PER_FRAME {
                    if solver.done() || solver.failed() {
                        break;
                    }
                    solver.step();
                }
            }
            gfx.display(solver.get_board());
            gfx.render();
        }

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = &event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}

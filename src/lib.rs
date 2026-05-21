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

const ROWS: usize = 12;
const COLS: usize = 12;
const SOLVE_STEPS_PER_FRAME: u32 = 16;

// The 150 12x12 puzzles, bundled in so the web build needs no filesystem.
const PUZZLES: &str = include_str!("puzzles_12x12.txt");

pub enum Mode {
    Play,
    Solve,
}

fn nth_puzzle(n: u32) -> &'static str {
    let all: Vec<&str> = PUZZLES.split("\n@@@\n").collect();
    all[((n.max(1) - 1) as usize) % all.len()].trim()
}

fn solver_for(n: u32) -> SolverStack {
    let mut board = Board::load_board(nth_puzzle(n), ROWS, COLS);
    board.strip();
    SolverStack::new(Solver::new(&board))
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    let solve = web_sys::window()
        .and_then(|w| w.location().search().ok())
        .map_or(false, |s| s.contains("solve"));
    let mode = if solve { Mode::Solve } else { Mode::Play };
    wasm_bindgen_futures::spawn_local(run(mode));
}

pub async fn run(mode: Mode) {
    match mode {
        Mode::Play => run_play().await,
        Mode::Solve => run_solve().await,
    }
}

async fn run_play() {
    let mut n = 1;
    let mut game = Game::new(nth_puzzle(n));
    let mut col = 0;
    let mut row = 0;
    let (mut gfx, event_loop) = gfx::Gfx::new(ROWS as u32, COLS as u32).await;

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
                game = Game::new(nth_puzzle(n));
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
                    let size = gfx.window.inner_size();
                    let new_col = min(
                        (position.x * COLS as f64 / size.width.max(1) as f64) as usize,
                        COLS - 1,
                    );
                    let new_row = min(
                        (position.y * ROWS as f64 / size.height.max(1) as f64) as usize,
                        ROWS - 1,
                    );
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

async fn run_solve() {
    let mut n = 1;
    let mut solver = solver_for(n);
    let mut hold = 0u32;
    let (mut gfx, event_loop) = gfx::Gfx::new(ROWS as u32, COLS as u32).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Event::MainEventsCleared = event {
            if hold > 0 {
                hold -= 1;
                if hold == 0 {
                    n += 1;
                    solver = solver_for(n);
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

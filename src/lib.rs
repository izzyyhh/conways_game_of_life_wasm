mod cell;
mod utils;

use cell::{Cell, CellState};
use utils::build_neighbors;
use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

const CANVAS_SIZE: usize = 400;
const CANVAS_ID: &str = "game";
const BOARD_SIZE: usize = 20;
const CELL_SIZE: usize = 20;
const ALIVE_COLOR: &str = "green";
const DEAD_COLOR: &str = "red";
const ZOMBIE_COLOR: &str = "black";

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn draw_board(ctx: &CanvasRenderingContext2d, board: &Vec<Vec<Cell>>) -> Result<(), JsValue> {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            match board[i][j].state {
                CellState::Alive => {
                    ctx.set_fill_style(&JsValue::from_str(ALIVE_COLOR));
                    ctx.fill_rect(
                        (j as f64) * CELL_SIZE as f64,
                        (i as f64) * CELL_SIZE as f64,
                        CELL_SIZE as f64,
                        CELL_SIZE as f64,
                    );
                }
                CellState::Dead => {
                    ctx.set_fill_style(&JsValue::from_str(DEAD_COLOR));
                    ctx.fill_rect(
                        (j as f64) * CELL_SIZE as f64,
                        (i as f64) * CELL_SIZE as f64,
                        CELL_SIZE as f64,
                        CELL_SIZE as f64,
                    );
                }
                _ => {
                    ctx.set_fill_style(&JsValue::from_str(ZOMBIE_COLOR));
                    ctx.fill_rect(
                        (j as f64) * CELL_SIZE as f64,
                        (i as f64) * CELL_SIZE as f64,
                        CELL_SIZE as f64,
                        CELL_SIZE as f64,
                    );
                }
            }
        }
    }

    Ok(())
}

#[wasm_bindgen]
pub fn life(iteration: i32) -> Result<(), JsValue> {
    let canvas_id = CANVAS_ID;

    let window = window().expect("cannot find window");
    let doc = window.document().expect("cannot find document");
    let canvas = doc
        .get_element_by_id(canvas_id)
        .expect(&format!("cannot find canvas with id {}", canvas_id))
        .dyn_into::<HtmlCanvasElement>()
        .expect("cannot cast to HtmlCanvasElement");

    canvas.set_height(CANVAS_SIZE as u32);
    canvas.set_width(CANVAS_SIZE as u32);

    let ctx = canvas
        .get_context("2d")
        .expect("cannot get context")
        .expect("2d rendering context does not exist")
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("cannot cast into CanvasRenderingContext2d");

    let mut board = utils::intialize_board(BOARD_SIZE);
    build_neighbors(&mut board, BOARD_SIZE);
    draw_board(&ctx, &board).expect("could not draw board");
    
    for _ in 0..iteration {
        let cloned_board = board.clone();

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                let cell = &mut board[i][j];
                cell.update(&cloned_board);
            }
        }

        draw_board(&ctx, &board).expect("cannot draw board");
    }

    Ok(())
}

//! Lit cells on the backdrop grid: a sparse scatter of grid squares a shade
//! off the sink, fixed like the grid so they stay on its lines, under the top
//! of the home page. They settle in with the hero's entrance and then hold;
//! a third of them breathe a few percent over half a minute. The scatter is
//! seeded, so it is the same squares on every load.

use dioxus::prelude::*;

/// The virtual grid the scatter is drawn on, in cells: wide and tall enough
/// to cover any viewport's top half at the stylesheet's cell size. The layer
/// clips whatever falls outside.
const COLUMNS: u32 = 80;
const ROWS: u32 = 24;
/// One cell in this many is lit.
const DENSITY: u32 = 9;
/// How far apart the columns settle, so the scatter arrives left to right
/// the way the heatmap does.
const SETTLE_STEP_MS: u32 = 14;

struct Cell {
    column: u32,
    row: u32,
    shade: u32,
    settle_ms: u32,
    breathe_ms: Option<u32>,
}

/// The scatter, from a small linear congruential generator with a fixed
/// seed: which cells light, how bright, and when they arrive.
fn cells() -> Vec<Cell> {
    let mut state: u32 = 0x9e37_79b9;
    let mut next = move || {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        state >> 8
    };
    let mut out = Vec::new();
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            if next() % DENSITY != 0 {
                continue;
            }
            let roll = next();
            out.push(Cell {
                column,
                row,
                shade: roll % 3 + 1,
                settle_ms: column * SETTLE_STEP_MS + (roll >> 4) % 300,
                breathe_ms: (roll % 3 == 0).then_some((roll >> 6) % 20_000),
            });
        }
    }
    out
}

#[component]
pub fn GridCells() -> Element {
    rsx! {
        div { class: "grid-cells", aria_hidden: "true",
            for cell in cells() {
                if let Some(breathe) = cell.breathe_ms {
                    span {
                        class: "grid-cell grid-cell-{cell.shade} grid-cell-breathes",
                        style: "--col: {cell.column}; --row: {cell.row}; animation-delay: {cell.settle_ms}ms, {breathe}ms",
                    }
                } else {
                    span {
                        class: "grid-cell grid-cell-{cell.shade}",
                        style: "--col: {cell.column}; --row: {cell.row}; animation-delay: {cell.settle_ms}ms",
                    }
                }
            }
        }
    }
}

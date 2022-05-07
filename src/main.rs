// use std::{thread, time};
use std::io::{stdout, Write};
use bracket_lib::prelude::*;
use fixed::types::I20F12;
use vecmath::*;

mod commons;
use commons::{Relation, CardinalDir};

type Fp32 = I20F12;

const VEHICLE_LENGTH: u32 = 7; /// Should be odd number
const VEHICLE_WIDTH: u32 = 2; /// Depth in layers from axis
const VEHICLE_CHAR: char = '=';

struct Vehicle {
    /// Center point
    pos: Vector2<u32>,

    /// Fraction of full clockwise circle rotation, each increment is 1 / 255 of rotation
    rotation: u8,

    // Tiles per tick
    // speed: Fp32,

    // Sub-grid position
    // grid_delta: Vector2<Fp32>,
}

/// Recursively calculates how many ring shifts are possible for rotation
fn collapse_depth(line: u8) -> u8 {
    if line == 0 {
        0
    } else {
        line - 1 + collapse_depth(line - 1)
    }
}

/// Find shift at particular ring in collapsed line
fn get_depth(line: u8, ring: u8, collapse: u8) -> u8 {
    if ring == 0 || collapse == 0 {
        0
    } else if line > collapse {
        if line - ring <= collapse {1} else {0}
    } else {
        1 + get_depth(line - 1, ring - 1, collapse - line)
    }
}

impl Vehicle {
    fn draw(&self, ctx: &mut BTerm) {
        let line = VEHICLE_LENGTH as u8 / 2 + 1;
        let angles = collapse_depth(line) + line;
        let dir = CardinalDir::from_circle_rotation(self.rotation);
        let angle = match dir {
            CardinalDir::East => {
                let norm = if self.rotation >= 225 { u8::MAX - self.rotation } else { self.rotation };
                norm / (32 / angles)
            },
            _ => {
                let qdrnt = 64 * dir as u8;
                let norm = if self.rotation <= qdrnt { qdrnt - self.rotation } else { self.rotation - qdrnt };
                norm / (32 / angles)
            },
        };
        let draw_fn = match dir {
            CardinalDir::East | CardinalDir::West => |ctx: &mut BTerm, x, y| {
                for yloc in y - VEHICLE_WIDTH ..= y + VEHICLE_WIDTH {
                    ctx.set(x, yloc, RGB::named(WHITE), RGB::named(BLACK), to_cp437(VEHICLE_CHAR));
                }
            },
            _ => |ctx: &mut BTerm, x, y| {
                for xloc in x - VEHICLE_WIDTH ..= x + VEHICLE_WIDTH {
                    ctx.set(xloc, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437(VEHICLE_CHAR));
                }
            },
        };
        draw_fn(ctx, self.pos[0], self.pos[1]);
        for ring in 1..=line-1 {
            let ring = ring as u32;
            let diff = get_depth(line, ring as u8, angle) as u32;
            let (dx0, dx1, dy0, dy1) = match dir {
                CardinalDir::East => {
                    if let 0..=32 = self.rotation {
                        (Relation::Add(ring), Relation::Sub(ring), Relation::Add(diff), Relation::Sub(diff))
                    } else {
                        (Relation::Add(ring), Relation::Sub(ring), Relation::Sub(diff), Relation::Add(diff))
                    }
                },
                CardinalDir::South => {
                    if let 33..=64 = self.rotation {
                        (Relation::Add(diff), Relation::Sub(diff), Relation::Add(ring), Relation::Sub(ring))
                    } else {
                        (Relation::Sub(diff), Relation::Add(diff), Relation::Add(ring), Relation::Sub(ring))
                    }
                },
                CardinalDir::West => {
                    if let 97..=128 = self.rotation {
                        (Relation::Sub(ring), Relation::Add(ring), Relation::Add(diff), Relation::Sub(diff))
                    } else {
                        (Relation::Sub(ring), Relation::Add(ring), Relation::Sub(diff), Relation::Add(diff))
                    }
                },
                CardinalDir::North => {
                    if let 161..=192 = self.rotation {
                        (Relation::Sub(diff), Relation::Add(diff), Relation::Sub(ring), Relation::Add(ring))
                    } else {
                        (Relation::Sub(diff), Relation::Add(diff), Relation::Add(ring), Relation::Sub(ring))
                    }
                },
            };
            draw_fn(ctx, dx0.apply(self.pos[0]), dy0.apply(self.pos[1]));
            draw_fn(ctx, dx1.apply(self.pos[0]), dy1.apply(self.pos[1]));
        }
    }
}

struct State {
    wagon: Vehicle,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(0, 0, format!("rotation: {}, dir: {:?}", self.wagon.rotation, CardinalDir::from_circle_rotation(self.wagon.rotation)));
        ctx.print(0, 1, format!("mouse pos: {:?}", ctx.mouse_pos()));
        if let Some(_) = ctx.key {
            self.wagon.rotation = self.wagon.rotation.wrapping_add(2);
        }
        self.wagon.draw(ctx);
        stdout().flush().expect("flush failed");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(80, 50)?
        .with_title("roguepunk")
        .build()?;
    let gs = State {
        wagon: Vehicle {
            pos: [10, 10],
            rotation: 0,
        }
    };
    main_loop(context, gs)
}

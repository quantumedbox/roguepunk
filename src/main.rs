// use std::{thread, time};
use std::io::{stdout, Write};
use bracket_lib::prelude::*;
// use fixed::types::I20F12;

mod commons;
use commons::{CardinalDir};
mod vehicle;

// type Fp32 = I20F12;

struct State {
    wagon: vehicle::Vehicle,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(0, 0, format!("rotation: {}, dir: {:?}", self.wagon.rotation, CardinalDir::from_circle_rotation(self.wagon.rotation)));
        if let Some(_) = ctx.key {
            self.wagon.rotation = self.wagon.rotation.wrapping_add(2);
        }
        self.wagon.draw(ctx);
        stdout().flush().expect("flush failed"); // todo: Only do it when breacket-lib terminal backend is used
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(80, 50)?
        .with_title("roguepunk")
        .build()?;
    let gs = State {
        wagon: vehicle::Vehicle {
            pos: [10, 10],
            rotation: 0,
        }
    };
    main_loop(context, gs)
}

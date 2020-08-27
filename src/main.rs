mod board;
mod life;

use bevy::prelude::*;
use life::Life;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(Life {
            board_color: Color::rgb(0.2, 0.2, 0.2),
            alive_color: Color::rgb(0.8, 0.8, 0.8),
            dead_color: Color::rgb(0.1, 0.1, 0.1),
        })
        .run();
}

mod board;
mod life;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    prelude::*,
};
use life::Life;

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(Life {
            board_color: Color::rgb(0.2, 0.2, 0.2),
            alive_color: Color::rgb(0.8, 0.8, 0.8),
            dead_color: Color::rgb(0.1, 0.1, 0.1),
        })
        .run();
}

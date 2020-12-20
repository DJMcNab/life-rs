use bevy::{core::FixedTimestep, prelude::*};
use rand::Rng;

use super::board::*;

#[derive(Copy, Clone)]
pub struct Life {
    pub width: i32,
    pub height: i32,
    pub border: Vec2,
    pub color_board: Color,
    pub color_alive: Color,
    pub color_dead: Color,
}

impl Plugin for Life {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.clone())
            .add_resource(Board::new(self.width, self.height))
            .add_stage_after(
                stage::UPDATE,
                "fixed_update",
                SystemStage::serial()
                    .with_run_criteria(FixedTimestep::step(0.050))
                    .with_system(rules.system())
                    .with_system(update_tiles.system()),
            )
            .add_startup_system(setup.system());
    }
}

#[derive(Default)]
struct Theme {
    border: Vec2,
    board: Handle<ColorMaterial>,
    alive: Handle<ColorMaterial>,
    dead: Handle<ColorMaterial>,
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut board: ResMut<Board>,
    life: Res<Life>,
) {
    let theme = Theme {
        border: life.border,
        board: materials.add(life.color_board.into()),
        alive: materials.add(life.color_alive.into()),
        dead: materials.add(life.color_dead.into()),
    };

    let pixel_size = Vec2::new(600.0, 600.0);
    let tile_size = pixel_size / board.size();

    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: theme.board.clone_weak(),
            transform: Transform::from_translation(Vec3::zero()),
            sprite: Sprite {
                size: pixel_size,
                resize_mode: SpriteResizeMode::default(),
            },
            ..Default::default()
        });

    for idx in 0..board.len() {
        let mut rng = rand::thread_rng();

        let coords = board.idx2cds(idx);

        let offset = tile_size / Vec2::new(2.0, 2.0);
        let center = pixel_size / Vec2::new(2.0, 2.0);

        let pos2 = board.idx2cds(idx).to_vec() * tile_size - center + offset;
        let pos3 = Vec3::new(pos2.x, pos2.y, 1.0);

        let state = if rng.gen_bool(0.5) {
            TileState::Alive
        } else {
            TileState::Dead
        };

        commands
            .spawn(SpriteBundle {
                material: theme.alive.clone_weak(),
                transform: Transform::from_translation(pos3),
                sprite: Sprite {
                    size: tile_size - theme.border,
                    resize_mode: SpriteResizeMode::default(),
                },
                ..Default::default()
            })
            .with(Generation::new(state))
            .with(coords);

        board.tiles.push(Tile::new(state, coords.get_neighbors()));
    }

    commands.insert_resource(theme);
}

fn rules(board: ResMut<Board>, mut query: Query<(&mut Generation, &Coordinates)>) {
    for (mut gen, coords) in query.iter_mut() {
        let tile = board.get_tile(*coords).unwrap();

        let alive_count = tile
            .neighbors
            .iter()
            .filter(|n| match board.get_tile(**n) {
                Some(tile) => {
                    if tile.state == TileState::Alive {
                        true
                    } else {
                        false
                    }
                }
                None => false,
            })
            .count();

        match tile.state {
            TileState::Alive => {
                if alive_count < 2 || alive_count > 3 {
                    gen.state = TileState::Dead;
                }
            }
            TileState::Dead => {
                if alive_count == 3 {
                    gen.state = TileState::Alive;
                }
            }
        }
    }
}

fn update_tiles(
    mut board: ResMut<Board>,
    colors: Res<Theme>,
    mut query: Query<(&Coordinates, &Generation, &mut Handle<ColorMaterial>), Changed<Generation>>,
) {
    for (coords, gen, mut mat) in query.iter_mut() {
        let mut tile = board.get_mut_tile(*coords).unwrap();
        tile.state = gen.state;
    
        match tile.state {
            TileState::Alive => {
                *mat = colors.alive.clone_weak();
            }
            TileState::Dead => {
                *mat = colors.dead.clone_weak();
            }
        }
    }
}

use std::time::Duration;

use bevy::{core::FixedTimestep, prelude::*};
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;

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
            .init_resource::<WrapsTime>()
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

    let mut rng = rand::rngs::StdRng::seed_from_u64(0xe505af0ab1519ba4);
    for idx in 0..board.len() {
        let coords = board.idx2cds(idx);

        let offset = tile_size / Vec2::new(2.0, 2.0);
        let center = pixel_size / Vec2::new(2.0, 2.0);

        let pos2 = board.idx2cds(idx).to_vec() * tile_size - center + offset;
        let pos3 = Vec3::new(pos2.x, pos2.y, 1.0);

        let state = **&[TileState::Alive, TileState::Dead]
            .choose(&mut rng)
            .unwrap();

        commands
            .spawn(SpriteBundle {
                material: theme.alive.clone(),
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

fn rules(
    board: ResMut<Board>,
    mut query: Query<(&mut Generation, &Coordinates)>,
    time: Res<Time>,
    mut storage: ResMut<WrapsTime>,
) {
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
    if storage.2 == false {
        storage.2 = true;
    } else {
        panic!("This should be impossible");
    }
    storage.0 = time.time_since_startup();
}

#[derive(Default)]
struct WrapsTime(pub Duration, pub Duration, pub bool);

fn update_tiles(
    mut board: ResMut<Board>,
    colors: Res<Theme>,
    time: Res<Time>,
    mut storage: ResMut<WrapsTime>,
    mut queries: QuerySet<(
        Query<(&Coordinates, &Generation, &mut Handle<ColorMaterial>)>,
        Query<(&Coordinates, &Generation)>,
    )>,
) {
    if storage.2 == true {
        storage.2 = false;
    } else {
        panic!("This should be impossible");
    }
    let new_len = time.time_since_startup() - storage.0;
    if time.time_since_startup() - storage.0 > storage.1 {
        storage.1 = new_len;
        println!(
            "update_tiles ran {:?} seconds after rules. This is a new maximum",
            time.time_since_startup() - storage.0
        );
    }
    for (coords, gen, mut mat) in queries.q0_mut().iter_mut() {
        let mut tile = board.get_mut_tile(*coords).unwrap();
        tile.state = gen.state;

        match tile.state {
            TileState::Alive => {
                *mat = colors.alive.clone();
            }
            TileState::Dead => {
                *mat = colors.dead.clone();
            }
        }
    }
    for (coords, gen) in queries.q1().iter() {
        let tile = board.get_mut_tile(*coords).unwrap();
        if tile.state != gen.state {
            error!(target: "Oh No", "{:?}", coords);
        }
    }
}

use super::board::Board;
use bevy::prelude::*;
use rand::Rng;

#[derive(Clone)]
pub struct Life {
    pub board_color: Color,
    pub alive_color: Color,
    pub dead_color: Color,
}

impl Plugin for Life {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Board::new(30, 30, 2.0))
            .add_resource(self.clone())
            .add_startup_system(setup.system())
            .add_system(rules.system())
            .add_system_to_stage(stage::POST_UPDATE, update_tiles.system())
            .add_system_to_stage(stage::LAST, draw_tiles.system());
    }
}
#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub enum TileState {
    Alive,
    Dead,
}

#[derive(Default)]
struct ColorTheme {
    board: Handle<ColorMaterial>,
    alive: Handle<ColorMaterial>,
    dead: Handle<ColorMaterial>,
}

struct Tile {
    next_state: TileState,
    neighbours: [Entity; 8],
}

fn setup(
    mut commands: Commands,
    life: Res<Life>,
    board: Res<Board>,
    mut assets: ResMut<Assets<ColorMaterial>>,
) {
    let mut rand = rand::thread_rng();
    let pixel_size = Vec2::new(600.0, 600.0);
    let tile_size = pixel_size / board.size();
    let color_theme = ColorTheme {
        alive: assets.add(life.alive_color.into()),
        dead: assets.add(life.dead_color.into()),
        board: assets.add(life.board_color.into()),
    };

    let exterior = Entity::new();
    commands.spawn_as_entity(exterior, (TileState::Dead,));

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: color_theme.board,
            translation: Translation(Vec3::zero()),
            sprite: Sprite {
                size: pixel_size + board.border,
            },
            ..Default::default()
        });
    let mut cells = Vec::with_capacity(board.length() as usize);
    for _ in 0..board.length() {
        cells.push(Entity::new());
    }

    let cells = cells;
    for (index, cell) in cells.iter().enumerate() {
        let state;
        let material;
        if rand.gen_bool(0.5) {
            state = TileState::Dead;
            material = color_theme.dead;
        } else {
            state = TileState::Alive;
            material = color_theme.alive;
        }

        let offset = tile_size / Vec2::new(2.0, 2.0);
        let center = pixel_size / Vec2::new(2.0, 2.0);

        let pos2: Vec2 = Vec2::from(board.idx2vec(index as i32)) * tile_size - center + offset;
        let pos3 = Vec3::new(pos2.x(), pos2.y(), 1.0);

        let sprite = SpriteComponents {
            material,
            translation: Translation(pos3),
            sprite: Sprite {
                size: tile_size - board.border,
            },
            ..Default::default()
        };

        let neighbours = neighbours(exterior, &*board, index as i32, &cells);
        commands
            .spawn_as_entity(*cell, sprite)
            .with(Tile {
                neighbours,
                next_state: state,
            })
            .with(state);
    }
    commands.insert_resource(color_theme);
}

fn neighbours(default: Entity, board: &Board, index: i32, cells: &[Entity]) -> [Entity; 8] {
    let mut neighbours = [default; 8];
    #[rustfmt::skip]
    const NEIGHBOURS: [(i32, i32); 8] = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0),           (1, 0),
        (-1, 1),  (0, 1),  (1, 1),
    ];
    let position = board.idx2vec(index);

    for (neighbour_pos, neighbour) in NEIGHBOURS.iter().zip(&mut neighbours) {
        if let Some(cell) = cells.get(board.vec2idx(position + (*neighbour_pos).into()) as usize) {
            *neighbour = *cell;
        };
    }

    return neighbours;
}

fn rules(
    board: Res<Board>,
    mut tiles: Query<(&mut Tile, &TileState)>,
    livenesses: Query<&TileState>,
) {
    let mut alive_vec = Vec::new();

    alive_vec.resize(board.length() as usize, 0);

    for (mut tile, state) in &mut tiles.iter() {
        let alive_count = tile
            .neighbours
            .iter()
            .filter(|&&n| {
                *(livenesses
                    .get::<TileState>(n)
                    .expect("Every neighbour has a state"))
                    == TileState::Alive
            })
            .count();
        match state {
            TileState::Alive => {
                if alive_count > 3 || alive_count < 2 {
                    tile.next_state = TileState::Dead;
                }
            }
            TileState::Dead => {
                if alive_count == 3 {
                    tile.next_state = TileState::Alive;
                }
            }
        }
    }
}

fn update_tiles(mut query: Query<(&mut TileState, &Tile)>) {
    for (mut state, tile) in &mut query.iter() {
        *state = tile.next_state;
    }
}

fn draw_tiles(theme: Res<ColorTheme>, mut query: Query<(&TileState, &mut Handle<ColorMaterial>)>) {
    for (state, mut mat) in &mut query.iter() {
        match state {
            TileState::Alive => {
                *mat = theme.alive;
            }
            TileState::Dead => {
                *mat = theme.dead;
            }
        }
    }
}

use bevy::{prelude::*, ui_widgets::Button};
use rand::seq::SliceRandom;

const GRID_SIZE: usize = 5;

// Color palette
const SCREEN_BG: Color = Color::srgb(0.09, 0.11, 0.15);      // Deep dark slate backdrop
const BOARD_BG: Color = Color::srgb(0.72, 0.16, 0.20);       // Polished ruby red board
const GRID_TRAY_BG: Color = Color::srgb(0.50, 0.08, 0.12);   // Deep wine tray
const NORMAL_BUTTON: Color = Color::srgb(0.97, 0.97, 0.98);  // Crisp tile white
const NORMAL_BORDER: Color = Color::srgb(0.78, 0.78, 0.82);  // 3D bottom bevel for normal tile
const HOVERED_BUTTON: Color = Color::srgb(1.0, 0.93, 0.78);  // Warm amber hover
const HOVERED_BORDER: Color = Color::srgb(0.85, 0.75, 0.55);

// Marked tile (clicked, waiting for line)
const MARKED_BUTTON: Color = Color::srgb(0.20, 0.62, 0.40);   // Emerald green stamp
const MARKED_BORDER: Color = Color::srgb(0.12, 0.45, 0.28);
const MARKED_TEXT: Color = Color::srgb(1.0, 1.0, 1.0);

// Struck line tile (completed row / col / diag)
const STRUCK_TILE_BG: Color = Color::srgb(0.98, 0.82, 0.22);  // Glowing victory gold
const STRUCK_TILE_BORDER: Color = Color::srgb(0.78, 0.60, 0.10);
const STRUCK_TILE_TEXT: Color = Color::srgb(0.15, 0.12, 0.05);

const TEXT_COLOR: Color = Color::srgb(0.12, 0.15, 0.20);      // Normal tile text

// Header B-I-N-G-O 3D styling
const LETTER_UNSTRUCK_BG: Color = Color::srgb(0.98, 0.86, 0.35);     // 3D Gold block
const LETTER_UNSTRUCK_BORDER: Color = Color::srgb(0.75, 0.60, 0.15); // Dark gold bottom bevel
const LETTER_UNSTRUCK_TEXT: Color = Color::srgb(0.18, 0.12, 0.04);
const LETTER_STRUCK_BG: Color = Color::srgb(0.35, 0.10, 0.14);       // Dark struck crimson
const LETTER_STRUCK_BORDER: Color = Color::srgb(0.22, 0.05, 0.08);
const LETTER_STRUCK_TEXT: Color = Color::srgb(0.80, 0.35, 0.38);

#[derive(Resource, Default)]
struct BingoState {
    lines_completed: usize,
    completed_rows: [bool; GRID_SIZE],
    completed_cols: [bool; GRID_SIZE],
    main_diag: bool,
    anti_diag: bool,
    is_over: bool,
}

#[derive(Component, Clone, Default)]
struct BingoTile {
    row: usize,
    col: usize,
    marked: bool,
}

#[derive(Component, Clone, Default)]
struct TileHovered;

#[derive(Component, Clone, Default)]
struct TileText;

#[derive(Component, Clone, Default)]
struct TileStrikeLine {
    row: usize,
    col: usize,
}

#[derive(Component, Clone, Default)]
struct BingoLetter {
    index: usize,
}

#[derive(Component, Clone, Default)]
struct LetterText;

#[derive(Component, Clone, Default)]
struct LetterStrikeLine {
    index: usize,
}

#[derive(Component, Clone, Default)]
struct FloatingBingoHeader;

#[derive(Component, Clone, Default)]
struct GameOverOverlay;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<BingoState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                float_header,
                check_completed_lines,
                update_tiles_appearance.after(check_completed_lines),
                update_letters_appearance.after(check_completed_lines),
                update_game_over_overlay.after(check_completed_lines),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn_scene(board());
}

fn board() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BackgroundColor(SCREEN_BG)
        Children [
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(px(20)),
                border_radius: BorderRadius::all(px(20)),
            }
            BackgroundColor(BOARD_BG)
            Children [
                @bingo_header()
                --
                @grid()
            ]
            --
            @game_over_modal()
        ]
    }
}

fn bingo_header() -> impl Scene {
    let letters = ["B", "I", "N", "G", "O"];
    let letter_scenes: Vec<_> = letters
        .iter()
        .enumerate()
        .map(|(i, &ch)| bingo_letter_tile(i, ch))
        .collect();

    bsn! {
        Node {
            flex_direction: FlexDirection::Row,
            column_gap: px(12),
            margin: UiRect::bottom(px(16)),
        }
        FloatingBingoHeader
        UiTransform::IDENTITY
        Children [
            { letter_scenes }
        ]
    }
}

fn bingo_letter_tile(index: usize, letter: &'static str) -> impl Scene {
    bsn! {
        BingoLetter { index }
        Node {
            width: px(58),
            height: px(64),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(px(12)),
            border: UiRect {
                left: px(2),
                right: px(2),
                top: px(2),
                bottom: px(6),
            },
        }
        BackgroundColor(LETTER_UNSTRUCK_BG)
        BorderColor::all(LETTER_UNSTRUCK_BORDER)
        Children [
            Text(letter)
            LetterText
            TextFont {
                font_size: px(34.0),
            }
            TextColor(LETTER_UNSTRUCK_TEXT)
            TextShadow {
                offset: Vec2::new(2.0, 2.5),
                color: Color::srgba(0.0, 0.0, 0.0, 0.45),
            }
            Pickable::IGNORE
            --
            Node {
                position_type: PositionType::Absolute,
                width: percent(84),
                height: px(5),
                border_radius: BorderRadius::all(px(2.5)),
            }
            LetterStrikeLine { index }
            BackgroundColor(Color::srgb(0.95, 0.15, 0.15))
            Visibility::Hidden
            Pickable::IGNORE
        ]
    }
}

fn grid() -> impl Scene {
    let mut numbers: Vec<usize> = (1..=GRID_SIZE * GRID_SIZE).collect();
    numbers.shuffle(&mut rand::rng());

    let buttons: Vec<_> = numbers
        .into_iter()
        .enumerate()
        .map(|(i, num)| {
            let row = i / GRID_SIZE;
            let col = i % GRID_SIZE;
            button(row, col, num)
        })
        .collect();

    bsn! {
        Node {
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex::<Vec<_>>(GRID_SIZE as u16, 1.0),
            grid_template_rows: RepeatedGridTrack::flex::<Vec<_>>(GRID_SIZE as u16, 1.0),
            width: px(460),
            height: px(460),
            row_gap: px(10),
            column_gap: px(10),
            padding: UiRect::all(px(12)),
            border_radius: BorderRadius::all(px(14)),
        }
        BackgroundColor(GRID_TRAY_BG)
        Children [
            { buttons }
        ]
    }
}

fn button(row: usize, col: usize, number: usize) -> impl Scene {
    bsn! {
        Button
        BingoTile {
            row,
            col,
            marked: false,
        }
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(px(8)),
            border: UiRect {
                left: px(1),
                right: px(1),
                top: px(1),
                bottom: px(4),
            },
        }
        BackgroundColor(NORMAL_BUTTON)
        BorderColor::all(NORMAL_BORDER)
        on(|event: On<PointerClick>,
            state: Res<BingoState>,
            mut q: Query<&mut BingoTile>| {
            if state.is_over {
                return;
            }
            if let Ok(mut tile) = q.get_mut(event.entity) {
                tile.marked = !tile.marked;
            }
        })
        on(|event: On<PointerEnter>,
            state: Res<BingoState>,
            mut commands: Commands| {
            if !state.is_over {
                commands.entity(event.entity).insert(TileHovered);
            }
        })
        on(|event: On<PointerLeave>, mut commands: Commands| {
            commands.entity(event.entity).remove::<TileHovered>();
        })
        Children [
            Text(format!("{number}"))
            TileText
            TextFont {
                font_size: px(22.0),
            }
            TextColor(TEXT_COLOR)
            TextShadow {
                offset: Vec2::new(1.0, 1.0),
                color: Color::srgba(0.0, 0.0, 0.0, 0.2),
            }
            Pickable::IGNORE
            --
            Node {
                position_type: PositionType::Absolute,
                width: percent(80),
                height: px(4),
                border_radius: BorderRadius::all(px(2)),
            }
            TileStrikeLine { row, col }
            BackgroundColor(Color::srgb(0.9, 0.15, 0.15))
            Visibility::Hidden
            Pickable::IGNORE
        ]
    }
}

fn game_over_modal() -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        GameOverOverlay
        Visibility::Hidden
        Children [
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::axes(px(44), px(32)),
                border_radius: BorderRadius::all(px(22)),
                border: UiRect {
                    left: px(2),
                    right: px(2),
                    top: px(2),
                    bottom: px(8),
                },
                row_gap: px(14),
            }
            BackgroundColor(Color::srgba(0.06, 0.08, 0.12, 0.95))
            BorderColor::all(Color::srgb(0.98, 0.82, 0.22))
            Children [
                Text("🎉 BINGO! 🎉")
                TextFont {
                    font_size: px(46.0),
                }
                TextColor(Color::srgb(1.0, 0.88, 0.35))
                TextShadow {
                    offset: Vec2::new(2.0, 3.0),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.7),
                }
                --
                Text("All letters completed! Game Over - You Won!")
                TextFont {
                    font_size: px(18.0),
                }
                TextColor(Color::srgb(0.92, 0.94, 0.96))
                --
                @restart_button()
            ]
        ]
    }
}

fn restart_button() -> impl Scene {
    bsn! {
        Button
        Node {
            padding: UiRect::axes(px(28), px(12)),
            border_radius: BorderRadius::all(px(10)),
            border: UiRect {
                left: px(1),
                right: px(1),
                top: px(1),
                bottom: px(4),
            },
            margin: UiRect::top(px(8)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BackgroundColor(Color::srgb(0.98, 0.82, 0.22))
        BorderColor::all(Color::srgb(0.78, 0.60, 0.10))
        on(|_event: On<PointerClick>,
            mut state: ResMut<BingoState>,
            mut q_tiles: Query<(&mut BingoTile, &Children)>,
            mut q_tile_text: Query<&mut Text, With<TileText>>| {
            state.is_over = false;
            let mut numbers: Vec<usize> = (1..=GRID_SIZE * GRID_SIZE).collect();
            numbers.shuffle(&mut rand::rng());

            for (mut tile, children) in &mut q_tiles {
                tile.marked = false;
                let idx = tile.row * GRID_SIZE + tile.col;
                let new_num = numbers[idx];
                for child in children.iter() {
                    if let Ok(mut text) = q_tile_text.get_mut(child) {
                        text.0 = format!("{new_num}");
                    }
                }
            }
        })
        on(|event: On<PointerEnter>, mut bg: Query<&mut BackgroundColor>| {
            if let Ok(mut color) = bg.get_mut(event.entity) {
                *color = BackgroundColor(Color::srgb(1.0, 0.90, 0.40));
            }
        })
        on(|event: On<PointerLeave>, mut bg: Query<&mut BackgroundColor>| {
            if let Ok(mut color) = bg.get_mut(event.entity) {
                *color = BackgroundColor(Color::srgb(0.98, 0.82, 0.22));
            }
        })
        Children [
            Text("Play Again")
            TextFont {
                font_size: px(19.0),
            }
            TextColor(Color::srgb(0.12, 0.10, 0.05))
            Pickable::IGNORE
        ]
    }
}

fn float_header(
    time: Res<Time>,
    mut query: Query<&mut UiTransform, With<FloatingBingoHeader>>,
) {
    let offset_y = (time.elapsed_secs() * 2.5).sin() * 5.0;
    for mut transform in &mut query {
        transform.translation = Val2::px(0.0, offset_y);
    }
}

/// Evaluates grid state, completed rows/cols/diagonals, and lines count
fn check_completed_lines(
    q_tiles: Query<&BingoTile>,
    mut state: ResMut<BingoState>,
) {
    let mut grid = [[false; GRID_SIZE]; GRID_SIZE];
    for tile in &q_tiles {
        if tile.row < GRID_SIZE && tile.col < GRID_SIZE {
            grid[tile.row][tile.col] = tile.marked;
        }
    }

    let mut rows = [false; GRID_SIZE];
    for r in 0..GRID_SIZE {
        if (0..GRID_SIZE).all(|c| grid[r][c]) {
            rows[r] = true;
        }
    }

    let mut cols = [false; GRID_SIZE];
    for c in 0..GRID_SIZE {
        if (0..GRID_SIZE).all(|r| grid[r][c]) {
            cols[c] = true;
        }
    }

    let main_d = (0..GRID_SIZE).all(|i| grid[i][i]);
    let anti_d = (0..GRID_SIZE).all(|i| grid[i][GRID_SIZE - 1 - i]);

    let lines = rows.iter().filter(|&&x| x).count()
        + cols.iter().filter(|&&x| x).count()
        + if main_d { 1 } else { 0 }
        + if anti_d { 1 } else { 0 };

    state.completed_rows = rows;
    state.completed_cols = cols;
    state.main_diag = main_d;
    state.anti_diag = anti_d;
    state.lines_completed = lines;
    if lines >= 5 {
        state.is_over = true;
    }
}

/// Updates tile colors, text colors, and strike lines
fn update_tiles_appearance(
    state: Res<BingoState>,
    mut q_tile_views: Query<(
        &BingoTile,
        Has<TileHovered>,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
    )>,
    mut q_tile_text: Query<&mut TextColor, With<TileText>>,
    mut q_tile_strikes: Query<(&TileStrikeLine, &mut Visibility)>,
) {
    for (tile, is_hovered, mut bg, mut border, children) in &mut q_tile_views {
        let is_in_completed_line = state.completed_rows[tile.row]
            || state.completed_cols[tile.col]
            || (state.main_diag && tile.row == tile.col)
            || (state.anti_diag && tile.row + tile.col == GRID_SIZE - 1);

        let (target_bg, target_border, text_color) = if is_in_completed_line {
            (STRUCK_TILE_BG, STRUCK_TILE_BORDER, STRUCK_TILE_TEXT)
        } else if tile.marked {
            (MARKED_BUTTON, MARKED_BORDER, MARKED_TEXT)
        } else if is_hovered && !state.is_over {
            (HOVERED_BUTTON, HOVERED_BORDER, TEXT_COLOR)
        } else {
            (NORMAL_BUTTON, NORMAL_BORDER, TEXT_COLOR)
        };

        *bg = BackgroundColor(target_bg);
        *border = BorderColor::all(target_border);

        for child in children.iter() {
            if let Ok(mut tc) = q_tile_text.get_mut(child) {
                *tc = TextColor(text_color);
            }
        }
    }

    for (strike_line, mut visibility) in &mut q_tile_strikes {
        let is_struck = state.completed_rows[strike_line.row]
            || state.completed_cols[strike_line.col]
            || (state.main_diag && strike_line.row == strike_line.col)
            || (state.anti_diag && strike_line.row + strike_line.col == GRID_SIZE - 1);

        *visibility = if is_struck {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Updates B-I-N-G-O letter badges and their strike lines
fn update_letters_appearance(
    state: Res<BingoState>,
    mut q_letters: Query<(
        &BingoLetter,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
    )>,
    mut q_letter_text: Query<&mut TextColor, With<LetterText>>,
    mut q_letter_strikes: Query<(&LetterStrikeLine, &mut Visibility)>,
) {
    for (letter, mut bg, mut border, children) in &mut q_letters {
        let is_letter_struck = letter.index < state.lines_completed;

        let (target_bg, target_border, text_color) = if is_letter_struck {
            (LETTER_STRUCK_BG, LETTER_STRUCK_BORDER, LETTER_STRUCK_TEXT)
        } else {
            (
                LETTER_UNSTRUCK_BG,
                LETTER_UNSTRUCK_BORDER,
                LETTER_UNSTRUCK_TEXT,
            )
        };

        *bg = BackgroundColor(target_bg);
        *border = BorderColor::all(target_border);

        for child in children.iter() {
            if let Ok(mut tc) = q_letter_text.get_mut(child) {
                *tc = TextColor(text_color);
            }
        }
    }

    for (letter_strike, mut visibility) in &mut q_letter_strikes {
        let is_struck = letter_strike.index < state.lines_completed;
        *visibility = if is_struck {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Updates game over overlay visibility
fn update_game_over_overlay(
    state: Res<BingoState>,
    mut q_game_over: Query<&mut Visibility, With<GameOverOverlay>>,
) {
    for mut vis in &mut q_game_over {
        *vis = if state.is_over {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

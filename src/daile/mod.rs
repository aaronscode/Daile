use std::fs;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use ron::de;
use serde::Deserialize;
use tcod::colors::*;
use tcod::console::*;
use tcod::input::{self, Event, Key, Mouse};

use crate::graphics::{Map, Tile}; 
use crate::ui::FrameBorder;
use crate::util::glyph_idx_to_char;

const FPS_LIMIT: u32 = 60;

struct Workspace {
    map: Map,
    current_tile: Tile,
}

impl Workspace {
    fn new(width: usize, height: usize) -> Workspace {
        let map = vec![vec![Tile::new(' ', WHITE, BLACK); width]; height];
        Workspace {
            map,
            current_tile: Tile::new(0x01 as char, WHITE, BLACK),
        }
    }
}

pub struct Tcod {
    pub root: Root,
    pub canvas: Offscreen,
    pub palette: Offscreen,
    pub color_palette: Offscreen,
    pub config: DaileConfig,
    pub key: Key,
    pub mouse: Mouse,
}


#[derive(Debug, Deserialize)]
pub struct DaileConfig {
    //pub screen_width: u32,
    //pub screen_height: u32,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub font_horizontal: u32,
    pub font_vertical: u32,
    pub font_path: String,
}
fn save_map(filepath: String, map: &Map) -> std::io::Result<()> {
    let width = map.len() as u64;
    let height = map[0].len() as u64; 

    let path = Path::new(&filepath);

    let file = fs::File::create(&path)?;
    let mut file = BufWriter::new(file);
    file.write_all(&height.to_be_bytes())?;
    file.write_all(&width.to_be_bytes())?;
    for col in map.iter() {
        for tile in col.iter() {
            file.write_all(tile.glyph.to_string().as_bytes())?;
            let fg = tile.foreground_color;
            let bg = tile.background_color;
            let tile_rgb_fgbg = [fg.r, fg.g, fg.b, bg.r, bg.g, bg.b];
            file.write_all(&tile_rgb_fgbg)?;
        }
    }
    Ok(())
}

fn read_map(filepath: String) -> std::io::Result<()> {
    let path = Path::new(&filepath);

    let file = fs::File::open(&path)?;
    let mut file = BufReader::new(file);
    //let width = file.read_exact(mut buf);

    let map = vec![vec![Tile::new(' ', WHITE, BLACK); 1]; 1];

    Ok(())
}

fn handle_keys(tcod: &mut Tcod, workspace: &mut Workspace) -> bool {
    use tcod::input::KeyCode::*;

    match tcod.key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true, // exit game
        Key { code: F5, .. } => {
            println! {"saving!"}
            match save_map(String::from("canvas.dal"), &workspace.map) {
                Err(why) => println!("Error saving: {}", why),
                _ => (),
            }
        }

        /*
        // movement keys
        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => *player_x -= 1,
        Key { code: Right, .. } => *player_x += 1,
        */
        _ => {}
    }

    if tcod.mouse.lbutton {
        let (mx, my) = get_mouse_usize(tcod.mouse);
        if mx > 0
            && mx < (tcod.config.canvas_width + 1) as usize
            && my > 0
            && my < (tcod.config.canvas_height + 1) as usize
        {
            workspace.map[mx - 1][my - 1] = workspace.current_tile;
        } else if mx > (tcod.config.canvas_width + 2) as usize
            && mx < (tcod.config.canvas_width + tcod.config.font_horizontal + 3) as usize
            && my > (tcod.config.canvas_height - tcod.config.font_vertical) as usize
            && my < (tcod.config.canvas_height + 1) as usize
        {
            let col = mx - (tcod.config.canvas_width + 2) as usize - 1;
            let row = my - (tcod.config.canvas_height - tcod.config.font_vertical) as usize - 1;
            workspace
                .current_tile
                .set_glyph(glyph_idx_to_char((16 * row + col) as u32));
        } else if mx > (tcod.config.canvas_width + 2) as usize
            && mx < (tcod.config.canvas_width + 10 + 3) as usize
            && my == 1
        {
            let preset_colors = vec![
                BLACK, WHITE, RED, GREEN, BLUE, ORANGE, YELLOW, PURPLE, SILVER, GOLD,
            ];
            let x = mx - tcod.config.canvas_width as usize - 3;
            if let Some(c) = preset_colors.get(x) {
                workspace.current_tile.set_color_fg(*c);
            }
        }
    } else if tcod.mouse.rbutton {
        let (mx, my) = get_mouse_usize(tcod.mouse);
        if mx > 0
            && mx < (tcod.config.canvas_width + 1) as usize
            && my > 0
            && my < (tcod.config.canvas_height + 1) as usize
        {
            workspace.map[mx - 1][my - 1].set_glyph_color(' ', WHITE, BLACK);
        } else if mx > (tcod.config.canvas_width + 2) as usize
            && mx < (tcod.config.canvas_width + 10 + 3) as usize
            && my == 1
        {
            let preset_colors = vec![
                BLACK, WHITE, RED, GREEN, BLUE, ORANGE, YELLOW, PURPLE, SILVER, GOLD,
            ];
            let x = mx - tcod.config.canvas_width as usize - 3;
            if let Some(c) = preset_colors.get(x) {
                workspace.current_tile.set_color_bkg(*c);
            }
        }
    }

    let font_horizontal = tcod.config.font_horizontal;
    let font_vertical = tcod.config.font_vertical;

    if tcod.mouse.wheel_down {
        let current_glyph = workspace.current_tile.glyph() as u32;
        if current_glyph != 0 {
            workspace
                .current_tile
                .set_glyph(glyph_idx_to_char(current_glyph - 1));
        } else {
            workspace
                .current_tile
                .set_glyph(glyph_idx_to_char(font_horizontal * font_vertical - 1));
        }
        tcod.mouse.wheel_down = false;
    } else if tcod.mouse.wheel_up {
        let current_glyph = workspace.current_tile.glyph() as u32;
        if current_glyph as u32 != (font_horizontal * font_vertical - 1) {
            workspace
                .current_tile
                .set_glyph(glyph_idx_to_char(current_glyph + 1));
        } else {
            workspace.current_tile.set_glyph(0 as char);
        }
        tcod.mouse.wheel_up = false;
    }

    false
}

fn get_mouse_i32(mouse: Mouse) -> (i32, i32) {
    (mouse.cx as i32, mouse.cy as i32)
}

fn get_mouse_usize(mouse: Mouse) -> (usize, usize) {
    (mouse.cx as usize, mouse.cy as usize)
}

fn render_all(tcod: &mut Tcod, workspace: &mut Workspace, mouse_pos: (i32, i32)) {
    for y in 0..tcod.config.canvas_height {
        for x in 0..tcod.config.canvas_width {
            workspace.map[x as usize][y as usize].draw(
                &mut tcod.canvas,
                (x + 1) as i32,
                (y + 1) as i32,
            );
        }
    }

    let corners: [char; 4] = [
        glyph_idx_to_char(218),
        glyph_idx_to_char(191),
        glyph_idx_to_char(217),
        glyph_idx_to_char(192),
    ];

    let canvas_frame = FrameBorder::new(
        0,
        0,
        tcod.config.canvas_width + 1,
        tcod.config.canvas_height + 1,
        corners,
        Some(String::from("Canvas")),
    )
    .unwrap();
    canvas_frame.draw(&mut tcod.canvas);

    if mouse_pos.0 > 0
        && mouse_pos.0 < (tcod.config.canvas_width + 1) as i32
        && mouse_pos.1 > 0
        && mouse_pos.1 < (tcod.config.canvas_height + 1) as i32
    {
        tcod.canvas.set_char_background(
            mouse_pos.0,
            mouse_pos.1,
            LIGHTER_GREY,
            BackgroundFlag::Overlay,
        );
        tcod.canvas
            .set_char(mouse_pos.0, mouse_pos.1, workspace.current_tile.glyph());
        tcod::input::show_cursor(false);
    } else {
        tcod::input::show_cursor(true);
    }
    blit(
        &tcod.canvas,
        (0, 0),
        (
            (tcod.config.canvas_width + 2) as i32,
            (tcod.config.canvas_height + 2) as i32,
        ),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );

    let palette_frame = FrameBorder::new(
        0,
        0,
        tcod.config.font_horizontal + 1,
        tcod.config.font_vertical + 1,
        corners,
        Some(String::from("Palette")),
    )
    .unwrap();
    palette_frame.draw(&mut tcod.palette);

    for y in 0..tcod.config.font_vertical {
        for x in 0..tcod.config.font_horizontal {
            let glyph = glyph_idx_to_char(16 * y + x);

            if glyph == workspace.current_tile.glyph() {
                tcod.palette.set_char_background(
                    (x + 1) as i32,
                    (y + 1) as i32,
                    LIGHTER_GREY,
                    BackgroundFlag::Overlay,
                );
            }

            tcod.palette.set_char((x + 1) as i32, (y + 1) as i32, glyph);
        }
    }

    blit(
        &tcod.palette,
        (0, 0),
        (
            (tcod.config.font_horizontal + 2) as i32,
            (tcod.config.font_vertical + 2) as i32,
        ),
        &mut tcod.root,
        (
            (tcod.config.canvas_width + 2) as i32,
            (tcod.config.canvas_height - tcod.config.font_vertical) as i32,
        ),
        1.0,
        1.0,
    );

    let preset_colors = vec![
        BLACK, WHITE, RED, GREEN, BLUE, ORANGE, YELLOW, PURPLE, SILVER, GOLD,
    ];
    let color_frame =
        FrameBorder::new(0, 0, 11, 11, corners, Some(String::from("Colors"))).unwrap();
    color_frame.draw(&mut tcod.color_palette);

    for (index, color) in preset_colors.iter().enumerate() {
        tcod.color_palette.set_char_background(
            (index + 1) as i32,
            1,
            *color,
            BackgroundFlag::Overlay,
        );
    }

    blit(
        &tcod.color_palette,
        (0, 0),
        (12, 12),
        &mut tcod.root,
        ((tcod.config.canvas_width + 2) as i32, 0),
        1.0,
        1.0,
    );
}

pub fn init_and_run() {
    let config_file = fs::read_to_string("config.ron").expect("Config file not found");
    let config: DaileConfig = de::from_str(config_file.as_str()).expect("Error parsing config.ron");

    let screen_width = config.canvas_width + 2 + config.font_horizontal + 2;
    let screen_height = config.canvas_height + 2;

    println!("{:?}", config);

    tcod::system::set_fps(FPS_LIMIT as i32);

    let root = Root::initializer()
        .font(config.font_path.as_str(), FontLayout::AsciiInRow)
        .font_type(FontType::Greyscale)
        .font_dimensions(config.font_horizontal as i32, config.font_vertical as i32)
        .size(screen_width as i32, screen_height as i32)
        .title("Daile")
        .init();

    let mut tcod = Tcod {
        root,
        canvas: Offscreen::new(
            (config.canvas_width + 2) as i32,
            (config.canvas_height + 2) as i32,
        ),
        palette: Offscreen::new(
            (config.font_horizontal + 2) as i32,
            (config.font_vertical + 2) as i32,
        ),
        color_palette: Offscreen::new(12, 12),
        config: config,
        key: Default::default(),
        mouse: Default::default(),
    };

    let mut workspace = Workspace::new(
        tcod.config.canvas_width as usize,
        tcod.config.canvas_height as usize,
    );

    tcod.root.set_default_foreground(WHITE);
    while !tcod.root.window_closed() {
        tcod.canvas.clear();
        tcod.palette.clear();
        tcod.color_palette.clear();

        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => tcod.mouse = m,
            Some((_, Event::Key(k))) => tcod.key = k,
            _ => tcod.key = Default::default(),
        }

        let mouse_pos = get_mouse_i32(tcod.mouse);

        render_all(&mut tcod, &mut workspace, mouse_pos);
        tcod.root.flush();

        let exit = handle_keys(&mut tcod, &mut workspace);

        if exit {
            break;
        }
    }
}

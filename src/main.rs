use crate::context::Context;
use crate::context::Textures;
use crate::editor::EditorState;
use crate::fn2::create_text_texture;
use crate::fn2::load_font;
use crate::graphics::Graphics;
use crate::level::Level;
use crate::types::NextMode::*;
use crate::types::*;
use crate::util::*;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::render::Texture;

mod context;
mod crates;
mod editor;
mod editor_textures;
mod fn2;
mod general_level_info;
mod graphics;
mod help;
mod level;
mod load_level;
mod random_item_editor;
mod render;
mod tile_selector;
mod types;
mod util;

pub fn main() {
    let sdl = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG);
    let video_subsystem = sdl.video().unwrap();
    let graphics = Graphics::new();
    let window = video_subsystem
        .window(
            "Ultimate Tapan Kaikki - Level Editor",
            graphics.resolution_x,
            graphics.resolution_y,
        )
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let font = load_font("./assets/TETRIS.FN2");
    let selected_icon = create_text_texture(&mut canvas, &texture_creator, &font, "*");
    let crate_textures: Vec<Texture> = crates::get_crates()
        .iter()
        .flatten()
        .map(|name| create_text_texture(&mut canvas, &texture_creator, &font, name))
        .collect();
    let context = Context {
        sdl,
        graphics,
        canvas,
        texture_creator: &texture_creator,
        font,
        textures: Textures {
            floor: texture_creator.load_texture("./assets/FLOOR1.PNG").unwrap(),
            walls: texture_creator.load_texture("./assets/WALLS1.PNG").unwrap(),
            shadows: texture_creator
                .load_texture("./assets/SHADOWS_ALPHA.PNG")
                .unwrap(),
            selected_icon,
            saved_level_name: None,
            crates: crate_textures,
        },
        level: Level::get_default_level((32, 22)),
        selected_tile_id: 0,
        texture_type_selected: TextureType::FLOOR,
        texture_type_scrolled: TextureType::FLOOR,
        mouse: (0, 0),
        level_save_name: String::new(),
        trigonometry: Trigonometry::new(),
        automatic_shadows: true,
    };

    let mut mode = Editor(EditorState::new(context));

    loop {
        mode = match mode {
            Editor(state) => state.exec(),
            TileSelect(state) => state.exec(),
            Help(state) => state.exec(),
            GeneralLevelInfo(state) => state.exec(),
            RandomItemEditor(state) => state.exec(),
            LoadLevel(state) => state.exec(),
            Quit => break,
        }
    }
}

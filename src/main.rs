use crate::context::Context;
use crate::context::Textures;
use crate::editor::EditorState;
use crate::fn2::create_text_texture;
use crate::fn2::load_font;
// use crate::general_level_info::GeneralLevelInfoState;
use crate::graphics::Graphics;
// use crate::help::HelpState;
use crate::level::Level;
// use crate::load_level::LoadLevelState;
// use crate::random_item_editor::RandomItemEditorState;
use crate::tile_selector::TileSelectState;
use crate::types::NextMode::*;
use crate::types::*;
use crate::util::*;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

mod context;
mod crates;
mod editor;
mod editor_textures;
mod fn2;
// mod general_level_info;
mod graphics;
// mod help;
mod level;
// mod load_level;
// mod random_item_editor;
mod render;
mod tile_selector;
mod types;
mod util;

struct VideoContext {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
}

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
    let mut video = {
        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        VideoContext {
            canvas,
            texture_creator,
        }
    };
    let font = load_font("./assets/TETRIS.FN2");
    let selected_icon = create_text_texture(&mut video.canvas, &video.texture_creator, &font, "*");
    let crate_textures: Vec<Texture> = crates::get_crates()
        .iter()
        .flatten()
        .map(|name| create_text_texture(&mut video.canvas, &video.texture_creator, &font, name))
        .collect();
    let textures = Textures {
        floor: video
            .texture_creator
            .load_texture("./assets/FLOOR1.PNG")
            .unwrap(),
        walls: video
            .texture_creator
            .load_texture("./assets/WALLS1.PNG")
            .unwrap(),
        shadows: video
            .texture_creator
            .load_texture("./assets/SHADOWS_ALPHA.PNG")
            .unwrap(),
        selected_icon,
        crates: crate_textures,
    };
    let mut context = Context {
        sdl,
        graphics,
        font,
        textures,
        level: Level::get_default_level((32, 22)),
        selected_tile_id: 0,
        texture_type_selected: TextureType::FLOOR,
        texture_type_scrolled: TextureType::FLOOR,
        mouse: (0, 0),
        level_save_name: String::new(),
        trigonometry: Trigonometry::new(),
        automatic_shadows: true,
    };

    let mut editor = EditorState::new(&mut video, &context);
    let mut tile_select = TileSelectState::new(&mut video, &context);
    // let mut help = HelpState::new(&mut canvas, &context);
    // let mut general_level_info = GeneralLevelInfoState::new(&mut canvas, &context);
    // let mut random_item_editor = RandomItemEditorState::new(&mut canvas, &context);
    // let mut load_level = LoadLevelState::new(&mut canvas, &context);

    let mut mode = Editor;
    loop {
        mode = match mode {
            Editor => editor.exec(&mut video, &mut context),
            TileSelect => tile_select.exec(&mut video, &mut context),
            _ => break
            // Help => help.exec(&mut canvas, &mut context),
            // GeneralLevelInfo => general_level_info.exec(&mut canvas, &mut context),
            // RandomItemEditor(game_type) => {
            //     random_item_editor.exec(&mut canvas, &mut context, game_type)
            // }
            // LoadLevel => load_level.exec(&mut canvas, &mut context),
            // Quit => break,
        }
    }
}

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::render::Texture;

use crate::context::Context;
use crate::context::Textures;
use crate::context_util::get_textures;
use crate::editor::EditorState;
use crate::fn2::load_font;
use crate::general_level_info::GeneralLevelInfoState;
use crate::graphics::Graphics;
use crate::help::HelpState;
use crate::level::Level;
use crate::load_level::LoadLevelState;
use crate::random_item_editor::RandomItemEditorState;
use crate::render::Renderer;
use crate::tile_selector::TileSelectState;
use crate::types::*;
use crate::util::*;

mod context;
mod context_util;
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
        .resizable()
        .build()
        .unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let renderer = Renderer::new(window);
    let font = load_font("./assets/TETRIS.FN2");
    let textures = get_textures(&renderer, &font);
    let mut context = Context {
        graphics,
        font,
        textures,
        level: Level::get_default_level((32, 22)),
        selected_tile_id: 0,
        texture_type_selected: TextureType::Floor,
        texture_type_scrolled: TextureType::Floor,
        mouse: (0, 0),
        level_save_name: String::new(),
        trigonometry: Trigonometry::new(),
        automatic_shadows: true,
        sdl_text_input: video_subsystem.text_input(),
    };

    let mut state = State::new(&renderer, &context);
    loop {
        for event in event_pump.poll_iter() {
            match state.handle_event(&mut context, event) {
                RunState::Quit => return,
                RunState::Run => {}
            }
        }
        state.render(&context)
    }
}

struct State<'a> {
    mode: Mode,
    editor: EditorState<'a>,
    tile_select: TileSelectState<'a>,
    help: HelpState<'a>,
    general_level_info: GeneralLevelInfoState<'a>,
    random_item_editor: RandomItemEditorState<'a>,
    load_level: LoadLevelState<'a>,
}

impl<'a> State<'a> {
    pub fn new(renderer: &'a Renderer, context: &Context<'a>) -> Self {
        Self {
            mode: Mode::Editor,
            editor: EditorState::new(renderer, context),
            tile_select: TileSelectState::new(renderer, context),
            help: HelpState::new(renderer, context),
            general_level_info: GeneralLevelInfoState::new(renderer, context),
            random_item_editor: RandomItemEditorState::new(renderer, context),
            load_level: LoadLevelState::new(renderer, context),
        }
    }

    pub fn handle_event(&mut self, context: &mut Context<'a>, event: Event) -> RunState {
        self.mode = match self.mode {
            Mode::Editor => self.editor.handle_event(context, event),
            Mode::TileSelect => self.tile_select.handle_event(context, event),
            Mode::Help => self.help.handle_event(context, event),
            Mode::GeneralLevelInfo => self.general_level_info.handle_event(context, event),
            Mode::RandomItemEditor(game_mode) => self
                .random_item_editor
                .handle_event(context, game_mode, event),
            Mode::LoadLevel => self.load_level.handle_event(context, event),
            Mode::Quit => Mode::Quit,
        };
        match self.mode {
            Mode::Quit => RunState::Quit,
            _ => RunState::Run,
        }
    }

    pub fn render(&mut self, context: &Context<'a>) {
        match self.mode {
            Mode::Editor => self.editor.render(context),
            Mode::TileSelect => self.tile_select.render(context),
            Mode::Help => self.help.render(context),
            Mode::GeneralLevelInfo => self.general_level_info.render(context),
            Mode::RandomItemEditor(game_type) => self.random_item_editor.render(context, game_type),
            Mode::LoadLevel => self.load_level.render(context),
            Mode::Quit => {}
        };
    }
}

enum RunState {
    Run,
    Quit,
}

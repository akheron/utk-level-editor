use crate::editor::EditorState;
use crate::general_level_info::GeneralLevelInfoState;
use crate::help::HelpState;
use crate::load_level::LoadLevelState;
use crate::random_item_editor::RandomItemEditorState;
use crate::tile_selector::TileSelectState;
use crate::Context;

#[derive(Clone, Copy, PartialEq)]
pub enum TextureType {
    FLOOR = 0,
    WALLS = 1,
    SHADOW = 2,
}

impl TextureType {
    pub fn from_u32(value: u32) -> TextureType {
        match value {
            0 => TextureType::FLOOR,
            1 => TextureType::WALLS,
            2 => TextureType::SHADOW,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub(crate) texture_type: TextureType,
    pub(crate) id: u32,
    pub(crate) shadow: u32,
}

pub type Tiles = Vec<Vec<Tile>>;

pub enum GameType {
    Normal,
    Deathmatch,
}

pub enum NextMode<'a> {
    Editor(EditorState<'a>),
    TileSelect(TileSelectState<'a>),
    Help(HelpState<'a>),
    GeneralLevelInfo(GeneralLevelInfoState<'a>),
    RandomItemEditor(RandomItemEditorState<'a>),
    LoadLevel(LoadLevelState<'a>),
    Quit,
}

impl NextMode<'_> {
    pub fn editor(context: Context) -> NextMode {
        NextMode::Editor(EditorState::new(context))
    }

    pub fn tile_select(context: Context) -> NextMode {
        NextMode::TileSelect(TileSelectState::new(context))
    }

    pub fn help(context: Context) -> NextMode {
        NextMode::Help(HelpState::new(context))
    }

    pub fn general_level_info(context: Context) -> NextMode {
        NextMode::GeneralLevelInfo(GeneralLevelInfoState::new(context))
    }

    pub fn random_item_editor(context: Context, game_type: GameType) -> NextMode {
        NextMode::RandomItemEditor(RandomItemEditorState::new(context, game_type))
    }

    pub fn load_level(context: Context) -> NextMode {
        NextMode::LoadLevel(LoadLevelState::new(context))
    }

    pub fn quit<'a>() -> NextMode<'a> {
        NextMode::Quit
    }
}

pub struct Trigonometry {
    pub(crate) sin: [f32; 360],
    pub(crate) cos: [f32; 360],
}

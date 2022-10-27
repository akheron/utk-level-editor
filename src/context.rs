use crate::fn2::FN2;
use crate::graphics::Graphics;
use crate::Level;
use crate::TextureType;
use crate::Trigonometry;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::Sdl;
use std::cell::RefCell;

pub struct Textures<'a> {
    pub floor: Texture<'a>,
    pub walls: Texture<'a>,
    pub shadows: Texture<'a>,
    pub selected_icon: Texture<'a>,
    pub crates: Vec<Texture<'a>>,
}

pub struct Context<'a> {
    pub sdl: Sdl,
    pub graphics: Graphics,
    pub font: FN2,
    pub textures: Textures<'a>,
    pub level: Level,
    pub selected_tile_id: u32,
    pub texture_type_selected: TextureType,
    pub texture_type_scrolled: TextureType,
    pub mouse: (u32, u32),
    pub level_save_name: String,
    pub trigonometry: Trigonometry,
    pub automatic_shadows: bool,
}

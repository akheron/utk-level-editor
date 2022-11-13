use crate::fn2::FN2;
use crate::graphics::Graphics;
use crate::render::Renderer;
use crate::Level;
use crate::TextureType;
use crate::Trigonometry;

pub struct Textures<Texture> {
    pub floor: Texture,
    pub walls: Texture,
    pub shadows: Texture,
    pub selected_icon: Texture,
    pub saved_level_name: Option<Texture>,
    pub crates: Vec<Texture>,
}

pub struct Context<'a, R: Renderer<'a>> {
    pub graphics: Graphics,
    pub font: FN2,
    pub textures: Textures<R::Texture>,
    pub level: Level,
    pub selected_tile_id: u32,
    pub texture_type_selected: TextureType,
    pub texture_type_scrolled: TextureType,
    pub mouse: (u32, u32),
    pub level_save_name: String,
    pub trigonometry: Trigonometry,
    pub automatic_shadows: bool,
    pub sdl_text_input: sdl2::keyboard::TextInputUtil,
}

impl<'a, R: Renderer<'a>> Context<'a, R> {
    pub fn start_text_input(&self) {
        self.sdl_text_input.start();
    }

    pub fn stop_text_input(&self) {
        self.sdl_text_input.stop();
    }
}

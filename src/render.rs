use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Point;
use sdl2::render::Texture as SdlTexture;
use sdl2::render::TextureQuery;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

use crate::font::Font;
use crate::level::{bullet_crates, energy_crates, weapon_crates, CrateClass};
use crate::level::{StaticCrate, StaticCrateType};
use crate::types::*;
use crate::util::*;
use crate::Graphics;
use crate::Level;
use crate::Textures;

pub enum RendererColor {
    Black,
    White,
    Red,
    Blue,
    LightBlue,
    LightGreen,
    LightGrey,
}

fn get_sdl_color(color: &RendererColor) -> Color {
    match &color {
        RendererColor::Black => Color::from((0, 0, 0)),
        RendererColor::White => Color::from((255, 255, 255)),
        RendererColor::Red => Color::from((255, 0, 0)),
        RendererColor::Blue => Color::from((0, 0, 255)),
        RendererColor::LightBlue => Color::from((100, 100, 255)),
        RendererColor::LightGreen => Color::from((100, 255, 100)),
        RendererColor::LightGrey => Color::from((200, 200, 200)),
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b, a: 255 }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Color> for sdl2::pixels::Color {
    fn from(color: Color) -> Self {
        Self::RGBA(color.r, color.g, color.b, color.a)
    }
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl From<Rect> for sdl2::rect::Rect {
    fn from(rect: Rect) -> Self {
        sdl2::rect::Rect::new(rect.x, rect.y, rect.width, rect.height)
    }
}

pub trait Renderer<'a> {
    type Texture;

    fn load_texture(&'a self, path: &str) -> Self::Texture;
    fn create_texture(&'a self, width: u32, height: u32, data: &[Color]) -> Self::Texture;
    fn clear_screen(&self);
    fn highlight_selected_tile(&self, graphics: &Graphics, id: u32, color: &RendererColor);
    fn draw_line(&self, x0: u32, y0: u32, x1: u32, y1: u32);
    fn render_texture(&self, texture: &Self::Texture, dst: Rect);
    fn fill_and_render_texture(&self, color: RendererColor, texture: &Self::Texture, dst: Rect);
    fn render_level(
        &self,
        graphics: &Graphics,
        level: &Level,
        textures: &Textures<Self::Texture>,
        trigonometry: &Trigonometry,
        font: &Font<'a, Self>,
    );
    fn get_texture_size(texture: &Self::Texture) -> (u32, u32);
    fn window_size(&self) -> (u32, u32);
}

pub struct SdlRenderer {
    canvas: RefCell<Canvas<Window>>,
    texture_creator: TextureCreator<WindowContext>,
}

impl SdlRenderer {
    pub fn new(window: Window) -> Self {
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        Self {
            canvas: RefCell::new(canvas),
            texture_creator,
        }
    }

    fn draw_circle(&self, x_center: i32, y_center: i32, radius: u32, color: &RendererColor) {
        self.canvas_mut().set_draw_color(get_sdl_color(color));

        // https://stackoverflow.com/a/48291620
        let diameter: i32 = radius as i32 * 2;
        let mut x: i32 = radius as i32 - 1;
        let mut y: i32 = 0;
        let mut tx: i32 = 1;
        let mut ty: i32 = 1;
        let mut error: i32 = tx - diameter;

        while x >= y {
            self.canvas_mut()
                .draw_point(Point::new(x_center + x, y_center - y))
                .unwrap();
            self.canvas_mut()
                .draw_point(Point::new(x_center + x, y_center + y))
                .unwrap();
            self.canvas_mut()
                .draw_point(Point::new(x_center - x, y_center - y))
                .unwrap();
            self.canvas_mut()
                .draw_point(Point::new(x_center - x, y_center + y))
                .unwrap();
            self.canvas_mut()
                .draw_point(Point::new(x_center + y, y_center - x))
                .unwrap();
            self.canvas_mut()
                .draw_point(Point::new(x_center + y, y_center + x))
                .unwrap();
            self.canvas_mut()
                .draw_point(Point::new(x_center - y, y_center - x))
                .unwrap();
            self.canvas_mut()
                .draw_point(Point::new(x_center - y, y_center + x))
                .unwrap();

            if error <= 0 {
                y += 1;
                error += ty;
                ty += 2;
            }

            if error > 0 {
                x -= 1;
                tx += 2;
                error += tx - diameter;
            }
        }
    }

    fn render_crates(
        &self,
        graphics: &Graphics,
        scroll: &(u32, u32),
        crates: &HashMap<(u32, u32), StaticCrateType>,
        font: &Font<'_, Self>,
    ) {
        for (coordinates, crate_item) in crates {
            let box_size = get_crate_render_size();
            let (x_screen, y_screen) =
                get_screen_coordinates_from_level_coordinates(graphics, coordinates, scroll);
            self.canvas_mut()
                .set_draw_color(get_sdl_color(match crate_item.crate_variant {
                    StaticCrate::Normal => &RendererColor::LightGreen,
                    StaticCrate::Deathmatch => &RendererColor::LightBlue,
                }));
            self.canvas_mut()
                .draw_rect(sdl2::rect::Rect::new(
                    x_screen, y_screen, box_size, box_size,
                ))
                .unwrap();
            self.canvas_mut()
                .draw_rect(sdl2::rect::Rect::new(
                    x_screen + 1,
                    y_screen + 1,
                    box_size - 2,
                    box_size - 2,
                ))
                .unwrap();

            let text = match crate_item.crate_class {
                CrateClass::Weapon => weapon_crates(),
                CrateClass::Bullet => bullet_crates(),
                CrateClass::Energy => energy_crates(),
            }[crate_item.crate_type as usize];
            let (_, height) = font.text_size(text);
            font.render_text(
                self,
                text,
                (
                    (x_screen - 10) as u32,
                    (y_screen - 9 - height as i32) as u32,
                ),
            );
        }
    }

    pub fn present(&self) {
        self.canvas_mut().present();
    }

    fn canvas_mut(&self) -> RefMut<Canvas<Window>> {
        self.canvas.borrow_mut()
    }
}

impl<'a> Renderer<'a> for SdlRenderer {
    type Texture = SdlTexture<'a>;

    fn load_texture(&'a self, path: &str) -> Self::Texture {
        self.texture_creator.load_texture(path).unwrap()
    }

    fn create_texture(&'a self, width: u32, height: u32, pixels: &[Color]) -> Self::Texture {
        let mut data = Vec::with_capacity(pixels.len() * 4);
        for pixel in pixels {
            data.push(pixel.r);
            data.push(pixel.g);
            data.push(pixel.b);
            data.push(pixel.a);
        }
        let surface = Surface::from_data(
            &mut data,
            width,
            height,
            width * 4,
            PixelFormatEnum::ABGR8888,
        );
        return surface.unwrap().as_texture(&self.texture_creator).unwrap();
    }

    fn clear_screen(&self) {
        self.canvas_mut()
            .set_draw_color(get_sdl_color(&RendererColor::Black));
        self.canvas_mut().clear();
    }

    fn highlight_selected_tile(&self, graphics: &Graphics, id: u32, color: &RendererColor) {
        self.canvas_mut().set_draw_color(get_sdl_color(color));

        let render_size = graphics.get_render_size();
        let render_multiplier = graphics.render_multiplier;
        let (x_logical, y_logical) = get_tile_coordinates(
            id,
            graphics.get_x_tiles_per_screen() * graphics.tile_size,
            graphics.tile_size,
        );
        let x = x_logical * render_multiplier;
        let y = y_logical * render_multiplier;

        self.draw_line(x, y, x, y + render_size - 1);
        self.draw_line(x, y, x + render_size - 1, y);
        self.draw_line(
            x + render_size - 1,
            y,
            x + render_size - 1,
            y + render_size - 1,
        );
        self.draw_line(
            x,
            y + render_size - 1,
            x + render_size - 1,
            y + render_size - 1,
        );
    }

    fn draw_line(&self, x0: u32, y0: u32, x1: u32, y1: u32) {
        let x0_signed = x0 as i32;
        let y0_signed = y0 as i32;
        let x1_signed = x1 as i32;
        let y1_signed = y1 as i32;

        self.canvas_mut()
            .draw_line(
                Point::from((x0_signed, y0_signed)),
                Point::from((x1_signed, y1_signed)),
            )
            .unwrap();
    }

    fn render_texture(&self, texture: &Self::Texture, dst: Rect) {
        self.canvas_mut()
            .copy(texture, None, Some(dst.into()))
            .unwrap();
    }

    fn fill_and_render_texture(&self, color: RendererColor, texture: &Self::Texture, dst: Rect) {
        let mut canvas = self.canvas_mut();
        canvas.set_draw_color(get_sdl_color(&color));
        canvas.fill_rect(Some(dst.into())).unwrap();
        canvas.copy(texture, None, Some(dst.into())).unwrap();
    }

    fn render_level(
        &self,
        graphics: &Graphics,
        level: &Level,
        textures: &Textures<Self::Texture>,
        trigonometry: &Trigonometry,
        font: &Font<'a, Self>,
    ) {
        self.canvas_mut().set_draw_color(Color::from((0, 0, 0)));
        self.canvas_mut().clear();
        let render_size = graphics.get_render_size();

        for y in 0..std::cmp::min(level.tiles.len() as u32, graphics.get_y_tiles_per_screen()) {
            for x in 0..std::cmp::min(
                level.tiles[y as usize].len() as u32,
                graphics.get_x_tiles_per_screen(),
            ) {
                let (x_index, y_index) = get_scroll_corrected_indexes(level.scroll, x, y);
                if y_index >= level.tiles.len() || x_index >= level.tiles[y_index].len() {
                    continue;
                }
                let texture = match level.tiles[y_index][x_index].texture_type {
                    TextureType::Floor => &textures.floor,
                    TextureType::Walls => &textures.walls,
                    TextureType::Shadow => unreachable!(),
                };
                let (texture_width, _texture_height) = Self::get_texture_size(texture);
                let src = get_block(
                    level.tiles[y_index][x_index].id,
                    texture_width,
                    graphics.tile_size,
                );
                let (x_absolute, y_absolute) =
                    get_absolute_coordinates_from_logical(x, y, graphics.get_render_size());
                let dst = sdl2::rect::Rect::new(x_absolute, y_absolute, render_size, render_size);
                self.canvas_mut().copy(texture, src, dst).unwrap();
                let (shadow_texture_width, _shadow_texture_height) =
                    Self::get_texture_size(&textures.shadows);
                if level.tiles[y_index][x_index].shadow > 0 {
                    let src = get_block(
                        level.tiles[y_index][x_index].shadow - 1,
                        shadow_texture_width,
                        graphics.tile_size,
                    );
                    self.canvas_mut().copy(&textures.shadows, src, dst).unwrap();
                }
            }
        }
        for (coordinates, spotlight) in &level.spotlights {
            let (x_screen, y_screen) =
                get_screen_coordinates_from_level_coordinates(graphics, coordinates, &level.scroll);
            self.draw_circle(
                x_screen,
                y_screen,
                get_spotlight_render_radius(spotlight),
                &RendererColor::Blue,
            );
        }
        for (coordinates, steam) in &level.steams {
            let (x_screen, y_screen) =
                get_screen_coordinates_from_level_coordinates(graphics, coordinates, &level.scroll);
            for x in 0..6 {
                let multiplier = x as f32 * 6.0 * steam.range as f32;
                self.draw_circle(
                    x_screen + (trigonometry.sin[steam.angle as usize] * multiplier) as i32,
                    y_screen + (trigonometry.cos[steam.angle as usize] * multiplier) as i32,
                    get_steam_render_radius() + x * 2,
                    &RendererColor::Red,
                );
            }
        }

        self.render_crates(graphics, &level.scroll, &level.crates.staticc, font);
    }

    fn get_texture_size(texture: &Self::Texture) -> (u32, u32) {
        let TextureQuery { width, height, .. } = texture.query();
        (width, height)
    }

    fn window_size(&self) -> (u32, u32) {
        self.canvas_mut().window().size()
    }
}

pub fn get_texture_rect<'a, R: Renderer<'a>>(texture: &R::Texture, render_multiplier: u32) -> Rect {
    let (width, height) = get_texture_render_size::<R>(texture, render_multiplier);
    Rect::new(0, 0, width, height)
}

pub fn get_texture_render_size<'a, R: Renderer<'a>>(
    texture: &R::Texture,
    render_multiplier: u32,
) -> (u32, u32) {
    let (width, height) = R::get_texture_size(texture);
    (width * render_multiplier, height * render_multiplier)
}

fn get_block(id: u32, width: u32, tile_size: u32) -> sdl2::rect::Rect {
    let (x, y) = get_tile_coordinates(id, width, tile_size);
    sdl2::rect::Rect::new(x as i32, y as i32, tile_size, tile_size)
}

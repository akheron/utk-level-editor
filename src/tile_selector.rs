use crate::render;
use crate::types::*;
use crate::util::*;
use crate::Context;
use crate::NextMode::*;
use crate::{create_text_texture, EditorState};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;

pub struct TileSelectState<'a> {
    context: Context<'a>,
}

impl TileSelectState<'_> {
    pub fn new(context: Context) -> TileSelectState {
        TileSelectState { context }
    }
}

impl<'a> TileSelectState<'a> {
    pub fn exec(mut self) -> NextMode<'a> {
        let floor_blocks_text_texture = create_text_texture(
            &mut self.context.canvas,
            &self.context.texture_creator,
            &self.context.font,
            "floor blocks (PAGEGUP/DOWN)",
        );
        let wall_blocks_text_texture = create_text_texture(
            &mut self.context.canvas,
            &self.context.texture_creator,
            &self.context.font,
            "wall blocks (PAGEGUP/DOWN)",
        );
        let shadow_blocks_text_texture = create_text_texture(
            &mut self.context.canvas,
            &self.context.texture_creator,
            &self.context.font,
            "shadows (PAGEGUP/DOWN) - clear with RIGHT CLICK",
        );
        let mut event_pump = self.context.sdl.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Editor(EditorState::new(self.context)),
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Space => {
                        return Editor(EditorState::new(self.context));
                    }
                    Keycode::PageDown => {
                        self.context.texture_type_scrolled =
                            if self.context.texture_type_scrolled == TextureType::FLOOR {
                                TextureType::WALLS
                            } else if self.context.texture_type_scrolled == TextureType::WALLS {
                                TextureType::SHADOW
                            } else {
                                TextureType::FLOOR
                            }
                    }
                    Keycode::PageUp => {
                        self.context.texture_type_scrolled =
                            if self.context.texture_type_scrolled == TextureType::FLOOR {
                                TextureType::SHADOW
                            } else if self.context.texture_type_scrolled == TextureType::SHADOW {
                                TextureType::WALLS
                            } else {
                                TextureType::FLOOR
                            }
                    }
                    _ => {}
                },
                Event::MouseMotion { x, y, .. } => {
                    self.context.mouse.0 = x as u32;
                    self.context.mouse.1 = y as u32;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    let texture_selected = match &self.context.texture_type_scrolled {
                        TextureType::FLOOR => &self.context.textures.floor,
                        TextureType::WALLS => &self.context.textures.walls,
                        TextureType::SHADOW => &self.context.textures.shadows,
                    };
                    let (texture_width, texture_height) = render::get_texture_render_size(
                        texture_selected,
                        self.context.graphics.render_multiplier,
                    );
                    let clicked_tile_id = get_tile_id_from_coordinates(
                        &self.context.graphics,
                        &limit_coordinates(&self.context.mouse, &(texture_width, texture_height)),
                        texture_width / self.context.graphics.get_render_size(),
                        None,
                    );
                    if clicked_tile_id
                        < get_number_of_tiles_in_texture(
                            texture_selected,
                            self.context.graphics.tile_size,
                        )
                    {
                        self.context.selected_tile_id = clicked_tile_id;
                        self.context.texture_type_selected = self.context.texture_type_scrolled;
                        return Editor(EditorState::new(self.context));
                    }
                }
                _ => {}
            }
        }

        self.context.canvas.set_draw_color(Color::from((0, 0, 0)));
        self.context.canvas.clear();
        let texture_selected = match self.context.texture_type_scrolled {
            TextureType::FLOOR => &self.context.textures.floor,
            TextureType::WALLS => &self.context.textures.walls,
            TextureType::SHADOW => &self.context.textures.shadows,
        };
        let render_multiplier = self.context.graphics.render_multiplier;
        let dst = render::get_texture_rect(texture_selected, render_multiplier);
        self.context
            .canvas
            .set_draw_color(Color::from((200, 200, 200)));
        self.context.canvas.fill_rect(dst).unwrap();
        self.context
            .canvas
            .copy(texture_selected, None, dst)
            .unwrap();
        let (texture_width, texture_height) =
            render::get_texture_render_size(texture_selected, render_multiplier);
        let highlighted_id = get_tile_id_from_coordinates(
            &self.context.graphics,
            &limit_coordinates(&self.context.mouse, &(texture_width, texture_height)),
            self.context.graphics.get_x_tiles_per_screen(),
            None,
        );
        render::highlight_selected_tile(
            &mut self.context.canvas,
            &self.context.graphics,
            highlighted_id,
            &render::RendererColor::White,
        );
        if self.context.texture_type_selected == self.context.texture_type_scrolled {
            let coordinates = get_tile_coordinates(
                self.context.selected_tile_id,
                texture_width / self.context.graphics.render_multiplier,
                self.context.graphics.tile_size,
            );
            let render_multiplier = self.context.graphics.render_multiplier;
            let screen_tile_id = get_tile_id_from_coordinates(
                &self.context.graphics,
                &(
                    coordinates.0 * render_multiplier,
                    coordinates.1 * render_multiplier,
                ),
                self.context.graphics.get_x_tiles_per_screen(),
                None,
            );
            render::highlight_selected_tile(
                &mut self.context.canvas,
                &self.context.graphics,
                screen_tile_id,
                &render::RendererColor::Red,
            );
        }
        let active_text = match self.context.texture_type_scrolled {
            TextureType::FLOOR => &floor_blocks_text_texture,
            TextureType::WALLS => &wall_blocks_text_texture,
            TextureType::SHADOW => &shadow_blocks_text_texture,
        };
        render::render_text_texture_coordinates(
            &mut self.context.canvas,
            active_text,
            get_bottom_text_position(self.context.graphics.resolution_y),
            self.context.graphics.get_render_size(),
            None,
        );
        render::render_and_wait(&mut self.context.canvas);

        TileSelect(self)
    }
}

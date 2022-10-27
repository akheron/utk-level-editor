use crate::render;
use crate::types::*;
use crate::util::*;
use crate::Context;
use crate::{create_text_texture, VideoContext};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::Texture;

pub struct TileSelectState<'a> {
    floor_blocks_text_texture: Texture<'a>,
    wall_blocks_text_texture: Texture<'a>,
    shadow_blocks_text_texture: Texture<'a>,
}

impl<'a> TileSelectState<'a> {
    pub fn new(video: &mut VideoContext, context: &Context) -> Self {
        TileSelectState {
            floor_blocks_text_texture: create_text_texture(
                &mut video.canvas,
                &video.texture_creator,
                &context.font,
                "floor blocks (PAGEGUP/DOWN)",
            ),
            wall_blocks_text_texture: create_text_texture(
                &mut video.canvas,
                &video.texture_creator,
                &context.font,
                "wall blocks (PAGEGUP/DOWN)",
            ),
            shadow_blocks_text_texture: create_text_texture(
                &mut video.canvas,
                &video.texture_creator,
                &context.font,
                "shadows (PAGEGUP/DOWN) - clear with RIGHT CLICK",
            ),
        }
    }

    pub fn exec(&mut self, video: &mut VideoContext, context: &mut Context) -> NextMode {
        let mut event_pump = context.sdl.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return NextMode::Editor,
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Space => {
                        return NextMode::Editor;
                    }
                    Keycode::PageDown => {
                        context.texture_type_scrolled =
                            if context.texture_type_scrolled == TextureType::FLOOR {
                                TextureType::WALLS
                            } else if context.texture_type_scrolled == TextureType::WALLS {
                                TextureType::SHADOW
                            } else {
                                TextureType::FLOOR
                            }
                    }
                    Keycode::PageUp => {
                        context.texture_type_scrolled =
                            if context.texture_type_scrolled == TextureType::FLOOR {
                                TextureType::SHADOW
                            } else if context.texture_type_scrolled == TextureType::SHADOW {
                                TextureType::WALLS
                            } else {
                                TextureType::FLOOR
                            }
                    }
                    _ => {}
                },
                Event::MouseMotion { x, y, .. } => {
                    context.mouse.0 = x as u32;
                    context.mouse.1 = y as u32;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    let texture_selected = match &context.texture_type_scrolled {
                        TextureType::FLOOR => &context.textures.floor,
                        TextureType::WALLS => &context.textures.walls,
                        TextureType::SHADOW => &context.textures.shadows,
                    };
                    let (texture_width, texture_height) = render::get_texture_render_size(
                        texture_selected,
                        context.graphics.render_multiplier,
                    );
                    let clicked_tile_id = get_tile_id_from_coordinates(
                        &context.graphics,
                        &limit_coordinates(&context.mouse, &(texture_width, texture_height)),
                        texture_width / context.graphics.get_render_size(),
                        None,
                    );
                    if clicked_tile_id
                        < get_number_of_tiles_in_texture(
                            texture_selected,
                            context.graphics.tile_size,
                        )
                    {
                        context.selected_tile_id = clicked_tile_id;
                        context.texture_type_selected = context.texture_type_scrolled;
                        return NextMode::Editor;
                    }
                }
                _ => {}
            }
        }

        video.canvas.set_draw_color(Color::from((0, 0, 0)));
        video.canvas.clear();
        let texture_selected = match context.texture_type_scrolled {
            TextureType::FLOOR => &context.textures.floor,
            TextureType::WALLS => &context.textures.walls,
            TextureType::SHADOW => &context.textures.shadows,
        };
        let render_multiplier = context.graphics.render_multiplier;
        let dst = render::get_texture_rect(texture_selected, render_multiplier);
        video.canvas.set_draw_color(Color::from((200, 200, 200)));
        video.canvas.fill_rect(dst).unwrap();
        video.canvas.copy(texture_selected, None, dst).unwrap();
        let (texture_width, texture_height) =
            render::get_texture_render_size(texture_selected, render_multiplier);
        let highlighted_id = get_tile_id_from_coordinates(
            &context.graphics,
            &limit_coordinates(&context.mouse, &(texture_width, texture_height)),
            context.graphics.get_x_tiles_per_screen(),
            None,
        );
        render::highlight_selected_tile(
            &mut video.canvas,
            &context.graphics,
            highlighted_id,
            &render::RendererColor::White,
        );
        if context.texture_type_selected == context.texture_type_scrolled {
            let coordinates = get_tile_coordinates(
                context.selected_tile_id,
                texture_width / context.graphics.render_multiplier,
                context.graphics.tile_size,
            );
            let render_multiplier = context.graphics.render_multiplier;
            let screen_tile_id = get_tile_id_from_coordinates(
                &context.graphics,
                &(
                    coordinates.0 * render_multiplier,
                    coordinates.1 * render_multiplier,
                ),
                context.graphics.get_x_tiles_per_screen(),
                None,
            );
            render::highlight_selected_tile(
                &mut video.canvas,
                &context.graphics,
                screen_tile_id,
                &render::RendererColor::Red,
            );
        }
        let active_text = match context.texture_type_scrolled {
            TextureType::FLOOR => &self.floor_blocks_text_texture,
            TextureType::WALLS => &self.wall_blocks_text_texture,
            TextureType::SHADOW => &self.shadow_blocks_text_texture,
        };
        render::render_text_texture_coordinates(
            &mut video.canvas,
            active_text,
            get_bottom_text_position(context.graphics.resolution_y),
            context.graphics.get_render_size(),
            None,
        );
        render::render_and_wait(&mut video.canvas);
        NextMode::TileSelect
    }
}

use sdl2::render::Texture;

use crate::context_util::resize;
use crate::event::{Event, Keycode, MouseButton};
use crate::types::*;
use crate::util::*;
use crate::Context;
use crate::{render, Renderer};
use crate::render::RendererColor;

pub struct TileSelectState<'a> {
    renderer: &'a Renderer,
    floor_blocks_text_texture: Texture<'a>,
    wall_blocks_text_texture: Texture<'a>,
    shadow_blocks_text_texture: Texture<'a>,
}

impl<'a> TileSelectState<'a> {
    pub fn new(renderer: &'a Renderer, context: &Context<'a>) -> Self {
        TileSelectState {
            renderer,
            floor_blocks_text_texture: renderer
                .create_text_texture(&context.font, "floor blocks (PAGEGUP/DOWN)"),
            wall_blocks_text_texture: renderer
                .create_text_texture(&context.font, "wall blocks (PAGEGUP/DOWN)"),
            shadow_blocks_text_texture: renderer.create_text_texture(
                &context.font,
                "shadows (PAGEGUP/DOWN) - clear with RIGHT CLICK",
            ),
        }
    }

    pub fn handle_event(&self, context: &mut Context<'a>, event: Event) -> Mode {
        match event {
            Event::Quit
            | Event::KeyDown {
                keycode: Keycode::Escape,
            } => return Mode::Editor,
            Event::Window { win_event } => {
                resize(self.renderer, context, win_event);
                return Mode::Editor;
            }
            Event::KeyDown { keycode } => match keycode {
                Keycode::Space => {
                    return Mode::Editor;
                }
                Keycode::PageDown => {
                    context.texture_type_scrolled =
                        if context.texture_type_scrolled == TextureType::Floor {
                            TextureType::Walls
                        } else if context.texture_type_scrolled == TextureType::Walls {
                            TextureType::Shadow
                        } else {
                            TextureType::Floor
                        }
                }
                Keycode::PageUp => {
                    context.texture_type_scrolled =
                        if context.texture_type_scrolled == TextureType::Floor {
                            TextureType::Shadow
                        } else if context.texture_type_scrolled == TextureType::Shadow {
                            TextureType::Walls
                        } else {
                            TextureType::Floor
                        }
                }
                _ => {}
            },
            Event::MouseMotion { x, y, .. } => {
                context.mouse.0 = x as u32;
                context.mouse.1 = y as u32;
            }
            Event::MouseButtonDown {
                button: MouseButton::Left,
                ..
            } => {
                let texture_selected = match &context.texture_type_scrolled {
                    TextureType::Floor => &context.textures.floor,
                    TextureType::Walls => &context.textures.walls,
                    TextureType::Shadow => &context.textures.shadows,
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
                    < get_number_of_tiles_in_texture(texture_selected, context.graphics.tile_size)
                {
                    context.selected_tile_id = clicked_tile_id;
                    context.texture_type_selected = context.texture_type_scrolled;
                    return Mode::Editor;
                }
            }
            _ => {}
        }
        Mode::TileSelect
    }

    pub fn render(&self, context: &Context<'a>) {
        self.renderer.clear_screen();
        let texture_selected = match context.texture_type_scrolled {
            TextureType::Floor => &context.textures.floor,
            TextureType::Walls => &context.textures.walls,
            TextureType::Shadow => &context.textures.shadows,
        };
        let render_multiplier = context.graphics.render_multiplier;
        let dst = render::get_texture_rect(texture_selected, render_multiplier);
        self.renderer
            .fill_and_render_texture(RendererColor::LightGrey, texture_selected, dst);
        let (texture_width, texture_height) =
            render::get_texture_render_size(texture_selected, render_multiplier);
        let highlighted_id = get_tile_id_from_coordinates(
            &context.graphics,
            &limit_coordinates(&context.mouse, &(texture_width, texture_height)),
            context.graphics.get_x_tiles_per_screen(),
            None,
        );
        self.renderer.highlight_selected_tile(
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
            self.renderer.highlight_selected_tile(
                &context.graphics,
                screen_tile_id,
                &render::RendererColor::Red,
            );
        }
        let active_text = match context.texture_type_scrolled {
            TextureType::Floor => &self.floor_blocks_text_texture,
            TextureType::Walls => &self.wall_blocks_text_texture,
            TextureType::Shadow => &self.shadow_blocks_text_texture,
        };
        self.renderer.render_text_texture_coordinates(
            active_text,
            get_bottom_text_position(context.graphics.resolution_y),
            context.graphics.get_render_size(),
            None,
        );
        self.renderer.render_and_wait();
    }
}

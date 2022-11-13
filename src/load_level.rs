use std::fs;

use crate::context_util::resize;
use crate::event::{Event, Keycode};
use crate::get_bottom_text_position;
use crate::render::Renderer;
use crate::types::*;
use crate::util::TITLE_POSITION;
use crate::Context;

struct LoadFile<'a, R: Renderer<'a>> {
    filename: String,
    texture: R::Texture,
}

pub struct LoadLevelState<'a, R: Renderer<'a>> {
    renderer: &'a R,
    load_level_text_texture: R::Texture,
    bottom_instruction_text: R::Texture,
    files: Vec<LoadFile<'a, R>>,
    selected: usize,
}

impl<'a, R: Renderer<'a>> LoadLevelState<'a, R> {
    pub fn new(renderer: &'a R, context: &Context<'a, R>) -> Self {
        let load_level_text_texture = renderer.create_text_texture(&context.font, "LOAD LEVEL:");
        let bottom_instruction_text =
            renderer.create_text_texture(&context.font, "ENTER to select or ESC to exit");
        let files = fs::read_dir("./")
            .unwrap()
            .filter_map(|read_dir_result| {
                let filename = read_dir_result.unwrap().path().display().to_string();
                if filename.to_uppercase().ends_with(".LEV") {
                    Some(filename)
                } else {
                    None
                }
            })
            .map(|ref filename| LoadFile {
                filename: filename.to_string(),
                texture: renderer
                    .create_text_texture(&context.font, &filename.clone().to_lowercase()),
            })
            .collect();
        LoadLevelState {
            renderer,
            load_level_text_texture,
            bottom_instruction_text,
            files,
            selected: 0,
        }
    }

    pub fn handle_event(&mut self, context: &mut Context<'a, R>, event: Event) -> Mode {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Keycode::Escape,
            } => return Mode::Editor,
            Event::Window { win_event, .. } => {
                resize(self.renderer, context, win_event);
                return Mode::Editor;
            }
            Event::KeyDown { keycode, .. } => match keycode {
                Keycode::Down => {
                    if self.selected < self.files.len() - 1 {
                        self.selected += 1;
                    }
                }
                Keycode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                Keycode::Return | Keycode::KpEnter => {
                    if !self.files.is_empty() {
                        context
                            .level
                            .deserialize(&self.files[self.selected].filename)
                            .unwrap();
                        let level_name = self.files[self.selected]
                            .filename
                            .strip_prefix("./")
                            .unwrap()
                            .to_string();
                        context.textures.saved_level_name = Some(
                            self.renderer
                                .create_text_texture(&context.font, &level_name.to_lowercase()),
                        );
                        context.level_save_name =
                            level_name.strip_suffix(".LEV").unwrap().to_string();
                    }
                    return Mode::Editor;
                }
                _ => {}
            },
            _ => {}
        }
        Mode::LoadLevel
    }

    pub fn render(&mut self, context: &Context<'a, R>) {
        self.renderer.clear_screen();
        let text_position = (40, 60);
        let render_size = context.graphics.get_render_size();
        self.renderer.render_text_texture_coordinates(
            &self.load_level_text_texture,
            TITLE_POSITION,
            render_size,
            None,
        );
        let line_spacing = 20;
        for x in 0..self.files.len() {
            if self.selected == x {
                self.renderer.render_text_texture(
                    &context.textures.selected_icon,
                    text_position.0 - 20,
                    text_position.1 + 3 + x as u32 * line_spacing,
                    render_size,
                    None,
                );
            }
            self.renderer.render_text_texture(
                &self.files[x].texture,
                text_position.0,
                text_position.1 + line_spacing * x as u32,
                render_size,
                None,
            );
        }
        self.renderer.render_text_texture_coordinates(
            &self.bottom_instruction_text,
            get_bottom_text_position(context.graphics.resolution_y),
            render_size,
            None,
        );
    }
}

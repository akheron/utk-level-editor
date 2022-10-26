use crate::render;
use crate::types::*;
use crate::util::TITLE_POSITION;
use crate::Context;
use crate::{create_text_texture, get_bottom_text_position};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use std::fs;

struct LoadFile<'a> {
    filename: String,
    texture: Texture<'a>,
}

pub struct LoadLevelState<'a> {
    context: Context<'a>,
    load_level_text_texture: Texture<'a>,
    bottom_instruction_text: Texture<'a>,
    files: Vec<LoadFile<'a>>,
    selected: usize,
}

impl LoadLevelState<'_> {
    pub fn new(mut context: Context) -> LoadLevelState {
        let load_level_text_texture = create_text_texture(
            &mut context.canvas,
            &context.texture_creator,
            &context.font,
            "LOAD LEVEL:",
        );
        let bottom_instruction_text = create_text_texture(
            &mut context.canvas,
            &context.texture_creator,
            &context.font,
            "ENTER to select or ESC to exit",
        );
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
                texture: create_text_texture(
                    &mut context.canvas,
                    &context.texture_creator,
                    &context.font,
                    &filename.clone().to_lowercase(),
                ),
            })
            .collect();
        LoadLevelState {
            context,
            load_level_text_texture,
            bottom_instruction_text,
            files,
            selected: 0usize,
        }
    }
}

impl<'a> LoadLevelState<'a> {
    pub fn exec(mut self) -> NextMode<'a> {
        let mut event_pump = self.context.sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return NextMode::editor(self.context),
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
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
                        if self.files.len() > 0 {
                            self.context
                                .level
                                .deserialize(&self.files[self.selected].filename)
                                .unwrap();
                            let level_name = self.files[self.selected]
                                .filename
                                .strip_prefix("./")
                                .unwrap()
                                .to_string();
                            self.context.textures.saved_level_name = Some(create_text_texture(
                                &mut self.context.canvas,
                                &self.context.texture_creator,
                                &self.context.font,
                                &level_name.clone().to_lowercase(),
                            ));
                            self.context.level_save_name =
                                level_name.strip_suffix(".LEV").unwrap().to_string();
                        }
                        return NextMode::editor(self.context);
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        self.context.canvas.set_draw_color(Color::from((0, 0, 0)));
        self.context.canvas.clear();
        let text_position = (40, 60);
        let render_size = self.context.graphics.get_render_size();
        render::render_text_texture_coordinates(
            &mut self.context.canvas,
            &self.load_level_text_texture,
            TITLE_POSITION,
            render_size,
            None,
        );
        let line_spacing = 20;
        for x in 0..self.files.len() {
            if self.selected == x {
                render::render_text_texture(
                    &mut self.context.canvas,
                    &self.context.textures.selected_icon,
                    text_position.0 - 20,
                    text_position.1 + 3 + x as u32 * line_spacing,
                    render_size,
                    None,
                );
            }
            render::render_text_texture(
                &mut self.context.canvas,
                &self.files[x].texture,
                text_position.0,
                text_position.1 + line_spacing * x as u32,
                render_size,
                None,
            );
        }
        render::render_text_texture_coordinates(
            &mut self.context.canvas,
            &self.bottom_instruction_text,
            get_bottom_text_position(self.context.graphics.resolution_y),
            render_size,
            None,
        );
        render::render_and_wait(&mut self.context.canvas);
        NextMode::LoadLevel(self)
    }
}

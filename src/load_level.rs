use std::fs;

use crate::context_util::resize;
use crate::event::{Event, Keycode};
use crate::get_bottom_text_position;
use crate::render::Renderer;
use crate::types::*;
use crate::util::TITLE_POSITION;
use crate::Context;

pub struct LoadLevelState<'a, R: Renderer<'a>> {
    renderer: &'a R,
    files: Vec<String>,
    selected: usize,
}

impl<'a, R: Renderer<'a>> LoadLevelState<'a, R> {
    pub fn new(renderer: &'a R) -> Self {
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
            .collect();
        LoadLevelState {
            renderer,
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
                            .deserialize(&self.files[self.selected])
                            .unwrap();
                        let level_name = self.files[self.selected]
                            .strip_prefix("./")
                            .unwrap()
                            .to_string();
                        context.saved_level_name = Some(level_name.clone());
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
        context
            .font
            .render_text(self.renderer, "LOAD LEVEL:", TITLE_POSITION);
        let line_spacing = 20;
        for x in 0..self.files.len() {
            if self.selected == x {
                context.font.render_text(
                    self.renderer,
                    "*",
                    (
                        text_position.0 - 20,
                        text_position.1 + 3 + x as u32 * line_spacing,
                    ),
                );
            }
            context.font.render_text(
                self.renderer,
                &self.files[x],
                (text_position.0, text_position.1 + line_spacing * x as u32),
            );
        }
        context.font.render_text(
            self.renderer,
            "ENTER to select or ESC to exit",
            get_bottom_text_position(context.graphics.resolution_y),
        );
    }
}

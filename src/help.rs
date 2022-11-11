use crate::context_util::resize;
use crate::event::Event;
use crate::render::Texture;
use crate::Mode;
use crate::{Context, Renderer};

const LINES: [&str; 19] = [
    "ESC - quit",
    "F1   - this help",
    "F2   - save level",
    "F3   - load level",
    "F4   - create new level",
    "F6   - enable/disable automatic shadows",
    "F7   - edit general level variables",
    "F8/F9 - edit random crates for normal/dm games",
    " ",
    "- EDITOR -",
    "Q/W  - place/delete spotlights",
    "A/S  - place/delete steams",
    "Z/X/C - place/delete crates",
    "1/2  - place pl1/pl2 start",
    "SPACE - tile selection/editing mode",
    "ARROW KEYS - move viewport",
    " ",
    "- WINDOW -",
    "+/- adjust rendering size",
];

pub struct HelpState<'a> {
    renderer: &'a Renderer,
    line_textures: Vec<Texture<'a>>,
}

impl<'a> HelpState<'a> {
    pub fn new(renderer: &'a Renderer, context: &Context<'a>) -> Self {
        let line_textures = LINES
            .iter()
            .map(|text| renderer.create_text_texture(&context.font, text))
            .collect();

        HelpState {
            renderer,
            line_textures,
        }
    }

    pub fn handle_event(&mut self, context: &mut Context<'a>, event: Event) -> Mode {
        match event {
            Event::Quit => return Mode::Editor,
            Event::KeyDown { .. } => {
                return Mode::Editor;
            }
            Event::Window { win_event, .. } => {
                resize(self.renderer, context, win_event);
                return Mode::Editor;
            }
            _ => {}
        }
        Mode::Help
    }

    pub fn render(&self, context: &Context<'a>) {
        self.renderer.clear_screen();
        let mut position = 6;
        for line_texture in &self.line_textures {
            self.renderer.render_text_texture(
                line_texture,
                10,
                position,
                context.graphics.get_render_size(),
                None,
            );
            position += 22;
        }
        self.renderer.render_and_wait();
    }
}

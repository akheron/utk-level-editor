use crate::event::Event;
use crate::render::Renderer;
use crate::Context;
use crate::Mode;

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

pub struct HelpState;

impl HelpState {
    pub fn new() -> Self {
        HelpState
    }

    pub fn handle_event(&self, event: Event) -> Mode {
        match event {
            Event::Quit => return Mode::Editor,
            Event::KeyDown { .. } => {
                return Mode::Editor;
            }
            Event::Window { .. } => {
                return Mode::Editor;
            }
            _ => {}
        }
        Mode::Help
    }

    pub fn render<'a, R: Renderer<'a>>(&self, renderer: &'a R, context: &Context<'a, R>) {
        renderer.clear_screen();
        let mut position = 6;
        for line_text in &LINES {
            context
                .font
                .render_text(renderer, line_text, (10, position));
            position += 22;
        }
    }
}

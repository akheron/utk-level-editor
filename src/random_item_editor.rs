use crate::event::{Event, Keycode};
use crate::level::Level;
use crate::level::ALL_CRATES;
use crate::render::Renderer;
use crate::types::*;
use crate::util::{get_bottom_text_position, TITLE_POSITION};
use crate::{Context, TextInput};

fn get_value(level: &Level, game_type: &GameType, index: usize) -> u32 {
    let crates = match game_type {
        GameType::Normal => &level.crates.random.normal,
        GameType::Deathmatch => &level.crates.random.deathmatch,
    };
    if index < crates.weapons.len() {
        crates.weapons[index]
    } else {
        let index = index - crates.weapons.len();
        if index < crates.bullets.len() {
            crates.bullets[index]
        } else {
            crates.energy
        }
    }
}

fn set_value(level: &mut Level, game_type: &GameType, index: usize, value: u32) {
    let crates = match game_type {
        GameType::Normal => &mut level.crates.random.normal,
        GameType::Deathmatch => &mut level.crates.random.deathmatch,
    };
    if index < crates.weapons.len() {
        crates.weapons[index] = value;
    } else {
        let index = index - crates.weapons.len();
        if index < crates.bullets.len() {
            crates.bullets[index] = value;
        } else {
            crates.energy = value;
        }
    }
}

pub struct RandomItemEditorState {
    selected: usize,
}

impl RandomItemEditorState {
    pub fn new() -> Self {
        RandomItemEditorState { selected: 0 }
    }

    pub fn handle_event<'a, R: Renderer<'a>, T: TextInput>(
        &mut self,
        context: &mut Context<'a, R>,
        text_input: &T,
        game_type: GameType,
        event: Event,
    ) -> Mode {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Keycode::Escape,
            } => {
                text_input.stop();
                return Mode::Editor;
            }
            Event::Window { .. } => {
                return Mode::Editor;
            }
            Event::KeyDown { keycode, .. } => match keycode {
                Keycode::Down => {
                    if self.selected < ALL_CRATES.len() - 1 {
                        self.selected += 1;
                    }
                }
                Keycode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                Keycode::Right => {
                    let value = get_value(&context.level, &game_type, self.selected);
                    set_value(&mut context.level, &game_type, self.selected, value + 1);
                }
                Keycode::Left => {
                    let value = get_value(&context.level, &game_type, self.selected);
                    if value > 0 {
                        set_value(&mut context.level, &game_type, self.selected, value - 1);
                    }
                }
                _ => (),
            },
            _ => {}
        }
        Mode::RandomItemEditor(game_type)
    }

    pub fn render<'a, R: Renderer<'a>>(
        &mut self,
        renderer: &'a R,
        context: &Context<'a, R>,
        game_type: GameType,
    ) {
        renderer.clear_screen();

        context.font.render_text(
            renderer,
            match game_type {
                GameType::Normal => "NORMAL GAME CRATES",
                GameType::Deathmatch => "DEATHMATCH CRATES",
            },
            TITLE_POSITION,
        );

        let y = 50;
        let mut option_position = (40, y);
        let mut value_position = (280, option_position.1);
        for x in 0..ALL_CRATES.len() {
            if self.selected == x {
                context.font.render_text(
                    renderer,
                    "*",
                    (option_position.0 - 20, option_position.1 + 3),
                );
            }
            context
                .font
                .render_text(renderer, ALL_CRATES[x], option_position);
            context.font.render_text(
                renderer,
                &get_value(&context.level, &game_type, x).to_string(),
                value_position,
            );
            if x == 10 {
                option_position.1 = y;
                value_position.1 = option_position.1;
                option_position.0 = 330;
                value_position.0 = option_position.0 + 250;
            } else {
                option_position.1 += 20;
                value_position.1 = option_position.1;
            }
        }
        context.font.render_text(
            renderer,
            "press ESC to exit",
            get_bottom_text_position(context.graphics.resolution_y),
        );
    }
}

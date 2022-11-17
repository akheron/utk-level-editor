use crate::event::{Event, Keycode};
use crate::render::Renderer;
use crate::types::*;
use crate::Context;
use crate::{get_bottom_text_position, TextInput};

enum Value {
    Comment,
    TimeLimit,
    Number(usize),
}

struct ConfigOption {
    text: &'static str,
    value: Value,
}

fn load_value_text<'a, R: Renderer<'a>>(context: &Context<'a, R>, value: &Value) -> Option<String> {
    let string = match value {
        Value::Number(number) => context.level.general_info.enemy_table[*number].to_string(),
        Value::TimeLimit => format!("{} seconds", context.level.general_info.time_limit),
        Value::Comment => context.level.general_info.comment.to_string(),
    };
    if !string.is_empty() {
        Some(string)
    } else {
        None
    }
}

fn sanitize_level_comment_input(new_text: &str, target_text: &mut String) {
    if (new_text.chars().all(char::is_alphanumeric) || new_text.chars().all(char::is_whitespace))
        && (target_text.len() + new_text.len() <= 19)
    {
        *target_text += new_text;
    }
}

pub struct GeneralLevelInfoState {
    options: [ConfigOption; 10],
    selected: usize,
}

impl GeneralLevelInfoState {
    pub fn new() -> Self {
        let options = [
            ConfigOption {
                text: "level comment:",
                value: Value::Comment,
            },
            ConfigOption {
                text: "time limit:",
                value: Value::TimeLimit,
            },
            ConfigOption {
                text: "pistol boys:",
                value: Value::Number(0),
            },
            ConfigOption {
                text: "shotgun maniacs:",
                value: Value::Number(1),
            },
            ConfigOption {
                text: "uzi rebels:",
                value: Value::Number(2),
            },
            ConfigOption {
                text: "commandos:",
                value: Value::Number(3),
            },
            ConfigOption {
                text: "granade mofos:",
                value: Value::Number(4),
            },
            ConfigOption {
                text: "civilians:",
                value: Value::Number(5),
            },
            ConfigOption {
                text: "punishers:",
                value: Value::Number(6),
            },
            ConfigOption {
                text: "flamers:",
                value: Value::Number(7),
            },
        ];
        GeneralLevelInfoState {
            options,
            selected: 0usize,
        }
    }

    pub fn handle_event<'a, R: Renderer<'a>, T: TextInput>(
        &mut self,
        context: &mut Context<'a, R>,
        text_input: &T,
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
            Event::TextInput { text, .. } => {
                if let Value::Comment = self.options[self.selected].value {
                    sanitize_level_comment_input(&text, &mut context.level.general_info.comment)
                }
            }
            Event::KeyDown { keycode, .. } => match keycode {
                Keycode::Down => {
                    if self.selected < self.options.len() - 1 {
                        self.selected += 1;
                        self.enable_text_editing_if_needed(text_input);
                    }
                }
                Keycode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                        self.enable_text_editing_if_needed(text_input);
                    }
                }
                Keycode::Right => match self.options[self.selected].value {
                    Value::Number(index) => context.level.general_info.enemy_table[index] += 1,
                    Value::TimeLimit => context.level.general_info.time_limit += 10,
                    _ => (),
                },
                Keycode::Left => match self.options[self.selected].value {
                    Value::Number(index) => {
                        let value = &mut context.level.general_info.enemy_table[index];
                        if *value > 0 {
                            *value -= 1;
                        }
                    }
                    Value::TimeLimit => {
                        let value = &mut context.level.general_info.time_limit;
                        if *value > 0 {
                            *value -= 10;
                        }
                    }
                    _ => (),
                },
                Keycode::Backspace => {
                    if let Value::Comment = self.options[self.selected].value {
                        context.level.general_info.comment.pop();
                    }
                }
                _ => (),
            },
            _ => {}
        }
        Mode::GeneralLevelInfo
    }

    pub fn render<'a, R: Renderer<'a>>(&mut self, renderer: &'a R, context: &Context<'a, R>) {
        renderer.clear_screen();
        let mut option_position = (40, 20);
        let mut value_position = (300, option_position.1);
        for x in 0..self.options.len() {
            let option = &self.options[x];
            if self.selected == x {
                context.font.render_text(
                    renderer,
                    "*",
                    (option_position.0 - 20, option_position.1 + 3),
                );
            }
            context
                .font
                .render_text(renderer, option.text, option_position);
            let value_text = &load_value_text(context, &option.value);
            match value_text {
                Some(text) => context.font.render_text(renderer, text, value_position),
                None => (),
            };
            option_position.1 += 20;
            value_position.1 = option_position.1;
        }
        context.font.render_text(
            renderer,
            "press ESC to exit",
            get_bottom_text_position(context.graphics.resolution_y),
        );
    }

    fn enable_text_editing_if_needed<T: TextInput>(&self, text_input: &T) {
        match self.options[self.selected].value {
            Value::Comment => text_input.start(),
            _ => text_input.stop(),
        }
    }
}

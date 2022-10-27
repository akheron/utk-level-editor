use crate::render;
use crate::Context;
use crate::NextMode;
use crate::{create_text_texture, get_bottom_text_position};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

enum Value {
    Comment(),
    TimeLimit(),
    Number(usize),
}

struct ConfigOption<'a> {
    texture: Texture<'a>,
    value: Value,
}

fn load_text<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    context: &Context,
    text: &str,
) -> Texture<'a> {
    create_text_texture(canvas, texture_creator, &context.font, text)
}

fn load_value_text<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    context: &Context<'a>,
    value: &Value,
) -> Option<Texture<'a>> {
    let string = match value {
        Value::Number(number) => context.level.general_info.enemy_table[*number].to_string(),
        Value::TimeLimit() => format!("{} seconds", context.level.general_info.time_limit),
        Value::Comment() => context.level.general_info.comment.to_string(),
    };
    if !string.is_empty() {
        Some(create_text_texture(
            canvas,
            texture_creator,
            &context.font,
            &string,
        ))
    } else {
        None
    }
}

fn enable_text_editing_if_needed<'a>(context: &Context, selected_option: &ConfigOption<'a>) {
    match selected_option.value {
        Value::Comment() => context.sdl.video().unwrap().text_input().start(),
        _ => context.sdl.video().unwrap().text_input().stop(),
    }
}

fn sanitize_level_comment_input(new_text: &str, target_text: &mut String) {
    if (new_text.chars().all(char::is_alphanumeric) || new_text.chars().all(char::is_whitespace))
        && (target_text.len() + new_text.len() <= 19)
    {
        *target_text += new_text;
    }
}

pub struct GeneralLevelInfoState<'a> {
    esc_instruction_text: Texture<'a>,
    options: [ConfigOption<'a>; 10],
    selected: usize,
}

impl<'a> GeneralLevelInfoState<'a> {
    pub fn new(
        canvas: &mut Canvas<Window>,
        texture_creator: &'a TextureCreator<WindowContext>,
        context: &Context,
    ) -> Self {
        let options = [
            ConfigOption {
                texture: load_text(canvas, texture_creator, context, "level comment:"),
                value: Value::Comment(),
            },
            ConfigOption {
                texture: load_text(canvas, texture_creator, context, "time limit:"),
                value: Value::TimeLimit(),
            },
            ConfigOption {
                texture: load_text(canvas, texture_creator, context, "pistol boys:"),
                value: Value::Number(0),
            },
            ConfigOption {
                texture: load_text(canvas, texture_creator, context, "shotgun maniacs:"),
                value: Value::Number(1),
            },
            ConfigOption {
                texture: load_text(canvas, texture_creator, context, "uzi rebels:"),
                value: Value::Number(2),
            },
            ConfigOption {
                texture: load_text(canvas, texture_creator, context, "commandos:"),
                value: Value::Number(3),
            },
            ConfigOption {
                texture: load_text(canvas, texture_creator, context, "granade mofos:"),
                value: Value::Number(4),
            },
            ConfigOption {
                texture: load_text(canvas, texture_creator, context, "civilians:"),
                value: Value::Number(5),
            },
            ConfigOption {
                texture: load_text(canvas, texture_creator, context, "punishers:"),
                value: Value::Number(6),
            },
            ConfigOption {
                texture: load_text(canvas, texture_creator, context, "flamers:"),
                value: Value::Number(7),
            },
        ];
        let esc_instruction_text = load_text(canvas, texture_creator, context, "press ESC to exit");

        GeneralLevelInfoState {
            options,
            esc_instruction_text,
            selected: 0usize,
        }
    }

    pub fn exec(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        context: &mut Context,
    ) -> NextMode {
        enable_text_editing_if_needed(context, &self.options[self.selected]);

        let mut event_pump = context.sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    context.sdl.video().unwrap().text_input().stop();
                    return NextMode::Editor;
                }
                Event::TextInput { text, .. } => match &self.options[self.selected].value {
                    Value::Comment() => {
                        sanitize_level_comment_input(&text, &mut context.level.general_info.comment)
                    }
                    _ => (),
                },
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Down => {
                        if self.selected < self.options.len() - 1 {
                            self.selected += 1;
                            enable_text_editing_if_needed(context, &self.options[self.selected]);
                        }
                    }
                    Keycode::Up => {
                        if self.selected > 0 {
                            self.selected -= 1;
                            enable_text_editing_if_needed(context, &self.options[self.selected]);
                        }
                    }
                    Keycode::Right => match &self.options[self.selected].value {
                        Value::Number(index) => context.level.general_info.enemy_table[*index] += 1,
                        Value::TimeLimit() => context.level.general_info.time_limit += 10,
                        _ => (),
                    },
                    Keycode::Left => match &self.options[self.selected].value {
                        Value::Number(index) => {
                            let value = &mut context.level.general_info.enemy_table[*index];
                            if *value > 0 {
                                *value = *value - 1;
                            }
                        }
                        Value::TimeLimit() => {
                            let value = &mut context.level.general_info.time_limit;
                            if *value > 0 {
                                *value = *value - 10;
                            }
                        }
                        _ => (),
                    },
                    Keycode::Backspace => match &self.options[self.selected].value {
                        Value::Comment() => {
                            context.level.general_info.comment.pop();
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.clear();
        let mut option_position = (40, 20);
        let mut value_position = (300, option_position.1);
        let render_size = context.graphics.get_render_size();
        for x in 0..self.options.len() {
            let option = &self.options[x];
            if self.selected == x {
                render::render_text_texture(
                    canvas,
                    &context.textures.selected_icon,
                    option_position.0 - 20,
                    option_position.1 + 3,
                    render_size,
                    None,
                );
            }
            render::render_text_texture(
                canvas,
                &option.texture,
                option_position.0,
                option_position.1,
                render_size,
                None,
            );
            let value_texture = &load_value_text(canvas, texture_creator, context, &option.value);
            match value_texture {
                Some(texture) => render::render_text_texture(
                    canvas,
                    texture,
                    value_position.0,
                    value_position.1,
                    render_size,
                    None,
                ),
                None => (),
            };
            option_position.1 += 20;
            value_position.1 = option_position.1;
        }
        render::render_text_texture_coordinates(
            canvas,
            &self.esc_instruction_text,
            get_bottom_text_position(context.graphics.resolution_y.clone()),
            render_size,
            None,
        );
        render::render_and_wait(canvas);
        NextMode::GeneralLevelInfo
    }
}

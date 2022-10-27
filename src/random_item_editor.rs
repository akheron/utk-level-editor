use crate::create_text_texture;
use crate::level::Level;
use crate::render;
use crate::types::*;
use crate::util::{get_bottom_text_position, TITLE_POSITION};
use crate::Context;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

fn load_text<'a>(canvas: &mut Canvas<Window>, context: &Context<'a>, text: &str) -> Texture<'a> {
    create_text_texture(canvas, &context.texture_creator, &context.font, text)
}

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

pub struct RandomItemEditorState<'a> {
    normal_game_instruction_text: Texture<'a>,
    deathmatch_instruction_text: Texture<'a>,
    esc_instruction_text: Texture<'a>,
    selected: usize,
}

impl<'a> RandomItemEditorState<'a> {
    pub fn new(canvas: &mut Canvas<Window>, context: &Context) -> Self {
        let normal_game_instruction_text = load_text(canvas, context, "NORMAL GAME CRATES");
        let deathmatch_instruction_text = load_text(canvas, context, "DEATHMATCH CRATES");
        let esc_instruction_text = load_text(canvas, context, "press ESC to exit");

        RandomItemEditorState {
            normal_game_instruction_text,
            deathmatch_instruction_text,
            esc_instruction_text,
            selected: 0,
        }
    }

    pub fn exec(
        &mut self,
        canvas: &mut Canvas<Window>,
        context: &mut Context,
        game_type: GameType,
    ) -> NextMode {
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
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Down => {
                        if self.selected < context.textures.crates.len() - 1 {
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
        }

        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.clear();
        let render_size = context.graphics.get_render_size();

        render::render_text_texture_coordinates(
            canvas,
            match game_type {
                GameType::Normal => &self.normal_game_instruction_text,
                GameType::Deathmatch => &self.deathmatch_instruction_text,
            },
            TITLE_POSITION,
            render_size,
            None,
        );

        let y = 50;
        let mut option_position = (40, y);
        let mut value_position = (280, option_position.1);
        for x in 0..context.textures.crates.len() {
            let option = &context.textures.crates[x];
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
                &option,
                option_position.0,
                option_position.1,
                render_size,
                None,
            );
            let value_texture = create_text_texture(
                canvas,
                &context.texture_creator,
                &context.font,
                &get_value(&context.level, &game_type, x).to_string(),
            );
            render::render_text_texture(
                canvas,
                &value_texture,
                value_position.0,
                value_position.1,
                render_size,
                None,
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
        render::render_text_texture_coordinates(
            canvas,
            &self.esc_instruction_text,
            get_bottom_text_position(context.graphics.resolution_y),
            render_size,
            None,
        );
        render::render_and_wait(canvas);
        NextMode::RandomItemEditor(game_type)
    }
}

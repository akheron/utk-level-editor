use crate::context_util::resize;
use crate::event::{Event, Keycode};
use crate::level::Level;
use crate::render::Renderer;
use crate::types::*;
use crate::util::{get_bottom_text_position, TITLE_POSITION};
use crate::Context;

fn load_text<'a, R: Renderer<'a>>(
    renderer: &'a R,
    context: &Context<'a, R>,
    text: &str,
) -> R::Texture {
    renderer.create_text_texture(&context.font, text)
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

pub struct RandomItemEditorState<'a, R: Renderer<'a>> {
    renderer: &'a R,
    normal_game_instruction_text: R::Texture,
    deathmatch_instruction_text: R::Texture,
    esc_instruction_text: R::Texture,
    selected: usize,
}

impl<'a, R: Renderer<'a>> RandomItemEditorState<'a, R> {
    pub fn new(renderer: &'a R, context: &Context<'a, R>) -> Self {
        let normal_game_instruction_text = load_text(renderer, context, "NORMAL GAME CRATES");
        let deathmatch_instruction_text = load_text(renderer, context, "DEATHMATCH CRATES");
        let esc_instruction_text = load_text(renderer, context, "press ESC to exit");

        RandomItemEditorState {
            renderer,
            normal_game_instruction_text,
            deathmatch_instruction_text,
            esc_instruction_text,
            selected: 0,
        }
    }

    pub fn handle_event(
        &mut self,
        context: &mut Context<'a, R>,
        game_type: GameType,
        event: Event,
    ) -> Mode {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Keycode::Escape,
            } => {
                context.stop_text_input();
                return Mode::Editor;
            }
            Event::Window { win_event } => {
                resize(self.renderer, context, win_event);
                return Mode::Editor;
            }
            Event::KeyDown { keycode, .. } => match keycode {
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
        Mode::RandomItemEditor(game_type)
    }

    pub fn render(&mut self, context: &Context<'a, R>, game_type: GameType) {
        self.renderer.clear_screen();
        let render_size = context.graphics.get_render_size();

        self.renderer.render_text_texture_coordinates(
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
                self.renderer.render_text_texture(
                    &context.textures.selected_icon,
                    option_position.0 - 20,
                    option_position.1 + 3,
                    render_size,
                    None,
                );
            }
            self.renderer.render_text_texture(
                option,
                option_position.0,
                option_position.1,
                render_size,
                None,
            );
            let value_texture = self.renderer.create_text_texture(
                &context.font,
                &get_value(&context.level, &game_type, x).to_string(),
            );
            self.renderer.render_text_texture(
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
        self.renderer.render_text_texture_coordinates(
            &self.esc_instruction_text,
            get_bottom_text_position(context.graphics.resolution_y),
            render_size,
            None,
        );
    }
}

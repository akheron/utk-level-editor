use crate::event::{Event, Keycode, MouseButton};
use crate::level::Steam;
use crate::level::{crates, StaticCrateType};
use crate::level::{CrateClass, StaticCrate};
use crate::render::{Renderer, RendererColor};
use crate::types::GameType;
use crate::util::*;
use crate::Context;
use crate::Graphics;
use crate::Level;
use crate::Mode;
use crate::TextInput;
use crate::TextureType;

#[derive(PartialEq)]
enum NewLevelState {
    Prompt,
    XSize,
    YSize,
}

#[derive(PartialEq)]
enum SaveLevelType {
    Prompt,
    NameInput,
}

#[derive(PartialEq)]
enum ShadowPromptType {
    Enabled,
    Disabled,
}

#[derive(PartialEq)]
enum PromptType {
    None,
    NewLevel(NewLevelState),
    Save(SaveLevelType),
    CreateShadows(ShadowPromptType),
    Quit,
}

#[derive(PartialEq)]
enum InsertState {
    Instructions((u32, u32)), // level coordinates of currently manipulated item
    Place,
    Delete,
}

#[derive(PartialEq)]
enum InsertType {
    None,
    Spotlight(InsertState),
    Steam(InsertState),
    NormalCrate(InsertState),
    DMCrate(InsertState),
}

pub struct EditorState {
    set_position: u8,
    mouse_left_click: Option<(u32, u32)>,
    mouse_right_click: bool,
    prompt: PromptType,
    insert_item: InsertType,
    new_level_size_x: String,
    new_level_size_y: String,
    drag_tiles: bool,
}

static DEFAULT_LEVEL_SIZE: (u32, u32) = (16, 12);

impl EditorState {
    pub fn new() -> Self {
        EditorState {
            set_position: 0,
            mouse_left_click: None,
            mouse_right_click: false,
            prompt: PromptType::None,
            insert_item: InsertType::None,
            new_level_size_x: DEFAULT_LEVEL_SIZE.0.to_string(),
            new_level_size_y: DEFAULT_LEVEL_SIZE.1.to_string(),
            drag_tiles: false,
        }
    }

    pub fn handle_event<'a, R: Renderer<'a>, T: TextInput>(
        &mut self,
        context: &mut Context<'a, R>,
        text_input: &T,
        event: Event,
    ) -> Mode {
        match event {
            Event::Quit
            | Event::KeyDown {
                keycode: Keycode::Escape,
            } => {
                self.prompt = if self.prompt != PromptType::None
                    || self.insert_item != InsertType::None
                    || self.set_position > 0
                {
                    self.insert_item = InsertType::None;
                    text_input.stop();
                    self.set_position = 0;
                    PromptType::None
                } else {
                    PromptType::Quit
                };
            }
            Event::TextInput { text, .. } => match &self.prompt {
                PromptType::NewLevel(new_level_state) => match new_level_state {
                    NewLevelState::XSize => {
                        sanitize_numeric_input(&text, &mut self.new_level_size_x)
                    }
                    NewLevelState::YSize => {
                        sanitize_numeric_input(&text, &mut self.new_level_size_y)
                    }
                    _ => {}
                },
                PromptType::Save(SaveLevelType::NameInput) => {
                    sanitize_level_name_input(&text, &mut context.level_save_name)
                }
                _ => {}
            },
            Event::Window { .. } => {}
            Event::KeyDown { keycode, .. } => match keycode {
                Keycode::Space => {
                    return Mode::TileSelect;
                }
                Keycode::F1 => {
                    return Mode::Help;
                }
                Keycode::F2 => {
                    text_input.stop();
                    self.prompt = PromptType::Save(SaveLevelType::Prompt);
                }
                Keycode::F3 => {
                    text_input.stop();
                    return Mode::LoadLevel;
                }
                Keycode::F4 => {
                    self.prompt = PromptType::NewLevel(NewLevelState::Prompt);
                    self.new_level_size_x = DEFAULT_LEVEL_SIZE.0.to_string();
                    self.new_level_size_y = DEFAULT_LEVEL_SIZE.1.to_string();
                }
                Keycode::F6 => {
                    text_input.stop();
                    self.prompt = PromptType::CreateShadows(if context.automatic_shadows {
                        ShadowPromptType::Enabled
                    } else {
                        ShadowPromptType::Disabled
                    });
                }
                Keycode::F7 => {
                    return Mode::GeneralLevelInfo;
                }
                Keycode::F8 => {
                    return Mode::RandomItemEditor(GameType::Normal);
                }
                Keycode::F9 => {
                    return Mode::RandomItemEditor(GameType::Deathmatch);
                }
                Keycode::Num1 | Keycode::Num2 => match self.prompt {
                    PromptType::NewLevel(_) | PromptType::Save(_) => {}
                    _ => {
                        self.set_position = if keycode == Keycode::Num1 { 1 } else { 2 };
                        self.prompt = PromptType::None;
                    }
                },
                Keycode::Q | Keycode::W => match self.prompt {
                    PromptType::Save(_) => {}
                    _ => {
                        self.insert_item = if keycode == Keycode::Q {
                            InsertType::Spotlight(InsertState::Place)
                        } else {
                            InsertType::Spotlight(InsertState::Delete)
                        };
                        text_input.stop();
                        self.prompt = PromptType::None;
                    }
                },
                Keycode::A | Keycode::S => match self.prompt {
                    PromptType::Save(_) => {}
                    _ => {
                        self.insert_item = if keycode == Keycode::A {
                            InsertType::Steam(InsertState::Place)
                        } else {
                            InsertType::Steam(InsertState::Delete)
                        };
                        text_input.stop();
                        self.prompt = PromptType::None;
                    }
                },
                Keycode::Z | Keycode::X | Keycode::C => match self.prompt {
                    PromptType::Save(_) => {}
                    _ => {
                        self.insert_item = if keycode == Keycode::Z {
                            InsertType::NormalCrate(InsertState::Place)
                        } else if keycode == Keycode::X {
                            InsertType::DMCrate(InsertState::Place)
                        } else {
                            InsertType::NormalCrate(InsertState::Delete)
                        };
                        text_input.stop();
                        self.prompt = PromptType::None;
                    }
                },
                Keycode::Y => match self.prompt {
                    PromptType::NewLevel(NewLevelState::Prompt) => {
                        self.prompt = PromptType::NewLevel(NewLevelState::XSize);
                        text_input.start();
                    }
                    PromptType::Save(SaveLevelType::Prompt) => {
                        self.prompt = PromptType::Save(SaveLevelType::NameInput);
                        text_input.start();
                    }
                    PromptType::CreateShadows(ref shadow_state) => {
                        context.automatic_shadows = match shadow_state {
                            ShadowPromptType::Enabled => false,
                            ShadowPromptType::Disabled => {
                                context.level.create_shadows();
                                true
                            }
                        };
                        self.prompt = PromptType::None;
                    }
                    PromptType::Quit => return Mode::Quit,
                    PromptType::None => {
                        self.prompt = PromptType::None;
                    }
                    _ => {}
                },
                Keycode::Up => match &self.insert_item {
                    InsertType::Spotlight(state) => {
                        if let InsertState::Instructions(coordinates) = state {
                            let spotlight_intensity =
                                context.level.get_spotlight_from_level(coordinates);
                            context
                                .level
                                .put_spotlight_to_level(coordinates, spotlight_intensity + 1)
                        }
                    }
                    InsertType::Steam(state) => {
                        if let InsertState::Instructions(coordinates) = state {
                            let steam = context.level.get_steam_from_level(coordinates);
                            if steam.range < 6 {
                                context.level.put_steam_to_level(
                                    coordinates,
                                    &Steam {
                                        angle: steam.angle,
                                        range: steam.range + 1,
                                    },
                                )
                            }
                        }
                    }
                    InsertType::NormalCrate(state) | InsertType::DMCrate(state) => {
                        if let InsertState::Instructions(coordinates) = state {
                            let mut crate_item = *context.level.get_crate_from_level(coordinates);
                            if (crate_item.crate_class as u32) < CrateClass::Energy as u32 {
                                crate_item.crate_type = 0;
                                crate_item.crate_class =
                                    CrateClass::from_u32(crate_item.crate_class as u32 + 1);
                                context.level.put_crate_to_level(coordinates, &crate_item)
                            }
                        }
                    }
                    _ => {
                        if context.level.scroll.1 > 0 {
                            context.level.scroll.1 -= 1
                        }
                    }
                },
                Keycode::Down => match &self.insert_item {
                    InsertType::Spotlight(state) => {
                        if let InsertState::Instructions(coordinates) = state {
                            let spotlight_intensity =
                                context.level.get_spotlight_from_level(coordinates);
                            if spotlight_intensity > 0 {
                                context
                                    .level
                                    .put_spotlight_to_level(coordinates, spotlight_intensity - 1)
                            }
                        }
                    }
                    InsertType::Steam(state) => {
                        if let InsertState::Instructions(coordinates) = state {
                            let steam = context.level.get_steam_from_level(coordinates);
                            if steam.range > 0 {
                                context.level.put_steam_to_level(
                                    coordinates,
                                    &Steam {
                                        angle: steam.angle,
                                        range: steam.range - 1,
                                    },
                                )
                            }
                        }
                    }
                    InsertType::NormalCrate(state) | InsertType::DMCrate(state) => {
                        if let InsertState::Instructions(coordinates) = state {
                            let mut crate_item = *context.level.get_crate_from_level(coordinates);
                            if crate_item.crate_class as u32 > 0 {
                                crate_item.crate_type = 0;
                                crate_item.crate_class =
                                    CrateClass::from_u32(crate_item.crate_class as u32 - 1);
                                context.level.put_crate_to_level(coordinates, &crate_item)
                            }
                        }
                    }
                    _ => {
                        if context.level.scroll.1 + context.graphics.get_full_y_tiles_per_screen()
                            < (context.level.tiles.len()) as u32
                        {
                            context.level.scroll.1 += 1;
                        }
                    }
                },
                Keycode::Left => match &self.insert_item {
                    InsertType::Steam(state) => {
                        if let InsertState::Instructions(coordinates) = state {
                            let steam = context.level.get_steam_from_level(coordinates);
                            context.level.put_steam_to_level(
                                coordinates,
                                &Steam {
                                    angle: (steam.angle + 360 - 5) % 360,
                                    range: steam.range,
                                },
                            )
                        }
                    }
                    InsertType::NormalCrate(state) | InsertType::DMCrate(state) => {
                        if let InsertState::Instructions(coordinates) = state {
                            let mut crate_item = *context.level.get_crate_from_level(coordinates);
                            if crate_item.crate_type > 0 {
                                crate_item.crate_type -= 1;
                                context.level.put_crate_to_level(coordinates, &crate_item);
                            }
                        }
                    }
                    _ => {
                        if context.level.scroll.0 > 0 {
                            context.level.scroll.0 -= 1;
                        }
                    }
                },
                Keycode::Right => match &self.insert_item {
                    InsertType::Steam(state) => {
                        if let InsertState::Instructions(coordinates) = state {
                            let steam = context.level.get_steam_from_level(coordinates);
                            context.level.put_steam_to_level(
                                coordinates,
                                &Steam {
                                    angle: (steam.angle + 5) % 360,
                                    range: steam.range,
                                },
                            )
                        }
                    }
                    InsertType::NormalCrate(state) | InsertType::DMCrate(state) => {
                        if let InsertState::Instructions(coordinates) = state {
                            let mut crate_item = *context.level.get_crate_from_level(coordinates);
                            if crate_item.crate_type
                                < (crates(crate_item.crate_class).len() - 1) as u8
                            {
                                crate_item.crate_type += 1;
                                context.level.put_crate_to_level(coordinates, &crate_item);
                            }
                        }
                    }
                    _ => {
                        if context.level.scroll.0 + context.graphics.get_full_x_tiles_per_screen()
                            < (context.level.tiles[0].len()) as u32
                        {
                            context.level.scroll.0 += 1;
                        }
                    }
                },
                Keycode::Return | Keycode::KpEnter => match self.insert_item {
                    InsertType::Spotlight(InsertState::Instructions(_)) => {
                        self.insert_item = InsertType::Spotlight(InsertState::Place);
                    }
                    InsertType::Steam(InsertState::Instructions(_)) => {
                        self.insert_item = InsertType::Steam(InsertState::Place);
                    }
                    InsertType::NormalCrate(InsertState::Instructions(_)) => {
                        self.insert_item = InsertType::NormalCrate(InsertState::Place);
                    }
                    InsertType::DMCrate(InsertState::Instructions(_)) => {
                        self.insert_item = InsertType::DMCrate(InsertState::Place);
                    }
                    _ => match self.prompt {
                        PromptType::NewLevel(NewLevelState::XSize)
                            if self.new_level_size_x.len() > 1
                                && self.new_level_size_x.parse::<u8>().unwrap() >= 16 =>
                        {
                            self.prompt = PromptType::NewLevel(NewLevelState::YSize);
                        }
                        PromptType::NewLevel(NewLevelState::YSize)
                            if self.new_level_size_x.len() > 1
                                && self.new_level_size_y.parse::<u8>().unwrap() >= 12 =>
                        {
                            context.level = Level::get_default_level((
                                self.new_level_size_x.parse::<u8>().unwrap(),
                                self.new_level_size_y.parse::<u8>().unwrap(),
                            ));
                            text_input.stop();
                            context.saved_level_name = None;
                            context.level_save_name.clear();
                            self.prompt = PromptType::None;
                        }
                        PromptType::Save(SaveLevelType::NameInput)
                            if context.level_save_name.len() > 1 =>
                        {
                            let level_save_name_uppercase = context.level_save_name.to_uppercase();
                            let level_saved_name = format!("{}.LEV", &level_save_name_uppercase);
                            context.level.serialize(&level_saved_name).unwrap();
                            text_input.stop();
                            context.saved_level_name = Some(level_saved_name.to_lowercase());
                            self.prompt = PromptType::None;
                        }
                        _ => {}
                    },
                },
                Keycode::Backspace => match &self.prompt {
                    PromptType::NewLevel(new_level_state) => match new_level_state {
                        NewLevelState::XSize => {
                            self.new_level_size_x.pop();
                        }
                        NewLevelState::YSize => {
                            self.new_level_size_y.pop();
                        }
                        _ => {}
                    },
                    PromptType::Save(SaveLevelType::NameInput) => {
                        context.level_save_name.pop();
                    }
                    _ => {}
                },
                Keycode::Plus | Keycode::KpPlus => {
                    if context.graphics.render_multiplier == 1 {
                        context.graphics.render_multiplier = 2;
                    }
                }
                Keycode::Minus | Keycode::KpMinus => {
                    if context.graphics.render_multiplier == 2 {
                        context.graphics.render_multiplier = 1;
                        context.level.scroll = (0, 0);
                    }
                }
                _ => {
                    if self.prompt != PromptType::NewLevel(NewLevelState::XSize)
                        && self.prompt != PromptType::NewLevel(NewLevelState::YSize)
                        && self.prompt != PromptType::Save(SaveLevelType::NameInput)
                    {
                        self.prompt = PromptType::None
                    }
                }
            },
            Event::MouseMotion { x, y, .. } => {
                context.mouse.0 = x as u32;
                context.mouse.1 = y as u32;
                if self.mouse_left_click.is_some() {
                    self.handle_mouse_left_down(context);
                }
                if self.mouse_right_click {
                    self.handle_mouse_right_down(context);
                }
            }
            Event::MouseButtonDown {
                button: MouseButton::Left,
            } => {
                self.mouse_left_click = Some(context.mouse);
                self.handle_mouse_left_down(context);
            }
            Event::MouseButtonUp {
                button: MouseButton::Left,
            } => {
                if self.drag_tiles {
                    self.drag_tiles = false;
                    if let Some(coordinates) = self.mouse_left_click {
                        let selected_level_tiles = get_selected_level_tiles(
                            &context.graphics,
                            &coordinates,
                            &get_limited_screen_level_size(
                                &context.graphics,
                                &context.mouse,
                                &context.level,
                                context.graphics.get_render_size(),
                            ),
                            context.level.tiles[0].len() as u32,
                            Some(context.level.scroll),
                        );
                        for level_tile_id in selected_level_tiles {
                            context.level.put_tile_to_level(
                                level_tile_id,
                                Some(context.selected_tile_id),
                                &context.texture_type_selected,
                            );
                        }
                        if context.texture_type_selected == TextureType::Shadow {
                            context.automatic_shadows = false;
                        } else if context.automatic_shadows {
                            context.level.create_shadows();
                        }
                    }
                };
                self.mouse_left_click = None;
            }
            Event::MouseButtonDown {
                button: MouseButton::Right,
            } => {
                self.mouse_right_click = true;
                self.handle_mouse_right_down(context);
            }
            Event::MouseButtonUp {
                button: MouseButton::Right,
            } => {
                self.mouse_right_click = false;
            }
        };
        Mode::Editor
    }

    pub fn render<'a, R: Renderer<'a>>(&mut self, renderer: &'a R, context: &Context<'a, R>) {
        renderer.render_level(
            &context.graphics,
            &context.level,
            &context.textures,
            &context.trigonometry,
            &context.font,
        );
        let highlighted_id = get_tile_id_from_coordinates(
            &context.graphics,
            &get_limited_screen_level_size(
                &context.graphics,
                &context.mouse,
                &context.level,
                context.graphics.get_render_size(),
            ),
            context.graphics.get_x_tiles_per_screen(),
            None,
        );
        renderer.highlight_selected_tile(&context.graphics, highlighted_id, &RendererColor::White);
        let render_size = context.graphics.get_render_size();
        context.font.render_text_relative(
            renderer,
            "PL1",
            context.level.origo(render_size),
            (
                context.level.p1_position.0 * render_size,
                context.level.p1_position.1 * render_size,
            ),
        );
        context.font.render_text_relative(
            renderer,
            "PL2",
            context.level.origo(render_size),
            (
                context.level.p2_position.0 * render_size,
                context.level.p2_position.1 * render_size,
            ),
        );
        let text = if self.set_position == 1 {
            "place PL1 start point"
        } else if self.set_position == 2 {
            "place PL2 start point"
        } else {
            match self.insert_item {
                InsertType::Spotlight(InsertState::Instructions(_)) => {
                    "use UP and DOWN keys to adjust size, ENTER to accept"
                }
                InsertType::Spotlight(InsertState::Place) => "place spotlight (ESC to cancel)",
                InsertType::Spotlight(InsertState::Delete) => "delete spotlight (ESC to cancel)",
                InsertType::Steam(InsertState::Instructions(_)) => {
                    "UP/DOWN: range, LEFT/RIGHT: dir, ENTER to accept"
                }
                InsertType::Steam(InsertState::Place) => "place steam (ESC to cancel)",
                InsertType::Steam(InsertState::Delete) => "delete steam (ESC to cancel)",
                InsertType::NormalCrate(InsertState::Place) => "place normal game crate",
                InsertType::DMCrate(InsertState::Place) => "place deathmatch game crate",
                InsertType::NormalCrate(InsertState::Instructions(_))
                | InsertType::DMCrate(InsertState::Instructions(_)) => {
                    "UP/DOWN/LEFT/RIGHT: select CRATE, ENTER to accept"
                }
                InsertType::NormalCrate(InsertState::Delete)
                | InsertType::DMCrate(InsertState::Delete) => "delete crate",
                _ => "F1 for help",
            }
        };
        context.font.render_text(renderer, text, (8, 8));
        self.render_prompt_if_needed(renderer, context);
        if self.insert_item == InsertType::None {
            if let Some(coordinates) = self.mouse_left_click {
                let selected_screen_tiles = get_selected_level_tiles(
                    &context.graphics,
                    &coordinates,
                    &get_limited_screen_level_size(
                        &context.graphics,
                        &context.mouse,
                        &context.level,
                        context.graphics.get_render_size(),
                    ),
                    context.graphics.get_x_tiles_per_screen(),
                    None,
                );
                for screen_tile_id in selected_screen_tiles {
                    renderer.highlight_selected_tile(
                        &context.graphics,
                        screen_tile_id,
                        &RendererColor::White,
                    );
                }
            }
        }
        if let Some(text) = &context.saved_level_name {
            let (x, y) = get_bottom_text_position(context.graphics.resolution_y);
            context.font.render_text(renderer, text, (x, y));
        }
    }

    fn render_input_prompt<'a, R: Renderer<'a>>(
        &self,
        renderer: &'a R,
        context: &Context<'a, R>,
        prompt_position: (u32, u32),
        prompt_line_spacing: u32,
        instruction_text: &str,
        input_text: &str,
    ) {
        context.font.render_text(
            renderer,
            instruction_text,
            (
                prompt_position.0,
                prompt_position.1 + 2 * prompt_line_spacing,
            ),
        );

        if !input_text.is_empty() {
            let (width, _) = context.font.text_size(instruction_text);
            context.font.render_text(
                renderer,
                input_text,
                (
                    prompt_position.0 + width,
                    prompt_position.1 + 2 * prompt_line_spacing,
                ),
            );
        }
    }

    fn render_prompt_if_needed<'a, R: Renderer<'a>>(
        &self,
        renderer: &'a R,
        context: &Context<'a, R>,
    ) {
        if self.prompt != PromptType::None {
            let prompt_position = (context.graphics.resolution_x / 2 - 100, 200);
            let prompt_line_spacing = 30;
            let prompt_texture = match &self.prompt {
                PromptType::NewLevel(state) => {
                    match state {
                        NewLevelState::Prompt => {}
                        input_state => {
                            if *input_state == NewLevelState::XSize
                                || *input_state == NewLevelState::YSize
                            {
                                self.render_input_prompt(
                                    renderer,
                                    context,
                                    prompt_position,
                                    prompt_line_spacing,
                                    "x-size (min. 16 blocks):",
                                    &self.new_level_size_x,
                                );
                            }
                            if *input_state == NewLevelState::YSize {
                                self.render_input_prompt(
                                    renderer,
                                    context,
                                    (prompt_position.0, prompt_position.1 + prompt_line_spacing),
                                    prompt_line_spacing,
                                    "y-size (min. 12 blocks):",
                                    &self.new_level_size_y,
                                );
                            }
                        }
                    }
                    "create new level?"
                }
                PromptType::Save(save_level_state) => {
                    match save_level_state {
                        SaveLevelType::Prompt => {}
                        SaveLevelType::NameInput => {
                            let level_save_name = context.level_save_name.clone();
                            self.render_input_prompt(
                                renderer,
                                context,
                                prompt_position,
                                prompt_line_spacing,
                                "filename:",
                                &level_save_name,
                            );
                        }
                    };
                    "save level?"
                }
                PromptType::Quit => "really wanna quit?",
                PromptType::CreateShadows(shadow_state) => match shadow_state {
                    ShadowPromptType::Enabled => "disable auto shadow?",
                    ShadowPromptType::Disabled => "enable auto shadow?",
                },
                PromptType::None => unreachable!(),
            };
            context
                .font
                .render_text(renderer, prompt_texture, prompt_position);
            context.font.render_text(
                renderer,
                "press Y to confirm",
                (prompt_position.0, prompt_position.1 + prompt_line_spacing),
            );
        }
    }

    fn handle_mouse_left_down<'a, R: Renderer<'a>>(&mut self, context: &mut Context<'a, R>) {
        if self.drag_tiles {
            return;
        }

        if self.set_position > 0 {
            let position = if self.set_position == 1 {
                &mut context.level.p1_position
            } else {
                &mut context.level.p2_position
            };
            *position = get_logical_coordinates(
                &context.graphics,
                context.mouse.0,
                context.mouse.1,
                Some(context.level.scroll),
            );
            self.set_position = 0;
        } else {
            let level_coordinates = get_level_coordinates_from_screen_coordinates(
                &context.graphics,
                &context.mouse,
                &context.level.scroll,
            );
            match self.insert_item {
                InsertType::Spotlight(InsertState::Place) => {
                    self.insert_item =
                        InsertType::Spotlight(InsertState::Instructions(level_coordinates));
                    context.level.put_spotlight_to_level(&level_coordinates, 0);
                }
                InsertType::Spotlight(InsertState::Delete) => {
                    context.level.delete_spotlight_if_near(
                        &level_coordinates,
                        context.graphics.render_multiplier,
                    );
                }
                InsertType::Steam(InsertState::Place) => {
                    self.insert_item =
                        InsertType::Steam(InsertState::Instructions(level_coordinates));
                    context
                        .level
                        .put_steam_to_level(&level_coordinates, &Steam { angle: 0, range: 1 });
                }
                InsertType::Steam(InsertState::Delete) => {
                    context.level.delete_steam_if_near(
                        &level_coordinates,
                        context.graphics.render_multiplier,
                    );
                }
                InsertType::NormalCrate(InsertState::Place) => {
                    self.insert_item =
                        InsertType::NormalCrate(InsertState::Instructions(level_coordinates));
                    context.level.put_crate_to_level(
                        &level_coordinates,
                        &StaticCrateType {
                            crate_variant: StaticCrate::Normal,
                            crate_class: CrateClass::Weapon,
                            crate_type: 0,
                        },
                    );
                }
                InsertType::DMCrate(InsertState::Place) => {
                    self.insert_item =
                        InsertType::DMCrate(InsertState::Instructions(level_coordinates));
                    context.level.put_crate_to_level(
                        &level_coordinates,
                        &StaticCrateType {
                            crate_variant: StaticCrate::Deathmatch,
                            crate_class: CrateClass::Weapon,
                            crate_type: 0,
                        },
                    );
                }
                InsertType::NormalCrate(InsertState::Delete) => {
                    context.level.delete_crate_if_near(
                        &level_coordinates,
                        context.graphics.render_multiplier,
                    );
                }
                InsertType::None => {
                    self.drag_tiles = true;
                }
                _ => {}
            };
        }
    }

    fn handle_mouse_right_down<'a, R: Renderer<'a>>(&self, context: &mut Context<'a, R>) {
        let pointed_tile = get_tile_id_from_coordinates(
            &context.graphics,
            &limit_coordinates(
                &context.mouse,
                &(context.graphics.resolution_x, context.graphics.resolution_y),
            ),
            context.level.tiles[0].len() as u32,
            Some(context.level.scroll),
        );
        context
            .level
            .put_tile_to_level(pointed_tile, None, &TextureType::Shadow);
        context.automatic_shadows = false;
    }
}

fn sanitize_numeric_input(new_text: &str, target_text: &mut String) {
    if new_text.chars().all(char::is_numeric) && (target_text.len() + new_text.len() <= 3) {
        *target_text += new_text;
    }
}

fn sanitize_level_name_input(new_text: &str, target_text: &mut String) {
    if new_text.chars().all(char::is_alphanumeric) && (target_text.len() + new_text.len() <= 11) {
        *target_text += new_text;
    }
}

fn get_limited_screen_level_size(
    graphics: &Graphics,
    mouse: &(u32, u32),
    level: &Level,
    render_size: u32,
) -> (u32, u32) {
    limit_coordinates(
        &(
            std::cmp::min(
                mouse.0,
                (level.tiles[0].len() as u32 - level.scroll.0) * render_size - 1,
            ),
            std::cmp::min(
                mouse.1,
                (level.tiles.len() as u32 - level.scroll.1) * render_size - 1,
            ),
        ),
        &(graphics.resolution_x, graphics.resolution_y),
    )
}

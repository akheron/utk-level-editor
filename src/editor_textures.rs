use crate::render::Renderer;
use crate::Context;

pub struct EditorTextures<'a, R: Renderer<'a>> {
    pub p1_text_texture: R::Texture,
    pub p2_text_texture: R::Texture,
    pub p1_set_text_texture: R::Texture,
    pub p2_set_text_texture: R::Texture,
    pub help_text_texture: R::Texture,
    pub create_new_level_text_texture: R::Texture,
    pub wanna_quit_text_texture: R::Texture,
    pub save_level_text_texture: R::Texture,
    pub filename_text_texture: R::Texture,
    pub press_y_text_texture: R::Texture,
    pub new_level_x_size_text_texture: R::Texture,
    pub new_level_y_size_text_texture: R::Texture,
    pub spotlight_place_text_texture: R::Texture,
    pub spotlight_delete_text_texture: R::Texture,
    pub spotlight_instructions_text_texture: R::Texture,
    pub steam_place_text_texture: R::Texture,
    pub steam_delete_text_texture: R::Texture,
    pub steam_instructions_text_texture: R::Texture,
    pub create_shadows_enabled_instructions_text_texture: R::Texture,
    pub create_shadows_disabled_instructions_text_texture: R::Texture,
    pub place_normal_crate_text_texture: R::Texture,
    pub place_deathmatch_create_text_texture: R::Texture,
    pub insert_crate_text_texture: R::Texture,
    pub delete_crate_text_texture: R::Texture,
}

impl<'a, R: Renderer<'a>> EditorTextures<'a, R> {
    pub fn new(renderer: &'a R, context: &Context<'a, R>) -> EditorTextures<'a, R> {
        EditorTextures {
            p1_text_texture: renderer.create_text_texture(&context.font, "PL1"),
            p2_text_texture: renderer.create_text_texture(&context.font, "PL2"),
            p1_set_text_texture: renderer
                .create_text_texture(&context.font, "place PL1 start point"),
            p2_set_text_texture: renderer
                .create_text_texture(&context.font, "place PL2 start point"),
            help_text_texture: renderer.create_text_texture(&context.font, "F1 for help"),
            create_new_level_text_texture: renderer
                .create_text_texture(&context.font, "create new level?"),
            wanna_quit_text_texture: renderer
                .create_text_texture(&context.font, "really wanna quit?"),
            save_level_text_texture: renderer.create_text_texture(&context.font, "save level?"),
            filename_text_texture: renderer.create_text_texture(&context.font, "filename:"),
            press_y_text_texture: renderer.create_text_texture(&context.font, "press Y to confirm"),
            new_level_x_size_text_texture: renderer
                .create_text_texture(&context.font, "x-size (min. 16 blocks):"),
            new_level_y_size_text_texture: renderer
                .create_text_texture(&context.font, "y-size (min. 12 blocks):"),
            spotlight_place_text_texture: renderer
                .create_text_texture(&context.font, "place spotlight (ESC to cancel)"),
            spotlight_delete_text_texture: renderer
                .create_text_texture(&context.font, "delete spotlight (ESC to cancel)"),
            spotlight_instructions_text_texture: renderer.create_text_texture(
                &context.font,
                "use UP and DOWN keys to adjust size, ENTER to accept",
            ),
            steam_place_text_texture: renderer
                .create_text_texture(&context.font, "place steam (ESC to cancel)"),
            steam_delete_text_texture: renderer
                .create_text_texture(&context.font, "delete steam (ESC to cancel)"),
            steam_instructions_text_texture: renderer.create_text_texture(
                &context.font,
                "UP/DOWN: range, LEFT/RIGHT: dir, ENTER to accept",
            ),
            create_shadows_enabled_instructions_text_texture: renderer
                .create_text_texture(&context.font, "disable auto shadow?"),
            create_shadows_disabled_instructions_text_texture: renderer
                .create_text_texture(&context.font, "enable auto shadow?"),
            place_normal_crate_text_texture: renderer
                .create_text_texture(&context.font, "place normal game crate"),
            place_deathmatch_create_text_texture: renderer
                .create_text_texture(&context.font, "place deathmatch game crate"),
            insert_crate_text_texture: renderer.create_text_texture(
                &context.font,
                "UP/DOWN/LEFT/RIGHT: select CRATE, ENTER to accept",
            ),
            delete_crate_text_texture: renderer.create_text_texture(&context.font, "delete crate"),
        }
    }
}

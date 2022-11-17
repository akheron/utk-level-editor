use crate::event::WindowEvent;
use crate::font::Font;
use crate::render::Renderer;
use crate::Context;
use crate::Textures;

fn refresh<'a, R: Renderer<'a>>(
    renderer: &'a R,
    context: &mut Context<'a, R>,
    window_size: (u32, u32),
) {
    context.graphics.resolution_x = window_size.0;
    context.graphics.resolution_y = window_size.1;
    context.font = Font::new(renderer, &context.fn2);
    context.textures = get_textures(renderer);
}

pub fn resize<'a, R: Renderer<'a>>(
    renderer: &'a R,
    context: &mut Context<'a, R>,
    event: WindowEvent,
) {
    match event {
        WindowEvent::Resized { width, height } => {
            refresh(renderer, context, (width, height));
        }
        WindowEvent::Maximized => {
            refresh(renderer, context, renderer.window_size());
        }
    }
}

pub fn get_textures<'a, R: Renderer<'a>>(renderer: &'a R) -> Textures<R::Texture> {
    Textures {
        floor: renderer.load_texture("./assets/FLOOR1.PNG"),
        walls: renderer.load_texture("./assets/WALLS1.PNG"),
        shadows: renderer.load_texture("./assets/SHADOWS_ALPHA.PNG"),
    }
}

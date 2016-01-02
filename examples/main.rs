#[macro_use]
extern crate allegro;
extern crate allegro_sys;
extern crate allegro_font;
extern crate allegro_primitives;

extern crate dungeon;

mod renderer;

use std::env;
use renderer::Renderer;

fn main() {

    let seed: &[_] = &[1, 2, 3, 8];
    let dungeon = dungeon::Dungeon::from_seed(seed, 19, 10);

    let render_type = env::args().skip(1).next().unwrap_or("ascii".into());

    if let Some(dungeon) = dungeon {

        if let Some(renderer) = match render_type.as_ref() {
            "allegro" => renderer::AllegroRenderer::from_dungeon(&dungeon),
            _ => renderer::AsciiRenderer::from_dungeon(&dungeon)

        } {
            renderer.draw();
        }

    }

}


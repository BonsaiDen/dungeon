use dungeon;

pub trait Renderer {
    fn from_dungeon(dungeon: &dungeon::Dungeon) -> Option<Box<Renderer>> where Self: Sized;
    fn draw(&self);
}

mod ascii;
mod allegro;

pub use renderer::ascii::AsciiRenderer;
pub use renderer::allegro::AllegroRenderer;


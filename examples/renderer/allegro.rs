use std::cmp;

use renderer::Renderer;

use dungeon;
use dungeon::room;
use dungeon::base::{Offset, Side};

use allegro::*;
use allegro_font::{FontAddon, Font};
use allegro_primitives::PrimitivesAddon;

pub struct AllegroRenderer {
    sx: usize,
    sy: usize,
    width: usize,
    height: usize
}

impl Renderer for AllegroRenderer {

    fn from_dungeon(dungeon: &dungeon::Dungeon) -> Option<Box<Renderer>> {

        // Calculate bounds
        let mut min = Offset { x: 9999, y: 9999 };
        let mut max = Offset { x: -9999, y: -9999 };

        if dungeon.rooms.len() > 0 {

            for (offset, _) in dungeon.rooms.iter() {
                min.x = cmp::min(offset.x, min.x);
                min.y = cmp::min(offset.y, min.y);
                max.x = cmp::max(offset.x, max.x);
                max.y = cmp::max(offset.y, max.y);
            }

            // Create drawing array
            let (width, height) = (
                ((max.x - min.x) + 1),
                ((max.y - min.y) + 1)
            );

            let mut renderer = AllegroRenderer::new(
                width as usize,
                height as usize,
                19,
                8
            );

            Some(Box::new(renderer))

        } else {
            None
        }

    }

    fn draw(&self) {

        // Setup Rendering (requires OpenGL)
        let mut core = Core::init().unwrap();
        core.set_new_display_flags(OPENGL);
        core.set_new_display_option(DisplayOption::SampleBuffers, 1, DisplayOptionImportance::Require);
        core.set_new_display_option(DisplayOption::Samples, 16, DisplayOptionImportance::Require);

        // Create display
        let mut disp = Display::new(&core, 640, 480).ok().expect("Failed to create OPENGL context.");
        disp.set_window_title("Dung(o)en");

        // Keyboard / Mouse
        core.install_keyboard().unwrap();
        core.install_mouse().unwrap();

        // Addons
        let timer = Timer::new(&core, 1.0 / 30 as f64).unwrap();
        let prim = PrimitivesAddon::init(&core).unwrap();
        let font_addon = FontAddon::init(&core).unwrap();
        let font = Font::new_builtin(&font_addon).unwrap();

        let q = EventQueue::new(&core).unwrap();
        q.register_event_source(disp.get_event_source());
        q.register_event_source(core.get_keyboard_event_source());
        q.register_event_source(core.get_mouse_event_source());
        q.register_event_source(timer.get_event_source());

        let white = Color::from_rgb_f(1.0, 1.0, 1.0);
        prim.draw_rectangle(0.5, 0.5, 100.5, 100.5, white, 2.0);
        core.flip_display();

        'exit: loop {

            match q.wait_for_event() {

                DisplayClose{source: src, ..} => {
                    assert!(disp.get_event_source().get_event_source() == src);
                    break 'exit;
                },

                KeyDown{keycode: k, ..} if (k as u32) < 255 => {
                    println!("key down");
                },

                KeyUp{keycode: k, ..} if (k as u32) < 255 => {
                    println!("key up");
                },

                _ => ()

            }

        }

    }

}

impl AllegroRenderer {

    fn new(
        width: usize, height: usize,
        sx: usize, sy: usize

    ) -> AllegroRenderer {
        AllegroRenderer {
            sx: sx,
            sy: sy,
            width: width,
            height: height,
        }
    }

}


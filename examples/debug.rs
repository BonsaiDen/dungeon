extern crate dungeon;
use std::cmp;
use dungeon::base::{Offset, Side};
use dungeon::room;

pub struct Renderer {
    sx: usize,
    sy: usize,
    width: usize,
    height: usize,
    buffer: Vec<char>
}

impl Renderer {

    pub fn from_dungeon(dungeon: &dungeon::Dungeon) -> Option<Renderer> {

        // Print Statistics
        println!("Dungeon with {} rooms", dungeon.rooms.len());

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

            let mut renderer = Renderer::new(
                width as usize,
                height as usize,
                19,
                8
            );

            // Draw rooms into buffer
            for (_, room) in dungeon.rooms.iter() {
                renderer.draw_room(room);
            }

            Some(renderer)

        } else {
            None
        }

    }

    pub fn draw(&self) {
        for y in 0..self.height * self.sy {
            let offset = y * self.width * self.sx;
            let line = &self.buffer[offset..(offset + self.width * self.sx)];
            println!("{}", line.iter().cloned().collect::<String>());
        }
    }

    // Internal ---------------------------------------------------------------
    //
    fn new(
        width: usize, height: usize,
        sx: usize, sy: usize

    ) -> Renderer {

        let size = width * sx * height * sy;
        let mut buffer: Vec<char> = Vec::with_capacity(size);
        for _ in 0..size {
            buffer.push(b' ' as char);
        }

        Renderer {
            sx: sx,
            sy: sy,
            width: width,
            height: height,
            buffer: buffer
        }

    }

    fn draw_room(&mut self, room: &room::Room) {

        let x = room.offset.x as usize;
        let y = room.offset.y as usize;
        let (sx, sy) = (self.sx, self.sy);

        // Lines
        self.draw_hline(y * sy, x * sx, x * sx + sx - 3, '\u{2501}');
        self.draw_hline(y * sy + sy - 2, x * sx, x * sx + sx - 3, '\u{2501}');
        self.draw_vline(x * sx, y * sy, y * sy + sy - 1, '\u{2503}');
        self.draw_vline(x * sx + sx - 3, y * sy, y * sy + sy - 1, '\u{2503}');

        // Corners
        self.buffer[y * sy * self.width * self.sx + x * sx] = '\u{250f}';
        self.buffer[y * sy * self.width * self.sx + x * sx + sx - 3] = '\u{2513}';
        self.buffer[(y * sy + sy - 2) * self.width * self.sx + x * sx] = '\u{2517}';
        self.buffer[(y * sy + sy - 2) * self.width * self.sx + x * sx + sx - 3] = '\u{251b}';

        // Doors
        for d in room.doors.iter() {
            self.draw_door(
                room.offset.x, room.offset.y, &d.side, d.lock.to_char()
            );
        }

        // Room Types
        let mut y_offset = 1;
        match room.typ {
            room::Type::Exit | room::Type::Entrance => {
                self.draw_text(
                    room.offset.x, room.offset.y,
                    1, y_offset,
                    &room.typ.to_string()[..]
                );
                y_offset += 1;
            },
            _ => {}
        }

        // Chest
        if let Some(ref chest) = room.chest {
            self.draw_text(
                room.offset.x, room.offset.y,
                1, y_offset,
                &chest.to_string()[..]
            );
            y_offset += 1;
        }

        // Enemy
        if let Some(ref enemy) = room.enemy {
            self.draw_text(
                room.offset.x, room.offset.y,
                1, y_offset,
                &enemy.to_string()[..]
            );
            y_offset += 1;
        }

        // Switch
        if let Some(ref switch) = room.switch {
            self.draw_text(
                room.offset.x, room.offset.y,
                1, y_offset,
                &switch.to_string()[..]
            );
        }

    }

    fn draw_door(&mut self, x: i32, y: i32, d: &Side, m: char) {

        let (x, y) = (x as usize, y as usize);
        let (sx, sy) = (self.sx, self.sy);

        match *d {
            Side::North => {
                self.buffer[y * sy * self.width * self.sx + x * sx + 8] = '\u{2580}';
                self.buffer[(y * sy - 1) * self.width * self.sx + x * sx + 8] = '\u{2588}';
                self.buffer[(y * sy + 1) * self.width * self.sx + x * sx + 8] = m;
            },
            Side::East => {
                self.buffer[(y * sy + 3) * self.width * self.sx + x * sx + sx - 4] = m;
                self.buffer[(y * sy + 3) * self.width * self.sx + x * sx + sx - 3] = '\u{2590}';
                self.buffer[(y * sy + 3) * self.width * self.sx + x * sx + sx - 2] = '\u{2588}';
            },
            Side::South => {
                self.buffer[(y * sy + sy - 2) * self.width * self.sx + x * sx + 8] = '\u{2584}';
                self.buffer[(y * sy + sy - 3) * self.width * self.sx + x * sx + 8] = m;
            },
            Side::West => {
                self.buffer[(y * sy + 3) * self.width * self.sx + x * sx + 1] = m;
                self.buffer[(y * sy + 3) * self.width * self.sx + x * sx] = '\u{258C}';
                self.buffer[(y * sy + 3) * self.width * self.sx + x * sx - 1] = '\u{2588}';
            },
            _ => {}
        }

    }

    fn draw_text(&mut self, x: i32, y: i32, ox: usize, oy: usize, text: &str) {

        let (x, y) = (x as usize, y as usize);
        let (sx, sy) = (self.sx, self.sy);

        for (index, t) in text.chars().enumerate() {
            self.buffer[(y * sy + oy) * self.width * self.sx + x * sx + ox + index] = t;
        }

    }

    fn draw_hline(&mut self, y: usize, x: usize, tx: usize, b: char) {
        for i in x..tx {
            self.buffer[i + y * self.width * self.sx] = b;
        }
    }

    fn draw_vline(&mut self, x: usize, y: usize, ty: usize, b: char) {
        for i in y..ty {
            self.buffer[i * self.width * self.sx + x] = b;
        }
    }

}


extern crate rand;
use std::collections::HashMap;
use std::cmp;
use rand::{SeedableRng, StdRng};
/*
#[derive(Debug)]
enum RoomSide {
    North = 1,
    East = 2,
    South = 4,
    West = 8
}

#[derive(Debug)]
enum TriggerType {
    None,
    Key,
    KillEnemies,
    Switch
}

#[derive(Debug)]
enum KeyType {
    Small,
    Boss
}

#[derive(Debug)]
enum EnemySet {
    Boss,
    SmallEnemies
}

#[derive(Debug)]
struct Door {
    side: RoomSide,
    trigger: TriggerType,
    from: Option<RoomOffset>,
    to: Option<RoomOffset>
}

#[derive(Debug)]
struct Key {
    trigger: TriggerType,
    typ: KeyType
}
*/
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct RoomOffset {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Room {
    offset: RoomOffset,
    //doors: Vec<Door>,
    //enemies: Option<EnemySet>,
    //key: Option<Key>
}

impl Room {

    pub fn new(x: i32, y: i32) -> Room {
        Room {
            offset: RoomOffset {
                x: x,
                y: y
            }
        }
    }

    pub fn draw(&self, draw_buffer: &mut DrawBuffer) {
        draw_buffer.draw_rect(self.offset.x, self.offset.y);
    }
}


// Top Level Dungeon Structure -------------------------------------------------
struct Dungeon {
    start_room: Option<RoomOffset>,
    final_room: Option<RoomOffset>,
    boss_room: Option<RoomOffset>,
    rooms: HashMap<RoomOffset, Room>
}

impl Dungeon {

    pub fn new() -> Dungeon {
        Dungeon {
            start_room: None,
            final_room: None,
            boss_room: None,
            rooms: HashMap::new()
        }
    }

    pub fn generate(&mut self, rng: &mut StdRng, max_rooms: usize) {

        let r = Room::new(2, 3);
        self.rooms.insert(r.offset, r);

        let offset = RoomOffset { x: 2, y: 2 };
        self.rooms.insert(offset, Room {
            offset: offset
        });

        let offset = RoomOffset { x: 2, y: 1 };
        self.rooms.insert(offset, Room {
            offset: offset
        });

        let offset = RoomOffset { x: 1, y: 1 };
        self.rooms.insert(offset, Room {
            offset: offset
        });

        let offset = RoomOffset { x: 4, y: 2 };
        self.rooms.insert(offset, Room {
            offset: offset,
        });

        let offset = RoomOffset { x: 3, y: 2 };
        self.rooms.insert(offset, Room {
            offset: offset,
        });

    }

    pub fn print(&self) {

        // Print Statistics
        println!("Dungeon with {} rooms", self.rooms.len());

        // Calculate bounds
        let mut min = RoomOffset { x: 9999, y: 9999 };
        let mut max = RoomOffset { x: -9999, y: -9999 };

        for (offset, _) in self.rooms.iter() {
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

        let mut draw_buffer = DrawBuffer::new(
            width as usize,
            height as usize,
            min.x,
            min.y,
            12,
            6
        );

        // Draw rooms into buffer
        for (_, room) in self.rooms.iter() {
            room.draw(&mut draw_buffer);
        }

        // Draw
        draw_buffer.print();

    }

}

// Debug Draw Buffer ----------------------------------------------------------
struct DrawBuffer {
    sx: usize,
    sy: usize,
    ox: i32,
    oy: i32,
    width: usize,
    height: usize,
    buffer: Vec<char>
}

impl DrawBuffer {
    pub fn new(width: usize, height: usize, ox: i32, oy: i32, sx: usize, sy: usize) -> DrawBuffer {

        let size = width * sx * height * sy;
        let mut buffer: Vec<char> = Vec::with_capacity(size);
        for _ in 0..size {
            buffer.push(b' ' as char);
        }

        DrawBuffer {
            sx: sx,
            sy: sy,
            ox: ox,
            oy: oy,
            width: width,
            height: height,
            buffer: buffer
        }

    }

    pub fn draw_rect(&mut self, x: i32, y: i32) {

        let x = (x - self.ox) as usize;
        let y = (y - self.oy) as usize;
        let sx = self.sx;
        let sy = self.sy;

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

    }

    pub fn draw_hline(&mut self, y: usize, x: usize, tx: usize, b: char) {
        for i in x..tx {
            self.buffer[i + y * self.width * self.sx] = b;
        }
    }

    pub fn draw_vline(&mut self, x: usize, y: usize, ty: usize, b: char) {
        for i in y..ty {
            self.buffer[i * self.width * self.sx + x] = b;
        }
    }

    pub fn print(&self) {
        for y in 0..self.height * self.sy {
            let offset = y * self.width * self.sx;
            let line = &self.buffer[offset..(offset + self.width * self.sx)];

            println!("{}", line.iter().cloned().collect::<String>());
        }
    }

}

fn main() {

    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    let mut dungeon = Dungeon::new();
    dungeon.generate(&mut rng, 12);
    dungeon.print();

}



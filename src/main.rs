extern crate rand;
use std::collections::HashMap;
use std::cmp;
use rand::{Rng, SeedableRng, StdRng};
use std::ops::Add;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
    Invalid = 4
}

impl Direction {

    pub fn offset(&self) -> Offset {
        match *self {
            Direction::North => Offset { x: 0, y: -1 },
            Direction::East => Offset { x: 1, y: 0 },
            Direction::South => Offset { x: 0, y: 1 },
            Direction::West => Offset { x: -1, y: 0 },
            Direction::Invalid => Offset { x: 0, y: 0 },
        }
    }

    pub fn from_i32(i: i32) -> Direction {
        match i {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::Invalid,
            _ => unreachable!()
        }
    }

    pub fn from_offsets(a: &Offset, b: &Offset) -> Direction {
        if a.x < b.x {
            Direction::East

        } else if a.x > b.x {
            Direction::West

        } else if a.y < b.y {
            Direction::South

        } else {
            Direction::North
        }
    }

    pub fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West
        ]
    }

}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Offset {
    x: i32,
    y: i32
}

impl Default for Offset {
    fn default() -> Offset {
        Offset {
            x: 0,
            y: 0
        }
    }
}

impl Add for Offset {
    type Output = Offset;

    fn add(self, other: Offset) -> Offset {
        Offset {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

/*

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
struct Key {
    trigger: TriggerType,
    typ: KeyType
}
*/

#[derive(Debug)]
struct Door {
    side: Direction,
    //trigger: TriggerType,
    from: Offset,
    to: Offset
}

#[derive(Debug)]
enum RoomType {
    End,
    Corridor,
    Intersection,
    Crossing,
    Invalid
}

#[derive(Debug)]
struct Room {
    offset: Offset,
    doors: Vec<Door>,
    typ: RoomType,
    //enemies: Option<EnemySet>,
    //key: Option<Key>
}

impl Room {

    pub fn new(x: i32, y: i32) -> Room {
        Room {
            offset: Offset {
                x: x,
                y: y
            },
            typ: RoomType::Invalid,
            doors: Vec::new()
        }
    }

    pub fn draw(&self, draw_buffer: &mut DrawBuffer) {
        draw_buffer.draw_rect(self.offset.x, self.offset.y);
        for d in self.doors.iter() {
            draw_buffer.draw_connection(self.offset.x, self.offset.y, d.side);
        }
    }

    pub fn add_door_to(&mut self, other: &Room) {
        self.doors.push(Door {
            side: Direction::from_offsets(&self.offset, &other.offset),
            from: self.offset,
            to: other.offset
        })
    }

}


// Top Level Dungeon Structure -------------------------------------------------
struct Dungeon {
    start_room: Option<Offset>,
    final_room: Option<Offset>,
    boss_room: Option<Offset>,
    rooms: HashMap<Offset, Room>
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

    pub fn print(&self) {

        // Print Statistics
        println!("Dungeon with {} rooms", self.rooms.len());

        // Calculate bounds
        let mut min = Offset { x: 9999, y: 9999 };
        let mut max = Offset { x: -9999, y: -9999 };

        if self.rooms.len() > 0 {

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
                11,
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

    pub fn generate(&mut self, rng: &mut StdRng, max_rooms: usize) {

        self.create_rooms(rng, max_rooms);
        self.calculate_paths();

        // Create some additional interconnections of adjacent AND inter-reachable rooms
        // TODO generate doors first to prevent shortcuts from generating

    }

    fn calculate_paths(&mut self) {
        // Flag room as:
        // - End (one door)
        // - Corridor (2 doors)
        // - Interconnect (more than 2 doors)
        for (_, room) in self.rooms.iter_mut() {
            room.typ = match room.doors.len() {
                1 => RoomType::End,
                2 => RoomType::Corridor,
                3 => RoomType::Intersection,
                4 => RoomType::Crossing,
                _ => RoomType::Invalid
            };
        }

        // Calculate distances to
    }

    fn create_rooms(&mut self, rng: &mut StdRng, max_rooms: usize) {
        let max_corridor_length = 2;
        let mut next_dir = Direction::from_i32(rng.gen_range(0, 4));
        let mut cor_length = rng.gen_range(1, max_corridor_length);
        let mut offset = Offset::default();
        let mut room_stack: Vec<Offset> = Vec::new();

        // Try to generate the requested number of rooms
        let mut index = 0;
        while index < max_rooms {

            // Drop a random number of rooms from the stack and continue generating
            let rooms_to_drop = rng.gen_range(0, 1 + room_stack.len() / 2);
            if rooms_to_drop > 0 {

                println!("drop count is: {}", rooms_to_drop);
                for _ in 0..rooms_to_drop - 1 {
                    room_stack.pop();
                }
                offset = *room_stack.last().unwrap();
                next_dir = Direction::from_i32(rng.gen_range(0, 4));
                cor_length = rng.gen_range(1, max_corridor_length);

            }

            // Create the next room
            while index < max_rooms {

                // Check if there is already a room at the next location
                let mut next_offset = offset + next_dir.offset();
                if self.rooms.contains_key(&next_offset) {

                    next_dir = Direction::Invalid;

                    // Try all possible directions to find a free adjacend location
                    let mut directions = Direction::all();
                    rng.shuffle(&mut directions);
                    for d in directions.iter() {
                        next_offset = offset + d.offset();
                        if self.rooms.contains_key(&next_offset) == false{
                            next_dir = *d;
                            break;
                        }
                    }

                    // No free adjacent space was found, break out and continue from another
                    // position in the stack
                    if next_dir == Direction::Invalid {
                        println!("overlapping room without any free direction: {:?}", offset);
                        break;

                    // Found a free direction, continue there
                    } else {
                        next_offset = offset + next_dir.offset();
                    }

                }

                // Add a small chance of choosing a different position for
                // continuing the room generation
                let variance = rng.gen_range(0, 255);
                if variance < 25 {
                    break;
                }

                // Go to next offset position
                offset = next_offset;
                cor_length -= 1;

                // Create new room at current offset
                println!("Create Room: {:?} ({:?})", offset, next_dir);
                let mut room = Room::new(offset.x, offset.y);

                // Connect it with the previous room
                match room_stack.last() {
                    Some(offset) => {
                        let other = self.rooms.get_mut(&offset).unwrap();
                        room.add_door_to(&other);
                        other.add_door_to(&room);
                    },
                    None => {}
                }

                self.rooms.insert(offset, room);
                room_stack.push(offset);
                index += 1;

                // Check if we run out of corridor length
                // or should otherwise change the direction
                if cor_length == 0 || variance < 100 {
                    next_dir = Direction::from_i32(rng.gen_range(0, 4));
                    cor_length = rng.gen_range(1, max_corridor_length);
                    println!("End of corridor. Next one will be {:?}, length {:?}", next_dir, cor_length);
                }

            }

            // Stack is already empty, so we cannot actually try any other positions
            if room_stack.len() == 0 {
                break;
            }

        }

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

    pub fn draw_connection(&mut self, x: i32, y: i32, d: Direction) {

        let x = (x - self.ox) as usize;
        let y = (y - self.oy) as usize;
        let sx = self.sx;
        let sy = self.sy;

        match d {
            Direction::North => {
                self.buffer[y * sy * self.width * self.sx + x * sx + 4] = '\u{2580}';
                self.buffer[(y * sy - 1) * self.width * self.sx + x * sx + 4] = '\u{2588}';
            },
            Direction::East => {
                self.buffer[(y * sy + 2) * self.width * self.sx + x * sx + sx - 3] = '\u{2590}';
                self.buffer[(y * sy + 2) * self.width * self.sx + x * sx + sx - 2] = '\u{2588}';
            },
            Direction::South => {
                self.buffer[(y * sy + sy - 2) * self.width * self.sx + x * sx + 4] = '\u{2584}';
            },
            Direction::West => {
                self.buffer[(y * sy + 2) * self.width * self.sx + x * sx] = '\u{258C}';
                self.buffer[(y * sy + 2) * self.width * self.sx + x * sx - 1] = '\u{2588}';
            },
            _ => {}
        }

    }

    pub fn draw_text(&mut self, x: i32, y: i32, text: &str) {

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
    dungeon.generate(&mut rng, 19);
    dungeon.print();

}



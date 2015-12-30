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

#[derive(Debug, Eq, PartialEq)]
enum EnemySet {
    Boss,
    Small,
    Big
}

impl EnemySet {
    pub fn to_string(&self) -> String {
        match *self {
            EnemySet::Boss => "E:Boss".to_owned(),
            EnemySet::Small => "E:Smll".to_owned(),
            EnemySet::Big => "E:Big".to_owned()
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum TriggerType {
    None,
    SmallKey,
    BossKey,
    KillEnemies,
    Chest,
    Pot,
    Switch
}

impl TriggerType {
    pub fn to_string(&self) -> String {
        match *self {
            TriggerType::None => "None".to_owned(),
            TriggerType::SmallKey => "SmllKey".to_owned(),
            TriggerType::BossKey => "BossKey".to_owned(),
            TriggerType::KillEnemies => "Enmy".to_owned(),
            TriggerType::Chest => "Chst".to_owned(),
            TriggerType::Pot => "Pot".to_owned(),
            TriggerType::Switch => "Swth".to_owned()
        }
    }

    pub fn to_char(&self) -> char {
        match *self {
            TriggerType::None => ' ',
            TriggerType::SmallKey => 'L',
            TriggerType::BossKey => 'B',
            TriggerType::KillEnemies => 'E',
            TriggerType::Chest => 'C',
            TriggerType::Pot => 'P',
            TriggerType::Switch => 'S'
        }
    }

}

#[derive(Debug)]
struct Key {
    trigger: TriggerType,
    typ: KeyType
}

impl Key {
    pub fn to_string(&self) -> String {
        let typ = match self.typ {
            KeyType::Boss => "K:Boss",
            KeyType::Small => "K:Smll",
        };
        format!("{}({})", typ, self.trigger.to_string()).to_string()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum KeyType {
    Small,
    Boss
}

#[derive(Debug)]
struct Door {
    side: Direction,
    trigger: TriggerType,
    from: Offset,
    to: Offset
}

#[derive(Debug, Eq, PartialEq)]
enum RoomType {
    End,
    Hallway,
    Intersection,
    Crossing,
    Entrance,
    Exit,
    Invalid,
    None
}

impl RoomType {
    pub fn to_string(&self) -> String {
        match *self {
            RoomType::End => "End".to_owned(),
            RoomType::Hallway => "Hallway".to_owned(),
            RoomType::Intersection => "Intersection".to_owned(),
            RoomType::Crossing => "Crossway".to_owned(),
            RoomType::Entrance => "Entrance".to_owned(),
            RoomType::Exit => "Exit".to_owned(),
            RoomType::Invalid => "Invalid".to_owned(),
            RoomType::None => "".to_owned()
        }
    }
}

#[derive(Debug)]
struct Room {
    offset: Offset,
    doors: Vec<Door>,
    typ: RoomType,
    enemy_set: Option<EnemySet>,
    key: Option<Key>
}

impl Room {

    pub fn new(x: i32, y: i32) -> Room {
        Room {
            offset: Offset {
                x: x,
                y: y
            },
            typ: RoomType::Invalid,
            key: None,
            enemy_set: None,
            doors: Vec::new()
        }
    }

    pub fn draw(&self, draw_buffer: &mut DrawBuffer) {
        draw_buffer.draw_rect(self.offset.x, self.offset.y);
        for d in self.doors.iter() {
            draw_buffer.draw_connection(self.offset.x, self.offset.y, d.side, d.trigger.to_char());
        }

        match self.typ {
            RoomType::Exit | RoomType::Entrance => {
                draw_buffer.draw_text(
                    self.offset.x, self.offset.y, 1, 1, &self.typ.to_string()[..]
                );
            },
            _ => {}
        }

        match self.key {
            Some(_) => {
                draw_buffer.draw_text(
                    self.offset.x, self.offset.y, 1, 2, &self.key.as_ref().unwrap().to_string()[..]
                );
            },
            None => {}
        }

        match self.enemy_set {
            Some(_) => {
                draw_buffer.draw_text(
                    self.offset.x, self.offset.y, 1, 3, &self.enemy_set.as_ref().unwrap().to_string()[..]
                );
            },
            None => {}
        }
    }

    pub fn get_door_to_offset_mut(&mut self, to: &Offset) -> Option<&mut Door> {

        let side = Direction::from_offsets(&self.offset, to);

        self.doors.iter_mut().filter(|d| {
            d.side == side

        }).next()

    }

    pub fn add_door_to(&mut self, other: &Room) {
        self.doors.push(Door {
            side: Direction::from_offsets(&self.offset, &other.offset),
            trigger: TriggerType::None,
            from: self.offset,
            to: other.offset
        })
    }

}

type RoomPath = Vec<Offset>;


// Top Level Dungeon Structure -------------------------------------------------
struct Dungeon {
    entrance_room: Option<Offset>,
    boss_room: Option<Offset>,
    exit_room: Option<Offset>,
    end_rooms: RoomPath,
    rooms: HashMap<Offset, Room>
}

impl Dungeon {

    pub fn new() -> Dungeon {
        Dungeon {
            entrance_room: None,
            exit_room: None,
            boss_room: None,
            end_rooms: Vec::new(),
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
                19,
                8
            );

            // Draw rooms into buffer
            for (_, room) in self.rooms.iter() {
                room.draw(&mut draw_buffer);
            }

            // Draw
            draw_buffer.print();

        }

    }

    pub fn generate(&mut self, rng: &mut StdRng, max_rooms: usize) -> bool {

        self.create_rooms(rng, max_rooms);

        if self.find_special_rooms(rng) == false {
            return false;

        } else if self.place_key_doors(rng) == false {
            return false;
        }

        // TODO from the entrance find all accessible locked doors
            // - place keys in the empty, accessible rooms infront of them
                // - prioritize end rooms for key placement
                    // - remove some rooms from the set of empty rooms but not the end rooms
                    // - then shuffle set set and use the first X to
                // - mark the found doors as done

            // - find the next set of accessible doors
        // - continue until no more locked doors are left


        // TODO Create some additional interconnections of adjacent AND inter-reachable rooms
        true

    }

    fn place_key_doors(&mut self, rng: &mut StdRng) -> bool {

        // Get path from entrance to boss key
        let mut boss_key_path = self.get_boss_key_path();

        // Get path from entrance to boss door
        let mut boss_door_path = self.get_boss_door_path();
        boss_door_path.pop(); // Don't override the boss door

        // Extract shared boss path
        let mut shared_boss_path = Vec::new();
        for (a, b) in boss_key_path.iter().zip(boss_door_path.iter()).filter(|&(a, b)| {
            a == b
        }) {
            shared_boss_path.push(a.clone());
        }

        // Drop shared path elements
        for _ in 0..shared_boss_path.len() - 1 {
            boss_key_path.remove(0);
            boss_door_path.remove(0);
        }

        // Get total number of locked doors to place
        let empty_room_count = self.find_empty_rooms().len();
        let door_count = empty_room_count / 4 + rng.gen_range(0, 1);

        fn to_connected_path(path: RoomPath) -> Vec<(Offset, Offset)> {
            // Convert everything into a connect path
            let mut connected_path = Vec::new();
            if path.len() > 1 {
                for i in 0..path.len() - 1 {
                    connected_path.push((path[i], path[i + 1]));
                }
            }
            connected_path
        }

        // Randomize paths
        let mut paths = vec![
            to_connected_path(shared_boss_path),
            to_connected_path(boss_key_path),
            to_connected_path(boss_door_path)
        ];

        // Prioritize the longer paths for door placement
        paths.sort_by(|a, b| {
            b.len().cmp(&a.len())
        });

        let mut path_index = 0;
        let mut path_count = paths.len();
        let mut doors_locked = 0;
        let mut doors_on_path = vec![0, 0, 0];

        // Try to lock the required number of doors
        while path_count > 0 && doors_locked < door_count {

            // Select the next available path
            let empty = {

                let mut path = &mut paths[path_index % path_count];

                // Get the all empty rooms from the current path
                if path.len() > 1 {

                    // If so, place a door on somewhere on the path
                    let door_index = rng.gen_range(0, path.len());
                    doors_on_path[path_index % path_count] += 1;

                    // Place locked door between the selected room and the one
                    // that comes after it on the path
                    let room = self.rooms.get_mut(&path[door_index].0).unwrap();
                    let door = room.get_door_to_offset_mut(&path[door_index].1).unwrap();
                    door.trigger = TriggerType::SmallKey;

                    doors_locked += 1;

                    // Remove the used room from the path
                    path.remove(door_index);

                    // Calculate the ratio of path len and door count
                    // we want longer paths to have more doors so we achieve
                    // a more even distribution
                    let door_ratio = path.len() / doors_on_path[path_index % path_count];
                    if door_ratio < 2 {
                        path_index += 1;
                    }

                }

                path.len() == 1

            };

            if empty {
                paths.remove(path_index % path_count);
                path_count -= 1;
            }

        }

        // Check if we could place all the doors
        if doors_locked != door_count {
            println!("Failed to lock the required number of doors!");
            return false;
        }

        true

    }

    fn get_boss_key_path(&self) -> RoomPath {

        // Get path from entrance to boss key
        self.visit_rooms(self.entrance_room.unwrap(), |room, path| {
            match &room.key {
                &Some(ref key) => {
                    key.typ == KeyType::Boss
                },
                &None => false
            }

        }).unwrap()

    }

    fn get_boss_door_path(&self) -> RoomPath {

        // Get path from entrance to boss room
        self.visit_rooms(self.entrance_room.unwrap(), |room, path| {
            match &room.enemy_set {
                &Some(ref enemy_set) => {
                    *enemy_set == EnemySet::Boss
                },
                &None => false
            }

        }).unwrap()

    }

    /*
    fn place_keys(&mut self, rng: &mut StdRng) {

        // TODO later allow the entrace to have pots and a key in them :)
        let mut empty_rooms = self.find_empty_rooms();
        rng.shuffle(&mut empty_rooms);

        let key_count = (empty_rooms.len() / 6) - rng.gen_range(0, 1);
        for offset in &empty_rooms[0..key_count] {
            let mut room = self.rooms.get_mut(&offset).unwrap();
            room.key = Some(Key {
                trigger: TriggerType::None,
                typ: KeyType::Small
            });
        }

    }
    */

    fn find_empty_rooms(&self) -> Vec<Offset> {

        let mut empty_rooms: Vec<Offset> = Vec::new();

        for (offset, room) in self.rooms.iter() {
            if room.typ == RoomType::Exit {
                continue;

            } else if let Some(_) = room.enemy_set {
                continue;

            } else if let Some(_) = room.key {
                continue;

            } else {
                empty_rooms.push(*offset);
            }
        }

        // HashMap iteration order is random, but we need the rooms to be
        // deterministic
        empty_rooms.sort_by(|a, b| {
            (a.x + a.y * 1000).cmp(&(b.x + b.y * 1000))
        });

        empty_rooms

    }

    fn find_special_rooms(&mut self, rng: &mut StdRng) -> bool {

        self.calculate_paths();

        // Get the rooms most distant from any intersection
        let mut ends = self.get_end_room_paths();
        ends.sort_by(|a, b| {
            b.len().cmp(&a.len())
        });

        // We need at least 3 ends, otherwise we have to retry
        if ends.len() < 2 {
            println!("Fatal: there must be at least 3 end paths in a dungeon");
            return false;
        }

        // Select the 3 longest ones and shuffle them
        rng.shuffle(&mut ends[0..3]);

        // Set Entrance
        {
            let mut entrance_room = self.rooms.get_mut(&ends[0][0]).unwrap();
            entrance_room.typ = RoomType::Entrance;
            self.entrance_room = Some(entrance_room.offset);
        }

        // Set Exit room
        {
            if ends[1].len() <= 1 {
                println!("Fatal: there must be at least 2 rooms on the exit path, so we can place the boss room infront of it");
                return false;
            }
            let mut exit_room = self.rooms.get_mut(&ends[1][0]).unwrap();
            exit_room.typ = RoomType::Exit;
        }

        // Set Boss Room infront of exit
        {
            // If the boos room has intersections we fail
            let mut boss_room = self.rooms.get_mut(&ends[1][1]).unwrap();
            if boss_room.doors.len() > 2 {
                println!("Fatal: boss room may not be a intersection");
                return false;
            }
            boss_room.enemy_set = Some(EnemySet::Boss);

        }

        // Lock the room to the boss door with a big key
        {
            let mut before_boss_room = self.rooms.get_mut(&ends[1][2]).unwrap();
            let door = before_boss_room.get_door_to_offset_mut(&ends[1][1]).unwrap();
            door.trigger = TriggerType::BossKey;
        }

        // Set Boss Key Room
        {
            let mut boss_key_room = self.rooms.get_mut(&ends[2][0]).unwrap();
            boss_key_room.key = Some(Key {
                trigger: TriggerType::Chest,
                typ: KeyType::Boss
            });
        }

        true

    }

    fn calculate_paths(&mut self) {

        // Set room types
        for (_, room) in self.rooms.iter_mut() {
            room.typ = match room.doors.len() {
                1 => RoomType::End,
                2 => RoomType::Hallway,
                3 => RoomType::Intersection,
                4 => RoomType::Crossing,
                _ => RoomType::Invalid
            };
        }

        // Collect all end rooms
        self.end_rooms.clear();
        for (offset, room) in self.rooms.iter_mut() {
            if room.typ == RoomType::End {
                self.end_rooms.push(*offset);
            }
        }

        // HashMap iteration order is random, but we need the rooms to be
        // deterministic
        self.end_rooms.sort_by(|a, b| {
            (a.x + a.y * 1000).cmp(&(b.x + b.y * 1000))
        });

    }

    fn get_end_room_paths(&self) -> Vec<RoomPath> {

        let mut paths: Vec<RoomPath> = Vec::new();
        for offset in self.end_rooms.iter() {

            // Collect paths from all end rooms to the first intersection
            let path = self.visit_rooms(*offset, |room, path| {
                // TODO this does not handle cases where there are no intersections
                // but just a linear dungeon hallway
                if room.typ == RoomType::Intersection || room.typ == RoomType::Crossing {
                    true

                } else {
                    false
                }
            });

            match path {
                Some(p) => {
                    paths.push(p);
                },
                None => {}
            };

        }

        paths

    }

    fn visit_rooms<F>(
        &self, start: Offset, callback: F

    ) -> Option<RoomPath> where F : Fn(&Room, &RoomPath) -> bool {

        let mut visited: HashMap<Offset, bool> = HashMap::new();
        let mut to_visit: Vec<(Offset, RoomPath)> = vec![(start, vec![start])];
        while to_visit.len() > 0 {

            // Add current room to visited list
            let (offset, path) = to_visit.remove(0);
            let room = self.rooms.get(&offset).unwrap();
            visited.insert(offset, true);

            // Invoke callback and return the path if it returns true
            if callback(&room, &path) == true {
                return Some(path);
            }

            // Add all connected rooms to the to_visit list
            for d in room.doors.iter() {
                if visited.contains_key(&d.to) == false  {
                    let mut to_path = path.clone();
                    to_path.push(d.to);
                    to_visit.push((d.to, to_path));
                    visited.insert(d.to, true);
                }
            }

        }

        None

    }

    fn create_rooms(&mut self, rng: &mut StdRng, max_rooms: usize) {
        let max_corridor_length = 2;
        let mut next_dir = Direction::from_i32(rng.gen_range(0, 4));
        let mut hall_length = rng.gen_range(1, max_corridor_length);
        let mut offset = Offset::default();
        let mut room_stack: RoomPath = Vec::new();

        // Try to generate the requested number of rooms
        let mut index = 0;
        while index < max_rooms {

            // Drop a random number of rooms from the stack and continue generating
            let rooms_to_drop = rng.gen_range(0, 1 + room_stack.len() / 2);
            if rooms_to_drop > 0 {

                for _ in 0..rooms_to_drop - 1 {
                    room_stack.pop();
                }
                offset = *room_stack.last().unwrap();
                next_dir = Direction::from_i32(rng.gen_range(0, 4));
                hall_length = rng.gen_range(1, max_corridor_length);

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
                hall_length -= 1;

                // Create new room at current offset
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

                // Check if we run out of hallway length
                // or should otherwise change the direction
                if hall_length == 0 || variance < 100 {
                    next_dir = Direction::from_i32(rng.gen_range(0, 4));
                    hall_length = rng.gen_range(1, max_corridor_length);
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

    pub fn draw_connection(&mut self, x: i32, y: i32, d: Direction, m: char) {

        let x = (x - self.ox) as usize;
        let y = (y - self.oy) as usize;
        let sx = self.sx;
        let sy = self.sy;

        match d {
            Direction::North => {
                self.buffer[y * sy * self.width * self.sx + x * sx + 8] = '\u{2580}';
                self.buffer[(y * sy - 1) * self.width * self.sx + x * sx + 8] = '\u{2588}';
                self.buffer[(y * sy + 1) * self.width * self.sx + x * sx + 8] = m;
            },
            Direction::East => {
                self.buffer[(y * sy + 3) * self.width * self.sx + x * sx + sx - 4] = m;
                self.buffer[(y * sy + 3) * self.width * self.sx + x * sx + sx - 3] = '\u{2590}';
                self.buffer[(y * sy + 3) * self.width * self.sx + x * sx + sx - 2] = '\u{2588}';
            },
            Direction::South => {
                self.buffer[(y * sy + sy - 2) * self.width * self.sx + x * sx + 8] = '\u{2584}';
                self.buffer[(y * sy + sy - 3) * self.width * self.sx + x * sx + 8] = m;
            },
            Direction::West => {
                self.buffer[(y * sy + 3) * self.width * self.sx + x * sx + 1] = m;
                self.buffer[(y * sy + 3) * self.width * self.sx + x * sx] = '\u{258C}';
                self.buffer[(y * sy + 3) * self.width * self.sx + x * sx - 1] = '\u{2588}';
            },
            _ => {}
        }

    }

    pub fn draw_text(&mut self, x: i32, y: i32, ox: usize, oy: usize, text: &str) {

        let x = (x - self.ox) as usize;
        let y = (y - self.oy) as usize;
        let sx = self.sx;
        let sy = self.sy;

        for (index, t) in text.chars().enumerate() {
            self.buffer[(y * sy + oy) * self.width * self.sx + x * sx + ox + index] = t;
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

    let seed: &[_] = &[1, 2, 3, 8];
    //let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    let mut tries = 0;
    while tries < 10 {
        let mut dungeon = Dungeon::new();
        if dungeon.generate(&mut rng, 19) == false {
            tries += 1;
            println!("---- Failed to generate dungeon on try #{}---- ", tries);
            continue;

        } else {
            dungeon.print();
            break;
        }
    }

}



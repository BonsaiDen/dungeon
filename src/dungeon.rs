extern crate rand;
use std::collections::HashMap;
use self::rand::{Rng, SeedableRng, StdRng};

use room;
use key;
use base::{Side, Offset};
use trigger::Trigger;
use enemy::Enemy;

pub struct Dungeon {
    entrance_room: Option<Offset>,
    boss_room: Option<Offset>,
    exit_room: Option<Offset>,
    end_rooms: room::Path,
    pub rooms: HashMap<Offset, room::Room>
}

impl Dungeon {

    pub fn from_seed(seed: &[usize], room_count: usize, max_tries: usize) -> Option<Dungeon> {

        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut tries = 0;

        while tries < max_tries {

            tries += 1;

            let mut dungeon = Dungeon::new();
            if dungeon.generate(&mut rng, room_count) == false {
                println!("Failed to generate dungeon on try #{}", tries);
                continue;

            } else {
                println!("Successfully generated dungeon on try #{} !", tries);
                return Some(dungeon);
            }

        }

        None

    }

    fn new() -> Dungeon {
        Dungeon {
            entrance_room: None,
            exit_room: None,
            boss_room: None,
            end_rooms: Vec::new(),
            rooms: HashMap::new()
        }
    }

    fn generate(&mut self, rng: &mut StdRng, max_rooms: usize) -> bool {

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
        for _ in 0..shared_boss_path.len() - 2 {
            boss_key_path.remove(0);
            boss_door_path.remove(0);
        }

        // Get total number of locked doors to place
        let empty_room_count = self.find_empty_rooms().len();
        let door_count = empty_room_count / 4 + rng.gen_range(0, 1);

        fn to_connected_path(path: room::Path) -> Vec<(Offset, Offset)> {
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
                    let door_index = rng.gen_range(0, 255) % path.len();

                    // Place locked door between the selected room and the one
                    // that comes after it on the path
                    let room = self.rooms.get_mut(&path[door_index].0).unwrap();
                    let door = room.get_door_to_offset_mut(&path[door_index].1).unwrap();

                    // Do not use the same door twice
                    if door.trigger != Trigger::None {
                        return false;
                    }

                    // Set trigger and lock the door
                    door.trigger = Trigger::SmallKey;
                    doors_on_path[path_index % path_count] += 1;
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

                path.len() <= 1

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

    fn get_boss_key_path(&self) -> room::Path {

        // Get path from entrance to boss key
        self.visit_rooms(self.entrance_room.unwrap(), |room, path| {
            match &room.key {
                &Some(ref key) => {
                    key.typ == key::Type::Boss
                },
                &None => false
            }

        }).unwrap()

    }

    fn get_boss_door_path(&self) -> room::Path {

        // Get path from entrance to boss room
        self.visit_rooms(self.entrance_room.unwrap(), |room, path| {
            match &room.enemy {
                &Some(ref enemy) => {
                    *enemy == Enemy::Boss
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
                trigger: Trigger::None,
                typ: KeyType::Small
            });
        }

    }
    */

    fn find_empty_rooms(&self) -> Vec<Offset> {

        let mut empty_rooms: Vec<Offset> = Vec::new();

        for (offset, room) in self.rooms.iter() {
            if room.typ == room::Type::Exit {
                continue;

            } else if let Some(_) = room.enemy {
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
            entrance_room.typ = room::Type::Entrance;
            self.entrance_room = Some(entrance_room.offset);
        }

        // Set Exit room
        {
            if ends[1].len() <= 1 {
                println!("Fatal: there must be at least 2 rooms on the exit path, so we can place the boss room infront of it");
                return false;
            }
            let mut exit_room = self.rooms.get_mut(&ends[1][0]).unwrap();
            exit_room.typ = room::Type::Exit;
        }

        // Set Boss Room infront of exit
        {
            // If the boos room has intersections we fail
            let mut boss_room = self.rooms.get_mut(&ends[1][1]).unwrap();
            if boss_room.doors.len() > 2 {
                println!("Fatal: boss room may not be a intersection");
                return false;
            }
            boss_room.enemy = Some(Enemy::Boss);

        }

        // Lock the room to the boss door with a big key
        {
            let mut before_boss_room = self.rooms.get_mut(&ends[1][2]).unwrap();
            let door = before_boss_room.get_door_to_offset_mut(&ends[1][1]).unwrap();
            door.trigger = Trigger::BossKey;
        }

        // Set Boss Key Room
        {
            let mut boss_key_room = self.rooms.get_mut(&ends[2][0]).unwrap();
            boss_key_room.key = Some(key::Key {
                trigger: Trigger::Chest,
                typ: key::Type::Boss
            });
        }

        true

    }

    fn calculate_paths(&mut self) {

        // Set room types
        for (_, room) in self.rooms.iter_mut() {
            room.typ = match room.doors.len() {
                1 => room::Type::End,
                2 => room::Type::Hallway,
                3 => room::Type::Intersection,
                4 => room::Type::Crossing,
                _ => room::Type::Invalid
            };
        }

        // Collect all end rooms
        self.end_rooms.clear();
        for (offset, room) in self.rooms.iter_mut() {
            if room.typ == room::Type::End {
                self.end_rooms.push(*offset);
            }
        }

        // HashMap iteration order is random, but we need the rooms to be
        // deterministic
        self.end_rooms.sort_by(|a, b| {
            (a.x + a.y * 1000).cmp(&(b.x + b.y * 1000))
        });

    }

    fn get_end_room_paths(&self) -> Vec<room::Path> {

        let mut paths: Vec<room::Path> = Vec::new();
        for offset in self.end_rooms.iter() {

            // Collect paths from all end rooms to the first intersection
            let path = self.visit_rooms(*offset, |room, path| {
                // TODO this does not handle cases where there are no intersections
                // but just a linear dungeon hallway
                if room.typ == room::Type::Intersection || room.typ == room::Type::Crossing {
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

    ) -> Option<room::Path> where F : Fn(&room::Room, &room::Path) -> bool {

        let mut visited: HashMap<Offset, bool> = HashMap::new();
        let mut to_visit: Vec<(Offset, room::Path)> = vec![(start, vec![start])];
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
        let mut next_dir = Side::from_i32(rng.gen_range(0, 4));
        let mut hall_length = rng.gen_range(1, max_corridor_length);
        let mut offset = Offset::default();
        let mut room_stack: room::Path = Vec::new();

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
                next_dir = Side::from_i32(rng.gen_range(0, 4));
                hall_length = rng.gen_range(1, max_corridor_length);

            }

            // Create the next room
            while index < max_rooms {

                // Check if there is already a room at the next location
                let mut next_offset = offset + next_dir.to_offset();
                if self.rooms.contains_key(&next_offset) {

                    next_dir = Side::None;

                    // Try all possible sides to find a free adjacend location
                    let mut sides = Side::get_all();
                    rng.shuffle(&mut sides);
                    for d in sides.iter() {
                        next_offset = offset + d.to_offset();
                        if self.rooms.contains_key(&next_offset) == false{
                            next_dir = *d;
                            break;
                        }
                    }

                    // No free adjacent space was found, break out and continue from another
                    // position in the stack
                    if next_dir == Side::None {
                        break;

                    // Found a free direction, continue there
                    } else {
                        next_offset = offset + next_dir.to_offset();
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
                let mut room = room::Room::new(offset.x, offset.y);

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
                    next_dir = Side::from_i32(rng.gen_range(0, 4));
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


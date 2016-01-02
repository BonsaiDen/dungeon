use std::cmp;
use std::collections::HashMap;
use rand::{Rng, SeedableRng, StdRng};

use base::{Side, Offset};
use room::{Room, Path as RoomPath, Type as RoomType};
use room::door::{Door, Lock as DoorLock};
use entity::chest::Chest;
use entity::item::{Item, Key};
use entity::enemy::{Enemy, Type as EnemyType};
use entity::switch::Switch;
use entity::trigger::Trigger;

pub struct Dungeon {
    entrance_room: Option<Offset>,
    boss_room: Option<Offset>,
    exit_room: Option<Offset>,
    pub rooms: HashMap<Offset, Room>
}

impl Dungeon {

    // Statics ----------------------------------------------------------------

    pub fn from_seed(
        seed: &[usize], room_count: usize, max_tries: usize

    ) -> Option<Dungeon> {

        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut tries = 0;

        while tries < max_tries {

            tries += 1;

            let mut dungeon = Dungeon {
                entrance_room: None,
                exit_room: None,
                boss_room: None,
                rooms: HashMap::new()
            };

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


    // Generation Methods -----------------------------------------------------

    fn generate(&mut self, rng: &mut StdRng, max_rooms: usize) -> bool {

        self.create_rooms(rng, max_rooms);

        if self.set_special_rooms(rng) == false {
            return false;

        } else if self.set_locked_doors(rng) == false {
            return false;

        } else if self.set_locked_keys(rng) == false {
            return false;

        } else {
            // TODO place compass / map in chests
            // TODO place enemies in rooms
            // TODO place other door locks
            // TODO Create some additional shortcuts between rooms (both rooms need to reach each
            // other without requiring small keys)
            true
        }

    }

    fn create_rooms(&mut self, rng: &mut StdRng, max_rooms: usize) {

        let max_corridor_length = 2;
        let mut next_dir = Side::from_i32(rng.gen_range(0, 4));
        let mut hall_length = rng.gen_range(1, max_corridor_length);
        let mut offset = Offset::default();
        let mut room_stack = RoomPath::new();
        let mut rooms: HashMap<Offset, Room> = HashMap::new();

        // Try to generate the requested number of rooms
        let mut index = 0;
        while index < max_rooms {

            // Drop a random number of rooms from the stack and continue
            // generating from the top most room
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

                // Check if there is already a room if we go to the current
                // side
                let mut next_offset = offset + next_dir.to_offset();
                if rooms.contains_key(&next_offset) {

                    next_dir = Side::None;

                    // Try all possible sides in random order to find a free
                    // adjacent location
                    let mut sides = Side::all();
                    rng.shuffle(&mut sides);

                    for d in sides.iter() {
                        next_offset = offset + d.to_offset();
                        if rooms.contains_key(&next_offset) == false{
                            next_dir = d.clone();
                            break;
                        }
                    }

                    // No free adjacent space was found, break out and continue
                    // from a previous room position in the stack
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

                // Create new room at current offset and connect it
                // with the previous room
                let mut room = Room::new(offset.x, offset.y);
                match room_stack.last() {
                    Some(offset) => {
                        let other = rooms.get_mut(&offset).unwrap();
                        room.add_door_to(&other);
                        other.add_door_to(&room);
                    },
                    None => {}
                }

                rooms.insert(offset, room);
                room_stack.push(offset);
                index += 1;

                // Check if we either run out of hallway length
                // or whether we should otherwise change the direction
                if hall_length == 0 || variance < 100 {
                    next_dir = Side::from_i32(rng.gen_range(0, 4));
                    hall_length = rng.gen_range(1, max_corridor_length);
                }

            }

            // Stack is already empty, so we cannot actually try any
            // other positions and need to exit
            if room_stack.len() == 0 {
                break;
            }

        }

        // TODO fail if we couldn't generate the desired number of rooms?

        // Set room connection types
        for (_, room) in rooms.iter_mut() {
            room.typ = match room.doors.len() {
                1 => RoomType::End,
                2 => RoomType::Hallway,
                3 => RoomType::Intersection,
                4 => RoomType::Crossing,
                _ => RoomType::Invalid
            };
        }

        // Calculate bounds
        let mut min = Offset { x: 9999, y: 9999 };
        for (offset, _) in rooms.iter() {
            min.x = cmp::min(offset.x, min.x);
            min.y = cmp::min(offset.y, min.y);
        }

        // Translate all rooms so 0,0 is the top left border of the dungeon
        for (_, mut room) in rooms.into_iter() {

            // Update all door offsets
            for d in room.doors.iter_mut() {
                d.to = d.to - min;
            }

            // Translate offset and insert into dungeon room map
            room.offset = room.offset - min;
            self.rooms.insert(room.offset, room);

        }

    }

    fn set_special_rooms(&mut self, rng: &mut StdRng) -> bool {

        // Get the rooms most distant from any intersection
        let mut ends = self.end_room_paths();
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
            self.exit_room = Some(exit_room.offset);
        }

        // Set Boss Room infront of exit
        {
            // If the boos room has intersections we fail
            let mut boss_room = self.rooms.get_mut(&ends[1][1]).unwrap();
            if boss_room.doors.len() > 2 {
                println!("Fatal: boss room may not be a intersection");
                return false;
            }

            let exit_offset = self.exit_room.unwrap();

            // Create Boss
            boss_room.enemy = Some(Enemy {
                typ: EnemyType::Boss,
                triggers: vec![
                    // Open door to exit once boss is defeated
                    Trigger::OpenDoor(exit_offset)
                ]
            });

            // Setup both doors
            for door in boss_room.doors.iter_mut() {

                // Lock door to exit
                if door.to == exit_offset {
                    door.lock = DoorLock::Trigger;

                // Lock entry door upon entrance
                } else {
                    door.lock = DoorLock::Trigger;
                    door.triggers.push(Trigger::LockDoor(door.to));
                }

            }

            self.boss_room = Some(boss_room.offset);

        }

        // Lock the room to the boss door with a big key
        {
            let mut before_boss_room = self.rooms.get_mut(&ends[1][2]).unwrap();
            let door = before_boss_room.get_door_to_offset_mut(&ends[1][1]).unwrap();
            door.lock = DoorLock::BossKey;
        }

        // Set Boss Key Room
        {
            let mut boss_key_room = self.rooms.get_mut(&ends[2][0]).unwrap();
            boss_key_room.chest = Some(Chest {
                item: Item::Key(Key::Boss)
            });
        }

        true

    }

    fn set_locked_doors(&mut self, rng: &mut StdRng) -> bool {

        // Get path from entrance to boss key
        let mut boss_key_path = self.boss_key_path();

        // Get path from entrance to boss door
        let mut boss_door_path = self.boss_door_path();
        boss_door_path.pop(); // Don't override the boss door

        // Extract shared boss path
        let mut shared_boss_path = RoomPath::new();
        for (offset, _) in boss_key_path.iter().zip(
            boss_door_path.iter()

        ).filter(|&(a, b)| {
            a == b

        }) {
            shared_boss_path.push(*offset);
        }

        // Drop shared path elements
        for _ in 0..shared_boss_path.len() - 2 {
            boss_key_path.remove(0);
            boss_door_path.remove(0);
        }

        // Get total number of locked doors to place
        let empty_room_count = self.empty_rooms().len();
        let door_count = empty_room_count / 4 + rng.gen_range(0, 1);

        // Randomize paths
        let mut paths = vec![
            shared_boss_path.into_connected_path(),
            boss_key_path.into_connected_path(),
            boss_door_path.into_connected_path()
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

                let index = path_index % path_count;
                let mut path = &mut paths[index];

                // Get the all empty rooms from the current path
                if path.len() > 1 {

                    // If so, place a door on somewhere on the path
                    let door_index = rng.gen_range(0, 255) % path.len();

                    // Place locked door between the selected room and the one
                    // that comes after it on the path
                    let room = self.rooms.get_mut(&path[door_index].0).unwrap();
                    let door = room.get_door_to_offset_mut(&path[door_index].1).unwrap();

                    // Do not use lock the same door twice
                    if door.lock != DoorLock::None {
                        return false;
                    }

                    // Set trigger and lock the door
                    door.lock = DoorLock::SmallKey;
                    doors_on_path[index] += 1;
                    doors_locked += 1;

                    // Remove the used room from the path
                    path.remove(door_index);

                    // Calculate the ratio of path len and door count
                    // we want longer paths to have more doors so we achieve
                    // a more even distribution
                    let door_ratio = path.len() / doors_on_path[index];
                    if door_ratio < 2 {
                        path_index += 1;
                    }

                }

                path.len() <= 1

            };

            // Remove paths once they're empty
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

    fn set_locked_keys(&mut self, rng: &mut StdRng) -> bool {

        // Keep track of the doors that we have unlocked
        let mut unlocked_doors: HashMap<(Offset, Offset), bool> = HashMap::new();
        let mut rooms_with_key: Vec<Offset> = Vec::new();

        loop {

            // Now search through the dungeon startin from the entrance and find
            // all reachable rooms before any locked doors
            let rooms = self.connected_rooms(self.entrance_room.unwrap(), |room, door| {

                // Always stop at the boss door
                if door.lock == DoorLock::BossKey {
                    false

                // See if the door is locked
                } else if door.lock == DoorLock::SmallKey {

                    // See if we already unlocked it, if so we can continue
                    // with the room behind it
                    if unlocked_doors.contains_key(&(room.offset, door.to)) {
                        true

                    // Otherwise we stop here
                    } else {
                        false
                    }

                // For all open doors visit the room behind them
                } else {
                    true
                }

            });

            // Go through all rooms and mark all doors from them as unlocked
            let mut doors_unlocked = 0;
            for offset in rooms.iter() {
                let room = self.rooms.get(&offset).unwrap();
                for d in room.doors.iter() {

                    // Mark all doors with small keys as unlocked
                    if d.lock == DoorLock::SmallKey {
                        if unlocked_doors.contains_key(&(*offset, d.to)) == false {
                            unlocked_doors.insert((*offset, d.to), true);
                            doors_unlocked += 1;
                        }
                    }

                }
            }

            // Get all empty rooms from the set of rooms we found infront of
            // the doors
            let mut empty_rooms: Vec<_> = rooms.iter().cloned().filter(|offset| {
                let room = self.rooms.get(offset).unwrap();
                room.chest.is_none() && room.enemy.is_none() && room.switch.is_none()

            }).collect();

            // Check if we have enough empty rooms to place the required keys in
            if empty_rooms.len() < doors_unlocked {
                println!("Not enough empty rooms to place small keys in");
                return false;
            }

            // 1. Randomize rooms to use for key placement
            rng.shuffle(&mut empty_rooms);

            // 2. Calculate room distance to the next room which contains a key
            let mut key_distances: HashMap<Offset, usize> = HashMap::new();
            for offset in empty_rooms.iter() {

                // Find the distances from the current empty room to all rooms
                // which already contain keys
                let mut min_key_distance = 9999;
                for key in rooms_with_key.iter() {
                    let path = self.find_room_path(*offset, |room, _| {
                        room.offset == *key

                    }).unwrap();
                    min_key_distance = cmp::min(min_key_distance, path.len() - 1);
                }

                key_distances.insert(*offset, min_key_distance);

            }

            // 3. Split between end rooms (not entrance(!)) and all others
            let (mut ends, empty_rooms): (Vec<_>, Vec<_>) = empty_rooms.into_iter().partition(|offset| {
                self.rooms.get(offset).unwrap().typ == RoomType::End
            });

            // 4. Split out rooms which are close to other rooms with keys and
            // move them to the back of the list
            let (mut close_to_keys, mut empty_rooms): (Vec<_>, Vec<_>) = empty_rooms.into_iter().partition(|offset| {
                *key_distances.get(&offset).unwrap() < 2
            });

            // 5. Merge them back together
            empty_rooms.append(&mut close_to_keys);

            // 6. Take the first end room (if any) and put it back
            match ends.pop() {
                Some(offset) => {
                    empty_rooms.insert(0, offset);
                },
                None => {}
            }

            // 7. Now place the keys in the first rooms from the list
            let mut key_triggers = vec![
                // TODO use enum
                0,
                1,
                1,
                1,
                2,
                2,
                2,
                2
            ];

            for i in 0..doors_unlocked {
                let mut room = self.rooms.get_mut(empty_rooms.get(i).unwrap()).unwrap();

                // Select a random trigger
                rng.shuffle(&mut key_triggers);

                // TODO create trigger sets
                match key_triggers[0] {
                    0 => {
                        room.chest = Some(Chest {
                            item: Item::Key(Key::Small)
                        })
                    },
                    1 => {
                        room.switch = Some(Switch {
                            // TODO chance of either dropping key or spawning chest with key
                            triggers: vec![Trigger::Item(Item::Key(Key::Small))]
                        })
                    },
                    2 => {
                        // TODO random chance for locking the door behind on entry and unlocking
                        // once enemies are defeated
                        room.enemy = Some(Enemy {
                            typ: EnemyType::Small,
                            // TODO random chance for big enemies which stay defeated?
                            // TODO chance of either dropping key or spawning chest with key
                            triggers: vec![Trigger::Item(Item::Key(Key::Small))]
                        })
                    },
                    _ => unreachable!()
                }

                rooms_with_key.push(room.offset);

            }

            // If we didn't find anymore doors to unlock we're done
            if doors_unlocked == 0 {
                break;
            }

        }

        true

    }


    // Room collection methods ------------------------------------------------

    fn connected_rooms<F>(
        &self, start: Offset, callback: F

    ) -> Vec<Offset> where F : Fn(&Room, &Door) -> bool {

        let mut visited: HashMap<Offset, bool> = HashMap::new();
        let mut to_visit: Vec<Offset> = vec![start];

        let mut rooms = Vec::new();
        while to_visit.len() > 0 {

            // Get next room to visit
            let offset = to_visit.remove(0);
            let room = self.rooms.get(&offset).unwrap();

            // Add current room to visited list
            visited.insert(offset, true);
            rooms.push(offset);

            // Add all connected rooms to the to_visit list
            for d in room.doors.iter() {
                if visited.contains_key(&d.to) == false  {

                    // Invoke callback and add the room behind the door
                    if callback(&room, &d) == true {
                        to_visit.push(d.to);
                        visited.insert(d.to, true);
                    }

                }
            }

        }

        rooms

    }

    fn empty_rooms(&self) -> Vec<Offset> {

        let mut empty_rooms: Vec<Offset> = Vec::new();

        for (offset, room) in self.rooms.iter() {
            if room.typ == RoomType::Exit {
                continue;

            } else if let Some(_) = room.enemy {
                continue;

            } else if let Some(_) = room.chest {
                continue;

            } else if let Some(_) = room.switch {
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


    // Path related methods ---------------------------------------------------

    fn find_room_path<F>(
        &self, start: Offset, callback: F

    ) -> Option<RoomPath> where F : Fn(&Room, &RoomPath) -> bool {

        let mut to_path = RoomPath::new();
        to_path.push(start);

        let mut visited: HashMap<Offset, bool> = HashMap::new();
        let mut to_visit: Vec<(Offset, RoomPath)> = vec![(start, to_path)];
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

    fn boss_key_path(&self) -> RoomPath {

        // Get path from entrance to boss key
        self.find_room_path(self.entrance_room.unwrap(), |room, _| {
            if let Some(ref chest) = room.chest {
                match &chest.item {
                    &Item::Key(ref key_typ) => {
                        *key_typ == Key::Boss
                    },
                    _ => false
                }

            } else {
                false
            }

        }).unwrap()

    }

    fn boss_door_path(&self) -> RoomPath {

        // Get path from entrance to boss room
        self.find_room_path(self.entrance_room.unwrap(), |room, _| {
            if let Some(ref enemy) = room.enemy {
                enemy.typ == EnemyType::Boss

            } else {
                false
            }

        }).unwrap()

    }

    fn end_room_paths(&mut self) -> Vec<RoomPath> {

        // Collect all end rooms
        let mut end_rooms = Vec::new();
        for (offset, room) in self.rooms.iter_mut() {
            if room.typ == RoomType::End {
                end_rooms.push(*offset);
            }
        }

        // HashMap iteration order is random, but we need the rooms to be
        // deterministic
        end_rooms.sort_by(|a, b| {
            (a.x + a.y * 1000).cmp(&(b.x + b.y * 1000))
        });

        let mut paths: Vec<RoomPath> = Vec::new();
        for offset in end_rooms.iter() {

            // Collect paths from all end rooms to the first intersection
            let path = self.find_room_path(*offset, |room, _| {

                // TODO this does not handle cases where there are no
                // intersections but just one linear dungeon hallway
                room.typ == RoomType::Intersection || room.typ == RoomType::Crossing
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

}


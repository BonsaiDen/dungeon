extern crate dungeon;
mod debug;

fn main() {

    let seed: &[_] = &[1, 2, 3, 8];
    let dungeon = dungeon::Dungeon::from_seed(seed, 19, 10);

    match dungeon {
        Some(dungeon) => {
            match debug::Renderer::from_dungeon(&dungeon) {
                Some(r) => {
                    r.draw();
                },
                None => {}
            }
        },
        None => {}
    }

}


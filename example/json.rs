mod file;

use serde::{ Serialize, Deserialize };

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NPC {
    name: String,
    position: Vec2,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Vec2 {
    x: f32,
    y: f32,
}

fn main() {
    let datas = file::read_files("npc", "json").unwrap();

    let mut npcs: Vec<NPC> = Vec::new();

    for data in datas {
        match serde_json::from_str::<NPC>(data.as_str()) {
            Ok(npc) => {
                npcs.push(npc);
            }
            Err(_) => (),
        }
    }

    for npc in npcs {
        println!("{:?}", npc);
    }
}
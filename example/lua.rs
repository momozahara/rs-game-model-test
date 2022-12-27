mod file;

use std::sync::Mutex;

static NPCS: Mutex<Option<Vec<NPC>>> = Mutex::new(None);

use rlua::{ Lua, Table };

#[derive(Debug)]
struct NPC {
    name: String,
    position: Vec2,
}

#[derive(Debug)]
struct Vec2 {
    x: f32,
    y: f32,
}

fn main() {
    let mut i = NPCS.lock().unwrap();
    *i = Some(Vec::new());
    drop(i);

    let datas = file::read_files("npc", "lua").unwrap();

    let lua = Lua::new();

    lua.context(|lua_ctx| {
        let global = lua_ctx.globals();

        let _push_npc = lua_ctx
            .create_function(|_, npc_table: Table| {
                let mut i = NPCS.lock().unwrap();
                let n = i.as_mut().unwrap();

                let mut npc = NPC { name: "".to_owned(), position: Vec2 { x: 0.0, y: 0.0 } };

                npc.name = npc_table.get::<_, String>("name").unwrap_or(npc.name);
                npc.position = match npc_table.get::<_, Table>("position") {
                    Ok(r) =>
                        Vec2 {
                            x: r.get::<_, f32>("x").unwrap_or(npc.position.x),
                            y: r.get::<_, f32>("y").unwrap_or(npc.position.y),
                        },
                    Err(_) => npc.position,
                };

                n.push(npc);

                Ok(())
            })
            .unwrap();
        global.set("push_npc", _push_npc).unwrap();

        for data in datas {
            lua_ctx.load(&data).exec().unwrap();
        }
    });

    i = NPCS.lock().unwrap();
    let n = i.as_mut().unwrap();
    for npc in n {
        println!("{:?}", npc);
    }
}
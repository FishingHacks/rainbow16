use rlua::{Lua, StdLib, Error, Value};
use crate::{c_singleton, Singleton, get_s_val, luautils::{init_ctx, run_function_if_function}, set_s_val, luastd::setup_stdlib, TIME, overlay::overlay::set_overlay};

pub struct GameState {
    code: Vec<String>,
    lua: Option<Lua>,
    filename: Option<String>,
    image_vec: Vec<u8>
}

impl GameState {
    fn new(code: String, filename: Option<String>) -> Self {
        let mut vec: Vec<String> = Vec::new();

        for l in code.split('\n') {
            vec.push(l.to_string());
        }

        let mut image_vec: Vec<u8> = Vec::new();

        for _ in 0..16384 { // prepare the vector for the 128x128 image made up of 16x16 sprites
            image_vec.push(0);
        }

        image_vec[258] = 12; // x: 2 y: 2
        image_vec[261] = 12; // x: 5 y: 2
        image_vec[642] = 12; // x: 2 y: 5
        image_vec[645] = 12; // x: 5 y: 5
        image_vec[516] = 12; // x: 4 y: 4
        image_vec[388] = 12; // x: 4 y: 3
        image_vec[387] = 12; // x: 3 y: 3
        image_vec[515] = 12; // x: 3 y: 4

        GameState {
            code: vec,
            lua: None,
            filename,
            image_vec,
        }
    }

    fn run_game(&mut self) -> Option<Error> {
        set_s_val!(TIME, 0);
        let (res, lua) = init_ctx(StdLib::BASE, |ctx| {
            setup_stdlib(ctx)?;
    
            ctx.load::<String>(&self.code.join("\n")).exec()?;
            
            if let Some(err) = run_function_if_function(ctx.globals().get("_init").ok(), ctx) {
                Err(err)
            } else {
                Ok(())
            }
        });

        if res.is_ok() && lua.context(|ctx| {
            let g = ctx.globals();
            if g.get::<&str, Value>("_draw").is_err() && g.get::<&str, Value>("_update").is_err() {
                Err(())
            } else {
                Ok(())
            }
        }).is_ok() {
            self.lua = Some(lua);
        }    
        res.err()
    }
}

pub fn get_image_vec() -> &'static mut Vec<u8> {
    &mut get_s_val!(GAME_STATE).image_vec
}

pub fn run_game() -> Option<Error> {
    get_s_val!(GAME_STATE).run_game()
}

pub fn get_path() -> Option<String> {
    get_s_val!(GAME_STATE).filename.clone()
}

pub fn get_code() -> Vec<String> {
    get_s_val!(GAME_STATE).code.clone()
}

pub fn set_code(code: &Vec<String>) {
    get_s_val!(GAME_STATE).code.clone_from(code);
}

c_singleton!(GAME_STATE, GameState, ||GameState::new("function _init()\n\nend\n\nfunction _update()\n\nend\n\nfunction _draw()\n\nend\n".to_string(), None));

pub fn game_is_running() -> bool {
    get_s_val!(GAME_STATE).lua.is_some()
}

pub fn load_game(code: String, filename: Option<String>) {
    set_s_val!(GAME_STATE, GameState::new(code, filename));
}

pub fn stop_game() {
    set_s_val!(TIME, 0);
    get_s_val!(GAME_STATE).lua = None;
    set_overlay(crate::overlay::OverlayType::None);
}

pub fn run_fn(fnname: &str) -> Option<Error> {
    if let Some(lua) = &get_s_val!(GAME_STATE).lua {
        lua.context(|ctx| {
            let e = run_function_if_function(ctx.globals().get(fnname).ok(), ctx);
            if let Some(err) = e {
                Err(err)
            } else {
                Ok(())
            }
        })
        .err()
    } else {
        None
    }
}

pub fn draw_game() -> Option<Error> {
    run_fn("_draw")
}

pub fn update_game() -> Option<Error> {
    run_fn("_update")
}
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rand::Rng;
use rlua::Value::Nil;
use rlua::{Context, Error, FromLua, Table, Value, StdLib};

use crate::gamestate::get_audio;
use crate::keyboard::{button_is_down, button_is_pressed, u8_to_button};
use crate::luautils::value_to_string;
use crate::memory::{peek, poke, sfx};
use crate::{canvas_functions::*, luautils::add_fn};
use crate::{get_s_val, RNG, TIME};

pub fn setup_stdlib<'a>(ctx: Context<'a>) -> Result<(), Error> {
    add_fn(ctx, "sleep", |_, ms: u64| {
        thread::sleep(Duration::from_millis(ms));
        Ok(())
    })?;
    add_fn(ctx, "add", |_, (table, value): (Table, Value<'a>)| {
        table.set(table.len()? + 1, value)?;

        Ok(())
    })?;
    add_fn(ctx, "del", |_, (table, idx): (Table, i64)| {
        // let length = table.len()?;

        // let to_pop = length - idx;

        // if to_pop < 0 {
        //     return Ok(());
        // }

        // let mut vec: Vec<Value> = Vec::with_capacity(to_pop as usize);

        // for i in 0..to_pop {
        //     vec.push(table.get(idx + i)?);
        //     table.set(i, Nil)?;
        // }
        // table.set(idx, Nil)?;

        // for i in 0..vec.len() {
        //     table.set(idx + i as i64, vec[i].clone())?;
        // }

        Ok(())
    })?;
    add_fn(ctx, "stop", |_, _: ()| {
        Err::<(), Error>(rlua::Error::external("Exit"))
    })?;
    add_fn(ctx, "peek", |_, address: u32| Ok(peek(address as usize)))?;
    add_fn(ctx, "poke", |_, (address, value): (u32, u8)| {
        poke(address as usize, value);
        Ok(())
    })?;
    add_fn(ctx, "btn", |_, button: u8| {
        Ok(button_is_down(u8_to_button(button)))
    })?;
    add_fn(ctx, "btnp", |_, button: u8| {
        Ok(button_is_pressed(u8_to_button(button)))
    })?;
    add_fn(ctx, "setp", |_, (x, y, color): (i32, i32, u8)| {
        set_pixel(x, y, color);
        Ok(())
    })?;
    add_fn(ctx, "cls", |_, color: Option<u8>| {
        clear(color);
        Ok(())
    })?;
    add_fn(
        ctx,
        "rectfill",
        |_, (x, y, w, h, color): (i32, i32, i32, i32, u8)| {
            rectfill(x, y, w, h, color);
            Ok(())
        },
    )?;
    add_fn(ctx, "cursor", |_, (x, y): (Option<i32>, Option<i32>)| {
        cursor(x, y);
        Ok(())
    })?;
    add_fn(
        ctx,
        "print",
        |_, (val, col, x, y): (Value, Option<u8>, Option<i32>, Option<i32>)| {
            print(value_to_string(val).to_lowercase(), x, y, col);
            Ok(())
        },
    )?;
    add_fn(
        ctx,
        "rect",
        |_, (x, y, w, h, color): (i32, i32, i32, i32, u8)| {
            rect(x, y, w, h, color);
            Ok(())
        },
    )?;
    add_fn(
        ctx,
        "ellipse",
        |_, (cx, cy, rx, ry, color): (i32, i32, i32, i32, u8)| {
            ellipse(cx, cy, rx, ry, color);
            Ok(())
        },
    )?;
    add_fn(ctx, "circle", |_, (cx, cy, r, c): (i32, i32, i32, u8)| {
        circle(cx, cy, r, c);
        Ok(())
    })?;
    add_fn(
        ctx,
        "line",
        |_, (x1, y1, x2, y2, color): (i32, i32, i32, i32, u8)| {
            line(x1, y1, x2, y2, color);
            Ok(())
        },
    )?;
    add_fn(ctx, "camera", |_, (x, y): (Option<i32>, Option<i32>)| {
        camera(x, y);
        Ok(())
    })?;
    add_fn(ctx, "pal", |_, (col1, col2): (Option<u8>, Option<u8>)| {
        pal(col1, col2);
        Ok(())
    })?;
    add_fn(
        ctx,
        "palt",
        |_, (col1, transparency): (Option<u8>, Option<bool>)| {
            palt(col1, transparency);
            Ok(())
        },
    )?;
    add_fn(ctx, "setpal", |_, palette: u8| {
        switch_palette(palette);
        Ok(())
    })?;
    add_fn(
        ctx,
        "sspr",
        |_, (sx, sy, x, y, w, h): (i32, i32, u32, u32, u32, u32)| {
            sspr(sx, sy, x, y, w, h);

            Ok(())
        },
    )?;
    add_fn(ctx, "spr", |_, (idx, x, y): (u32, i32, i32)| {
        spr(idx, x, y);

        Ok(())
    })?;
    add_fn(ctx, "sfx", |_, idx: i32| {
        let mem = get_s_val!(sfx);
        if idx < 32 && idx >= 0 {
            get_audio(idx as usize).write_to_memory(mem, 0);
            mem.set_at_addr(102, 1);
            mem.set_at_addr_u32(
                98,
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u32,
            );
        } else if idx < 0 {
            for i in 0..=102 {
                mem.set_at_addr(i, 0);
            }
        }

        Ok(())
    })?;

    add_fn(ctx, "rnd", |_, value: Option<RndArgument>| {
        if let Some(v) = value {
            let val = match v {
                RndArgument::None => Value::Nil,
                RndArgument::Number(num) => Value::Number(get_s_val!(RNG).gen_range(0.0f64..=num)),
                RndArgument::Table(table) => {
                    table[get_s_val!(RNG).gen_range(0..table.len())].clone()
                }
            };
            Ok(val)
        } else {
            Ok(Value::Number(get_s_val!(RNG).gen_range(0.0f64..=1.0f64)))
        }
    })?;
    add_fn(ctx, "time", |_, _: ()| Ok(*get_s_val!(TIME)))?;
    add_fn(ctx, "cos", |_, num: f64| Ok(num.cos()))?;
    add_fn(ctx, "sin", |_, num: f64| Ok(num.sin()))?;
    add_fn(ctx, "sqrt", |_, num: f64| Ok(num.sqrt()))?;
    add_fn(ctx, "flr", |_, num: f64| Ok(num.floor()))?;

    Ok(())
}

enum RndArgument<'a> {
    Number(f64),
    Table(Vec<Value<'a>>),
    None,
}

impl<'a> FromLua<'a> for RndArgument<'a> {
    fn from_lua(lua_value: Value<'a>, lua: Context<'a>) -> rlua::Result<Self> {
        match lua_value.type_name() {
            "integer" | "number" => {
                let value = f64::from_lua(lua_value, lua);
                if let Ok(v) = value {
                    Ok(RndArgument::Number(v))
                } else {
                    Ok(RndArgument::None)
                }
            }
            "table" => {
                let try_table = Table::from_lua(lua_value, lua);

                if let Ok(table) = try_table {
                    let len = table.len().unwrap_or(0);
                    let mut vec = <Vec<Value<'a>>>::new();
                    for i in 1..=len {
                        let val = table.get::<i64, Value<'a>>(i);
                        if let Ok(v) = val {
                            vec.push(v);
                        } else {
                            vec.push(Value::Nil);
                        }
                    }
                    Ok(RndArgument::Table(vec))
                } else {
                    Ok(RndArgument::None)
                }
            }
            _ => Ok(RndArgument::None),
        }
    }
}

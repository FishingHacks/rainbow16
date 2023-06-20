use rlua::{Context, Error, FromLuaMulti, ToLuaMulti};
use rlua::{FromLua, Function, Lua, MultiValue, Result as LuaResult, StdLib, Value};

pub fn print_err(err: Error) -> String {
    let msg: String = match err {
        Error::BindError => "too many arguments to function::bind".to_string(),
        Error::CallbackDestructed => {
            "The function or variable you tried to call/access does no longer exist".to_string()
        }
        Error::CallbackError { traceback, cause } => {
            if cause.to_string() == "Exit" {
                "".to_string()
            } else {
                format!("Error: {}\n\n{}", cause, traceback)
            }
        }
        Error::CoroutineInactive => {
            "you tried to call an inactive coroutine. use costatus to check its status.".to_string()
        }
        Error::ExternalError(e) => format!("External Error: {:?}", e),
        Error::FromLuaConversionError { from, to, message } => format!(
            "Could not convert to rust {} from lua {}: {}",
            from,
            to,
            message.unwrap_or(String::default())
        ),
        Error::MemoryError(e) => format!("Out of memory: {}", e),
        Error::MismatchedRegistryKey => {
            "A registry key from a different state was used!".to_string()
        }
        Error::RecursiveMutCallback
        | Error::UserDataBorrowError
        | Error::UserDataBorrowMutError => "rust magic fucked up lol".to_string(),
        Error::RuntimeError(e) => format!("Runtime Error: {}", e),
        Error::StackError => "Ran out of stack space (dafuq)".to_string(),
        Error::SyntaxError {
            message,
            incomplete_input,
        } => {
            if incomplete_input {
                format!("an instruction is incomplete: {}", message)
            } else {
                format!("syntax error: {}", message)
            }
        }
        Error::ToLuaConversionError { from, to, message } => {
            let v = format!(
                "Could not convert to lua {} from rust {}: {}",
                from,
                to,
                message.unwrap_or(String::default())
            );
            v
        }
        Error::UserDataTypeMismatch => "idk know what happened but an error occurred".to_string(),
    };

    eprintln!("{msg}");
    return format!("{msg}");
}

pub fn add_fn<'a, A, R, F>(ctx: Context<'a>, name: &str, func: F) -> Result<(), Error>
where
    A: FromLuaMulti<'a>,
    R: ToLuaMulti<'a>,
    F: 'static + Send + Fn(Context<'a>, A) -> LuaResult<R>,
{
    let fnc = ctx.create_function(func)?;
    ctx.globals().set(name, fnc)?;

    Ok(())
}

pub fn run_function_if_function<'a>(value: Option<Value<'a>>, ctx: Context<'a>) -> Option<Error> {
    value.and_then(|f| {
        if f.type_name() == "function" {
            return Function::from_lua(f, ctx)
                .ok()
                .and_then(|f| f.call::<(), MultiValue>(()).err());
        }

        None
    })
}

pub fn init_ctx<F, R>(lua_mod: StdLib, f: F) -> (R, Lua)
where
    F: FnOnce(Context) -> R,
{
    let lua = Lua::new_with(lua_mod);
    return (lua.context(f), lua);
}

pub fn value_to_string(value: Value) -> String {
    match value {
        Value::Boolean(v) => v.to_string(),
        Value::Error(e) => e.to_string(),
        Value::Function(f) => format!("[function {:?}]", f),
        Value::Integer(i) => i.to_string(),
        Value::Nil => "[nil]".to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(str) => str.to_str().unwrap_or("").to_string(),
        Value::Thread(t) => format!("[thread {:?}]", t),
        Value::UserData(d) => format!("{:?}", d),
        Value::LightUserData(d) => format!("[ptr {:#x}]", d.0 as u32),
        Value::Table(..) => "{..}".to_string(),
        #[allow(unreachable_patterns)]
        _ => "[unknown]".to_string(),
    }
}

use crate::{
    gamestate::game_is_running, overlay::message::set_message, screenshot_saver::screenshot, system::Keycode,
};

pub fn game_handle_keydown(code: Keycode) {
    if !game_is_running() {
        return;
    }
    if code == Keycode::F2 {
        screenshot();
        set_message("screenshot saved");
    }
}

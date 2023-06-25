use std::fmt::Debug;

use crate::{
    audio::Audio,
    gamestate::GameState,
    image::parse_image,
    utils::{__from_hex, __to_hex},
};

#[derive(Clone, Copy, PartialEq, Debug)]
enum HeaderType {
    Script,
    Sfx,
    Images,
    PreviewImage,
    Unknown = 255,
}

impl HeaderType {
    fn from_u8(u8: u8) -> Self {
        match u8 {
            0 => Self::Script,
            1 => Self::Sfx,
            2 => Self::Images,
            3 => Self::PreviewImage,
            0xff => Self::Unknown,
            _ => Self::Script,
        }
    }
}

struct MetaHeader {
    typ: HeaderType,
    data: String,
}

impl Debug for MetaHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "MetaHeader<{:?}>({} bytes data)",
            self.typ,
            self.data.len()
        ))
    }
}

impl MetaHeader {
    fn string(&self) -> String {
        // 0x12: Header Start
        let mut str = "\x12".to_string();
        str.push(self.typ as u8 as char);
        // 4 bytes for the data length
        let dat_len = self.data.len();
        let bytes = unsafe { str.as_mut_vec() };
        bytes.push((dat_len & 0xff) as u8);
        bytes.push(((dat_len >> 8) & 0xff) as u8);
        bytes.push(((dat_len >> 16) & 0xff) as u8);
        bytes.push(((dat_len >> 24) & 0xff) as u8);

        str.push_str(&self.data);

        str
    }

    fn from_string(str: &String, off: usize) -> Option<(Self, usize)> {
        if str.len() < off + 5 {
            return None;
        }
        let bytes = str.as_bytes();
        if bytes[off] != 0x12 {
            return None;
        }
        let mut new = Self::new(HeaderType::Unknown, String::default());

        new.typ = HeaderType::from_u8(bytes[off + 1]);

        let mut sz: u32 = 0;
        sz |= bytes[off + 2] as u32;
        sz |= (bytes[off + 3] as u32) << 8;
        sz |= (bytes[off + 4] as u32) << 16;
        sz |= (bytes[off + 5] as u32) << 24;

        if str.len() < off + 6 + sz as usize {
            return None;
        }

        let mut str = String::with_capacity(sz as usize);

        for i in 0..sz as usize {
            str.push(bytes[off + 6 + i] as char);
        }

        new.data = str;

        Some((new, off + 6 + sz as usize))
    }

    fn new(typ: HeaderType, data: String) -> Self {
        Self { typ, data }
    }
}

pub fn game_data_to_string(data: &GameState) -> String {
    let script_header = MetaHeader::new(HeaderType::Script, data.code.join("\n"));
    let sfx_header = MetaHeader::new(HeaderType::Sfx, data.audios.map(|f| f.to_string()).join(""));
    let mut img_str = String::with_capacity(16384);
    for i in 0..16384usize {
        img_str.push(__to_hex(data.image_vec[i]));
    }
    let spr_header = MetaHeader::new(HeaderType::Images, img_str);

    let mut str = "R16\x10".to_string();

    if let Some(img) = &data.preview_image {
        str.push_str(&MetaHeader::new(HeaderType::PreviewImage, img.stringify_vec()).string());
    }

    str.push_str(&script_header.string());
    str.push_str(&sfx_header.string());
    str.push_str(&spr_header.string());

    str
}

pub fn string_to_game_data(str: String, filename: Option<String>) -> Option<GameState> {
    let mut headers: Vec<MetaHeader> = Vec::new();

    if str.len() < 4 || !str.starts_with("R16\x10") {
        return None;
    }

    let mut offset: usize = 4;
    while offset < str.len() {
        if let Some((header, new_off)) = MetaHeader::from_string(&str, offset) {
            offset = new_off;
            headers.push(header);
        } else {
            offset = str.len();
        }
    }

    let script_header = headers
        .iter()
        .find(|f| f.typ == HeaderType::Script)
        .and_then(|h| Some(h.data.clone()))
        .unwrap_or(String::new());
    let image_header = headers.iter().find(|f| f.typ == HeaderType::Images);
    let sfx_header = headers.iter().find(|f| f.typ == HeaderType::Sfx);
    let prev_img_header = headers.iter().find(|f| f.typ == HeaderType::PreviewImage);

    let mut gamestate = GameState {
        audios: [Audio::new(); 32],
        code: script_header
            .split("\n")
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
        lua: None,
        filename,
        image_vec: Vec::new(),
        preview_image: prev_img_header.and_then(|str| parse_image(200, 180, str.data.clone())),
    };

    for _ in 0..16384 {
        // prepare the vector for the 128x128 image made up of 16x16 sprites
        gamestate.image_vec.push(0);
    }

    if let Some(sfx) = sfx_header {
        let sfx = &sfx.data;
        for i in 0..32usize {
            gamestate.audios[i] = Audio::from_string(sfx[i * 194..(i + 1) * 194].to_string());
        }
    }

    if let Some(images) = image_header {
        let bytes = images.data.as_bytes();
        for i in 0..16384usize {
            gamestate.image_vec[i] = __from_hex(bytes[i] as char);
        }
    } else {
        gamestate.image_vec[258] = 12; // x: 2 y: 2
        gamestate.image_vec[261] = 12; // x: 5 y: 2
        gamestate.image_vec[642] = 12; // x: 2 y: 5
        gamestate.image_vec[645] = 12; // x: 5 y: 5
        gamestate.image_vec[516] = 12; // x: 4 y: 4
        gamestate.image_vec[388] = 12; // x: 4 y: 3
        gamestate.image_vec[387] = 12; // x: 3 y: 3
        gamestate.image_vec[515] = 12; // x: 3 y: 4
    }

    Some(gamestate)
}

pub fn load_r16_png(data: Vec<u8>, filename: Option<String>) -> Option<GameState> {
    let mut len: u32 = 0;
    len |= data[data.len() - 4] as u32;
    len |= (data[data.len() - 3] as u32) << 8;
    len |= (data[data.len() - 2] as u32) << 16;
    len |= (data[data.len() - 1] as u32) << 24;

    let mut new_data: Vec<u8> = Vec::with_capacity(len as usize);

    
    for i in 0..len as usize {
        new_data.push(data[data.len() - 4 - len as usize + i]);
    }

    string_to_game_data(unsafe { String::from_utf8_unchecked(new_data) }, filename)
}

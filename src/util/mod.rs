use serde::Deserialize;

// wrap an unsafe function call
pub fn glyph_idx_to_char(idx: u32) -> char {
    let res: char;
    // this unsafe is okay because this char gets passed to a function
    // in the tcod library which immediately casts it to an i32 anyway
    unsafe {
        res = std::char::from_u32_unchecked(idx);
    }
    res
}

#[derive(Debug, Deserialize)]
pub struct DaileConfig {
    //pub screen_width: u32,
    //pub screen_height: u32,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub font_horizontal: u32,
    pub font_vertical: u32,
    pub font_path: String,
}

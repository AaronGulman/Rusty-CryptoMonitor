use mlua::{Lua, Table};
use slint::{Brush, Color};

pub struct UIStore{
    pub bg: Brush,
    pub t: Brush,
}
pub fn from_argb_f32(a:f32,r:f32,g:f32,b:f32) -> Color{
    let rgb_color = Color::from_argb_f32(a,r,g,b);
    rgb_color
}

pub fn ui_bridge() -> UIStore{
    let lua = Lua::new();
    lua.load(include_str!("../lua_scripts/ui_config.lua")).exec().unwrap();
    let globals = lua.globals();
    let config: Table = globals.get("config").unwrap();
    
    let bg_table: Table = config.get("background").unwrap();
    let a:f32 = bg_table.get(1).unwrap();
    let r:f32 = bg_table.get(2).unwrap();
    let g:f32 = bg_table.get(3).unwrap();
    let b:f32 = bg_table.get(4).unwrap();
    
    let bg_color = from_argb_f32(a/255.0,r/255.0,g/255.0,b/255.0);
    let bg_brush = Brush::from(bg_color);

    let txt_table: Table = config.get("text_color").unwrap();
    let t_a:f32 = txt_table.get(1).unwrap();
    let t_red:f32 = txt_table.get(2).unwrap();
    let t_green:f32 = txt_table.get(3).unwrap();
    let t_blue:f32 = txt_table.get(4).unwrap();

    println!("{} {} {}",r,g,b);

    let t_color = from_argb_f32(t_a/255.0,t_red/255.0,t_green/255.0,t_blue/255.0);
    let t_brush  = Brush::from(t_color);

    println!("Background color: {}",bg_color);

    UIStore{bg: bg_brush,t: t_brush}

}
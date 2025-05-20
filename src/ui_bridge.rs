use mlua::{Lua, Table};
use slint::{Brush, Color};

pub struct UIStore{
    pub bg: Brush,
    pub t: Brush,
}
fn color_parser(element: String) -> Color{
    let element:Color = Color::from_rgb_f32(element.parse::<f32>().unwrap(),
                                            element.parse::<f32>().unwrap(),
                                            element.parse::<f32>().unwrap());
    element
}



pub fn ui_bridge() -> UIStore{
    let lua = Lua::new();
    lua.load(include_str!("../lua_scripts/ui_config.lua")).exec().unwrap();
    let globals = lua.globals();
    let config: Table = globals.get("config").unwrap();
    let bg: String = config.get("background").unwrap();
    let txt: String = config.get("text_color").unwrap();

    let bg_color:Color = color_parser(bg);
    let bg_brush = Brush::from(bg_color);

    let t_color:Color = color_parser(txt);

    let t_brush = Brush::from(t_color);

    UIStore{bg: bg_brush,t: t_brush}

}
#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Red,
    Yellow,
    Blue,
}

#[derive(Clone, Debug)]
pub struct ColorSettings {
    red_time: u32,
    yellow_time: u32,
    blue_time: u32,
}

impl ColorSettings {
    pub fn new(red_time: u32, yellow_time: u32, blue_time: u32) -> ColorSettings {
        ColorSettings {
            red_time, yellow_time, blue_time
        }
    }

    pub fn get_time(&self, color: &Color) -> u32 {
        match *color {
            Color::Red => self.red_time,
            Color::Yellow => self.yellow_time,
            Color::Blue => self.blue_time
        }
    }
}

trait TrafficLight {
    fn next() -> Color;
}

#[derive(Clone, Debug)]
pub struct StandardTrafficLight<'a> {
    color_settings: &'a ColorSettings,
    color: Color,
    time: u32
}

impl StandardTrafficLight<'_> {
    pub fn new(initial_color: Color, color_settings: &ColorSettings) -> StandardTrafficLight {
        let time = color_settings.get_time(&initial_color);
        StandardTrafficLight {
            color_settings: color_settings,
            color: initial_color,
            time: time,
        }
    }

    fn next() -> Color {
        //stub
        Color::Red
    }
}
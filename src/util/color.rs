use crate::util::Color;

impl Color {
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha
        }
    }

    pub const fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha: 255
        }
    }

    pub fn rgb(&self) -> [f32; 4] {
        [
            self.red as f32 / 255f32,
            self.green as f32 / 255f32,
            self.blue as f32 / 255f32,
            1.0
        ]
    }

    pub fn rgba(&self) -> [f32; 4] {
        [
            self.red as f32 / 255f32,
            self.green as f32 / 255f32,
            self.blue as f32 / 255f32,
            self.alpha as f32 / 255f32
        ]
    }

    pub const RED: Color = Color::new_rgb(255, 0, 0);
    pub const GREEN: Color = Color::new_rgb(0, 255, 0);
    pub const BLUE: Color = Color::new_rgb(0, 0, 255);
    pub const BLACK: Color = Color::new_rgb(0, 0, 0);
    pub const WHITE: Color = Color::new_rgb(255, 255, 255);
    pub const YELLOW: Color = Color::new_rgb(255, 255, 0);
    pub const PINK: Color = Color::new_rgb(255, 192, 203);
}
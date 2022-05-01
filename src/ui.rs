use sfml::graphics::{Color, Font, RenderTarget, RenderWindow, Text, TextStyle};
use sfml::system::Vector2f;
use sfml::SfBox;

const DEFAULT_TEXT_SIZE: u32 = 12;
const DEFAULT_TEXT_COLOR: Color = Color::RED;

pub struct UIText<'a> {
   text: String,
   font: &'a SfBox<Font>,
   sfml_text: Text<'a>,
   sfml_vec2: Vector2f,
}
impl<'a> UIText<'a> {
   pub fn new(text: &str, font: &'a SfBox<Font>) -> UIText<'a> {
      let mut ui_text = UIText {
         text: text.to_owned(),
         font,
         sfml_text: Text::new(text, font, DEFAULT_TEXT_SIZE),
         sfml_vec2: Vector2f::new(0., 0.),
      };
      ui_text.set_color(DEFAULT_TEXT_COLOR);

      return ui_text;
   }
   pub fn set_position(&mut self, coord_x: f32, coord_y: f32) {
      self.sfml_vec2.x = coord_x;
      self.sfml_vec2.y = coord_y;
   }
   pub fn get_position(&self) -> (f32, f32) {
      (self.sfml_vec2.x, self.sfml_vec2.y)
   }
   pub fn set_color(&mut self, color: Color) {
      self.sfml_text.set_fill_color(color);
   }
   pub fn set_bold(&mut self, active: bool) {
      let style = if active {
         TextStyle::BOLD
      } else {
         TextStyle::REGULAR
      };
      self.sfml_text.set_style(style);
   }
   pub fn display(&self, window: &mut RenderWindow) {
      window.draw(&self.sfml_text);
   }
}

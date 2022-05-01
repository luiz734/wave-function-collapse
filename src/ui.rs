use sfml::graphics::{
   Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text, TextStyle, Transformable,
};
use sfml::system::Vector2f;
use sfml::SfBox;

const DEFAULT_TEXT_SIZE: u32 = 12;
const DEFAULT_TEXT_COLOR: Color = Color::RED;
const DEFAULT_BUTTON_BACKGROUND: Color = Color::YELLOW;

pub trait Displayable {
   fn display(&self, window: &mut RenderWindow);
}

pub struct UIElement<'a> {
   position: Vector2f,
   size: Option<Vector2f>,
   background: Option<RectangleShape<'a>>,
}
impl<'a> UIElement<'a> {
   fn new(position: (f32, f32)) -> UIElementBuilder<'a> {
      let mut ui_element = UIElementBuilder {
         position: Vector2f::from(position),
         size: None,
         background: None,
      };
      ui_element
   }
   pub fn set_position(&mut self, coord_x: f32, coord_y: f32) {
      self.position.x = coord_x;
      self.position.y = coord_y;
      match &mut self.background {
         Some(x) => x.set_position(self.position),
         None => (),
      }
   }
   pub fn get_position(&self) -> (f32, f32) {
      (self.position.x, self.position.y)
   }
}
impl<'a> Displayable for UIElement<'a> {
   fn display(&self, window: &mut RenderWindow) {
      match &self.background {
         Some(x) => window.draw(x),
         None => (),
      }
   }
}

pub struct UIElementBuilder<'a> {
   position: Vector2f,
   size: Option<Vector2f>,
   background: Option<RectangleShape<'a>>,
}
impl<'a> UIElementBuilder<'a> {
   fn build(&mut self) -> UIElement<'a> {
      UIElement {
         position: self.position,
         size: self.size,
         background: self.background.clone(),
      }
   }
   fn size(&mut self, size: (f32, f32)) -> &mut UIElementBuilder<'a> {
      self.size = Some(Vector2f::from(size));
      self
   }
   fn background(&mut self, color: Color) -> &mut UIElementBuilder<'a> {
      let mut rect = RectangleShape::new();
      rect.set_fill_color(color);
      self.background = Some(rect);
      self
   }
}

pub struct UIText<'a> {
   ui_element: UIElement<'a>,
   text: String,
   font: &'a SfBox<Font>,
   sfml_text: Text<'a>,
}
impl<'a> UIText<'a> {
   pub fn new(text: &str, font: &'a SfBox<Font>) -> UIText<'a> {
      let mut ui_text = UIText {
         ui_element: UIElement::new((0., 0.)).background(Color::MAGENTA).build(),
         text: text.to_owned(),
         font,
         sfml_text: Text::new(text, font, DEFAULT_TEXT_SIZE),
      };
      ui_text.set_color(DEFAULT_TEXT_COLOR);

      return ui_text;
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
}
impl<'a> Displayable for UIText<'a> {
   fn display(&self, window: &mut RenderWindow) {
      self.ui_element.display(window);
      window.draw(&self.sfml_text);
   }
}
pub struct UIButton<'a> {
   ui_element: UIElement<'a>,
   label: UIText<'a>,
   callback: Box<dyn FnMut()>,
}
impl<'a> UIButton<'a> {
   pub fn new(
      label: &str,
      font: &'a SfBox<Font>,
      position: (f32, f32),
      size: (f32, f32),
      on_click: impl FnMut() + 'a + 'static,
   ) -> UIButton<'a> {
      let mut ui_btn = UIButton {
         ui_element: UIElement::new(position)
            .background(DEFAULT_BUTTON_BACKGROUND)
            .size(size)
            .build(),
         label: UIText::new(label, font),
         callback: Box::new(on_click),
      };
      ui_btn
   }
   pub fn click(&mut self) {
      (self.callback)();
   }
}
impl<'a> Displayable for UIButton<'a> {
   fn display(&self, window: &mut RenderWindow) {
      self.ui_element.display(window);
      self.label.display(window);
   }
}

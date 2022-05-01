use sfml::graphics::{
	CircleShape, Color, Font, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Shape,
	Transformable, Vertex,
};
use sfml::system::Vector2f;
use sfml::window::{Event, Style, VideoMode};
use sfml::SfBox;

use crate::ui::UIText;

#[derive(Debug)]
pub struct Cell {
	row: usize,
	col: usize,
	values: Vec<usize>,
}

impl Cell {
	pub fn new(row: usize, col: usize) -> Cell {
		let mut cell = Cell {
			row,
			col,
			values: Vec::new(),
		};
		for i in 1..9 {
			cell.values.push(i);
		}
		return cell;
	}
	pub fn calc_position(&self, cell_size: usize, spacement: usize) -> (usize, usize) {
		let pos_x = self.col * (cell_size + spacement);
		let pos_y = self.row * (cell_size + spacement);

		(pos_x, pos_y)
	}
}

pub struct Canvas<'a> {
	pub window: RenderWindow,
	font: &'a SfBox<Font>,
	text: UIText<'a>,
}
impl<'a> Canvas<'a> {
	pub fn new(width: u32, height: u32, title: &str, font: &'a SfBox<Font>) -> Canvas<'a> {
		Canvas {
			window: RenderWindow::new(
				VideoMode::new(width, height, 32),
				title,
				Style::CLOSE,
				&Default::default(),
			),
			font,
			text: UIText::new("hello world!", font),
		}
	}
	pub fn setup(&mut self) {
		self.window.set_framerate_limit(60);
	}
	pub fn draw(&mut self) {
		while self.window.is_open() {
			// Event processing
			while let Some(event) = self.window.poll_event() {
				// Request closing for the window
				if event == Event::Closed {
					self.window.close();
				}
			}

			self.window.clear(Color::BLACK);
			// try draw vertices (connections)
			// draw points and vertices
			self.text.display(&mut self.window);

			self.window.display();
		}
	}
}

use std::fmt::{Display, Formatter, Result};
use termion::color::{Fg, Rgb, Reset};

#[derive(Clone, Copy)]
pub struct Node {
	col: (u8, u8, u8),
	kind: u8,
}
impl Display for Node {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if self.kind == 0 {
			return write!(f, "{} .{}", Fg(Rgb(self.col.0, self.col.1, self.col.2)), Fg(Reset));
		}
		return write!(f, "{}[]{}", Fg(Rgb(self.col.0, self.col.1, self.col.2)), Fg(Reset));
	}
}

impl Node {
	pub fn init(col: (u8, u8, u8), kind: u8) -> Self {
		Self {
			col,
			kind,
		}
	}

	pub fn change(&mut self, col: (u8, u8, u8), kind: u8) {
		self.col = col;
		self.kind = kind;
	}
}

/// {kind, pos, now_shape} all about u8
pub struct Block {
	kind: u8,
	pos: (u8, u8),
	now_shape: u8,
}

impl Block {
	pub fn init(kind: u8, pos: (u8, u8), now_shape: u8) -> Self {
		Self {
			kind,
			pos,
			now_shape,
		}
	}

	pub fn next(&mut self, kind: u8) {
		self.kind = kind;
		self.now_shape = 0;
		self.pos = (1, 4);
	}

	pub fn get_pos(&self) -> (u8, u8) {
		self.pos
	}

	pub fn get_shape(&self) -> [u8; 16] {
		Block::TYPES[self.kind as usize][self.now_shape as usize]
	}

	pub fn get_color(&self) -> (u8, u8, u8) {
		Block::COLOR[self.kind as usize]
	}

	pub fn down(&mut self) {
		self.pos.0 += 1;
	}

	pub fn quick_down(&mut self) {
	}

	pub fn right(&mut self) {
		self.pos.1 += 1;
	}

	pub fn left(&mut self) {
		self.pos.1 -= 1;
	}

	pub fn rotate(&mut self) {
		self.now_shape = (self.now_shape + 1) % 4;
	}

	pub fn invrot(&mut self) {
		self.now_shape = (self.now_shape - 1) % 4;
	}

	const COLOR: [(u8, u8, u8); 8] = [
		(0, 0, 0),
		(0, 255, 255),
		(255, 0, 0),
		(255, 0, 255),
		(255, 255, 0),
		(0, 255, 0),
		(255, 153, 51),
		(0, 0, 255)
	];
	const TYPES: [[[u8; 16]; 4]; 8] = [
		[ // nothing: 0
			[0, 0, 0, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0],

			[0, 0, 0, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0],

			[0, 0, 0, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0],

			[0, 0, 0, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0]
		],
		[ // I: 1
			[0, 0, 0, 0,
			 1, 1, 1, 1,
			 0, 0, 0, 0,
			 0, 0, 0, 0],

			[0, 0, 1, 0,
			 0, 0, 1, 0,
			 0, 0, 1, 0,
			 0, 0, 1, 0],

			[0, 0, 0, 0,
			 0, 0, 0, 0,
			 1, 1, 1, 1,
			 0, 0, 0, 0],

			[0, 1, 0, 0,
			 0, 1, 0, 0,
			 0, 1, 0, 0,
			 0, 1, 0, 0]
		],
		[ // Z: 2
			[1, 1, 0, 0,
			 0, 1, 1, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0],

			[0, 0, 1, 0,
			 0, 1, 1, 0,
			 0, 1, 0, 0,
			 0, 0, 0, 0],

			[0, 0, 0, 0,
			 1, 1, 0, 0,
			 0, 1, 1, 0,
			 0, 0, 0, 0],

			[0, 1, 0, 0,
			 1, 1, 0, 0,
			 1, 0, 0, 0,
			 0, 0, 0, 0]
		],
		[ // T: 3
			[0, 0, 0, 0,
			 1, 1, 1, 0,
			 0, 1, 0, 0,
			 0, 0, 0, 0],

			[0, 1, 0, 0,
			 1, 1, 0, 0,
			 0, 1, 0, 0,
			 0, 0, 0, 0],

			[0, 1, 0, 0,
			 1, 1, 1, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0],

			[0, 1, 0, 0,
			 0, 1, 1, 0,
			 0, 1, 0, 0,
			 0, 0, 0, 0]
		],
		[ // O: 4
			[0, 0, 0, 0,
			 0, 1, 1, 0,
			 0, 1, 1, 0,
			 0, 0, 0, 0],

			[0, 0, 0, 0,
			 0, 1, 1, 0,
			 0, 1, 1, 0,
			 0, 0, 0, 0],

			[0, 0, 0, 0,
			 0, 1, 1, 0,
			 0, 1, 1, 0,
			 0, 0, 0, 0],

			[0, 0, 0, 0,
			 0, 1, 1, 0,
			 0, 1, 1, 0,
			 0, 0, 0, 0]
		],
		[ // S: 5
			[0, 1, 1, 0,
			 1, 1, 0, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0],

			[0, 1, 0, 0,
			 0, 1, 1, 0,
			 0, 0, 1, 0,
			 0, 0, 0, 0],

			[0, 0, 0, 0,
			 0, 1, 1, 0,
			 1, 1, 0, 0,
			 0, 0, 0, 0],

			[1, 0, 0, 0,
			 1, 1, 0, 0,
			 0, 1, 0, 0,
			 0, 0, 0, 0]
		],
		[ // L: 6
			[0, 0, 1, 0,
			 1, 1, 1, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0],

			[0, 1, 0, 0,
			 0, 1, 0, 0,
			 0, 1, 1, 0,
			 0, 0, 0, 0],

			[0, 0, 0, 0,
			 1, 1, 1, 0,
			 1, 0, 0, 0,
			 0, 0, 0, 0],

			[1, 1, 0, 0,
			 0, 1, 0, 0,
			 0, 1, 0, 0,
			 0, 0, 0, 0]
		],
		[ // J: 7
			[1, 0, 0, 0,
			 1, 1, 1, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0],

			[0, 1, 1, 0,
			 0, 1, 0, 0,
			 0, 1, 0, 0,
			 0, 0, 0, 0],

			[0, 0, 0, 0,
			 1, 1, 1, 0,
			 0, 0, 1, 0,
			 0, 0, 0, 0],

			[0, 1, 0, 0,
			 0, 1, 0, 0,
			 1, 1, 0, 0,
			 0, 0, 0, 0]
		]
	];
}

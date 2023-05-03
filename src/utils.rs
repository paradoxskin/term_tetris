use std::fmt::{Display, Formatter, Result};
use termion::color::{Fg, Rgb, Reset};
use termion::style;
use std::sync::Mutex;

#[derive(Clone, Copy)]
pub struct Node {
	col: (u8, u8, u8),
	pub kind: u8,
}
impl Display for Node {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if self.kind == 0 {
			return write!(f, "{}{} .{}{}", Fg(Rgb(self.col.0, self.col.1, self.col.2)), style::Italic, style::Reset, Fg(Reset));
		}
		return write!(f, "{}{}[]{}{}", Fg(Rgb(self.col.0, self.col.1, self.col.2)), style::Bold, style::Reset, Fg(Reset));
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
	pos: (i8, i8),
	now_shape: u8,
}

impl Block {
	pub fn init(kind: u8, pos: (i8, i8), now_shape: u8) -> Self {
		Self {
			kind,
			pos,
			now_shape,
		}
	}

	pub fn next(&mut self, kind: u8) {
		self.kind = kind;
		self.now_shape = 0;
		self.pos = (0, 3);
	}

	pub fn get_pos(&self) -> (i8, i8) {
		self.pos
	}

	pub fn get_shape(&self) -> [u8; 16] {
		Block::TYPES[self.kind as usize][self.now_shape as usize]
	}

	pub fn get_color(&self) -> (u8, u8, u8) {
		Block::COLOR[self.kind as usize]
	}

	pub fn down(&mut self, map: &Mutex<Vec<Vec<Node>>>) -> bool {
		let is_crash = self.is_crash(&map);
		if !is_crash {
			self.pos.0 += 1;
		}
		else {
			self.stop(map);
		}
		is_crash
	}

	fn stop(&self, map: &Mutex<Vec<Vec<Node>>>) {
		let shape = self.get_shape();
		let mut map = map.lock().unwrap();
		let col = self.get_color();
		for x in 0..4_usize {
			for y in (0..4_usize).rev() {
				if shape[4 * y + x] == 1 {
					map[self.pos.0 as usize + y][(self.pos.1 + x as i8) as usize].change(col, 1);
				}
			}
		}
	}

	pub fn quick_down(&mut self, map: &Mutex<Vec<Vec<Node>>>) {
		while !self.down(map) {}
	}

	pub fn is_crash(&self, map: &Mutex<Vec<Vec<Node>>>) -> bool{
		let shape = self.get_shape();
		let map = map.lock().unwrap();
		for x in 0..4_usize {
			for y in (0..4_usize).rev() {
				if shape[4 * y + x] == 1 {
					// -1 as usize happens overflow
					if (self.pos.0 as usize + y + 1 == 20) || (map[self.pos.0 as usize + y + 1][(self.pos.1 + x as i8) as usize].kind == 1) {
						return true;
					}
					break;
				}
			}
		}
		return false;
	}

	pub fn debug(&self) -> usize {
		let shape = self.get_shape();
		for y in 0..4_usize {
			for x in (0..4_usize).rev() {
				if shape[4 * y + x] == 1 {
					return (self.pos.1 + x as i8 + 1) as usize;
				}
			}
		}
		return 0;
	}

	fn is_crash_right(&self, map: &Mutex<Vec<Vec<Node>>>) -> bool {
		let shape = self.get_shape();
		let map = map.lock().unwrap();
		for y in 0..4_usize {
			for x in (0..4_usize).rev() {
				if shape[4 * y + x] == 1 {
					if map[self.pos.0 as usize + y][(self.pos.1 + x as i8 + 1) as usize].kind == 1 {
						return true;
					}
					break;
				}
			}
		}
		return false;
	}

	fn is_crash_left(&self, map: &Mutex<Vec<Vec<Node>>>) -> bool {
		let shape = self.get_shape();
		let map = map.lock().unwrap();
		for y in 0..4_usize {
			for x in 0..4_usize {
				if shape[4 * y + x] == 1 {
					if map[self.pos.0 as usize + y][(self.pos.1 + x as i8 - 1) as usize].kind == 1 {
						return true;
					}
					break;
				}
			}
		}
		return false;
	}

	pub fn right(&mut self, map: &Mutex<Vec<Vec<Node>>>) {
		if self.pos.1 + self.get_right() < 9 && !self.is_crash_right(map) {
			self.pos.1 += 1;
		}
	}

	pub fn left(&mut self, map: &Mutex<Vec<Vec<Node>>>) {
		if self.pos.1 + self.get_left() > 0 && !self.is_crash_left(map) {
			self.pos.1 -= 1;
		}
	}

	fn get_left(&self) -> i8 {
		let now = self.get_shape();
		for x in 0..4_usize {
			for y in 0..4_usize {
				if now[4 * y + x] == 1 {
					return x as i8;
				}
			}
		}
		return 0_i8;
	}

	fn get_right(&self) -> i8 {
		let now = self.get_shape();
		for x in (0..4_usize).rev() {
			for y in 0..4_usize {
				if now[4 * y + x] == 1 {
					return x as i8;
				}
			}
		}
		return 0_i8;
	}

	fn get_down(&self) -> i8 {
		let now = self.get_shape();
		for y in (0..4_usize).rev() {
			for x in 0..4_usize {
				if now[4 * y + x] == 1 {
					return y as i8;
				}
			}
		}
		return 0_i8;
	}

	pub fn rotate(&mut self, map: &Mutex<Vec<Vec<Node>>>) {
		let next_shape = (self.now_shape + 1) % 4;
		let shape = Self::TYPES[self.kind as usize][next_shape as usize];
		let map = map.lock().unwrap();
		let mut next_pos = self.pos;
		for y in 0..4_usize {
			for x in 0..4_usize {
				if shape[4 * y + x] == 1 {
					while (next_pos.1 + x as i8) < 0 {
						next_pos.1 += 1;
					}
					while (next_pos.1 + x as i8) > 9 {
						next_pos.1 -= 1;
					}
					while (next_pos.0 + y as i8) > 19 {
						next_pos.0 -= 1;
					}
				}
			}
		}
		for y in 0..4_usize {
			for x in 0..4_usize {
				if shape[4 * y + x] == 1 {
					let next_x = next_pos.1 + x as i8;
					let next_y = next_pos.0 + y as i8;
					if map[next_y as usize][next_x as usize].kind == 1 {
						return;
					}
				}
			}
		}
		self.now_shape = (self.now_shape + 1) % 4;
		while self.get_left() + self.pos.1 < 0 {
			self.pos.1 += 1;
		}
		while self.get_right() + self.pos.1 > 9 {
			self.pos.1 -= 1;
		}
		while self.get_down() + self.pos.0 > 19 {
			self.pos.0 -= 1;
		}
	}

	pub fn invrot(&mut self, map: &Mutex<Vec<Vec<Node>>>) {
		let next_shape = (self.now_shape + 3) % 4;
		let shape = Self::TYPES[self.kind as usize][next_shape as usize];
		let map = map.lock().unwrap();
		let mut next_pos = self.pos;
		for y in 0..4_usize {
			for x in 0..4_usize {
				if shape[4 * y + x] == 1 {
					while (next_pos.1 + x as i8) < 0 {
						next_pos.1 += 1;
					}
					while (next_pos.1 + x as i8) > 9 {
						next_pos.1 -= 1;
					}
					while (next_pos.0 + y as i8) > 19 {
						next_pos.0 -= 1;
					}
				}
			}
		}
		for y in 0..4_usize {
			for x in 0..4_usize {
				if shape[4 * y + x] == 1 {
					let next_x = next_pos.1 + x as i8;
					let next_y = next_pos.0 + y as i8;
					if map[next_y as usize][next_x as usize].kind == 1 {
						return;
					}
				}
			}
		}
		self.now_shape = (4 + self.now_shape - 1) % 4;
		while self.get_left() + self.pos.1 < 0 {
			self.pos.1 += 1;
		}
		while self.get_right() + self.pos.1 > 9 {
			self.pos.1 -= 1;
		}
		while self.get_down() + self.pos.0 > 19 {
			self.pos.0 -= 1;
		}
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
			[0, 1, 1, 0,
			 0, 1, 1, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0],

			[0, 1, 1, 0,
			 0, 1, 1, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0],

			[0, 1, 1, 0,
			 0, 1, 1, 0,
			 0, 0, 0, 0,
			 0, 0, 0, 0],

			[0, 1, 1, 0,
			 0, 1, 1, 0,
			 0, 0, 0, 0,
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

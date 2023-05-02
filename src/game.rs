use crate::utils::{Block, Node};
use std::collections::VecDeque;
use std::sync::Mutex;
use std::{time, thread};
use rand::random;
use termion::{cursor, clear};

pub struct Game {
	map: Mutex<[[Node; 10]; 20]>,
	now_block: Mutex<Block>,
	packs: Mutex<VecDeque<u8>>,
	score: Mutex<i32>,
}

impl Game {

	const FPS: f64 = 30.0;
	const WAIT: f64 = 1.0 / Self::FPS;

	pub fn init() -> Self {
		let score = Mutex::new(0 as i32);
		let map = Mutex::new(
					[[Node::init([' ', '.'], (255, 255, 255), 0); 10]; 20]);
		let now_block = Mutex::new(
					Block::init(1, (1, 4), 4));
		let packs = Mutex::new(
					VecDeque::<u8>::new());
		Self {
			map,
			score,
			now_block,
			packs,
		}
	}

	pub fn run(&self) {
		self.create_packs();
		{
			let mut now_block = self.now_block.lock().unwrap();
			now_block.next(self.pick_next_block());
		}
		print!("{}", cursor::Hide);
		loop {
			let begin = time::Instant::now();
			if self.update() {
				break
			}
			self.draw();
			let end = time::Instant::now();
			let wait = time::Duration::from_secs_f64(Self::WAIT);
			thread::sleep(
					 wait - end.duration_since(begin));
		}
		print!("{}", cursor::Show);
	}

	fn update(&self) -> bool{
		false
	}

	// TODO just draw what changed can better
	fn draw(&self) {
		println!("{}{}", clear::All, cursor::Goto(1, 1));
		let map = self.map.lock().unwrap();
		for i in map.iter() {
			for j in i {
				print!("{}", j);
			}
			println!();
		}
		//println!("{}", self.pick_next_block());
	}

	fn create_packs(&self) {
		let mut packs = self.packs.lock().unwrap();
		let mut tmp_vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];
		for i in (1..=7).rev() {
			let x = random::<usize>() % i;
			packs.push_back(tmp_vec[x]);
			(tmp_vec[i - 1], tmp_vec[x]) = (tmp_vec[x], tmp_vec[i - 1]);
		}
	}

	fn pick_next_block(&self) -> u8 {
		let flag: bool;
		let out: u8;
		{
			let mut packs = self.packs.lock().unwrap();
			out = packs.pop_front().unwrap();
			flag = packs.len() < 7;
		}
		if flag {
			self.create_packs();
		}
		out
	}
}

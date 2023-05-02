use crate::utils::{Block, Node};
use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::RwLock;
use std::{time, thread};
use rand::random;
use termion::{cursor, clear, raw::{IntoRawMode,RawTerminal}, event::Key, input::TermRead};
use std::io::{Write, stdout, Stdout, stdin};

pub struct Game {
	map: Arc<Mutex<Vec<Vec<Node>>>>,
	now_block: Arc<Mutex<Block>>,
	//hold_block
	packs: Mutex<VecDeque<u8>>,
	score: Mutex<i32>,
	end_flag: Arc<RwLock<u8>>,
}

impl Game {

	const FPS: f64 = 30.0;
	const WAIT: f64 = 1.0 / Self::FPS;

	pub fn init() -> Self {
		let score = Mutex::new(0 as i32);
		let map = Arc::new(Mutex::new(
					vec![vec![Node::init((255, 255, 255), 0); 10]; 20]));
		let now_block = Arc::new(Mutex::new(
					Block::init(1, (1, 4), 4)));
		let packs = Mutex::new(
					VecDeque::<u8>::new());
		let end_flag = Arc::new(RwLock::new(0_u8));
		Self {
			map,
			score,
			now_block,
			packs,
			end_flag,
		}
	}

	pub fn run(&self) {
		self.create_packs();
		{
			let mut now_block = self.now_block.lock().unwrap();
			now_block.next(self.pick_next_block());
		}
		print!("{}", cursor::Hide);
		let mut stdout = stdout().into_raw_mode().unwrap();
		let listen_key = self.listen_key();
		loop {
			{
				if *(self.end_flag.read().unwrap()) == 1 {
					break;
				}
			}
			let begin = time::Instant::now();
			self.update();
			self.draw(&mut stdout);
			let end = time::Instant::now();
			let wait = time::Duration::from_secs_f64(Self::WAIT);
			thread::sleep(
					 wait - end.duration_since(begin));
		}
		listen_key.join().unwrap();
		print!("{}", cursor::Show);
	}

	fn listen_key(&self) -> thread::JoinHandle<()>{
		let end_flag = self.end_flag.clone();
		let stdin = stdin();
		let now_block_mutex = self.now_block.clone();
		let map_mutex = self.map.clone();
		return thread::spawn(move || {
			for key in stdin.keys() {
				match key.unwrap() {
					Key::Ctrl('c') => {
						let mut x = end_flag.write().unwrap();
						*x = 1;
						return;
					}
					Key::Char('n') => {
						let mut now_block = now_block_mutex.lock().unwrap();
						now_block.rotate();
					}
					Key::Char('m') => {
						let mut now_block = now_block_mutex.lock().unwrap();
						now_block.invrot();
					}
					Key::Char('a') => {
						let mut now_block = now_block_mutex.lock().unwrap();
						now_block.left();
					}
					Key::Char('d') => {
						let mut now_block = now_block_mutex.lock().unwrap();
						now_block.right();
					}
					Key::Char('s') => {
						let mut now_block = now_block_mutex.lock().unwrap();
						if now_block.down(map_mutex.clone()) {
							let mut now_block = now_block_mutex.lock().unwrap();
							//now_block.next(1);
						}
					}
					_ => {}
				}
			}
		});
	}

	fn update(&self){
	}

	// TODO just draw what changed can better
	fn draw(&self, stdout: &mut RawTerminal<Stdout>) {
		write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
		let map = self.map.lock().unwrap();
		for i in map.iter() {
			write!(stdout, "<!").unwrap();
			for j in i {
				write!(stdout, "{}", j).unwrap();
			}
			write!(stdout, "!>\r\n").unwrap();
		}
		write!(stdout, "<!====================!>\r\n").unwrap();
		write!(stdout, "  \\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\r\n").unwrap();
		// block
		let now_block = self.now_block.lock().unwrap();
		let pos = now_block.get_pos();
		let shape = now_block.get_shape();
		let col = now_block.get_color();
		for y in 0..4_usize {
			for x in 0..4_usize {
				if shape[4 * y + x] == 1 {
					write!(stdout,
							"{}{}",
							cursor::Goto(3 + ((pos.1 + x as i8)* 2) as u16,
							(1 + pos.0 + y as i8) as u16),
							Node::init(col, 1),
						  ).unwrap();
				}
			}
		}
		write!(stdout, "{}| score: {} {} |", cursor::Goto(28, 3), pos.0, now_block.debug()).unwrap();
		{
			let score = self.score.lock().unwrap();
			write!(stdout, "{}| score: {} |", cursor::Goto(28, 2), *score).unwrap();
		}

		write!(stdout, "{}", cursor::Goto(1, 233)).unwrap();
		stdout.flush().unwrap();
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

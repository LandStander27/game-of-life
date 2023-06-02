use std::time::Instant;

use macroquad::prelude::*;

fn window_conf() -> Conf {
	return Conf {
		window_title: "Conway's Game of Life".to_string(),
		fullscreen: true,
		window_resizable: false,
		..Default::default()
	};
}

#[derive(Debug, Copy, Clone)]
struct Amount {
	amount_x: i32,
	amount_y: i32,
	offset_x: f32,
	offset_y: f32
}

fn calculate_square_amount(size: f32) -> Amount {

	let w = screen_width();
	let h = screen_height();

	let mut amount = (0, 0);
	let mut offset = (0.0, 0.0);

	let mut current = 0.0;
	loop {
		if current+size > w {
			offset.0 = (w - current) / 2.0;
			break;
		}
		current += size;
		amount.0 += 1;
	}

	current = 0.0;
	loop {
		if current+size > h {
			offset.1 = (h - current) / 2.0;
			break;
		}
		current += size;
		amount.1 += 1;
	}

	return Amount {
		amount_x: amount.0,
		amount_y: amount.1,
		offset_x: offset.0,
		offset_y: offset.1
	};

	// let x = w / size;
	// let y = h / size;

	// let offset_x = (x - x.floor()) / 2.0 * size;
	// let offset_y = (y - y.floor()) / 2.0 * size;

	// return Amount {
	// 	amount_x: x.floor() as i32,
	// 	amount_y: y.floor() as i32,
	// 	offset_x: offset_x,
	// 	offset_y: offset_y
	// };


}

#[derive(Debug, Copy, Clone, PartialEq)]
enum CellState {
	Alive,
	Dead
}

#[derive(Clone)]
struct Cell {
	x: f32,
	y: f32,
	w: f32,
	state: CellState,
	offset: (f32, f32),
	wanted_offset: (f32, f32),
}

impl Cell {
	fn new(x: f32, y: f32, w: f32) -> Self {
		return Self {
			x: x,
			y: y,
			w: w,
			state: CellState::Dead,
			offset: (w/2.0, -w),
			wanted_offset: (w/2.0, -w),
		};
	}

	fn draw(&self, draw_grid: bool) {

		draw_rectangle(self.x+self.offset.0, self.y+self.offset.0, self.w+self.offset.1, self.w+self.offset.1, WHITE);

		if self.state == CellState::Dead {
			if draw_grid {
				draw_rectangle_lines(self.x, self.y, self.w, self.w, 0.25, GRAY);
			} else {
				draw_rectangle(self.x, self.y, self.w, self.w, BLACK);
			}
		}

	}

	fn update(&mut self) {
		self.offset.0 = self.offset.0 + (self.wanted_offset.0 - self.offset.0) / 5.0;
		self.offset.1 = self.offset.1 + (self.wanted_offset.1 - self.offset.1) / 5.0;
	}

	fn kill(&mut self) {
		self.state = CellState::Dead;
		self.wanted_offset = (self.w/2.0, -self.w);
	}

	fn live(&mut self) {
		self.state = CellState::Alive;
		self.wanted_offset = (0.0, 0.0);
	}
	
}

#[derive(Clone)]
struct Game {
	cells: Vec<Vec<Cell>>,
	amount: Amount,
	paused: bool,
	draw_grid: bool,
	speed: f32,
	last_update: Instant,
}

impl Game {
	fn new(size: f32) -> Self {

		let amount = calculate_square_amount(size);

		let mut cells: Vec<Vec<Cell>> = Vec::new();
		for x in 0..amount.amount_x {
			let mut row: Vec<Cell> = Vec::new();
			for y in 0..amount.amount_y {
				row.push(Cell::new(x as f32*size+amount.offset_x, y as f32*size+amount.offset_y, size));
			}
			cells.push(row);
		}

		return Self {
			cells: cells,
			amount: amount,
			paused: true,
			draw_grid: true,
			speed: 0.25,
			last_update: Instant::now(),
		};
	}

	fn get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
		
		if x >= self.cells.len() as i32 || y >= self.cells[0].len() as i32 || x < 0 || y < 0 {
			return None;
		}
		return Some(&self.cells[x as usize][y as usize]);

	}

	fn get_cell_mut(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
		
		if x >= self.cells.len() as i32 || y >= self.cells[0].len() as i32 || x < 0 || y < 0 {
			return None;
		}
		return Some(&mut self.cells[x as usize][y as usize]);

	}

	fn handle_mouse(&mut self, x: f32, y: f32, button: MouseButton) {
		for x2 in 0..self.amount.amount_x {
			for y2 in 0..self.amount.amount_y {
				let cell = self.get_cell_mut(x2, y2).unwrap();
				if x > cell.x && y > cell.y && x < cell.x + cell.w && y < cell.y + cell.w {
					match button {
						MouseButton::Right => cell.kill(),
						MouseButton::Left => cell.live(),
						_ => ()
					};
				}
			}
		}
	}

	fn toggle_pause(&mut self) {
		self.draw_grid = !self.draw_grid;
		self.paused = !self.paused;
	}

	fn amount_around(&self, x: i32, y: i32, board: &Vec<Vec<Cell>>) -> u32 {

		let mut amount = 0;

		for x_offset in -1..=1 {
			for y_offset in -1..=1 {
				if x_offset == 0 && y_offset == 0 {
					continue;
				}

				if (x+x_offset >= board.len() as i32 || y+y_offset >= board[0].len() as i32) || (x+x_offset < 0 || y+y_offset < 0) {
					continue;
				}

				if board[(x+x_offset) as usize][(y+y_offset) as usize].state == CellState::Alive {
					amount += 1;
				}

			}
		}

		return amount;

	}

	fn draw(&self) {
		for x in 0..self.amount.amount_x {
			for y in 0..self.amount.amount_y {
				self.get_cell(x, y).unwrap().draw(self.draw_grid);
			}
		}
	}

	fn update(&mut self) {

		if self.paused || self.last_update.elapsed().as_millis() as f32 / 1000.0 < self.speed {

			for x in 0..self.amount.amount_x {
				for y in 0..self.amount.amount_y {
					self.get_cell_mut(x, y).unwrap().update();
				}
			}

		} else {
			self.last_update = Instant::now();
			let board = self.cells.clone();

			for x in 0..self.amount.amount_x {
				for y in 0..self.amount.amount_y {
	
					let amount = self.amount_around(x, y, &board);
	
					let cell = self.get_cell_mut(x, y).unwrap();
					cell.update();
	
					// if amount != 0 && cell.state == CellState::Alive {
					// 	println!("{:?}", amount);
					// }
	
					match cell.state {
						CellState::Alive => {
	
							if amount == 2 || amount == 3 {
								continue;
							}
	
							if amount < 2 {
								cell.kill();
							} else if amount > 3 {
								cell.kill();
							}
	
						},
						CellState::Dead => {
							if amount == 3 {
								cell.live();
								continue;
							}
						}
					}
	
					
					
				}
			}
		}


	}

}

#[macroquad::main(window_conf)]
async fn main() {

	let mut game = Game::new(20.0);

	loop {

		clear_background(BLACK);

		let pos = mouse_position();
		if is_mouse_button_down(MouseButton::Left) {
			game.handle_mouse(pos.0, pos.1, MouseButton::Left);
		}
		if is_mouse_button_down(MouseButton::Right) {
			game.handle_mouse(pos.0, pos.1, MouseButton::Right);
		}

		if is_key_pressed(KeyCode::Space) {
			game.toggle_pause();
		}

		game.update();
		game.draw();

		next_frame().await;

	}


}

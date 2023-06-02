use macroquad::prelude::*;
use egui_macroquad::egui;

fn window_conf() -> Conf {
	return Conf {
		window_title: "Conway's Game of Life".to_string(),
		fullscreen: true,
		window_resizable: false,
		..Default::default()
	};
}

#[derive(Debug, Clone, Copy)]
struct Point {
	x: i32,
	y: i32
}

fn plot_high(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<Point> {
	let mut dx = x1 - x0;
	let dy = y1 - y0;
	let mut xi = 1;
	if dx < 0 {
		xi = -1;
		dx = -dx;
	}
	let mut d = (2 * dx) - dy;
	let mut x = x0;

	let mut points: Vec<Point> = Vec::new();

	for y in y0..=y1 {
		points.push(Point { x: x, y: y });
		if d > 0 {
			x = x + xi;
			d = d + (2 * (dx - dy));
		} else {
			d = d + 2*dx;
		}
	}
	return points;
}


fn plot_low(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<Point> {

	let dx = x1 - x0;
	let mut dy = y1 - y0;
	let mut yi = 1;
	if dy < 0 {
		yi = -1;
		dy = -dy;
	}
	let mut d = (2 * dy) - dx;
	let mut y = y0;

	let mut points: Vec<Point> = Vec::new();

	for x in x0..=x1 {
		points.push(Point { x: x, y: y });
		if d > 0 {
			y = y + yi;
			d = d + (2 * (dy - dx));
		} else {
			d = d + 2*dy;
		}
	}
	return points;
}


fn plot_line(x0: f32, y0: f32, x1: f32, y1: f32) -> Vec<Point> {
	if (y1 - y0).abs() < (x1 - x0).abs() {
		if x0 > x1 {
			return plot_low(x1.round() as i32, y1.round() as i32, x0.round() as i32, y0.round() as i32);
		} else {
			return plot_low(x0.round() as i32, y0.round() as i32, x1.round() as i32, y1.round() as i32);
		}
	} else {
		if y0 > y1 {
			return plot_high(x1.round() as i32, y1.round() as i32, x0.round() as i32, y0.round() as i32);
		} else {
			return plot_high(x0.round() as i32, y0.round() as i32, x1.round() as i32, y1.round() as i32);
		}
	}
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
				draw_rectangle_lines(self.x, self.y, self.w, self.w, 0.25, DARKGRAY);
			} else {
				draw_rectangle(self.x, self.y, self.w, self.w, BLACK);
			}
		}

	}

	fn update(&mut self) {
		self.offset.0 = self.offset.0 + (self.wanted_offset.0 - self.offset.0) / 5.0;
		self.offset.1 = self.offset.1 + (self.wanted_offset.1 - self.offset.1) / 5.0;
	}

	fn kill(&mut self, animations: bool) {
		self.state = CellState::Dead;
		self.wanted_offset = (self.w/2.0, -self.w);
		if !animations {
			self.offset = self.wanted_offset;
		}
	}

	fn live(&mut self, animations: bool) {
		self.state = CellState::Alive;
		self.wanted_offset = (0.0, 0.0);
		if !animations {
			self.offset = self.wanted_offset;
		}
	}
	
}

#[derive(Clone)]
struct Game {
	cells: Vec<Vec<Cell>>,
	amount: Amount,
	paused: bool,
	draw_grid: bool,
	speed: f64,
	last_update: f64,
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
			last_update: macroquad::miniquad::date::now(),
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
						MouseButton::Right => cell.kill(true),
						MouseButton::Left => cell.live(true),
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

	fn set_pause(&mut self, paused: bool) {
		self.draw_grid = paused;
		self.paused = paused;
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

	fn update(&mut self, animations: bool) {

		// if self.paused || self.last_update.elapsed().as_millis() as f32 / 1000.0 < self.speed {
		if self.paused || macroquad::miniquad::date::now() - self.last_update < self.speed {

			for x in 0..self.amount.amount_x {
				for y in 0..self.amount.amount_y {
					self.get_cell_mut(x, y).unwrap().update();
				}
			}

		} else {
			self.last_update = macroquad::miniquad::date::now();
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
								cell.kill(animations);
							} else if amount > 3 {
								cell.kill(animations);
							}
	
						},
						CellState::Dead => {
							if amount == 3 {
								cell.live(animations);
								continue;
							}
						}
					}
	
					
					
				}
			}
		}


	}

}

struct Settings {
	mouse_over: bool,
	speed: f64,
	animate_while_sim: bool,
	size: f32,
	clear_screen: bool,
	paused: bool,
	swap_buttons: bool,
}

impl Settings {

	fn new() -> Self {
		return Self {
			mouse_over: false,
			speed: 0.75,
			animate_while_sim: true,
			size: 20.0,
			clear_screen: false,
			paused: true,
			swap_buttons: false,
		};
	}

	fn draw(&mut self) {
		egui_macroquad::ui(|ctx| {
			self.mouse_over = ctx.is_pointer_over_area() || ctx.is_using_pointer();
			egui::Window::new("Menu")
				.title_bar(true)
				.default_pos(egui::pos2(0.0, 0.0))
				.show(ctx, |ui| {

					ui.horizontal(|ui| {
						if ui.button("Play/pause").clicked() {
							self.paused = !self.paused;
						}
						if ui.button("Clear screen").clicked() {
							self.clear_screen = true;
						}
						if ui.button("Swap").clicked() {
							self.swap_buttons = !self.swap_buttons;
						}
					});

					ui.heading("Options");

					ui.spacing_mut().slider_width = 200.0;

					ui.add(egui::Slider::new(&mut self.speed, 0.05..=1.0).text("Speed").show_value(false));
					ui.add(egui::Slider::new(&mut self.size, 10.0..=30.0).text("Size").show_value(false));

					ui.checkbox(&mut self.animate_while_sim, "Animations while simulating");

					ui.heading("Controls");

					let controls = 
"Space to play/pause
Left click to create
Right click to remove
Q to clear screen";
					for i in controls.split("\n") {
						ui.label(i);
					}


				});
		});
		egui_macroquad::draw();

	}
}

#[macroquad::main(window_conf)]
async fn main() {

	let mut game = Game::new(20.0);
	let mut settings = Settings::new();

	let mut last_mouse_pos: Option<(f32, f32)> = None;

	loop {

		clear_background(BLACK);

		if settings.size != game.get_cell(0, 0).unwrap().w {
			game = Game::new(settings.size);
		}
		if settings.clear_screen || is_key_pressed(KeyCode::Q) {
			settings.clear_screen = false;
			game = Game::new(settings.size);
		}

		let pos = mouse_position();
		if !settings.mouse_over {
			if is_mouse_button_down(MouseButton::Left) {

				let wanted = if !settings.swap_buttons {
					MouseButton::Left
				} else {
					MouseButton::Right
				};

				if last_mouse_pos.is_some() {
					let points = plot_line(pos.0, pos.1, last_mouse_pos.unwrap().0, last_mouse_pos.unwrap().1);
					for i in points {
						game.handle_mouse(i.x as f32, i.y as f32, wanted);
					}
				}

				game.handle_mouse(pos.0, pos.1, wanted);
				last_mouse_pos = Some(pos);
			} else if is_mouse_button_down(MouseButton::Right) {

				let wanted = if !settings.swap_buttons {
					MouseButton::Right
				} else {
					MouseButton::Left
				};

				if last_mouse_pos.is_some() {
					let points = plot_line(pos.0, pos.1, last_mouse_pos.unwrap().0, last_mouse_pos.unwrap().1);
					for i in points {
						game.handle_mouse(i.x as f32, i.y as f32, wanted);
					}
				}

				game.handle_mouse(pos.0, pos.1, wanted);
				last_mouse_pos = Some(pos);
			} else {
				last_mouse_pos = None;
			}
		}

		if is_key_pressed(KeyCode::Space) {
			game.toggle_pause();
			settings.paused = game.paused;
		}
		game.set_pause(settings.paused);

		game.speed = 1.0 - settings.speed;
		game.update(settings.animate_while_sim);
		game.draw();
		settings.draw();

		next_frame().await;

	}


}

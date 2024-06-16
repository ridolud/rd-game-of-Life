use ggez::{conf::WindowMode, event::{self, EventHandler}, graphics::{self, Color, DrawMode, Mesh, Rect}, mint::Point2, timer, ContextBuilder, GameError, GameResult};

const CELL_SIZE: f32 = 20.0;
const GRID_SIZE: (f32, f32) = (120.0, 80.0);
const WINDOW_SIZE: (f32, f32) = (CELL_SIZE * GRID_SIZE.0, CELL_SIZE * GRID_SIZE.1);

#[allow(dead_code)]
pub struct State {
    grid: Vec<Vec<bool>>,
    fps: u32,
    running: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            grid: vec![vec![false; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize],
            fps: 10,
            running: true,
        }
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        while timer::check_update_time(ctx, self.fps) && self.running {
            let mut coordinates: Vec<(usize, usize)> = vec![];
            for x in 0..GRID_SIZE.0 as usize {
                let left = if x > 0 { x - 1 } else { GRID_SIZE.0 as usize - 1 };
                let right = if x < GRID_SIZE.0 as usize - 1 { x + 1 } else { 0 };

                for y in 0..GRID_SIZE.1 as usize {
                    let up = if y > 0 { y - 1 } else { GRID_SIZE.1 as usize - 1 };
                    let down = if y < GRID_SIZE.1 as usize - 1 { y + 1 } else { 0 };

                    let neighbors = 
                        self.grid[left][y] as u8
                        + self.grid[left][up] as u8
                        + self.grid[x][up] as u8
                        + self.grid[right][up] as u8
                        + self.grid[right][y] as u8
                        + self.grid[right][down] as u8
                        + self.grid[x][down] as u8
                        + self.grid[left][down] as u8;

                    if self.grid[x][y] && (neighbors < 2 || neighbors > 3) {
                        coordinates.push((x, y));
                    } else if !self.grid[x][y] && neighbors == 3 {
                        coordinates.push((x,y))
                    }
                }
            }

            for coordinate in coordinates  {
                self.grid[coordinate.0][coordinate.1] ^= true;
            }
        }

        Ok(())    
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let bg_color = if self.running { Color::WHITE } else { Color::from_rgb(220, 220, 220) };
        let cell_color = if self.running { Color::BLACK } else { Color::from_rgb(50, 50, 50) };

        graphics::clear(ctx, bg_color);

        for x in 0..GRID_SIZE.0 as usize  {
            for y in 0..GRID_SIZE.1 as usize {
                if self.grid[x][y] {
                    let cell = Mesh::new_rectangle(
                        ctx, 
                        DrawMode::fill(),
                        Rect::new(x as f32 * CELL_SIZE , y as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE ), 
                        cell_color
                    )?;

                    graphics::draw(
                        ctx, 
                        &cell, 
                        graphics::DrawParam::default().dest(Point2 { x: 0.0, y: 0.0 })
                    )?;
                }
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }
    
    fn key_down_event(
            &mut self,
            _ctx: &mut ggez::Context,
            keycode: event::KeyCode,
            _keymods: event::KeyMods,
            repeat: bool,
        ) {
        if keycode == event::KeyCode::Space && !repeat {
            self.running ^= true
        }

        match keycode {
            event::KeyCode::Up => self.fps = if self.fps >= 80 { 80 } else { self.fps + 10 },
            event::KeyCode::Down => self.fps = if self.fps == 10 { 10 } else { self.fps - 10 },
            event::KeyCode::Delete => self.grid = State::new().grid,
            _ => {}
        }
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut ggez::Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) {
        self.running = false;
    }

    fn mouse_button_up_event(
            &mut self,
            _ctx: &mut ggez::Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) {
        self.running = true;
    }

    fn mouse_motion_event(&mut self, _ctx: &mut ggez::Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        if self.running { return };

        let x_coor = (x / CELL_SIZE).floor() as usize;
        let y_coor = (y / CELL_SIZE).floor() as usize;

        if x_coor < self.grid.len() && y_coor < self.grid[0].len() {
            self.grid[x_coor][y_coor] = true;
        }
    }
}

fn main() -> GameResult {
    let state = State::new();

    let (ctx, event_loop) = ContextBuilder::new("Conway\'s Game of Life", "ridolud")
        .window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
        .build()?;

    event::run(ctx, event_loop, state);
}
use std::{
    io::stdin,
    time::{Duration, Instant},
};

use game_of_life::Rgba;
use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

fn main() {
    let mut width = String::new();
    let mut height = String::new();
    let mut tick_len = String::new();

    println!("Enter Width: ");
    stdin().read_line(&mut width).unwrap();
    println!("Enter Hight: ");
    stdin().read_line(&mut height).unwrap();
    println!("Enter Tick Length (seconds): ");
    stdin().read_line(&mut tick_len).unwrap();

    let width: isize = width.trim().parse().unwrap();
    let height: isize = height.trim().parse().unwrap();
    let tick_len: f64 = tick_len.trim().parse().unwrap();

    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();
    let size = Size::new(width, height);
    let window = WindowBuilder::new()
        .with_title("Game of Life")
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)
        .unwrap();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(size.w as u32, size.h as u32, surface_texture).unwrap()
    };

    let mut world = World::new(size, Duration::from_secs_f64(tick_len));
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        if let Event::RedrawRequested(_) = event {
            draw(&world, pixels.frame_mut());

            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        };
        if let Event::MainEventsCleared = event {
            world.update();
            window.request_redraw();
        }
        if input.update(&event) {
            if input.key_released(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Space) {
                world.toggle_running();
            }
            if input.key_pressed(VirtualKeyCode::C) {
                world.pause();
                world.cells.clear();
            }
            if input.key_pressed(VirtualKeyCode::S) {
                world.tick_len += Duration::from_secs_f64(0.05);
            }
            if input.key_pressed(VirtualKeyCode::W) {
                if world.tick_len > Duration::from_secs_f64(0.05) {
                    world.tick_len -= Duration::from_secs_f64(0.05);
                } else if world.tick_len > Duration::from_secs_f64(0.001) {
                    world.tick_len -= Duration::from_secs_f64(0.001);
                }
            }

            let (mx, my): (isize, isize) = input
                .mouse()
                .map(|(mx, my)| {
                    // let (dx, dy) = input.mouse_diff();
                    let (mx_i, my_i) = pixels
                        .window_pos_to_pixel((mx, my))
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));
                    (mx_i as isize, my_i as isize)
                })
                .unwrap_or_default();

            if (input.mouse_pressed(0) || input.mouse_held(0)) && !world.cells.is_alive(my, mx) {
                world.cells.make_alive(my, mx);
            }

            if let Some(size) = input.window_resized() {
                if pixels.resize_surface(size.width, size.height).is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                };
            }

            world.update();
        }
    });
}

struct Size<T> {
    w: T,
    h: T,
}

impl<T> Size<T> {
    fn new(w: T, h: T) -> Self {
        Self { w, h }
    }
}

#[derive(Clone)]
enum Cell {
    Dead,
    Alive,
}

impl Cell {
    fn filp(&mut self) {
        *self = match self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}

struct Cells {
    vec: Vec<Vec<Cell>>,
}

impl Cells {
    // returns 1 if cell is alive else 0
    fn is_alive_usize(&self, i: isize, j: isize) -> usize {
        match self.vec.get(i as usize) {
            Some(row) => match row.get(j as usize) {
                Some(c) => match c {
                    Cell::Alive => 1,
                    Cell::Dead => 0,
                },
                None => 0,
            },
            None => 0,
        }
    }
    fn is_alive(&self, i: isize, j: isize) -> bool {
        self.is_alive_usize(i, j) == 1
    }
    fn make_alive(&mut self, i: isize, j: isize) {
        match self.vec.get_mut(i as usize) {
            Some(row) => match row.get_mut(j as usize) {
                Some(c) => match c {
                    Cell::Alive => (),
                    Cell::Dead => *c = Cell::Alive,
                },
                None => (),
            },
            None => (),
        };
    }
    fn make_dead(&mut self, i: isize, j: isize) {
        match self.vec.get_mut(i as usize) {
            Some(row) => match row.get_mut(j as usize) {
                Some(c) => match c {
                    Cell::Alive => *c = Cell::Dead,
                    Cell::Dead => (),
                },
                None => (),
            },
            None => (),
        };
    }
    fn alive_neighbors(&self, i: isize, j: isize) -> usize {
        let mut count = 0;
        count += self.is_alive_usize(i - 1, j - 1);
        count += self.is_alive_usize(i - 1, j - 0);
        count += self.is_alive_usize(i - 1, j + 1);
        count += self.is_alive_usize(i - 0, j - 1);
        count += self.is_alive_usize(i - 0, j + 1);
        count += self.is_alive_usize(i + 1, j - 1);
        count += self.is_alive_usize(i + 1, j - 0);
        count += self.is_alive_usize(i + 1, j + 1);

        count
    }
    fn clear(&mut self) {
        let row_len = self.vec[0].len();
        for i in 0..self.vec.len() {
            for j in 0..row_len {
                self.make_dead(i as isize, j as isize);
            }
        }
    }
}

enum WorldState {
    Initial,
    Running,
    Paused,
}
struct World {
    tick: Instant,
    tick_len: Duration,
    state: WorldState,
    size: Size<isize>,
    cells: Cells,
}

impl World {
    fn new(size: Size<isize>, tick_len: Duration) -> Self {
        let mut cells_vec: Vec<Vec<Cell>> =
            vec![vec![Cell::Dead; size.w as usize]; size.h as usize];

        let mut rng = rand::thread_rng();
        for i in cells_vec.iter_mut() {
            for j in i.iter_mut() {
                if rng.gen::<u8>() < 10 {
                    *j = Cell::Alive;
                }
            }
        }

        Self {
            tick: Instant::now(),
            tick_len,
            state: WorldState::Paused,
            size,
            cells: Cells { vec: cells_vec },
        }
    }
    fn update(&mut self) {
        if let WorldState::Paused = self.state {
            self.tick = Instant::now();
            return;
        }
        if Instant::now() - self.tick < self.tick_len {
            return;
        }
        self.tick = Instant::now();

        let row_len = self.cells.vec[0].len();
        let mut to_flip: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.cells.vec.len() {
            for j in 0..row_len {
                let alive_neighbors = self.cells.alive_neighbors(i as isize, j as isize);
                if alive_neighbors < 2 {
                    if let Cell::Alive = self.cells.vec[i][j] {
                        to_flip.push((i, j));
                    }
                } else if alive_neighbors > 3 {
                    if let Cell::Alive = self.cells.vec[i][j] {
                        to_flip.push((i, j));
                    }
                } else if alive_neighbors == 3 {
                    if let Cell::Dead = self.cells.vec[i][j] {
                        to_flip.push((i, j));
                    }
                }
            }
        }
        for (i, j) in to_flip {
            self.cells.vec[i][j].filp();
        }
    }
    fn toggle_running(&mut self) {
        if let WorldState::Paused = self.state {
            self.run();
        } else {
            self.pause();
        }
    }
    fn pause(&mut self) {
        self.state = WorldState::Paused;
    }
    fn run(&mut self) {
        self.state = WorldState::Running;
    }
}

fn draw(world: &World, frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i as isize % world.size.w as isize;
        let y = i as isize / world.size.w as isize;

        if world.cells.is_alive(y as isize, x as isize) {
            pixel.copy_from_slice(&Rgba::WHITE.as_slice());
        } else {
            pixel.copy_from_slice(&Rgba::BLACK.as_slice());
        }
    }
}
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
use framebuffer::Framebuffer;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const GRID_WIDTH: usize = 80;
const GRID_HEIGHT: usize = 60;
const CELL_SIZE: usize = 10;

fn render(framebuffer: &mut Framebuffer, grid: &Vec<Vec<bool>>) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if grid[y][x] {
                framebuffer.set_current_color(0xFFFFFF);
            } else {
                framebuffer.set_current_color(0x000000);
            }
            framebuffer.point(x * CELL_SIZE, y * CELL_SIZE);
        }
    }
}

fn update_grid(grid: &mut Vec<Vec<bool>>) {
    let mut new_grid = grid.clone();
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let mut live_neighbors = 0;
            for j in (y as isize - 1)..=(y as isize + 1) {
                for i in (x as isize - 1)..=(x as isize + 1) {
                    if j >= 0 && j < GRID_HEIGHT as isize && i >= 0 && i < GRID_WIDTH as isize && !(i == x as isize && j == y as isize) {
                        if grid[j as usize][i as usize] {
                            live_neighbors += 1;
                        }
                    }
                }
            }

            new_grid[y][x] = match (grid[y][x], live_neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }
    *grid = new_grid;
}

fn main() {
    let frame_delay = Duration::from_millis(100);
    let mut grid = vec![vec![false; GRID_WIDTH]; GRID_HEIGHT];
    // Initialize with a simple pattern
    grid[1][2] = true;
    grid[2][3] = true;
    grid[3][1] = true;
    grid[3][2] = true;
    grid[3][3] = true;

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut window = Window::new(
        "Conway's Game of Life",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Update the game state
        update_grid(&mut grid);

        // Render the current state of the game
        render(&mut framebuffer, &grid);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(framebuffer.get_buffer(), WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

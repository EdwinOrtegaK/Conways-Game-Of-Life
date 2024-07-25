use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
use framebuffer::Framebuffer;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const CELL_SIZE: usize = 5;
const GRID_WIDTH: usize = WIDTH / CELL_SIZE;
const GRID_HEIGHT: usize = HEIGHT / CELL_SIZE;

fn render(framebuffer: &mut Framebuffer, grid: &Vec<Vec<bool>>) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if grid[y][x] {
                framebuffer.set_current_color(0xFFFFFF); // Célula viva: blanca
            } else {
                framebuffer.set_current_color(0x000000); // Célula muerta: negra
            }
            for i in 0..CELL_SIZE {
                for j in 0..CELL_SIZE {
                    framebuffer.point(x * CELL_SIZE + i, y * CELL_SIZE + j);
                }
            }
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
                    if !(i == x as isize && j == y as isize) {
                        let neighbor_x = (i + GRID_WIDTH as isize) % GRID_WIDTH as isize;
                        let neighbor_y = (j + GRID_HEIGHT as isize) % GRID_HEIGHT as isize;
                        if grid[neighbor_y as usize][neighbor_x as usize] {
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

fn initialize_pattern(grid: &mut Vec<Vec<bool>>, pattern: &Vec<usize>, offset_x: usize, offset_y: usize) {
    for &index in pattern {
        let x = (index - 1) % 5;
        let y = (index - 1) / 5;
        grid[y + offset_y][x + offset_x] = true;
    }
}

fn main() {
    let frame_delay = Duration::from_millis(100);
    let mut grid = vec![vec![false; GRID_WIDTH]; GRID_HEIGHT];

    // Patrón inicial para Unown A
    let unown_a = vec![3, 7, 8, 9, 11, 15, 16, 20, 22, 23, 24, 27, 29, 31, 32, 34, 35];

    // Patrón inicial para Unown B
    let unown_b = vec![2, 3, 4, 6, 10, 11, 13, 15, 16, 20, 22, 23, 24, 26, 30, 32, 33, 34];

    // Patrón inicial para Unown C
    let unown_c = vec![2, 3, 7, 12, 17, 22, 27, 28];

    // Patrón inicial para Unown D
    let unown_d = vec![2, 3, 4, 5, 7, 11, 13, 15, 17, 21, 26, 27, 28, 29];

    // Patrón inicial para Unown E
    let unown_e = vec![2, 3, 4, 5, 6, 8, 14, 15, 16, 17, 20, 26, 27, 28, 29, 30];

    // Patrón inicial para Unown F
    let unown_f = vec![2, 3, 4, 5, 6, 8, 14, 15, 16, 17, 20, 26];

    // Patrón inicial para Unown G
    let unown_g = vec![2, 3, 4, 6, 10, 12, 16, 17, 18, 22, 27, 28, 29];

    // Inicializar letra A cerca del centro
    initialize_pattern(&mut grid, &unown_a, GRID_WIDTH / 2 - 15, GRID_HEIGHT / 2 - 3);

    // Inicializar letra B cerca del centro
    initialize_pattern(&mut grid, &unown_b, GRID_WIDTH / 2 - 7, GRID_HEIGHT / 2 - 3);

    // Inicializar letra C cerca del centro
    initialize_pattern(&mut grid, &unown_c, GRID_WIDTH / 2 + 1, GRID_HEIGHT / 2 - 3);

    // Inicializar letra D cerca del centro
    initialize_pattern(&mut grid, &unown_d, GRID_WIDTH / 2 + 9, GRID_HEIGHT / 2 - 3);

    // Inicializar letra E cerca del centro
    initialize_pattern(&mut grid, &unown_e, GRID_WIDTH / 2 + 17, GRID_HEIGHT / 2 - 3);

    // Inicializar letra F en la parte inferior izquierda
    initialize_pattern(&mut grid, &unown_f, GRID_WIDTH / 2 - 15, GRID_HEIGHT / 2 + 10);

    // Inicializar letra G en la parte inferior derecha
    initialize_pattern(&mut grid, &unown_g, GRID_WIDTH / 2 - 7, GRID_HEIGHT / 2 + 10);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut window = Window::new(
        "Conway's Game of Life - Unown Alphabet",
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

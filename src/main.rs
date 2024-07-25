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

    // Patrón inicial para Unown H
    let unown_h = vec![2, 3, 6, 8, 11, 13, 15, 18, 21, 23, 26, 28, 31, 32];

    // Patrón inicial para Unown I
    let unown_i = vec![2, 3, 4, 5, 7, 13, 19, 25, 27, 28, 29];

    // Patrón inicial para Unown J
    let unown_j = vec![3, 4, 5, 7, 13, 19, 25, 30, 33, 35];

    // Patrón inicial para Unown K
    let unown_k = vec![2, 6, 7, 10, 12, 13, 17, 20, 22, 26, 27, 28];

    // Patrón inicial para Unown L
    let unown_l = vec![2, 6, 7, 11, 12, 16, 20, 22, 23, 24, 25, 26];

    // Patrón inicial para Unown M
    let unown_m = vec![2, 6, 8, 12, 14, 18, 22, 26, 28, 32, 33];
    
    // Patrón inicial para Unown N
    let unown_n = vec![2, 3, 4, 5, 6, 8, 12, 16, 20, 22, 26, 30];

    // Patrón inicial para Unown O
    let unown_o = vec![2, 3, 4, 7, 9, 12, 14, 17, 19, 22, 24, 27, 28, 29];

    // Patrón inicial para Unown P
    let unown_p = vec![2, 3, 4, 7, 9, 12, 13, 16, 20, 21, 25];

    // Patrón inicial para Unown Q
    let unown_q = vec![2, 3, 4, 7, 9, 12, 14, 17, 19, 22, 24, 27, 28, 29, 34];

    // Patrón inicial para Unown R
    let unown_r = vec![2, 3, 4, 7, 9, 12, 13, 14, 17, 19, 22, 23, 26];

    // Patrón inicial para Unown S
    let unown_s = vec![2, 3, 4, 6, 12, 13, 14, 19, 25, 27, 28, 29];

    // Patrón inicial para Unown T
    let unown_t = vec![2, 3, 4, 5, 7, 13, 19, 25, 27];

    // Patrón inicial para Unown U
    let unown_u = vec![2, 6, 10, 12, 16, 20, 22, 26, 27, 28];

    // Patrón inicial para Unown V
    let unown_v = vec![2, 6, 10, 12, 16, 20, 23, 24, 25];

    // Patrón inicial para Unown W
    let unown_w = vec![2, 6, 10, 12, 16, 18, 20, 21, 22, 23, 24, 25];

    // Patrón inicial para Unown X
    let unown_x = vec![2, 6, 10, 13, 15, 16, 20, 22, 26, 27, 31];
    
    // Patrón inicial para Unown Y
    let unown_y = vec![2, 6, 8, 12, 16, 20, 22, 26, 27, 31];

    // Patrón inicial para Unown Z
    let unown_z = vec![2, 6, 8, 11, 13, 16, 18, 21, 23, 27, 29, 34];

    // Inicializar letras en el centro
    initialize_pattern(&mut grid, &unown_a, GRID_WIDTH / 2 - 12, GRID_HEIGHT / 2 - 7);
    initialize_pattern(&mut grid, &unown_b, GRID_WIDTH / 2 - 5, GRID_HEIGHT / 2 - 7);
    initialize_pattern(&mut grid, &unown_c, GRID_WIDTH / 2 + 2, GRID_HEIGHT / 2 - 7);
    initialize_pattern(&mut grid, &unown_d, GRID_WIDTH / 2 + 9, GRID_HEIGHT / 2 - 7);
    initialize_pattern(&mut grid, &unown_e, GRID_WIDTH / 2 + 16, GRID_HEIGHT / 2 - 7);
    initialize_pattern(&mut grid, &unown_f, GRID_WIDTH / 2 - 12, GRID_HEIGHT / 2);
    initialize_pattern(&mut grid, &unown_g, GRID_WIDTH / 2 - 5, GRID_HEIGHT / 2);
    initialize_pattern(&mut grid, &unown_h, GRID_WIDTH / 2 + 2, GRID_HEIGHT / 2);
    initialize_pattern(&mut grid, &unown_i, GRID_WIDTH / 2 + 9, GRID_HEIGHT / 2);
    initialize_pattern(&mut grid, &unown_j, GRID_WIDTH / 2 + 16, GRID_HEIGHT / 2);
    initialize_pattern(&mut grid, &unown_k, GRID_WIDTH / 2 - 12, GRID_HEIGHT / 2 + 7);
    initialize_pattern(&mut grid, &unown_l, GRID_WIDTH / 2 - 5, GRID_HEIGHT / 2 + 7);
    initialize_pattern(&mut grid, &unown_m, GRID_WIDTH / 2 + 2, GRID_HEIGHT / 2 + 7);
    initialize_pattern(&mut grid, &unown_n, GRID_WIDTH / 2 + 9, GRID_HEIGHT / 2 + 7);
    initialize_pattern(&mut grid, &unown_o, GRID_WIDTH / 2 + 16, GRID_HEIGHT / 2 + 7);
    initialize_pattern(&mut grid, &unown_p, GRID_WIDTH / 2 - 12, GRID_HEIGHT / 2 + 14);
    initialize_pattern(&mut grid, &unown_q, GRID_WIDTH / 2 - 5, GRID_HEIGHT / 2 + 14);
    initialize_pattern(&mut grid, &unown_r, GRID_WIDTH / 2 + 2, GRID_HEIGHT / 2 + 14);
    initialize_pattern(&mut grid, &unown_s, GRID_WIDTH / 2 + 9, GRID_HEIGHT / 2 + 14);
    initialize_pattern(&mut grid, &unown_t, GRID_WIDTH / 2 + 16, GRID_HEIGHT / 2 + 14);
    initialize_pattern(&mut grid, &unown_u, GRID_WIDTH / 2 - 12, GRID_HEIGHT / 2 + 21);
    initialize_pattern(&mut grid, &unown_v, GRID_WIDTH / 2 - 5, GRID_HEIGHT / 2 + 21);
    initialize_pattern(&mut grid, &unown_w, GRID_WIDTH / 2 + 2, GRID_HEIGHT / 2 + 21);
    initialize_pattern(&mut grid, &unown_x, GRID_WIDTH / 2 + 9, GRID_HEIGHT / 2 + 21);
    initialize_pattern(&mut grid, &unown_y, GRID_WIDTH / 2 + 16, GRID_HEIGHT / 2 + 21);
    initialize_pattern(&mut grid, &unown_z, GRID_WIDTH / 2 - 12, GRID_HEIGHT / 2 + 28);

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

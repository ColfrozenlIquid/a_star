use egui::Color32;
use rand::Rng;
use crate::app::Coordinate;

pub fn generate_grid(columns: i32, rows: i32) -> Vec<Vec<Color32>> {
    let mut grid: Vec<Vec<Color32>> = Vec::new();
    for _y in 0..rows {
        let mut temp: Vec<Color32> = Vec::new();
        for _x in 0..columns {
            temp.push(Color32::GRAY);
        }
        grid.push(temp);
    }
    return grid
}

pub fn populate_random_grid_obstacles(grid: &mut Vec<Vec<Color32>>, rows: i32, columns: i32, density: f64) -> Vec<Vec<bool>>{
    let mut obstacles: Vec<Vec<bool>> = Vec::new();
    for _y in 0..rows {
        let mut temp: Vec<bool> = Vec::new();
        for _x in 0..columns {
            let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
            let rng_gen: f64 = rng.gen();
            if rng_gen < density {
                temp.push(true);
                grid[_y as usize][_x as usize] = Color32::DARK_GRAY;
            } else {
                temp.push(false);
            }
        }
        obstacles.push(temp);
    }
    return obstacles
}

pub fn set_grid_coordinate(grid: &mut Vec<Vec<Color32>>, coordinate: &Coordinate, colour: Color32) {
    coordinate.x;
    grid[coordinate.y as usize][coordinate.x as usize] = colour;
}
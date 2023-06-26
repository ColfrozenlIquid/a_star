use std::collections::BinaryHeap;
use std::cmp::Ordering;
use egui::Color32;

pub struct Cell {
    x: i32, y: i32,
    f: f64, g: f64, h: f64,
    obstacle: bool,
    visited: bool,
    path: bool,
    neighbours: Vec<[i32;2]>,
    parent: [i32;2],
}

#[derive(PartialEq)]
struct Node {
    x: i32, y: i32,
    f: f64,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Node {
    fn assert_receiver_is_total_eq(&self) {}
}

pub fn a_star_search(grid: &mut Vec<Vec<Color32>> , obstacles: &Vec<Vec<bool>>, rows: usize, columns: usize){
    let mut adjacency_list: Vec<Vec<Cell>> = generate_adjacency_list(obstacles, rows as i32, columns as i32);
    let mut closed_list: Vec<Vec<bool>> = generate_closed_list(rows, columns);
    let mut open_list: BinaryHeap<Node> = BinaryHeap::<Node>::new();

    let start_node: Node = Node{ f: 0.0, x: 0, y: 0 };
    let destination_node: Node = Node {f: f64::MAX, x: columns as i32 - 1, y: rows as i32 - 1};

    adjacency_list[0][0].f = 0.0;
    adjacency_list[0][0].g = 0.0;
    adjacency_list[0][0].h = f64::MAX;
    adjacency_list[0][0].parent = [start_node.x, start_node.y];
    
    open_list.push(start_node);

    while !open_list.is_empty() {
        let current_node: Node = open_list.pop().unwrap();

        closed_list[current_node.x as usize][current_node.y as usize] = true;

        let mut g_new: f64;
        let mut h_new: f64;
        let mut f_new: f64;
        
        let neighbours: Vec<[i32; 2]> = adjacency_list[current_node.x as usize][current_node.y as usize].neighbours.clone();
        
        for neighbour in neighbours {
            if neighbour == [destination_node.x, destination_node.y] {
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].f = f64::MAX;
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].g = f64::MAX;
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].h = 0.0;
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].parent = [current_node.x, current_node.y];
                let path: Vec<[i32; 2]> = generate_path(&adjacency_list, &destination_node, [0,0]);
                for x in &path {
                    grid[x[0] as usize][x[1] as usize] = Color32::GOLD;
                    adjacency_list[x[0] as usize][x[1] as usize].path = true;
                }
                return;
            }
            if closed_list[neighbour[0] as usize][neighbour[1] as usize] == true{
                continue;
            }
            if adjacency_list[neighbour[0] as usize][neighbour[1] as usize].obstacle {
                closed_list[neighbour[0] as usize][neighbour[1] as usize] = true;
                continue;
            }
            
            g_new = adjacency_list[current_node.x as usize][current_node.y as usize].g + 1.0;
            h_new = calculate_h_value(neighbour[0], neighbour[1], &destination_node);
            f_new = g_new + h_new;

            let adj_f = adjacency_list[neighbour[0] as usize][neighbour[1] as usize].f.clone();

            if adj_f == f64::MAX || adj_f > f_new {
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].f = f_new;
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].g = g_new;
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].h = h_new;
                open_list.push( Node {f: f_new, x: neighbour[0], y: neighbour[1]});
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].parent = [current_node.x, current_node.y];
            }
            closed_list[neighbour[0] as usize][neighbour[1] as usize] = true
        }
    }
    panic!()
}

fn generate_path(adjacency_list: &Vec<Vec<Cell>>, destination_node: &Node, start_node: [i32;2]) -> Vec<[i32;2]> {
    let mut path: Vec<[i32;2]> = Vec::new();
    let mut current = [destination_node.x, destination_node.y];

    while current != start_node {
        for neighbour in &adjacency_list[current[0] as usize][current[1] as usize].neighbours{
            if &adjacency_list[neighbour[0] as usize][neighbour[1] as usize].g < &adjacency_list[current[0] as usize][current[1] as usize].g {
                path.push(*neighbour);
                current = *neighbour;
                break;
            }
        }
    }
    return path;
}

fn generate_adjacency_list(obstacles: &Vec<Vec<bool>>, rows: i32, columns: i32) -> Vec<Vec<Cell>>{
    let mut adjacency_list: Vec<Vec<Cell>> = Vec::new();
    for x in 0..rows {
        let mut row: Vec<Cell> = Vec::new();
        for y in 0..columns {
            let mut neighbours: Vec<[i32;2]> = Vec::new();
            
            if x - 1 >= 0 {
                neighbours.push([x-1, y])
            }
            if x + 1 <= rows-1 {
                neighbours.push([x+1, y])
            }
            if y - 1 >= 0 {
                neighbours.push([x, y-1])
            }
            if y + 1 <= columns-1 {
                neighbours.push([x, y+1])
            }

            let mut obstacle_gen = false;
            if obstacles[x as usize][y as usize] {
                obstacle_gen = true;
            }

            let cell: Cell = Cell { 
                obstacle: obstacle_gen, 
                visited: false,
                path: false,
                x, y,
                f: f64::MAX, g: f64::MAX, h: f64::MAX, 
                neighbours, 
                parent: [0, 0]
            };
            row.push(cell);
        }
        adjacency_list.push(row);
    }
    return adjacency_list;
}

fn generate_closed_list(rows: usize, columns: usize) -> Vec<Vec<bool>> {
    let mut closed_list: Vec<Vec<bool>> = Vec::new();
    for _y in 0..rows {
        let mut row: Vec<bool> = Vec::new();
        for _x in 0..columns {
            row.push(false);
        }
        closed_list.push(row);
    }
    closed_list
}

fn calculate_h_value(x: i32, y: i32, destination_node: &Node) -> f64 {
    //let number = (x-destination_node.x) * (x-destination_node.x) + (y-destination_node.y) * (y-destination_node.y);
    //f64::sqrt(number as f64)
    let number = (x-destination_node.x).abs() + (y-destination_node.y).abs();
    return number as f64;
}
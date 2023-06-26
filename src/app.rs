use egui::{Rounding, Color32, Vec2};
use crate::generate_2d_grid::{self, set_grid_coordinate};
use crate::a_star;
use crate::toggle_switch::{toggle};

const ROW: i32 = 94;
const COLUMN: i32 = 94;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    density: f64,
    value: f32,
    #[serde(skip)]
    grid: Vec<Vec<Color32>>,
    #[serde(skip)]
    show_labels: bool,

    #[serde(skip)]
    obstacles: Vec<Vec<bool>>,

    #[serde(skip)]
    start_tile: Coordinate,
    #[serde(skip)]
    end_tile: Coordinate,
    
    #[serde(skip)]
    rows: usize,
    #[serde(skip)]
    columns: usize,
}

#[derive(Default)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            density: 0.1,
            value: 0.45,
            grid: generate_2d_grid::generate_grid(COLUMN, ROW),
            show_labels: false,
            rows: ROW as usize,
            columns: COLUMN as usize,
            start_tile: Coordinate::default(),
            end_tile: Coordinate::default(),
            obstacles: Vec::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            value,
            density,
            grid, 
            show_labels, 
            rows,
            columns,
            start_tile,
            end_tile, 
            obstacles,
        } = self;

        //*grid = generate_2d_grid::generate_grid(*columns as i32, *rows as i32);

        *start_tile = Coordinate::from(Coordinate { x: 0, y: 0 });
        *end_tile = Coordinate::from(Coordinate { x: *columns as i32 - 1, y: *rows as i32 - 1 });
        set_grid_coordinate(grid, start_tile, Color32::GREEN);
        set_grid_coordinate(grid, end_tile, Color32::RED);

        egui::SidePanel::left("side_panel").show(ctx, |ui| {

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("Grid Size"));

            ui.add(toggle(show_labels));

            ui.add(egui::Slider::new(density, 0.0..=1.0).text("Obstacle density"));

            if ui.button("Generate obstacles").clicked() {
                *grid = generate_2d_grid::generate_grid(*columns as i32, *rows as i32);
                *obstacles = generate_2d_grid::populate_random_grid_obstacles(grid, *rows as i32, *columns as i32, *density);
            }

            if ui.button("Generate Path").clicked() {
                a_star::a_star_search(grid, obstacles, *rows, *columns);
            }
        });

        let frame: egui::CentralPanel = egui::CentralPanel::default();
        frame.show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            let mut _grid = egui::Grid::new("some_unique_id")
                .spacing(Vec2::new(1.5, 1.5))
                .min_col_width(0.05)
                .min_row_height(0.05);

            _grid.show(ui, |ui| {
                for x in 0..*columns {
                    for y in 0..*rows {
                        let (rect, _response) = ui.allocate_exact_size(Vec2::new(*value * 15.0, *value * 15.0), egui::Sense::click());
                        ui.painter().rect(
                            rect,
                            Rounding::default(),
                            grid[y][x],
                            egui::Stroke::default(),
                        );
                    };
                    ui.end_row();
                }
            });
        });
    }
}
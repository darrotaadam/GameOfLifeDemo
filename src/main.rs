use macroquad::prelude;
use std::collections::HashMap;
use std::collections::HashSet;

use macroquad::input::mouse_position;
use macroquad::math::{clamp, Vec2};

const ZOOM_WHEEL_SENSITIVITY: f32 = 0.1;

const BACKGROUND_COLOR: prelude::Color = prelude::Color::from_hex(0x424242);
const ALIVE_CELL_COLOR: prelude::Color = prelude::Color::from_hex(0xEBE8E8);



static PADDING:i32 = 4;
static CELL_SIZE:i32 = 8;

type Cell = (i32, i32);


fn find_neighbors(alive_cells: &HashSet<Cell>  ) -> HashMap<Cell,u32>
{
    let mut neighbors_count: HashMap<Cell,u32> = HashMap::new();

    for &(x,y) in alive_cells {
        for dx in -1..=1 { // ..=1 ça inclut 1
            for dy in -1..=1 {

                if dx == 0 && dy == 0 {continue};

                //if cells.contains(&( x+dx , y+dy ) ){
                *neighbors_count.entry((x+dx, y+dy)).or_insert(0) +=1 ;
                //}
            }
        }
    }

    neighbors_count
}

fn print_hashset(cells : &HashSet<Cell>) {
    println!("Cells HashSet:");
    for cell in cells{
        println!("Cell: ({}, {})", cell.0, cell.1);
    }
}

fn next_generation(live_cells: &HashSet<Cell> ) -> HashSet<Cell>{
    let mut next_gen_cells : HashSet<Cell>= HashSet::new();

    let neighbors = find_neighbors(&live_cells);

    for (cell, neighbor_count) in neighbors{
        if neighbor_count == 3 || (neighbor_count == 2 && live_cells.contains(&cell)) {
            next_gen_cells.insert(cell);
        }
    }

    next_gen_cells
}


fn world_to_screen(
    x: i32,
    y: i32,
    cell_size: f32,
    camera_offset: (f32, f32),
    zoom_factor:f32
) -> (f32, f32) {
    (
        x as f32 * cell_size * zoom_factor + camera_offset.0,
        y as f32 * cell_size * zoom_factor + camera_offset.1
    )
}
fn screen_to_world(
    screen: (f32, f32),
    cell_size: f32,
    camera_offset: (f32, f32),
) -> (i32, i32) {
    (
        ((screen.0 - camera_offset.0) / cell_size).floor() as i32,
        ((screen.1 - camera_offset.1) / cell_size).floor() as i32,
    )
}



#[macroquad::main("Conway's Game of Life")]
async fn main() {

    let mut density:f64 = 0.5;
    let mut height:f32 = 1000.0;
    let mut width:f32 = 2000.0;

    if std::env::args().len() > 1 {
        density = std::env::args().nth(1).unwrap().parse::<f64>().unwrap_or(density);
    }

    if std::env::args().len() > 2 {
        height = std::env::args().nth(2).unwrap().parse::<f32>().unwrap_or(height);
    }

    if std::env::args().len() > 3 {
        width = std::env::args().nth(3).unwrap().parse::<f32>().unwrap_or(width);
    }

    let mut camera_offset = (
        prelude::screen_width() / 2.0,
        prelude::screen_height() / 2.0,
    );



    let mut alive_cells :HashSet<Cell> = initialize_cells(
        screen_to_world( (-width/2.0, -height/2.0), CELL_SIZE as f32, camera_offset ),
        screen_to_world(( width/2.0, height/2.0 ), CELL_SIZE as f32, camera_offset),
        density
    );


    let mut zoom_factor: f32 = 1.0;
    let start_time = std::time::SystemTime::now();
    let mut timer = 0.0;
    let mut zoomed: f32 ;
    let mut is_dragging = false;
    let mut last_mouse_pos = Vec2::ZERO;

    loop {

        let mouse_pos = Vec2::from(prelude::mouse_position());

        if prelude::is_mouse_button_pressed(prelude::MouseButton::Left) {
            is_dragging = true;
            last_mouse_pos = mouse_pos;
        }
        if prelude::is_mouse_button_released(prelude::MouseButton::Left) {
            is_dragging = false;
        }
        if is_dragging {
            let delta = mouse_pos - last_mouse_pos;

            camera_offset.0 += delta.x;
            camera_offset.1 += delta.y;

            last_mouse_pos = mouse_pos;
        }


        zoomed = prelude::mouse_wheel().1;
        if zoomed != 0.0{
            zoom_factor += zoomed * ZOOM_WHEEL_SENSITIVITY;
            camera_offset.0 = mouse_position().0 - ( ( mouse_position().0 - camera_offset.0 ) * zoom_factor / (zoom_factor - zoomed * ZOOM_WHEEL_SENSITIVITY) );
            camera_offset.1 = mouse_position().1 - ( ( mouse_position().1 - camera_offset.1 ) * zoom_factor / (zoom_factor - zoomed * ZOOM_WHEEL_SENSITIVITY));
            zoomed = 0.0;
        }

        prelude::clear_background(BACKGROUND_COLOR);


        timer += prelude::get_frame_time();
        if timer > 0.01 {
        timer = 0.0;
            alive_cells = next_generation(&alive_cells);
        }


        for cell in &alive_cells {

            let coordinates = world_to_screen(cell.0, cell.1, CELL_SIZE as f32, camera_offset, zoom_factor);

            prelude::draw_rectangle(
                coordinates.0 - (CELL_SIZE as f32  / 2.0 ) + PADDING as f32,
                coordinates.1 - (CELL_SIZE as f32  / 2.0) + PADDING as f32,
                (CELL_SIZE - PADDING ) as f32 * zoom_factor,
                (CELL_SIZE - PADDING ) as f32* zoom_factor,
                ALIVE_CELL_COLOR
            );

        }


        prelude::next_frame().await
    }
}




fn initialize_cells(top_left: (i32,i32), bottom_right:(i32,i32), density:f64) -> HashSet<Cell> {
    /*

        * Initialise les cellules avec une chance de density/1.0 (doit être entre 0.0 et 1.0)
        * Recupere les coordonnées du coin supérieur gauche et du coin inférieur droit pour créer un rectangle dans lequel les cellules seront initialisées
        * Les coordonnées sont celles des cellules.
    */

    let mut alive_cells :HashSet<Cell> = HashSet::new();

    let min_x = top_left.0.min(bottom_right.0);
    let max_x = top_left.0.max(bottom_right.0);
    let min_y = top_left.1.min(bottom_right.1);
    let max_y = top_left.1.max(bottom_right.1);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if rand::random_bool( density ) {
                alive_cells.insert((x, y));
            }
        }
    }


    alive_cells
}
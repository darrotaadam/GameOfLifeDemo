use std::collections::{HashMap, HashSet};
use macroquad::math::{clamp, Vec2};
use std::option::Option;

use macroquad::prelude;

const DEFAULT_DENSITY:f64 = 0.5;
const DEFAULT_HEIGHT:u32 = 100;
const DEFAULT_WIDTH:u32 = 200;
pub const DEFAULT_PAUSE_TIME:f32 = 0.25;
pub const DEFAULT_MINIMAL_PAUSE_TIME:f32 = 0.00001;
pub const DEFAULT_MAX_PAUSE_TIME:f32 = 1.0;

static DEFAULT_PADDING:f32 = 1.0;
static DEFAULT_CELL_SIZE:f32 = 8.0;
const BACKGROUND_COLOR: prelude::Color = prelude::Color::from_hex(0x424242);
const ALIVE_CELL_COLOR: prelude::Color = prelude::Color::from_hex(0xEBE8E8);
const ZOOM_WHEEL_SENSITIVITY: f32 = 0.1;
const DEFAULT_PENCIL_THICKNESS: i32 = 15;

type Cell = (i32, i32);

pub struct GameOfLife {

    pub density:f64,
    pub height:u32 ,
    pub width:u32,

    pub pause_time: f32,
    zoom_factor: f32 ,
    pub cell_size: f32,
    pub padding: f32,
    pencil_thickness: f32,

    is_ctrl_pressed :bool,
    is_dragging:bool,
    is_drawing: bool,

    timer :f32,
    zoomed: f32 ,
    last_mouse_pos:Vec2,

    camera_offset: (f32, f32),


    pub alive_cells: HashSet<Cell>,
    asked_exit:bool,
    pub paused:bool,
}



impl GameOfLife {
    pub fn new( density: Option<f64>, height: Option<u32>, width:Option<u32> )-> GameOfLife {
        GameOfLife{
            density: density.unwrap_or(DEFAULT_DENSITY),
            height: height.unwrap_or(DEFAULT_HEIGHT),
            width: width.unwrap_or(DEFAULT_WIDTH),

            pause_time: DEFAULT_PAUSE_TIME,
            zoom_factor: 1.0,
            cell_size: DEFAULT_CELL_SIZE,
            padding: DEFAULT_PADDING,
            pencil_thickness: DEFAULT_PENCIL_THICKNESS as f32,

            is_ctrl_pressed: ctrl_pressed(),
            is_dragging: false,
            is_drawing: false,

            timer: 0.0,
            zoomed:0.0,
            last_mouse_pos: Vec2::new(0.0, 0.0),

            camera_offset: ( prelude::screen_width() / 2.0, prelude::screen_height() / 2.0 ),

            alive_cells: HashSet::new(),
            asked_exit: false,
            paused:false,
        }
    }


    pub fn initialize( &mut self ) {
        self.alive_cells = HashSet::new();

        let x_half: i32 = self.width as i32/2;
        let y_half: i32 = self.height as i32 /2 ;

        for x in (0-x_half)..=x_half  {
            for y in (0-y_half)..=y_half {
                if rand::random_bool( self.density ) {
                    self.alive_cells.insert((x as i32, y as i32));
                }
            }
        }
    }


    pub fn draw_generation(&mut self){
        for cell in &self.alive_cells {

            let coordinates = self.world_to_screen(cell.clone());
            prelude::draw_rectangle(
                coordinates.0 - (self.cell_size as f32  / 2.0 ) + self.padding as f32,
                coordinates.1 - (self.cell_size as f32  / 2.0) + self.padding as f32,
                (self.cell_size - self.padding) as f32 * self.zoom_factor,
                (self.cell_size - self.padding) as f32* self.zoom_factor,
                ALIVE_CELL_COLOR
            );
        }
    }



    fn world_to_screen(
        &self,
        universe_coordinates:Cell // ( 0, 0 ) est au milieu
    )->(f32,f32){   // ( 0.0, 0.0 ) est en haut a gauche
        (
           universe_coordinates.0 as f32 * self.cell_size * self.zoom_factor + self.camera_offset.0,
           universe_coordinates.1 as f32 * self.cell_size * self.zoom_factor + self.camera_offset.1
        )
    }

    fn screen_to_world( &self, screen_coordinates:(f32, f32) )->Cell {
        (
            ((screen_coordinates.0 - self.camera_offset.0 ) / self.zoom_factor / self.cell_size ).round() as i32,
            ((screen_coordinates.1 - self.camera_offset.1 ) / self.zoom_factor / self.cell_size).round() as i32
        )
    }



    fn find_neighbors( &mut self ) -> HashMap<Cell,u32>
    {
        let mut neighbors_count: HashMap<Cell,u32> = HashMap::new();

        for &(x,y) in &self.alive_cells {
            for dx in -1..=1 { // ..=1 ça inclut 1
                for dy in -1..=1 {
                    // on prend toutes les cellules autour de la cellule vivante (x,y)
                    if dx == 0 && dy == 0 {continue};
                    *neighbors_count.entry((x+dx, y+dy)).or_insert(0) +=1 ;
                }
            }
        }
        neighbors_count
    }

    fn next_generation(&mut self){
        let mut next_gen_cells : HashSet<Cell>= HashSet::new();

        let neighbors = self.find_neighbors();

        for (cell, neighbor_count) in neighbors{
            if neighbor_count == 3 || (neighbor_count == 2 && self.alive_cells.contains(&cell)) {
                next_gen_cells.insert(cell);
            }
        }
        self.alive_cells = next_gen_cells
    }



    fn handle_input(&mut self){

        if escape_pressed(){
            self.asked_exit = true;
        }

        if spacebar_pressed(){
            self.paused = !self.paused;
        }
        
        if r_pressed(){
            self.initialize()
        }
       
        if shift_pressed(){
            self.is_drawing = true;
        }
        if shift_released(){
            self.is_drawing = false;
        }

        self.handle_thickness_change();

        self.handle_regenerate_button();

        self.handle_mouse_drag();

        self.handle_mouse_wheel();

    }



    fn handle_mouse_drag(&mut self){

        let current_mouse_pos = Vec2::from(prelude::mouse_position());

        if mouse_left_pressed() {
            self.is_dragging = true;
            self.last_mouse_pos = current_mouse_pos;
        }
        if mouse_left_released() {
            self.is_dragging = false;
        }
        if self.is_dragging {
       
            if self.is_drawing{
                self.handle_mouse_drag_draw(current_mouse_pos);
                self.last_mouse_pos = current_mouse_pos;
            }
            else{
                self.handle_mouse_drag_move(current_mouse_pos);
            }

                    }

    }

    
    fn handle_mouse_drag_move( &mut self, current_mouse_pos: Vec2){
        let delta = current_mouse_pos - self.last_mouse_pos;
        self.camera_offset.0 += delta.x;
        self.camera_offset.1 += delta.y;
        self.last_mouse_pos = current_mouse_pos;
    }

    fn handle_mouse_drag_draw( &mut self, current_mouse_pos: Vec2){
        /*
            a le rôle de créer un tracé de cellules vivantes : si une cellule est morte elle passe
            vivante, si elle est vivante elle le reste
            TODO : ajouter une "gomme" qui permet de tuer des cellules au lieu de les rendre vivantes
        */
        let delta = current_mouse_pos - self.last_mouse_pos;
        // consituer la liste des cellules situées sur le chemin du delta
        // ajoute simplement cellule dans le set de cellules vivantes
        



        // doit réaliser une ligne droite entre self.last_mouse_pos et delta, donc pythagore mais il
        // faut trouver la liste des cellules (x,y) dans la range (donc distance divisée par
        // self.cell_size
        // et ensuite ajouter chaque cellule en considérant: 
        // self.zoom_factor , self.camera_offset (position de la souris par rapport au décalage de
        // la caméra), self.cell_size

        let last_mouse_pos_cell = self.screen_to_world( (self.last_mouse_pos.x, self.last_mouse_pos.y) );
        let current_mouse_pos_cell = self.screen_to_world( (current_mouse_pos.x, current_mouse_pos.y) );
    
        // utilise l'algorithme de Bresenham pour créer une ligne de cellules 
        
        self.bresenham( last_mouse_pos_cell, current_mouse_pos_cell );

    }

    
    fn bresenham( &mut self, origin: Cell, end: Cell )
    { 
        let mut origin = origin;
        let mut end = end;


        let dx = end.0.abs() - origin.0.abs();
        let dy = end.1.abs() - origin.1.abs();

                
        
        if dx >= dy {
            
            if origin.0 > end.0{
                let tmp:(i32, i32) = (origin.0, origin.1);
                origin.0 = end.0;
                origin.1 = end.1;
                end.0 = tmp.0;
                end.0 = tmp.1; 
            }
            let m_new = 2 * (end.1 - origin.1);
            let mut slope_error_new = m_new - (end.0 - origin.0);

            let mut y = origin.1;
            for x in origin.0 ..= end.0 {
                
                let offset_thickness = self.pencil_thickness as i32 - 1;
                for _x in (-offset_thickness)..offset_thickness{

                    for _y in (-offset_thickness) .. offset_thickness {

                        self.alive_cells.insert( (x + _x , y + _y) );
                    }
                }
                slope_error_new += m_new;

                if slope_error_new >= 0 {
                    y +=1 ;
                    slope_error_new -= 2 * (end.0 - origin.0);
                }

            }
        }
        else {
            if origin.1 > end.1{
                let tmp:(i32, i32) = (origin.0, origin.1);
                origin.0 = end.0;
                origin.1 = end.1;
                end.0 = tmp.0;
                end.1 = tmp.1; 
            }

            let m_new = 2 * (end.0 - origin.0);
            let mut slope_error_new = m_new - (end.1 - origin.1);
    
            let mut x = origin.0;   
            for y in origin.1 ..= end.1 {
                
                self.alive_cells.insert( (x,y) );
                
                slope_error_new += m_new;

                if slope_error_new >= 0 {
                    x +=1 ;
                    slope_error_new -= 2 * (end.1 - origin.1);
                }

            }
        }
                
    }


    fn handle_thickness_change(&mut self){
        //if macroquad::ui::widgets::Slider::new().ui(&mut macroquad::ui::root_ui()) {
        macroquad::ui::root_ui().slider(0x10, "[0 .. 100]", 1f32 .. 1000f32, &mut self.pencil_thickness );                
        
    }

    fn handle_mouse_wheel(&mut self){
        if ctrl_pressed() {
            self.is_ctrl_pressed = true;
        }
        if ctrl_released() {
            self.is_ctrl_pressed = false;
        }


        self.zoomed = prelude::mouse_wheel().1;
        if self.zoomed != 0.0{
            if self.is_ctrl_pressed {
                // on fait évoluer le temps de pause relativement a sa valeur actuelle au lieu de simplement ajouter self.zoomed, pour avoir un contrôle précis lors d'un temps proche de 0.0
                self.set_pause_time( self.pause_time + self.zoomed.signum() * self.pause_time * ( self.zoomed.abs()/50.0 ).powf(self.pause_time) );

            }
            else{
                // il faut moduler le zoom factor de façon à ce qu'il reste supérieur à 0.0,
                let old_zoom = self.zoom_factor;
                self.zoom_factor *= (1.0 + ZOOM_WHEEL_SENSITIVITY).powf(self.zoomed);
                let mouse = prelude::mouse_position();
                self.camera_offset.0 = mouse.0 - (mouse.0 - self.camera_offset.0) * self.zoom_factor / old_zoom;
                self.camera_offset.1 = mouse.1 - (mouse.1 - self.camera_offset.1) * self.zoom_factor / old_zoom;
                self.zoomed = 0.0;
            }
        }
    }
    fn handle_regenerate_button(&mut self){
        let text = "Regenerate";
        let font_size:f32 = 7.0;
        let dimensions = Vec2::new(text.len() as f32* font_size * 2.0, text.len() as f32 * font_size/2.0 );

        if macroquad::ui::widgets::Button::new(text)
            .size( dimensions )
            .position(  Vec2::new(prelude::screen_width()/2.0 - dimensions.x /2.0 , 30.0)  )
            .ui(&mut macroquad::ui::root_ui()) {
            self.initialize();
        }
    }


    fn update(&mut self){
        self.timer += prelude::get_frame_time();
        if self.timer > self.pause_time {
            self.timer = 0.0;
            self.next_generation();
        }
    }

    pub fn set_pause_time(&mut self, time:f32){
        self.pause_time = clamp(  time, DEFAULT_MINIMAL_PAUSE_TIME, DEFAULT_MAX_PAUSE_TIME);
    }

    fn draw(&mut self){
        prelude::clear_background(BACKGROUND_COLOR);

        self.draw_metadata();

        self.draw_generation();

        self.draw_exit_msg();

        self.draw_paused_status();
    }

    fn draw_metadata(& self){
        prelude::draw_text( &format!("Coordinates {} {}", self.camera_offset.0, self.camera_offset.1 ), 10.0, 60.0, 20.0, prelude::WHITE);
        prelude::draw_text( &format!("Zoom {} ", self.zoom_factor ), 10.0, 40.0, 20.0, prelude::WHITE);
        prelude::draw_text( &format!("Pause time : {:.5} s", self.pause_time), 10.0, 80.0, 20.0, prelude::WHITE);
        prelude::draw_text( &format!("Live cells : {}", self.alive_cells.len()), 10.0, 100.0, 20.0, prelude::WHITE);
    }
    fn draw_exit_msg(&self){
        prelude::draw_rectangle(10.0, 10.0, 200.0, 30.0, prelude::Color::from_hex(0x212121));
        prelude::draw_text( "Press ESCAPE to exit", 30.0, 30.0, 20.0, prelude::WHITE);
    }
    fn draw_paused_status(&self){
        if self.paused{
            prelude::draw_rectangle(prelude::screen_width() - 80.0 - 10.0, prelude::screen_height() - 50.0, 80.0, 30.0, prelude::Color::from_hex(0x212121));
            prelude::draw_text( "Paused", prelude::screen_width() - 78.0, prelude::screen_height() - 30.0, 20.0, prelude::WHITE);
        }
    }




    pub async fn run(&mut self){
        loop
        {
            if self.asked_exit{
                break;
            }
            self.handle_input();
            if !self.paused{
                self.update();
            }
            self.draw();
            prelude::next_frame().await;
        }
    }

}




fn escape_pressed()->bool{prelude::is_key_pressed(prelude::KeyCode::Escape)}

fn ctrl_pressed()->bool{
    prelude::is_key_pressed(prelude::KeyCode::LeftControl)
}
fn ctrl_released()->bool{
    prelude::is_key_released(prelude::KeyCode::LeftControl)
}

fn mouse_left_pressed()->bool{
    prelude::is_mouse_button_pressed(prelude::MouseButton::Left)
}
fn mouse_left_released()->bool{
    prelude::is_mouse_button_released(prelude::MouseButton::Left)
}
fn spacebar_pressed()->bool{
    prelude::is_key_pressed(prelude::KeyCode::Space)
}
fn r_pressed()->bool{
    prelude::is_key_pressed(prelude::KeyCode::R)
}

fn shift_pressed()->bool{
    prelude::is_key_pressed(prelude::KeyCode::LeftShift)
}
fn shift_released()->bool{
    prelude::is_key_released(prelude::KeyCode::LeftShift)
}




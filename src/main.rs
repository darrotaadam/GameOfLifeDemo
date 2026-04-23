use macroquad::prelude;
use clap::{Arg, Command};
pub mod game_of_life;


#[macroquad::main("Conway's Game of Life")]
async fn main(){

    let matches = Command::new("Game of Life")
        .about("Conway's Game of Life")
        .arg(Arg::new("density")
            .short('d')
            .long("density")
            .help("Initial density of alive cells (0.0 to 1.0)")
            .value_parser(clap::value_parser!(f64)))
        .arg(Arg::new("height")
            .long("height")
            .help("Height of the grid in cells")
            .value_parser(clap::value_parser!(u32)))
        .arg(Arg::new("width")
            .short('w')
            .long("width")
            .help("Width of the grid in cells")
            .value_parser(clap::value_parser!(u32)))
        .arg(Arg::new("fullscreen")
            .long("fullscreen")
            .short('f')
            .help("Fullscreen mode")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("pausetime")
            .long("pausetime")
            .help(&format!("Pause time between each generation in seconds between {} and {} ; {} by default", game_of_life::DEFAULT_MINIMAL_PAUSE_TIME, game_of_life::DEFAULT_MAX_PAUSE_TIME, game_of_life::DEFAULT_PAUSE_TIME))
            .value_parser(clap::value_parser!(f32)))
        .arg(Arg::new("paused")
            .long("paused")
            .help("Start in paused mode")
            .action(clap::ArgAction::SetTrue))
        .get_matches();


    let density = matches.get_one::<f64>("density").copied();
    let height  = matches.get_one::<u32>("height").copied();
    let width   = matches.get_one::<u32>("width").copied();
    let fullscreen   = matches.get_one::<bool>("fullscreen").copied();
    let pause_time   = matches.get_one::<f32>("pausetime").copied();
    let paused = matches.get_one::<bool>("paused").copied();

    prelude::set_fullscreen(fullscreen.unwrap_or(false));
    let mut gol = game_of_life::GameOfLife::new(density, height, width);
    if pause_time.is_some() {
        gol.set_pause_time(pause_time.unwrap());
    }
    if paused.is_some(){
        gol.paused = paused.unwrap();
    }
    gol.initialize();
    gol.run().await;
}
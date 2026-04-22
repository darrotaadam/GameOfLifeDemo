use macroquad::prelude;
use clap::{Arg, Command, builder::PathBufValueParser};
pub mod game_of_life;


#[macroquad::main("Conway's Game of Life")]
async fn main(){
    prelude::set_fullscreen(true);

    let matches = Command::new("Game of Life")
        .about("Conway's Game of Life")
        .arg(Arg::new("density")
            .short('d')
            .long("density")
            .help("Initial density of alive cells (0.0 to 1.0)")
            .value_parser(clap::value_parser!(f64)))
        .arg(Arg::new("height")
            .short('h')
            .long("height")
            .help("Height of the grid in cells")
            .value_parser(clap::value_parser!(u32)))
        .arg(Arg::new("width")
            .short('w')
            .long("width")
            .help("Width of the grid in cells")
            .value_parser(clap::value_parser!(u32)))
        .get_matches();

    let density = matches.get_one::<f64>("density").copied();
    let height  = matches.get_one::<u32>("height").copied();
    let width   = matches.get_one::<u32>("width").copied();

    let mut gol = game_of_life::GameOfLife::new(density, height, width);
    gol.initialize();
    gol.run().await;
}
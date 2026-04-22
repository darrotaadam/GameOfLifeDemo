# Conway's Game of Life (poorly implemented)

![Preview](Preview.png)


## Build :                                                                        
                                                                                  
```                                                                               
cargo build -r                                                                    
cp target/release/GOL .                                                           
```

## Run : 

*Syntax :*
`./GOL [--density|-d : 0.0 to 1.0] [--height|-h : 100 by default] [--width|-w : 200 by default]`

*Examples :*  
*   `./GOL --density 0.5 --height 1500 --width 1500`
*  `cargo run`


## Controls :
*  `Mouse Pressed + Drag` : Move in the world
* `Mouse Scroll` : Zoom in/out
* `Left Ctrl + Mouse Scroll` : Change the speed of the simulation
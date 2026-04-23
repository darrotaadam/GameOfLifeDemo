# Conway's Game of Life (poorly implemented)

![Preview](Preview.png)


## Build :                                                                        
                                                                                  
```                                                                               
cargo build -r                                                                    
cp target/release/GOL .                                                           
```

## Run : 

*Syntax :*
`./GOL [--density|-d : 0.0 to 1.0] [--height : 100 by default] [--width|-w : 200 by default]`

*Examples :*  
* `./GOL`
* `./GOL --density 0.5 --height 1500 --width 1500`
* `cargo run`
* `cargo run -r  -- --density 0.6 --height 1000 --width 1600`


## Controls :
*  `Mouse Pressed + Drag` : Move in the world
* `Mouse Scroll` : Zoom in/out
* `Left Ctrl + Mouse Scroll` : Change the speed of the simulation


# Extras :
*You can do some pretty cool things by playing with the density and the size of the generated area :*  

* With `--density 0.999`
    ![long](long_output.gif)
    ![short](short_output.gif)

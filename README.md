# Conway's Game of Life (poorly implemented)

![Preview](Preview.png)


## Build :                                                                        
                                                                                  
```                                                                               
cargo build -r                                                                    
cp target/release/GOL .                                                           
```

## Run : 

*Syntax :*
`./GOL [ args... ]`

*Options :*
* `--density <float>` : The density of the generated world (between 0 and 1, default: 0.5)
* `--height <int>` : The height of the generated world (default: 1000)
* `--width <int>` : The width of the generated world (default: 1000)
* `-f, --fullscreen` : Fullscreen mode
* `--pausetime <float>` :  Pause time between each generation in seconds ; 0.25 by default
* `--paused` : Start in paused mode
* `-h` : Show this help message and exit

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
    <br>
  
* With `--density 1.0 --height 1000 --width 1000` if you want to create a perfect symetrical square 
    ![perfect_square](perfect_square.gif)
    <br>
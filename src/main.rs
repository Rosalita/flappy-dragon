use bracket_lib::prelude::*; // the * is a wildcard, it means use everything from bracket-lib
                             // using the prelude is a rust convention that organises imports.
                             // It saves prefixing every call to the library with bracket-lib::prelude::.

enum GameMode {
    Menu,
    Playing,
    End,
}
//const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGTH: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}
impl Player {
    fn new(x: i32, y: i32) -> Self {
        return Self {
            x,
            y,
            velocity: 0.0, // floats must be fractional, 0 would cause a mis-matched type error.
        };
    }
    fn render(&mut self, ctx: &mut BTerm) {
        // set is a bracket-lib function that sets a single char on the screen.
        // to_cp437 converts a unicode char from source code to the matching Codepage 437 char number.
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'))
    }
    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 { // check for terminal velocity, 
            self.velocity += 0.2; // if not moving at terminal velocity, increase velocity
        }
        // add the velocity to the players y, increasing y moves the player down.
        // to add a float to an int, the float must be converted to int.
        self.y += self.velocity as i32; // this conversion will always round the float down to i32.
        // even thought the player isn't moving, incrementing x tracks progression through the level
        self.x += 1; 
        if self.y < 0 {
            self.y = 0;
        }
    }
    fn flap(&mut self){
        // setting velocity to -2.0 will move the player upwards.
        self.velocity = -2.0;
    }
}

struct State {
    player: Player,
    frame_time: f32, // used to track the time between frames to control game speed.
    mode: GameMode,
}
impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.cls_bg(NAVY); // set the background colour
        // tick() runs as fast as it can.
        // ctx.frame_time_ms contains the time elapsed since the last time tick() was called.
        self.frame_time += ctx.frame_time_ms; 
        // self.frame_time counts up until FRAME_DURATION is reached
        if self.frame_time > FRAME_DURATION{
            self.frame_time = 0.0; // then it resets
            self.player.gravity_and_move(); // and updates the game.
        }
        if let Some(VirtualKeyCode::Space) = ctx.key { // if the user is pressing spacebar.
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        // if flappy dragon falls off the bottom of the screen, the game ends.
        if self.player.y > SCREEN_HEIGTH{
            self.mode = GameMode::End;
        }
    }
    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        // Rust provides a shortened version of match for matching aginst a single case
        // named if let. Option is an enum of Some(data) and None.
        // Using match to work with Option would look like:

        // match my_option {
        // Some ( my_value ) => do_something_with(my_value),
        // _ => {} // Do nothing for the None arm.
        // }

        // using if let works the same way:
        // if let Some(my_value) = my_option {
        // do_something_with(my_value);
        //}

        // you can use an else statement at the end of an if let.
        // Rust treats _ as anything, so _ => {} tells Rust to ignore any match arms that
        // aren't listed.

        if let Some(key) = ctx.key {
            //if the user presses a key, extract the keys value
            // if let implies that if no key is pressed, do nothing.
            match key {
                VirtualKeyCode::P => self.restart(), // if P was pressed, restart the game.
                VirtualKeyCode::Q => ctx.quitting = true, // if Q was pressed set quitting to true
                _ => {}                              // ignore any matches that aren't listed.
            }
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(), // if P was pressed, restart the game.
                VirtualKeyCode::Q => ctx.quitting = true, // if Q was pressed set quitting to true
                _ => {}                              // ignore any matches that aren't listed.
            }
        }
    }
}
// GameState is a trait defined in bracket-lib
impl GameState for State {
    // this is similar to implementing functions for a struct
    // but this is a trait that's being implemented for a struct.
    // traits are similar to interfaces in other languages.
    fn tick(&mut self, ctx: &mut BTerm) {
        // GameState trait requires a tick function with this signature.
        // self in the signature means the instance of the state struct will be accessible at this scope.
        // as self is mutable the tick function will be able to change the state.
        // ctx provides a window into the currently running bracket terminal.
        // ctx.cls(); //ctx is short for context, this clears the screen.
        // print is an interface to print to the game window.
        //ctx.print(1,1,"Hello, Bracket Terminal!");

        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    // this is a builder pattern. An initial constructor function returns the builder
    // then chained function calls add options which are finalised with a call to build()
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    // main_loop starts the game and begins calling tick() on every frame
    main_loop(context, State::new())
}

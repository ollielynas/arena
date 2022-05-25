
use std::thread;
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};
use rand::Rng;

extern crate piston_window;


extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod smooth_brain;

use once_cell::sync::Lazy; // 1.3.1
use std::sync::Mutex;

static ARRAY: Lazy<Mutex<Vec<PhysObj>>> = Lazy::new(|| Mutex::new(vec![]));




#[derive(Debug, Clone, Copy)]




// -------------------------------------------------------------|
//                      Variables                               |
// -------------------------------------------------------------|


struct PhysObj {
    x: i16,
    y: i16,
    vy: i16,
    vx: i16,
    width: u8,
    height: u8,
    hp: i8,
    iframes: i8,
}



impl PhysObj{
    fn gravity(&mut self) { 
        if self.vy <= 10{
            self.vy += 1;
    }}

    fn collision(&mut self) {
        if self.y - self.height as i16 <= 500 {
            if self.vy > 0 {
                self.vy = 0;
                self.y = 500 + self.height as i16;
            }
        }
        if self.y < 0 {
            self.vy = 0;
            self.y = 0;
        }
        if self.x < 0 {
            self.vx = 0;
            self.x = 0;
        }
        if self.x + self.width as i16 > 500 {
            self.vx = 0;
            self.x = 500 - self.width as i16;
        }
    }
    fn move_obj(&mut self) {
        if self.iframes > 0{
            self.iframes -= 1;
        }
        self.x += self.vx;
        self.y += self.vy;
        if self.x < 0 {
            self.x = 0;
        }
        if  self.y+ self.height as i16 > 500 {
            self.y = 500 - self.height as i16;
        }}

    fn jump(&mut self) {
        if self.y == 500 - self.height as i16 {
            self.vy = -10;
        }
    }

    fn move_left(&mut self) {
        if self.vx < -5 {self.vx += 1;
        } else if self.vx == -5 {self.vx = -5;}
        else {self.vx -= 1;}
    }

    fn move_right(&mut self) {
        if self.vx > 5 {self.vx -= 1;
        } else if self.vx == 5 {self.vx = 5;}
        else {self.vx += 1;}
    }



    fn zombie_ai(&mut self, x:i16, y:i16) -> bool {
        if self.x < x{
            self.move_right();
        } if self.x > x {
            self.move_left();
        }

        if self.x() < x+40 
        && (self.x()+self.width() as i16) >  x 
        && (self.y()+self.height() as i16) > y
        && self.y() < y+70 {
            return true
        }
        return false
    }

    fn vy(&self) -> i16{self.vy}
    fn vx(&self) -> i16{self.vx}
    fn x(&self) -> i16{self.x}
    fn y(&self) -> i16{self.y}
    fn hp(&self) -> i8{self.hp}
    fn width(&self) -> u8{self.width}
    fn height(&self) -> u8{self.height}
    fn iframes(&self) -> i8{self.iframes}
}

// -------------------------------------------------------------|
//                      Ai                                      |
// -------------------------------------------------------------|

fn main_sim(brain:u8, render:bool) {



    let mut level_num = 1;


    loop {
        let ret:u8 = start_level(level_num, brain, render);
        if ret == 0 {
            level_num += 1;
        } else if ret == 1 {
            println!("made it to level {}", level_num);
            break;
        }
    }
}

fn start_level(level_num: u32, brain: u8, render:bool) -> u8 {
    let mut player = PhysObj {
        x: 250,
        y: 250,
        vx: 0,
        vy: 0,
        width: 40,
        height: 70,
        hp: 10,
        iframes: 0,
    };
    player.jump();



    fn new_enemy(brain: u8) -> PhysObj{

        const ENEMY_HEIGHT: [u8; 3] = [30, 75, 80];
        const ENEMY_WIDTH: [u8; 3] = [60, 40, 50];
        const ENEMY_HP: [i8; 3] = [8, 5, 10];

        let mut enemy_type = rand::thread_rng().gen_range(0..ENEMY_WIDTH.len());

        if brain == 0 {
            enemy_type = 1;
        }

        return PhysObj {
            x: rand::thread_rng().gen_range(0..400),
            y: rand::thread_rng().gen_range(200..500),
            vx: 0,
            vy: 0,
            width: ENEMY_WIDTH[enemy_type],
            height: ENEMY_HEIGHT[enemy_type],
            hp: ENEMY_HP[enemy_type],
            iframes: 10,
        };
    }



    let mut enemy_list = [
        &mut new_enemy(brain),
    ];






    loop {
    if player.hp() >= 0 {
    player.gravity();
    player.collision();
    for i in &mut enemy_list {
        i.gravity();
        i.collision();
        i.move_obj();

        if i.height() == 75 {
            if player.zombie_ai(player.x(), player.y()) {
                player.hp -= 1;
            }
        }
        if render {
            ARRAY.lock().unwrap().push(**i);
        }
    }

    // return list of bool in this order to indicate weather to perform this action
    // outputs include ["move_left", "move_right", "jump", "attack_up", "attack_down", "attack"]

    let actions: Vec<bool> = smooth_brain::v1::run(0, 0);

    player.move_obj();

    let mut stdout = stdout();


    if render {
        stdout.flush().unwrap();
        sleep(Duration::from_millis(100));
        ARRAY.lock().unwrap().push(player);
        ARRAY.lock().unwrap().clear();
    }

    }else{
        return 1;
    }


    }
}



// -------------------------------------------------------------|
//                      Gui                                     |
// -------------------------------------------------------------|




pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    objects: Vec<PhysObj>
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const BG: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
        const MG: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
        // const FG: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

        let arena = rectangle::square(500.0, 200.0, 500.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BG, gl);
            let transform = c
                .transform
                .trans(0.0, 0.0);
            rectangle(MG, arena, transform, gl);
        });

        let obj = ARRAY.lock().unwrap()[0];

        for j in 0..ARRAY.lock().unwrap().len() {
            println!("{}", j);

            println!("{}", obj.y());
            self.gl.draw(args.viewport(), |c, gl | {
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                          [
                            obj.x() as f64+ 500.0,
                            obj.y() as f64+200.0,
                            obj.width() as f64,
                            obj.height() as f64
                            ],
                          c.transform, gl); 
    });
}
}


}


fn main() {


    for _t in 0..10 {
        thread::spawn(|| {
            for _i in 0..100 {
            main_sim(0, false);
            }
        });
    };


    thread::spawn(|| {
        for _i in 0..100 {
        main_sim(0, true);
        }
    });

    println!("Hello, world!");


    // different networks indicator
    // run:0, 
    let mut brain = 0;

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", [1050, 750])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        objects: Vec::new(),
    };



    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}
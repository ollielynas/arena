// #![windows_subsystem = "windows"]


use std::thread;
use std::{
    thread::sleep,
    time::Duration,
};
use rand::Rng;

extern crate piston_window;


extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use bevy::prelude::*;




mod smooth_brain;

use once_cell::sync::Lazy; // 1.3.1
use std::sync::Mutex;

static ARRAY: Lazy<Mutex<Vec<PhysObj>>> = Lazy::new(|| Mutex::new(vec![]));

use std::fs;

extern crate find_folder;


use native_dialog::{MessageDialog, MessageType};





// -------------------------------------------------------------|
//                      Variables                               |
// -------------------------------------------------------------|

#[derive(Copy, Clone)]
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
        if self.vy <= 5{
            self.vy += 1;
        }else {
            self.vy -= 1;
        }
    }

    fn collision(&mut self) {

        self.gravity();

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
            if self.vx > 0 {
                self.vx = 0;
            }
            self.x = 500 - self.width as i16;
        }
    }
    fn move_obj(&mut self) {
        if self.iframes > 0 {
            self.iframes -= 1;
        }

        self.x += self.vx;
        self.y += self.vy;



        if self.vy > 0 {
            self.vy -= 1;
        }else if self.vy < 0 {
            self.vy += 1;
        }

        if self.vx > 0 {
            self.vx -= 1;
        }else if self.vx < 0 {
            self.vx += 1;
        }

        if self.x < 0 {
            self.x = 0;
        }
        if  self.y+ self.height as i16 > 500 {
            self.y = 500 - self.height as i16;
        }}

    fn jump(&mut self) {
        if self.y < 495 - self.height as i16 {
            self.vy = -10;
        }
    }

    fn move_left(&mut self) {
        if self.vx > -5 {self.vx += -3}
    }

    fn move_right(&mut self) {
    if self.vx < 5 {self.vx += 3;}
    }



    fn zombie_ai(&mut self, x:i16, y:i16) -> bool {
        if self.x < x{
            self.vx -=1;
            self.move_right();
        } if self.x > x {
            self.vx +=1;
            self.move_left();
        }

        if self.x() <= x+40
        && (self.x()+self.width() as i16) >=  x 
        && (self.y()+self.height() as i16) >= y
        && self.y() <= y+70 {
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

    fn set_hp(&mut self, hp:i8){self.hp = hp;}
    fn set_iframes(&mut self, iframes:i8){self.iframes = iframes;}
}



// -------------------------------------------------------------|
//                      Ai                                      |
// -------------------------------------------------------------|

fn main_sim(brain:u8, render:bool) -> BrainScore {


    let mut matrix_str: String = std::fs::read_to_string("src/JSON/run.json").unwrap();
    matrix_str = smooth_brain::v1::mutate(&matrix_str);

    let mut level_num = 1;
    let mut score = 0;

    loop {
        let ret:Vec<i32> = start_level(level_num, brain, render, matrix_str.clone(), score);
        score = ret[1];
        if ret[0] == 0 {
            level_num += 1;
        } else if ret[0] == 1 {
            break;
        }
    }

    BrainScore {
        brain: matrix_str.clone(),
        score: score,
    }
}


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

fn start_level(level_num: u32, brain: u8, render:bool, matrix_s:String, mut score: i32) -> Vec<i32> {

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


    let mut loop_counter = 0;

    let mut enemy_list:Vec<PhysObj> = vec![];
    for _ in 0..level_num {
        enemy_list.push(new_enemy(brain));
    }

    loop { // Main Loop

    // create loop_counter

    loop_counter += 1;

    if loop_counter > 50000 {
        println!("Loop Counter Exceeded");
        drop(loop_counter);
        return [1, score].to_vec();
    }






    let mut new_frame:Vec<PhysObj> = vec![];

    if render {
    new_frame.push(player.clone());
    }

    if brain == 0 {
        score += 1; // reward brain 0 just for surviving another frame, because that is all it can do
    }
    
    player.collision();


    for i in &mut enemy_list {
        i.collision();
        if i.height() == 75 {
            if i.zombie_ai(player.x(), player.y()) {
                if player.iframes() == 0 {
                    player.set_hp(player.hp() - 1);

                    player.set_iframes(10);
                    score -= 100;  // Punishment for being hit
                }
            }
        }
        if render {
            new_frame.push(i.clone());
        }
        i.move_obj();
    }



    // return list of bool in this order to indicate weather to perform this action
    // outputs include ["move_left", "move_right", "jump", "attack_up", "attack_down", "attack"]

    let actions: Vec<bool> = smooth_brain::v1::run(serde_json::from_str(&matrix_s).unwrap(), vec![player.x() as f64, enemy_list[0].x() as f64]);

    if actions[0] {
        player.move_left();
    }
    if actions[1] {
        player.move_right();
    }

    player.move_obj();

    // add new frame to array

    if render {
        ARRAY.lock().unwrap().clear();
        ARRAY.lock().unwrap().append(&mut new_frame);
        sleep(Duration::from_millis(10));
    }

    if player.hp() <= 0 {
        println!("player died: Score {}", score);
        return [1, score].to_vec();
    }
    if enemy_list.len() == 0 {

        return [0, score].to_vec();
    }
    }
}



// -------------------------------------------------------------|
//                      Gui                                     |
// -------------------------------------------------------------|






struct BrainScore {
    brain: String,
    score: i32,
}

fn main() {

    // different networks indicator
    // run:0, 
    let mut brain = 0;

    // rust piston popup input 




    
    let mut thread_count:u16 = 5;
    
    
    let mut rsrc_dir = std::env::current_exe()
    .expect("Can't find path to executable");
    rsrc_dir.pop();
    rsrc_dir.push("JSON");
    

        
        thread::spawn(move|| {
            
    println!("work Thread");
    for generation in 0..100 { // for now its caped at 100 generations
        println!("Generation {}", generation);
        let mut hands = vec![
            thread::spawn(|| {
                {
                main_sim(0, false)
                }
            }),
        ];
            for _ in 0..thread_count-1 { // 10 threads/ or generations are created where they will play till they die. they will then be given a score
                hands.push( thread::spawn(|| {
                    main_sim(0, false)
                }));
            };
        
            println!("started all threads: {}", hands.len());


        let mut generation_result = Vec::new();
        
        while hands.len() > 0 { // the 10 threads are put in a list
            let cur_thread = hands.remove(0); // moves it into cur_thread
            generation_result.push(cur_thread.join().unwrap());
        }

        println!("finished all threads: {}", generation_result.len());

        // once all the threads have finished they are ranked
        let mut best = 0;
        for x in 0..generation_result.len() {
            if generation_result[x].score > generation_result[best].score {
                best = x;
            }
        }

        let brain_lookup:Vec<String> = vec!["src/JSON/run.json".to_string()];


        fs::write(brain_lookup[0].clone(), generation_result[best].brain.clone()).expect("Unable to write file");

        println!("Best score: {}", generation_result[best].score);

    }
    });


    thread::spawn(|| {
        println!("display Thread");
        for _ in 0..100 {
        main_sim(0, true);
        }
    });

    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(render)
    .run();
}

fn render() {

}

fn setup(mut commands: Commands) {
commands.spawn_bundle(OrthographicCameraBundle::new_2d());
commands.spawn_bundle(SpriteBundle {
    sprite: Sprite {
        color: Color::rgb(0.25, 0.25, 0.75),
        custom_size: Some(Vec2::new(500.0, 500.0)),
        ..default()
    },
    ..default()
});

}


// #![windows_subsystem = "windows"]


use std::thread;
use std::{
    thread::sleep,
    time::Duration,
};
use rand::Rng;


mod smooth_brain;
mod seg;
use once_cell::sync::Lazy; // 1.3.1

use std::sync::Mutex;
static ARRAY: Lazy<Mutex<Vec<PhysObj>>> = Lazy::new(|| Mutex::new(vec![]));

use std::fs;

extern crate find_folder;


use speedy2d::color::Color;
use speedy2d::shape::{Rectangle};
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{MouseButton, WindowHandler, WindowHelper, WindowStartupInfo};

extern crate num_cpus;


// -------------------------------------------------------------|
//                      Variables                               |
// -------------------------------------------------------------|



#[derive(Copy, Clone)]
struct PhysObj {
    x: f32,
    y: f32,
    vy: f32,
    vx: f32,
    width: f32,
    height: f32,
    hp: f32,
    iframes: f32,
    // 0 = no attack, 1 = up, 2 = forward, 3 = down
}


/*
ooooooooo.   oooo                                    .oooooo.    .o8           o8o 
`888   `Y88. `888                                   d8P'  `Y8b  "888           `"' 
 888   .d88'  888 .oo.   oooo    ooo  .oooo.o      888      888  888oooo.     oooo 
 888ooo88P'   888P"Y88b   `88.  .8'  d88(  "8      888      888  d88' `88b    `888 
 888          888   888    `88..8'   `"Y88b.       888      888  888   888     888 
 888          888   888     `888'    o.  )88b      `88b    d88'  888   888     888 
o888o        o888o o888o     .8'     8""888P'       `Y8bood8P'   `Y8bod8P'     888 
                         .o..P'                                                888 
                         `Y8P'                                             .o. 88P 
                                                                           `Y888P  */

impl PhysObj{
    fn gravity(&mut self) { 
        if self.vy <= 20.0{
            self.vy += 3.0;
        }else {
            self.vy -= 1.0;
        }
    }

    fn collision(&mut self) {


        self.gravity();

        if self.y as f32 + self.height as f32>= 500.0 {
            if self.vy > 0.0 {
                self.vy = 0.0;
        }
        self.y = 500.0 - self.height;
        }
        if self.y < 0.0 {
            self.vy = 0.0;
            self.y = 0.0;
        }
        if self.x < 0.0 {
            self.vx = 0.0;
            self.x = 0.0;
        }
        if self.x + self.width > 500.0 {
            if self.vx > 0.0 {
                self.vx = 0.0;
            }
            self.x = 500.0 - self.width;
        }
    }
    fn move_obj(&mut self) {
        if self.iframes > 0.0 {
            self.iframes -= 1.0;
        }

        self.x += self.vx;
        self.y += self.vy;

        if self.vy > 0.0 {
            self.vy -= 1.0;
        }else if self.vy < 0.0 {
            self.vy += 1.0;
        }

        if self.vx > 0.0 {
            self.vx -= 1.0;
        }else if self.vx < 0.0 {
            self.vx += 1.0;
        }

        if self.x < 0.0 {
            self.x = 0.0;
        }
        if  self.y+ self.height > 500.0 {
            self.y = 500.0 - self.height;
        }}

    fn jump(&mut self) {
        if self.y < 495.0 - self.height {
            self.vy = -10.0;
        }
    }

    fn move_left(&mut self) {
        if self.vx > -5.0 {self.vx += -3.0}
    }

    fn move_right(&mut self) {
    if self.vx < 5.0 {self.vx += 3.0;}
    }

    fn attack(&mut self, enemy_list: &mut Vec<PhysObj>) {
        let attack = ((0.0,0.0),(0.0,0.0));

        for i in enemy_list {
            
        }
    }


    fn zombie_ai(&mut self, x:f32, y:f32) -> bool {
        if self.x < x{
            self.move_right();
            self.vx /= 2.0;
        } if self.x > x {
            self.move_left();
            self.vx /= 2.0;
        }
        if self.y < y && self.x < x+20.0 && self.x > x-20.0 {
            self.jump();
        }



        if self.x() <= x+40.0
        && (self.x()+self.width()) >=  x 
        && (self.y()+self.height()) >= y
        && self.y() <= y+70.0 {
            return true
        }
        return false
    }

    fn vy(&self) -> f32{self.vy}
    fn vx(&self) -> f32{self.vx}
    fn x(&self) -> f32{self.x}
    fn y(&self) -> f32{self.y}
    fn hp(&self) -> f32{self.hp}
    fn width(&self) -> f32{self.width}
    fn height(&self) -> f32{self.height}
    fn iframes(&self) -> f32{self.iframes}

    fn set_hp(&mut self, hp:f32){self.hp = hp;}
    fn set_iframes(&mut self, iframes:f32){self.iframes = iframes;}
}



/*
ooo        ooooo            o8o                    .oooooo..o  o8o                          .o88o.             
`88.       .888'            `"'                   d8P'    `Y8  `"'                          888 `"             
888b     d'888   .oooo.   oooo  ooo. .oo.        Y88bo.      oooo  ooo. .oo.  .oo.        o888oo  ooo. .oo.   
8 Y88. .P  888  `P  )88b  `888  `888P"Y88b        `"Y8888o.  `888  `888P"Y88bP"Y88b        888    `888P"Y88b  
8  `888'   888   .oP"888   888   888   888            `"Y88b  888   888   888   888        888     888   888  
8    Y     888  d8(  888   888   888   888       oo     .d8P  888   888   888   888        888     888   888  
o8o        o888o `Y888""8o o888o o888o o888o      8""88888P'  o888o o888o o888o o888o      o888o   o888o o888o 

*/

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

    const ENEMY_HEIGHT: [f32; 3] = [30.0, 75.0, 80.0];
    const ENEMY_WIDTH: [f32; 3] = [60.0, 30.0, 50.0];
    const ENEMY_HP: [f32; 3] = [8.0, 5.0, 10.0];

    let mut enemy_type = rand::thread_rng().gen_range(0..ENEMY_WIDTH.len());

    if brain == 0 || brain == 1 {
        enemy_type = 1;
    }

    return PhysObj {
        x: rand::thread_rng().gen_range(0.0..400.0),
        y: 300.0,
        vx: 0.0,
        vy: 0.0,
        width: ENEMY_WIDTH[enemy_type],
        height: ENEMY_HEIGHT[enemy_type],
        hp: ENEMY_HP[enemy_type],
        iframes: 10.0,
    };
}

fn start_level(level_num: u32, brain: u8, render:bool, matrix_s:String, mut score: i32) -> Vec<i32> {

    let mut player = PhysObj {
        x: rand::thread_rng().gen_range(0.0..500.0),
        y: 250.0,
        vx: 0.0,
        vy: 0.0,
        width: 40.0,
        height: 70.0,
        hp: 10.0,
        iframes: 0.0,
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
        if i.height() == 75.0 {
            if i.zombie_ai(player.x(), player.y()) {
                if player.iframes() == 0.0 {
                    player.set_hp(player.hp() - 1.0);
                    player.set_iframes(10.0);
                    score -= 100;  // Punishment for being hit
                }
            }
        }
        i.collision();

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

    if player.hp() <= 0.0 {
        return [1, score].to_vec();
    }
    if enemy_list.len() == 0 {

        return [0, score].to_vec();
    }
    }
}


/*
  .oooooo.    ooooo     ooo ooooo 
 d8P'  `Y8b   `888'     `8' `888' 
888            888       8   888  
888            888       8   888  
888     ooooo  888       8   888  
`88.    .88'   `88.    .8'   888  
 `Y8bood8P'      `Yb0dP'    o888o 

*/


struct Switch {
    x: f32,
    y: f32,
    length: f32,
    text: String,
    state: bool,
}

struct MyWindowHandler {
    //text: Rc<FormattedTextBlock>
    switches: Vec<Switch>,
    rsrc_dir: String,
}

impl WindowHandler for MyWindowHandler
{

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        
    }




    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {

        graphics.clear_screen(Color::from_rgb(0.25, 0.25, 0.25));
        graphics.draw_rectangle(
        Rectangle::from_tuples((650.0, 150.0), (1150.0, 650.0)),
        Color::from_rgb(0.1, 0.1, 0.1));


        let array = ARRAY.lock().unwrap();


    
        seg::display::text16(graphics, &mut vec![
            (0.0, String::from("hello")),
            (1.0, String::from(format!("player x{} y{} vx{} vy{} ifr{} hp{}", array[0].x(), array[0].y(), array[0].vx(), array[0].vy(), array[0].iframes(), array[0].hp()))),
            (2.0, String::from(format!("en1 x{} y{} vx{} vy{} ifr{} hp{}", array[1].x().floor(), array[1].y().floor(), (array[1].vx()*10.0).floor()/10.0, array[1].vy(), array[1].iframes(), array[1].hp()))),
            (3.0, String::from(format!("number of obj: {}", array.len()))),
            (26.6, String::from(format!("{}", self.rsrc_dir))),
        ]);

        for i in array.iter() {
            let mut obj_color:Vec<f32> = vec![0.5, 0.5, 0.5];
            if i.height() == 70.0 {
                obj_color = vec![0.5,0.5,0.8];
            }
            if i.height() == 75.0 {
                obj_color = vec![0.4,0.8,0.4];
            }

            if i.iframes() <= 0.0 {
                obj_color[0] = 1.0;
            }
            graphics.draw_rectangle(
            Rectangle::from_tuples(((i.x() as f32) + 650.0, (i.y() as f32) + 150.0),
            ((i.x() as f32 + i.width() as f32)+650.0, (i.y() as f32 + i.height() as f32) +150.0)),
            Color::from_rgb(obj_color[0], obj_color[1], obj_color[2]));
        }

        for s in &mut self.switches {
            let color;
            if s.state {color = Color::from_rgb(0.0, 0.5, 0.)}
            else {color = Color::from_rgb(0.3, 0.1, 0.1)}
            graphics.draw_rectangle(
                Rectangle::from_tuples((s.x, s.y), (s.x + 10.0 + (s.length * 12.0), s.y + 30.0)),
                color);

            let mut text: Vec<char> = s.text.clone().chars().collect();
            for t in 0..text.len() {
            seg::display::write_letter(graphics, &mut text[t], (s.x+(t as f32)*14.0)+2.0, s.y+5.0);
            }}

        helper.request_redraw();
    }
}



struct BrainScore {
    brain: String,
    score: i32,
}

/*
    ooo        ooooo            o8o              
    `88.       .888'            `"'              
    888b     d'888   .oooo.   oooo  ooo. .oo.   
    8 Y88. .P  888  `P  )88b  `888  `888P"Y88b  
    8  `888'   888   .oP"888   888   888   888  
    8    Y     888  d8(  888   888   888   888  
    o8o        o888o `Y888""8o o888o o888o o888o 

*/

fn main() {

    // different networks indicator
    // run:0, 
    // let mut brain = 0;

    // rust piston popup input 




    let switches = vec![
        Switch {
            x: 10.0,
            y: 400.0,
            length: 4.0,
            text: "test".to_string(),
            state: false,
        },

    ];

    

    let mut rsrc_dir = std::env::current_exe()
    .expect("Can't find path to executable");
    rsrc_dir.pop();
    rsrc_dir.push("JSON");
    
    let brain:u8 = 0;
    let mut generation = 0;

        thread::spawn( move|| {
    println!("work Thread");
    loop {
        generation += 1;
        println!("Generation {}", generation);
        let mut hands = vec![
            thread::spawn(move || {
                {
                main_sim(brain, false)
                }
            }),
        ];
            for _ in 0..5 { // 10 threads/ or generations are created where they will play till they die. they will then be given a score
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

        let brain_lookup:Vec<String> = vec![
            "src/JSON/run.json".to_string(),
            "src/JSON/run_jump.json".to_string()
            ];

        if generation_result[best].brain.chars().count() > 10 {
            fs::write(brain_lookup[0].clone(), generation_result[best].brain.clone()).expect("Unable to write file");
        }
    }
    });


    thread::spawn(move || {
        println!("display Thread");
        loop {
        main_sim(brain, true);
        }
    });




    let window = Window::new_centered("Title", (1200, 700)).unwrap();
    window.run_loop(MyWindowHandler{switches: switches, rsrc_dir: rsrc_dir.clone().to_str().unwrap().to_string()});


}
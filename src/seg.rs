

pub mod display {
    use speedy2d::{Graphics2D};
    use speedy2d::color::Color;

    use speedy2d::shape::{Rectangle};





    pub fn write_letter(g: &mut Graphics2D, c: &mut char , x:f32, y: f32) {

        c.make_ascii_lowercase();

        let mut color = Color::from_rgb(0.0, 0.0, 0.0);
        

        if "-0123456789".chars().collect::<Vec<char>>().contains(&c) {
            color = Color::from_rgb(0.0, 0.0, 0.2);
        }

        



        // ------

        if "02356789?abcdefgiopqrstz$&".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+0.0, y+0.0), (x+10.0, y+0.0), 1.5, color);
        }

        // |\|/|

        if "045689acefghklmnopqrsuvwy".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+0.0, y+0.0), (x+0.0, y+10.0), 1.5, color);
        }
        if "mnx\\)&>".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+0.0, y+0.0), (x+5.0, y+10.0), 1.5, color);
        }
        if "$+|bdit".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+5.0, y+0.0), (x+5.0, y+10.0), 1.5, color);
        }
        if "10<kmvxz(/".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+5.0, y+10.0), (x+10.0, y+0.0), 1.5, color);
        }
        if "01234789?abdhjmnopqruwy".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+10.0, y+0.0), (x+10.0, y+10.0), 1.5, color);
        }

        // --- ---

        if "$245689=aefhkrsy".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+0.0, y+10.0), (x+5.0, y+10.0), 1.5, color);
        }
        if "234689=abghprsy".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+5.0, y+10.0), (x+10.0, y+10.0), 1.5, color);
        }

        // |/|\|

        if "0268acefghjklmnopqruvw".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+0.0, y+10.0), (x+0.0, y+20.0), 1.5, color);
        }
        if "0vwx/)z&".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+0.0, y+20.0), (x+5.0, y+10.0), 1.5, color);
        }
        if "|?bdit".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+5.0, y+10.0), (x+5.0, y+20.0), 1.5, color);
        }
        if "5knr(wqx\\&".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+10.0, y+20.0), (x+5.0, y+10.0), 1.5, color);
        }
        if "89a3bd1gh6jm4noq0suw7y".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+10.0, y+10.0), (x+10.0, y+20.0), 1.5, color);
        }

        // --- ---

        if ".".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+4.0, y+20.0), (x+7.0, y+20.0), 3.0, color);
        }

        if ":".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+0.0, y+5.0), (x+3.0, y+5.0), 3.0, color);
            g.draw_line((x+0.0, y+15.0), (x+3.0, y+15.0), 3.0, color);
        }
        if "0235689bcdeijsgloquyz_&".chars().collect::<Vec<char>>().contains(&c) {
            g.draw_line((x+0.0, y+20.0), (x+10.0, y+20.0), 1.5, color);
        }
    }

    pub fn text16(g: &mut Graphics2D, d: &mut Vec<(f32, String)>) {

        for i in 0..d.len() {
            let mut text: Vec<char> = d[i].1.clone().chars().collect();
            for t in 0..text.len() {
            write_letter(g, &mut text[t], (((t as f32)+1.0)*14.0)+2.0, ((d[i].0+1.0)*24.0) + 2.0);
            }
        }

    }



}
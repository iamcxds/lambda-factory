use std::fmt;

use crate::lambda::*;
use raylib::prelude::*;

pub const OBJECT_SIZE: i32 = 300;

#[derive(Debug, Default)]
pub struct Game {
    pub lam_objs: Vec<LambdaObj<String>>,
    pub grab_obj: Option<LambdaObj<String>>,
    // the target obj drag into
    pub target_id: Option<usize>,

    pub factories: Vec<Factory<String>>,
    pub trashbin: Factory<String>,
}
#[derive(Debug, Default)]
pub struct Factory<T> {
    pub display: String,
    pub generator: Option<fn() -> LambdaBox<T>>,
    pub position: Vector2,
    pub size: f32,
    text_x: f32,
    text_y: f32,
    font_size: i32,
}
impl<T: fmt::Display> Factory<T> {
    fn set_up_text(&mut self, rl: &RaylibHandle) {
        let text_w = rl.measure_text(&self.display, 10);
        self.font_size = self.size.min(self.size * 10.0 / text_w as f32) as i32;
        let text_w = rl.measure_text(&self.display, self.font_size as i32);
        self.text_x = self.position.x + (self.size - text_w as f32) / 2.0;
        self.text_y = self.position.y + (self.size - self.font_size as f32) / 2.0;
    }
    pub fn new_factory(
        rl: &RaylibHandle,
        display: &str,
        generator: fn() -> LambdaBox<T>,
        x: f32,
        y: f32,
        size: f32,
    ) -> Self {
        let mut fac = Self {
            display: display.to_string(),
            generator: Some(generator),
            position: Vector2 { x, y },
            text_x: 0.0,
            text_y: 0.0,
            font_size: 0,
            size,
        };
        fac.set_up_text(rl);
        fac
    }
    pub fn new_trashbin(rl: &RaylibHandle, x: f32, y: f32, size: f32) -> Self {
        let display = "Trash Bin".to_string();
        let mut fac = Self {
            display,
            generator: None,
            position: Vector2 { x, y },
            text_x: 0.0,
            text_y: 0.0,
            font_size: 0,
            size,
        };
        fac.set_up_text(rl);
        fac
    }
    pub fn produce(&self) -> Option<LambdaObj<T>> {
        self.generator.map(|gener| {
            let obj = LambdaObj::new(gener(), self.position.x, self.position.y, self.size);
            println!("{}", obj.string);
            obj
        })
    }
    pub fn render(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rec(self.get_rect(), Color::GRAY);

        d.draw_text(
            &self.display,
            self.text_x as i32,
            self.text_y as i32,
            self.font_size as i32,
            Color::BLACK,
        );
    }
    pub fn get_rect(&self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.size,
            height: self.size,
        }
    }
}

#[derive(Debug, Default)]
pub struct LambdaObj<T>
where
    T: fmt::Display,
{
    lam_box: LambdaBox<T>,
    pub string: String,
    pub mino: LambdaMino<T>,
    // pub lego: LambdaLego,
    pub position: Vector2,
    pub size: f32,
    pub bkg_color: Color,
    can_eval: bool,
}
impl<T: fmt::Display> LambdaObj<T> {
    pub fn new(lam_box: LambdaBox<T>, x: f32, y: f32, size: f32) -> Self {
        Self {
            string: lam_box.to_string(),
            mino: lam_box.gen_mino(),
            // lego: lam_box.gen_lego(),
            lam_box,
            position: Vector2 { x, y },
            size,
            bkg_color: Color::LIGHTCYAN.alpha(0.7),
            can_eval: true,
        }
    }
    pub fn eval_onestep(&mut self) -> bool {
        if self.can_eval {
            let res = self.lam_box.eval_onestep();
            if res {
                self.string = self.lam_box.to_string();
                self.mino = self.lam_box.gen_mino();
                // self.lego = self.lam_box.gen_lego();
                println!("{}", self.string);
                // println!("width:{}", self.mino.width);
                // println!("height:{}", self.mino.height);
                // println!(
                //     "s-width:{},{}",
                //     self.mino.skew_width_l, self.mino.skew_width_r
                // );
                // println!("s-height:{}", self.mino.skew_height);
            }
            self.can_eval = res;
            res
        } else {
            false
        }
    }
    pub fn compose(&mut self, other: Self) {
        self.lam_box.compose("<", other.lam_box);
        self.string = self.lam_box.to_string();
        self.mino = self.lam_box.gen_mino();
        // self.lego = self.lam_box.gen_lego();
        self.can_eval = true;
        println!("{}", self.string);
        // println!("width:{}", self.mino.width);
        // println!("height:{}", self.mino.height);
        // println!(
        //     "s-width:{},{}",
        //     self.mino.skew_width_l, self.mino.skew_width_r
        // );
        // println!("s-height:{}", self.mino.skew_height);
    }
    pub fn render(&self, d: &mut RaylibDrawHandle, color: Color) {
        d.draw_rectangle_rec(self.get_rect(), color);
        self.mino.render(d, self.position, self.size);
        // self.lego.render(d, self.position, 30.0);
    }
    pub fn get_rect(&self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.size,
            height: self.size,
        }
    }
}

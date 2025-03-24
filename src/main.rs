use rand::prelude::*;
use raylib::prelude::*;
use std::{
    cmp::{max, min},
    fmt,
    ops::ControlFlow,
};

mod lambda;
use crate::lambda::*;

#[derive(Debug, Default)]
struct Game {
    lam_objs: Vec<LambdaObj<String>>,
    grab_obj: Option<LambdaObj<String>>,
    grab_offset: Vector2,
    // the target obj drag into
    target_id: Option<usize>,

    factories: Vec<Factory<String>>,
    trashbin: Factory<String>,
}

const SCR_W: i32 = 1600;
const SCR_H: i32 = 800;

const OBJECT_SIZE: i32 = 30;

fn main() {
    let (mut rl, thread) = raylib::init().size(SCR_W, SCR_H).title("λ-factory").build();
    // let font = rl
    //     .load_font_ex(&thread, "static/Acme_9_Regular_Bold.ttf", 30, None)
    //     .expect("couldn't load font");

    let mut game = Game::default();

    load(&mut game, &rl);
    while !rl.window_should_close() {
        update(&mut game, &rl);
        let mut d = rl.begin_drawing(&thread);
        // d.draw_text_ex(
        //     &font,
        //     "λ",
        //     Vector2::new(100.0, 200.0),
        //     OBJECT_SIZE as f32,
        //     1.0,
        //     Color::BLACK,
        // );
        draw(&game, &mut d);
    }
}

fn load(game: &mut Game, rl: &RaylibHandle) {
    // game.lam_objs.push(LambdaObj::new(
    //     LambdaBox::b_factory()
    //         .composition(LambdaBox::id_factory())
    //         .composition(LambdaBox::new_const("x"))
    //         .composition(LambdaBox::c_factory())
    //         .composition(LambdaBox::k_factory()),
    //     500.0,
    //     200.0,
    //     10.0 * OBJECT_SIZE as f32,
    // ));
    // game.lam_objs.push(LambdaObj::new(
    //     LambdaBox::b_factory()
    //         .composition(LambdaBox::c_factory())
    //         .composition(LambdaBox::k_factory()),
    //     100.0,
    //     200.0,
    //     10.0 * OBJECT_SIZE as f32,
    // ));
    game.factories.push(Factory::new_factory(
        &rl,
        "B-factory",
        LambdaBox::b_factory,
        50.0,
        500.0,
        10.0 * OBJECT_SIZE as f32,
    ));
    game.factories.push(Factory::new_factory(
        &rl,
        "C-factory",
        LambdaBox::c_factory,
        450.0,
        500.0,
        10.0 * OBJECT_SIZE as f32,
    ));
    game.factories.push(Factory::new_factory(
        &rl,
        "K-factory",
        LambdaBox::k_factory,
        850.0,
        500.0,
        10.0 * OBJECT_SIZE as f32,
    ));
    game.factories.push(Factory::new_factory(
        &rl,
        "W-factory",
        LambdaBox::w_factory,
        850.0,
        100.0,
        10.0 * OBJECT_SIZE as f32,
    ));
    game.factories.push(Factory::new_factory(
        &rl,
        "X-factory",
        || {
            let mut rng = rand::rng();
            LambdaBox::new_const((rng.sample(rand::distr::Alphanumeric) as char).to_string())
        },
        1250.0,
        500.0,
        10.0 * OBJECT_SIZE as f32,
    ));
    game.trashbin = Factory::new_trashbin(&rl, 10.0, 10.0, 2.0 * OBJECT_SIZE as f32);
}
fn update(game: &mut Game, rl: &RaylibHandle) {
    // use raylib::consts::KeyboardKey::*;
    // let position = &mut game.position;
    // if rl.is_key_released(KEY_UP) {
    //     position.1 -= 1;
    // }
    // if rl.is_key_released(KEY_DOWN) {
    //     position.1 += 1;
    // }
    // if rl.is_key_released(KEY_LEFT) {
    //     position.0 -= 1;
    // }
    // if rl.is_key_released(KEY_RIGHT) {
    //     position.0 += 1;
    // }
    // if rl.is_key_released(KEY_SPACE) {}

    use raylib::consts::MouseButton::*;
    let mouse_pos = rl.get_mouse_position();
    //grab object
    if rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) {
        let mut id = None;
        let _ = game
            .lam_objs
            .iter()
            .enumerate()
            .rev()
            .try_for_each(|(i, obj)| {
                if obj.get_rect().check_collision_point_rec(mouse_pos) {
                    id = Some(i);
                    return ControlFlow::Break(());
                }
                ControlFlow::Continue(())
            });
        if let Some(i) = id {
            let obj = game.lam_objs.remove(i);
            game.grab_offset = obj.position - mouse_pos;
            game.grab_obj = Some(obj);
        } else {
            let _ = game
                .factories
                .iter()
                // .enumerate()
                .rev()
                .try_for_each(|fac| {
                    if fac.get_rect().check_collision_point_rec(mouse_pos) {
                        if let Some(obj) = fac.produce() {
                            game.grab_offset = obj.position - mouse_pos;
                            game.grab_obj = Some(obj);
                            return ControlFlow::Break(());
                        }
                    }
                    ControlFlow::Continue(())
                });
        }
    }
    //release object
    else if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
        if let Some(obj) = game.grab_obj.take() {
            if let Some(t_id) = game.target_id {
                game.lam_objs[t_id].compose(obj);
            } else if game
                .trashbin
                .get_rect()
                .check_collision_point_rec(mouse_pos)
            {
            } else {
                game.lam_objs.push(obj);
            }
        }
    }

    game.target_id = None;
    //drag object
    if let Some(g_obj) = &mut game.grab_obj {
        if rl.is_mouse_button_down(MOUSE_BUTTON_LEFT) {
            g_obj.position = rl.get_mouse_position() + game.grab_offset;
            let grab_rect = g_obj.get_rect();
            let _ = game
                .lam_objs
                .iter()
                .enumerate()
                .rev()
                .try_for_each(|(i, obj)| {
                    if obj.get_rect().check_collision_recs(&grab_rect) {
                        game.target_id = Some(i);
                        return ControlFlow::Break(());
                    }
                    ControlFlow::Continue(())
                });
        }
    }
    // eval
    else if rl.is_mouse_button_released(MOUSE_BUTTON_RIGHT) {
        let mouse_pos = rl.get_mouse_position();
        let _ = &mut game.lam_objs.iter_mut().rev().try_for_each(|obj| {
            if obj.get_rect().check_collision_point_rec(mouse_pos) {
                let _ = obj.eval_onestep();
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
    }
}

fn draw(game: &Game, d: &mut RaylibDrawHandle) {
    d.clear_background(Color::WHITE);
    d.draw_text(
        "Drag to create and apply boxes, right click to evaluate.",
        300,
        10,
        30,
        Color::GRAY,
    );
    game.trashbin.render(d);
    game.factories.iter().for_each(|fac| {
        fac.render(d);
    });
    game.lam_objs.iter().enumerate().for_each(|(i, obj)| {
        if game.target_id.map_or(false, |id| id == i) {
            obj.render(d, Color::CYAN.alpha(0.7));
        } else {
            obj.render(d, obj.bkg_color);
        }
    });
    game.grab_obj.iter().for_each(|obj| {
        obj.render(d, obj.bkg_color);
    });
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
        self.font_size = min(self.size as i32, self.size as i32 * 10 / text_w);
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
        self.generator
            .map(|gener| LambdaObj::new(gener(), self.position.x, self.position.y, self.size))
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
                println!("{}", self.string);
                println!("width:{}", self.mino.width);
                println!("height:{}", self.mino.height);
                println!(
                    "s-width:{},{}",
                    self.mino.skew_width_l, self.mino.skew_width_r
                );
                println!("s-height:{}", self.mino.skew_height);
            }
            self.can_eval = res;
            res
        } else {
            false
        }
    }
    pub fn compose(&mut self, other: Self) {
        self.lam_box.compose(other.lam_box);
        self.string = self.lam_box.to_string();
        self.mino = self.lam_box.gen_mino();
        self.can_eval = true;
        println!("{}", self.string);
        println!("width:{}", self.mino.width);
        println!("height:{}", self.mino.height);
        println!(
            "s-width:{},{}",
            self.mino.skew_width_l, self.mino.skew_width_r
        );
        println!("s-height:{}", self.mino.skew_height);
    }
    pub fn render(&self, d: &mut RaylibDrawHandle, color: Color) {
        d.draw_rectangle_rec(self.get_rect(), color);
        self.mino.render(d, self.position, self.size);
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
impl<T: fmt::Display> LambdaMino<T> {
    /// position is left-up corner
    fn render(&self, d: &mut RaylibDrawHandle, position: Vector2, size: f32) {
        let mino = self;
        //draw outline
        let margin_rate = 0.015;
        let length = max(self.skew_width_l + self.skew_width_r, self.skew_height);
        let scale = (1.0 - 2.0 * margin_rate) * size / length as f32;
        //line thick
        let thick = scale * 0.1;
        let t_x = |pos: (i32, i32)| {
            position.x + size * (1.0 - margin_rate)
                - (pos.0 - pos.1 + (1 + length + mino.skew_width_r - mino.skew_width_l) / 2) as f32
                    * scale
        };
        let t_y = |pos: (i32, i32)| {
            position.y + size * (1.0 - margin_rate)
                - (pos.0 + pos.1 + 1 + (length - mino.skew_height) / 2) as f32 * scale
        };
        // draw the conection lines
        mino.squares.iter().for_each(|(_, sq)| {
            // draw the link lambda curves
            if let LambdaSqType::MLink(_, lk_pos) = &sq.sq_type {
                //get the block position
                let pos = (t_x(sq.pos), t_y(sq.pos));
                let target = (t_x(sq.target), t_y(sq.target));
                let lk_pos = *lk_pos.borrow();
                let link = (t_x(lk_pos), t_y(lk_pos));
                // if the the link apply to block in the same level
                if sq.pos.1 == sq.target.1 {
                    let color = Color::BROWN;
                    let mid_x = (pos.0 + target.0) / 2.0;
                    let mid_y = (pos.1 + target.1) / 2.0;
                    let point_lst = &vec![
                        Vector2::new(target.0, target.1),
                        Vector2::new(mid_x, mid_y),
                        Vector2::new(mid_x, pos.1),
                        Vector2::new(target.0, pos.1),
                        Vector2::new(target.0 + scale, pos.1),
                        Vector2::new(link.0 + scale, link.1 - scale),
                        Vector2::new(link.0, link.1),
                    ];
                    d.draw_spline_bezier_cubic(&point_lst, thick, color);
                }
                // if link apply from above
                else {
                    let point_lst = &vec![
                        Vector2::new(target.0, target.1),
                        Vector2::new(pos.0, pos.1),
                        Vector2::new(link.0 + scale, link.1 - scale),
                        Vector2::new(link.0, link.1),
                    ];
                    d.draw_spline_bezier_cubic(&point_lst, thick, Color::PURPLE);
                };
            }
            //draw the straight apply lines
            else {
                d.draw_line_ex(
                    Vector2::new(t_x(sq.pos), t_y(sq.pos)),
                    Vector2::new(t_x(sq.target), t_y(sq.target)),
                    thick,
                    Color::RED,
                );
            };
        });
        //draw the block symbols
        mino.squares.iter().for_each(|(_, sq)| {
            if let LambdaSqType::MLink(_, _) = sq.sq_type {
                // (
                //     (t_x(sq.pos.0) + t_x(sq.target.0)) / 2,
                //     (t_y(sq.pos.1) + t_y(sq.target.1)) / 2,
                // )
            } else {
                d.draw_text(
                    &sq.sq_type.to_string(),
                    (t_x(sq.pos) - scale / 4.0) as i32,
                    (t_y(sq.pos) - scale / 2.0) as i32,
                    scale as i32,
                    Color::BLUE,
                );
            };
        });
    }
}

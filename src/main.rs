use std::fmt;

use raylib::prelude::*;

mod lambda;
use crate::lambda::*;

#[derive(Debug, Default)]
struct Game {
    position: (i32, i32),
    lam_obj: LambdaObj<&'static str>,
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

    game.lam_obj = LambdaObj::new(
        LambdaBox::b_factory()
            .compose(LambdaBox::id_factory())
            .compose(LambdaBox::new_const("x"))
            .compose(LambdaBox::c_factory())
            .compose(LambdaBox::k_factory()),
    );

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

fn load(game: &mut Game, rl: &RaylibHandle) {}
fn update(game: &mut Game, rl: &RaylibHandle) {
    use raylib::consts::KeyboardKey::*;
    let position = &mut game.position;
    if rl.is_key_released(KEY_UP) {
        position.1 -= 1;
    }
    if rl.is_key_released(KEY_DOWN) {
        position.1 += 1;
    }
    if rl.is_key_released(KEY_LEFT) {
        position.0 -= 1;
    }
    if rl.is_key_released(KEY_RIGHT) {
        position.0 += 1;
    }
    if rl.is_key_released(KEY_SPACE) {
        let res = game.lam_obj.eval_onestep();
        println!("{}", game.lam_obj.string);
        println!("width:{}", game.lam_obj.mino.width);
        println!("height:{}", game.lam_obj.mino.height);
        println!("Eval made:{}", res);
    }
}

fn draw(game: &Game, d: &mut RaylibDrawHandle) {
    let position = Vector2::new(
        (SCR_W / 2 - 200 + &game.position.0 * OBJECT_SIZE) as f32,
        (SCR_H / 2 - 200 + &game.position.1 * OBJECT_SIZE) as f32,
    );
    d.clear_background(Color::WHITE);
    d.draw_text(&game.lam_obj.string, 100, 100, OBJECT_SIZE, Color::BLUE);
    // d.draw_circle_v(position, 10.0, Color::BLACK);
    // d.draw_spline_bezier_cubic(
    //     &vec![
    //         Vector2::new(100.0, 100.0),
    //         Vector2::new(150.0, 100.0),
    //         Vector2::new(200.0, 150.0),
    //         Vector2::new(200.0, 200.0),
    //         Vector2::new(200.0, 250.0),
    //         Vector2::new(220.0, 250.0),
    //         Vector2::new(300.0, 300.0),
    //     ],
    //     3.0,
    //     Color::PURPLE,
    // );
    let mino = &game.lam_obj.mino;
    mino.render(d, position, (10.0 * OBJECT_SIZE as f32));
}
impl<T: fmt::Display> LambdaMino<T> {
    /// position is left-up corner
    fn render(&self, d: &mut RaylibDrawHandle, position: Vector2, size: f32) {
        let mino = self;
        //draw outline
        let margin_rate = 0.015;
        let scale = (1.0 - 2.0 * margin_rate) * size / ((self.width + self.height) as f32);
        //line thick
        let thick = scale * 0.1;
        let t_x = |pos: (i32, i32)| {
            position.x + size * (1.0 - margin_rate) - (pos.0 - pos.1 + mino.height) as f32 * scale
        };
        let t_y = |pos: (i32, i32)| {
            position.y + size * (1.0 - margin_rate) - (pos.0 + pos.1 + 1) as f32 * scale
        };
        // draw the conection lines
        mino.squares.iter().for_each(|(_, sq)| {
            // draw the link lambda curves
            if let LambdaSqType::MLink(lk_ref) = &sq.sq_type {
                //get the block position
                let pos = (t_x(sq.pos), t_y(sq.pos));
                let target = (t_x(sq.target), t_y(sq.target));

                //get the linked position
                if let Some(lk_sq) = mino.squares.get(lk_ref) {
                    let link = (t_x(lk_sq.pos), t_y(lk_sq.pos));
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
                            Vector2::new(target.0 + scale / 2.0, pos.1),
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
            if let LambdaSqType::MLink(_) = sq.sq_type {
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
        d.draw_rectangle_lines_ex(
            Rectangle {
                x: position.x,
                y: position.y,
                width: size,
                height: size,
            },
            2.0,
            Color::BLACK,
        );
    }
}

// use bevy_ecs::prelude::*;
use raylib::prelude::*;

mod draw;
mod helper;
mod lambda;
use crate::draw::*;
mod update;
use crate::update::*;
mod game;
use crate::game::*;

const SCR_W: i32 = 1600;
const SCR_H: i32 = 800;

// #[derive(Resource)]
// struct RayLib(RaylibHandle);
// #[derive(Resource)]
// struct RLDraw<'a>(RaylibDrawHandle<'a>);

fn main() {
    let (mut rl, thread) = raylib::init().size(SCR_W, SCR_H).title("λ-factory").build();
    // let font = rl
    //     .load_font_ex(&thread, "static/Acme_9_Regular_Bold.ttf", 30, None)
    //     .expect("couldn't load font");

    let mut game = Game::default();
    let mut camera = Camera2D {
        offset: Vector2 {
            x: SCR_W as f32 / 2.0,
            y: SCR_H as f32 / 2.0,
        },
        target: Vector2 {
            x: SCR_W as f32 / 2.0,
            y: SCR_H as f32 / 2.0,
        },
        rotation: 0.0,
        zoom: 1.0,
    };

    load(&mut game, &rl);
    while !rl.window_should_close() {
        update(&mut game, &rl, &mut camera);

        let mut d = rl.begin_drawing(&thread);
        // d.draw_text_ex(
        //     &font,
        //     "λ",
        //     Vector2::new(100.0, 200.0),
        //     OBJECT_SIZE as f32,
        //     1.0,
        //     Color::BLACK,
        // );

        draw(&game, &mut d, &camera);
    }
}

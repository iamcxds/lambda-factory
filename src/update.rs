use crate::game::*;
use crate::lambda::*;
use crate::SCR_H;
use crate::SCR_W;
use rand::prelude::*;
use raylib::prelude::*;

pub fn load(game: &mut Game, rl: &RaylibHandle) {
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
    game.factories.insert(
        TilePosition(0, 0),
        Factory::new_factory(&rl, "S-factory", LambdaBox::s_factory, OBJECT_SIZE as f32),
    );
    game.factories.insert(
        TilePosition(2, 0),
        Factory::new_factory(
            &rl,
            "K-factory/TRUE",
            LambdaBox::k_factory,
            OBJECT_SIZE as f32,
        ),
    );
    game.factories.insert(
        TilePosition(4, 0),
        Factory::new_factory(&rl, "I-factory", LambdaBox::i_factory, OBJECT_SIZE as f32),
    );
    game.factories.insert(
        TilePosition(6, 0),
        Factory::new_factory(&rl, "Y-factory", LambdaBox::y_factory, OBJECT_SIZE as f32),
    );
    game.factories.insert(
        TilePosition(0, 2),
        Factory::new_factory(
            &rl,
            "B-factory/MULT",
            LambdaBox::b_factory,
            OBJECT_SIZE as f32,
        ),
    );
    game.factories.insert(
        TilePosition(2, 2),
        Factory::new_factory(&rl, "C-factory", LambdaBox::c_factory, OBJECT_SIZE as f32),
    );
    game.factories.insert(
        TilePosition(4, 2),
        Factory::new_factory(&rl, "W-factory", LambdaBox::w_factory, OBJECT_SIZE as f32),
    );
    game.factories.insert(
        TilePosition(6, 2),
        Factory::new_factory(
            &rl,
            "X-factory",
            || {
                let mut rng = rand::rng();
                LambdaBox::new_const((rng.sample(rand::distr::Alphanumeric) as char).to_string())
            },
            OBJECT_SIZE as f32,
        ),
    );
    game.factories.insert(
        TilePosition(0, 4),
        Factory::new_factory(&rl, "FALSE/0", LambdaBox::false_factory, OBJECT_SIZE as f32),
    );
    game.factories.insert(
        TilePosition(2, 4),
        Factory::new_factory(&rl, "SUCCESSOR", LambdaBox::succ, OBJECT_SIZE as f32),
    );
    game.factories.insert(
        TilePosition(-2, 2),
        Factory::new_factory(
            &rl,
            "PLUS",
            || {
                LambdaBox::c_factory()
                    .composition("<", LambdaBox::i_factory())
                    .composition("<", LambdaBox::succ())
            },
            OBJECT_SIZE as f32,
        ),
    );
    game.factories.insert(
        TilePosition(4, 4),
        Factory::new_factory(&rl, "PREDECESSOR", LambdaBox::pred, OBJECT_SIZE as f32),
    );
    game.factories.insert(
        TilePosition(6, 4),
        Factory::new_factory(&rl, "IS ZERO?", LambdaBox::is_zero, OBJECT_SIZE as f32),
    );
    game.factories.insert(
        TilePosition(8, 0),
        Factory::new_factory(&rl, "FOLD", LambdaBox::to_fold, OBJECT_SIZE as f32),
    );
    game.factories.insert(
        TilePosition(-2, -2),
        Factory::new_trashbin(&rl, OBJECT_SIZE as f32),
    );
}
pub fn update(game: &mut Game, rl: &RaylibHandle, camera: &mut Camera2D) {
    use raylib::consts::MouseButton::*;

    let scr_mouse_pos = rl.get_mouse_position();
    camera.zoom *= 1.0 + rl.get_mouse_wheel_move() * 0.05;
    camera.zoom = camera.zoom.max(0.1).min(10.0);
    let mouse_delta = rl.get_mouse_delta() / camera.zoom;
    if rl.is_mouse_button_down(MOUSE_BUTTON_MIDDLE) {
        camera.target -= mouse_delta;
    }

    game.screen_range = (
        TilePosition::from_vec2(rl.get_screen_to_world2D(Vector2 { x: 0.0, y: 0.0 }, *camera)),
        TilePosition::from_vec2(rl.get_screen_to_world2D(
            Vector2 {
                x: SCR_W as f32,
                y: SCR_H as f32,
            },
            *camera,
        )),
    );

    let mouse_pos = rl.get_screen_to_world2D(scr_mouse_pos, *camera);
    let mouse_tile_pos = TilePosition::from_vec2(mouse_pos);
    game.pointer_tile_pos = Some(mouse_tile_pos);
    //grab object or produce new
    if rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) {
        if let Some(obj) = game.lam_objs.remove(&mouse_tile_pos) {
            game.grab_obj = Some((mouse_tile_pos, obj));
        } else {
            if let Some(fac) = game.factories.get(&mouse_tile_pos) {
                if let Some(obj) = fac.produce() {
                    game.grab_obj = Some((mouse_tile_pos, obj));
                }
            }
        }
    }
    //release object
    else if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
        if let Some((_o_pos, obj)) = game.grab_obj.take() {
            if let Some(t_obj) = game.lam_objs.get_mut(&mouse_tile_pos) {
                t_obj.compose(obj);
            }
            //to trashbin
            else if game
                .factories
                .get_mut(&mouse_tile_pos)
                .map_or(false, |fac| fac.generator.is_none())
            {
            } else {
                game.lam_objs.insert(mouse_tile_pos, obj);
            }
        }
    }
    // eval
    else if rl.is_mouse_button_released(MOUSE_BUTTON_RIGHT)
        || rl.is_mouse_button_down(MOUSE_BUTTON_MIDDLE)
    {
        if let Some(t_obj) = game.lam_objs.get_mut(&mouse_tile_pos) {
            t_obj.eval_onestep();
        }
    }
}

use crate::game::*;
use crate::lambda::*;
use rand::prelude::*;
use raylib::prelude::*;
use std::ops::ControlFlow;

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
    game.factories.push(Factory::new_factory(
        &rl,
        "S-factory",
        LambdaBox::s_factory,
        50.0,
        100.0,
        OBJECT_SIZE as f32,
    ));
    game.factories.push(Factory::new_factory(
        &rl,
        "K-factory",
        LambdaBox::k_factory,
        450.0,
        100.0,
        OBJECT_SIZE as f32,
    ));
    game.factories.push(Factory::new_factory(
        &rl,
        "I-factory",
        LambdaBox::i_factory,
        850.0,
        100.0,
        OBJECT_SIZE as f32,
    ));
    game.factories.push(Factory::new_factory(
        &rl,
        "Y-factory",
        LambdaBox::y_factory,
        1250.0,
        100.0,
        OBJECT_SIZE as f32,
    ));
    game.factories.push(Factory::new_factory(
        &rl,
        "B-factory",
        LambdaBox::b_factory,
        50.0,
        500.0,
        OBJECT_SIZE as f32,
    ));
    game.factories.push(Factory::new_factory(
        &rl,
        "C-factory",
        LambdaBox::c_factory,
        450.0,
        500.0,
        OBJECT_SIZE as f32,
    ));
    game.factories.push(Factory::new_factory(
        &rl,
        "W-factory",
        LambdaBox::w_factory,
        850.0,
        500.0,
        OBJECT_SIZE as f32,
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
        OBJECT_SIZE as f32,
    ));
    game.trashbin = Factory::new_trashbin(&rl, 10.0, 10.0, OBJECT_SIZE as f32 / 5.0);
}
pub fn update(game: &mut Game, rl: &RaylibHandle, camera: &mut Camera2D) {
    use raylib::consts::MouseButton::*;

    let scr_mouse_pos = rl.get_mouse_position();
    let mouse_pos = rl.get_screen_to_world2D(scr_mouse_pos, *camera);
    camera.zoom *= 1.0 + rl.get_mouse_wheel_move() * 0.05;
    camera.zoom = camera.zoom.max(0.1).min(10.0);
    let mouse_delta = rl.get_mouse_delta() / camera.zoom;
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
    if rl.is_mouse_button_down(MOUSE_BUTTON_LEFT) {
        if let Some(g_obj) = &mut game.grab_obj {
            g_obj.position += mouse_delta;
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
        } else {
            camera.target -= mouse_delta;
        }
    }
    // eval
    else if rl.is_mouse_button_released(MOUSE_BUTTON_RIGHT) {
        let _ = &mut game.lam_objs.iter_mut().rev().try_for_each(|obj| {
            if obj.get_rect().check_collision_point_rec(mouse_pos) {
                let _ = obj.eval_onestep();
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
    }
}

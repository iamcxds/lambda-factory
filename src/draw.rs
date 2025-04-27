use crate::game::*;
use crate::lambda::*;
use raylib::prelude::*;
pub fn draw(game: &Game, d: &mut RaylibDrawHandle, camera: &Camera2D) {
    d.clear_background(Color::WHITE);
    {
        let d2 = &mut d.begin_mode2D(camera);
        draw_grid(
            d2,
            game.screen_range.0,
            game.screen_range.1 + TilePosition(1, 1),
        );
        game.factories.iter().for_each(|(pos, fac)| {
            fac.render(d2, *pos);
        });
        game.lam_objs.iter().for_each(|(pos, obj)| {
            if game.pointer_tile_pos.map_or(false, |p_pos| p_pos == *pos) {
                obj.render(d2, *pos, Color::CYAN.alpha(0.7));
            } else {
                obj.render(d2, *pos, obj.bkg_color);
            }
        });
        game.grab_obj.iter().for_each(|(pos, obj)| {
            let tile_pos = if let Some(p_pos) = game.pointer_tile_pos {
                p_pos
            } else {
                *pos
            };
            obj.render(d2, tile_pos, obj.bkg_color);
        });
    }
    d.draw_text(
        "Drag to create and apply boxes, right click to evaluate.",
        300,
        10,
        30,
        Color::GRAY,
    );
}
fn draw_grid(d: &mut RaylibDrawHandle, st_pos: TilePosition, ed_pos: TilePosition) {
    let min_x = st_pos.0;
    let max_x = ed_pos.0;
    let min_y = st_pos.1;
    let max_y = ed_pos.1;
    for x in min_x..=max_x {
        let start_pos = TilePosition(x, min_y).to_vec2();
        let end_pos = TilePosition(x, max_y).to_vec2();
        d.draw_line_ex(start_pos, end_pos, 10.0, Color::LIGHTGRAY);
    }
    for y in min_y..=max_y {
        let start_pos = TilePosition(min_x, y).to_vec2();
        let end_pos = TilePosition(max_x, y).to_vec2();
        d.draw_line_ex(start_pos, end_pos, 10.0, Color::LIGHTGRAY);
    }
}

impl<T: std::fmt::Display> LambdaMino<T> {
    /// position is left-up corner
    pub fn render(&self, d: &mut RaylibDrawHandle, position: Vector2, size: f32) {
        let mino = self;
        //draw outline
        let margin_rate = 0.015;
        let width = self.skew_width_l + self.skew_width_r;
        let height = self.skew_height;
        let length = width.max(height);
        let scale = (1.0 - 2.0 * margin_rate) * size / length as f32;
        //line thick
        let thick = scale * 0.1;
        let t_x = |pos: (i32, i32)| {
            position.x //+ size * 0.5 + 0.5 * (width as f32) * scale
                - (pos.0 - pos.1  + mino.skew_width_r -(length+width)/2) as f32 * scale
        };
        let t_y = |pos: (i32, i32)| {
            position.y //+ size * (1.0 - margin_rate)
                - (pos.0 + pos.1-(length + height-1) / 2) as f32 * scale
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
                    let point_lst = [
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
                    let point_lst = [
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
        if scale >= 5.0 {
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
}
// impl LambdaLego {
//     pub fn render(&self, d: &mut RaylibDrawHandle, position: Vector2, scale: f32) {
//         let face_color = Color::YELLOWGREEN;
//         let thick_scale = scale * 0.3;
//         let t_x = |i, x: i32| position.x + x as f32 * scale - i as f32 * thick_scale;
//         let t_y = |i, y: i32| position.y + y as f32 * scale + i as f32 * thick_scale;
//         self.rect_list
//             .iter()
//             .zip(self.symbol_list.iter())
//             .enumerate()
//             .for_each(|(i, (rects, sybs))| {
//                 rects.iter().for_each(|(rect, is_top)| {
//                     let vo = Vector2 {
//                         x: t_x(i, rect.x + rect.w),
//                         y: t_y(i, rect.y),
//                     };
//                     let vx = Vector2 {
//                         x: -rect.w as f32 * scale,
//                         y: 0.0,
//                     };
//                     let vy = Vector2 {
//                         x: 0.0,
//                         y: rect.h as f32 * scale,
//                     };
//                     let f_rect = |i| Rectangle {
//                         x: t_x(i, rect.x),
//                         y: t_y(i, rect.y),
//                         width: scale * rect.w as f32,
//                         height: scale * rect.h as f32,
//                     };
//                     if *is_top {
//                         d.draw_rectangle_rec(f_rect(i), face_color);
//                         // d.draw_rectangle_lines_ex(f_rect, 1.0, Color::BLACK);
//                         d.draw_spline_linear(
//                             &[vo, vo + vx, vo + vx + vy, vo + vy, vo],
//                             1.0,
//                             Color::BLACK,
//                         );
//                     } else {
//                         let vz = Vector2 {
//                             x: -thick_scale,
//                             y: thick_scale,
//                         };
//                         d.draw_rectangle_rec(f_rect(i + 1), face_color);
//                         let points = [vo, vo + vx, vo + vx + vz, vo + vz, vo + vz + vy, vo + vy];
//                         d.draw_triangle_fan(&points, face_color);
//                         d.draw_spline_linear(
//                             &[
//                                 vo,
//                                 vo + vx,
//                                 vo + vx + vz,
//                                 vo + vz,
//                                 vo,
//                                 vo + vy,
//                                 vo + vy + vz,
//                                 vo + vz,
//                             ],
//                             1.0,
//                             Color::BLACK,
//                         );
//                         d.draw_spline_linear(
//                             &[vo + vx + vz, vo + vx + vy + vz, vo + vy + vz],
//                             1.0,
//                             Color::BLACK,
//                         );
//                     }
//                 });
//                 sybs.iter().for_each(|(pos, sybs, is_top)| {
//                     if *is_top {
//                         d.draw_text(
//                             sybs,
//                             (t_x(i, pos.0) - scale * 0.9) as i32,
//                             (t_y(i, pos.1)) as i32,
//                             (scale) as i32,
//                             Color::BLACK,
//                         );
//                     } else {
//                         d.draw_text(
//                             sybs,
//                             (t_x(i, pos.0) - thick_scale * 0.7) as i32,
//                             (t_y(i, pos.1)) as i32,
//                             (thick_scale) as i32,
//                             Color::BLACK,
//                         );
//                     }
//                 });
//             });
//     }
// }

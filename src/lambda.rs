use std::cmp::max;
use std::{cell::RefCell, collections::HashMap, fmt, mem::swap, rc::Rc};

const LAMBDA: char = 'λ';

#[derive(Debug)]
pub struct LambdaBox<T>(Rc<RefCell<LamExpr<T>>>);
#[derive(Debug)]
pub struct LambdaRef<T>(Rc<RefCell<LamExpr<T>>>);
#[derive(Debug)]
enum LamExpr<T> {
    //binding varible
    Var,
    //free varible Const
    Con(T),
    App(LambdaBox<T>, LambdaBox<T>),
    // Lam label x f, x is jut pointer to the variable
    Lam(LambdaRef<T>, LambdaBox<T>),
    Link(LambdaBox<T>),
    DuplRef(LambdaRef<T>),
    // duplication (origin,duplication)
    DuplLink(LambdaBox<T>, LambdaBox<T>),
}

use crate::lambda::LamExpr::*;
impl<T> LamExpr<T> {
    fn wrap(self) -> LambdaBox<T> {
        LambdaBox(Rc::new(RefCell::new(self)))
    }
    fn wrap_ref(self) -> (LambdaBox<T>, LambdaRef<T>) {
        let block = Rc::new(RefCell::new(self));
        (LambdaBox(block.clone()), LambdaRef(block.clone()))
    }
}
impl<T> LambdaBox<T> {
    fn take(&mut self) -> LamExpr<T> {
        let LambdaBox(expr) = self;
        let mut var = LamExpr::default();
        swap(&mut var, &mut expr.borrow_mut());
        var
    }
    fn get_ref(&self) -> LambdaRef<T> {
        LambdaRef(self.0.clone())
    }

    ///return if the expr changed
    pub fn eval(&mut self) -> bool {
        let expr = self.0.clone();
        match &mut *expr.borrow_mut() {
            Link(f) => {
                let res = f.eval();
                *self = LambdaBox(f.0.clone());
                res
            }
            Lam(_, f) => f.eval(),
            App(f, g) => {
                let res = f.eval() || g.eval();
                match &mut *f.0.borrow_mut() {
                    Lam(LambdaRef(y), h) => {
                        *y.borrow_mut() = Link(LambdaBox(g.0.clone()));
                        h.eval();
                        *self = LambdaBox(h.0.clone());
                        true
                    }
                    _ => res,
                }
            }
            _ => false,
        }
    }
    pub fn eval_value(mut self) -> Self {
        self.eval();
        self
    }
    /// process Link and duplication.
    fn do_link(&mut self, //  ref_map: HashMap<LambdaRef<T>, LambdaRef<T>>
    ) {
        let expr = self.0.clone();
        match &mut *expr.borrow_mut() {
            Link(f) => {
                f.do_link();
                *self = LambdaBox(f.0.clone());
            }
            //
            DuplRef(f) => {
                let mut f_box = f.0.borrow_mut();
                // f_box.do_link();
                // *f = f_box.get_ref();
                match &mut *f_box {
                    Link(g) => {
                        let mut dlk = DuplRef(g.get_ref()).wrap();
                        dlk.do_link();
                        *self = dlk
                    }
                    DuplLink(_, b) => {
                        b.do_link();
                        *self = LambdaBox(b.0.clone());
                        // *f_box = a.take();
                    }
                    DuplRef(x) => {
                        // let (mut var, var_ref) = DuplRef(x.clone()).wrap_ref();
                        // var.do_link();
                        // {
                        //     let mut x_box = LambdaBox(x.0.clone());
                        //     let x_expr = x_box.take();
                        //     let new_box = x_expr.wrap();
                        //     let mut x_ref = x.0.borrow_mut();
                        //     *x_ref = DuplLink(new_box, var)
                        // }
                        if let DuplLink(_, b) = &*x.0.clone().borrow() {
                            *self = DuplRef(b.get_ref()).wrap();
                        }
                    }
                    Lam(x, g) => {
                        let (var, var_ref) = Var.wrap_ref();
                        {
                            // let mut x_box = LambdaBox(x.0.clone());
                            // let x_expr = x_box.take();
                            let mut x_ref = x.0.borrow_mut();
                            *x_ref = DuplLink(Var.wrap(), var)
                        }
                        let mut lk_g = DuplRef(g.get_ref()).wrap();
                        lk_g.do_link();
                        *self = Lam(var_ref, lk_g).wrap();
                    }
                    App(h, g) => {
                        let mut lk_h = DuplRef(h.get_ref()).wrap();
                        let mut lk_g = DuplRef(g.get_ref()).wrap();
                        lk_h.do_link();
                        lk_g.do_link();
                        *self = App(lk_h, lk_g).wrap();
                    }

                    _ => {}
                }
            }
            Lam(_, f) => {
                f.do_link();
            }
            App(f, g) => {
                f.do_link();
                g.do_link();
            }
            _ => {}
        }
    }
    ///return if the expr changed
    pub fn eval_onestep(&mut self) -> bool {
        let expr = self.0.clone();
        match &mut *expr.borrow_mut() {
            Link(f) => {
                let res = f.eval_onestep();
                *self = LambdaBox(f.0.clone());
                res
            }
            Lam(_, f) => f.eval_onestep(),
            App(f, g) => {
                // f.eval_onestep();
                {
                    let f_ref = &mut *(f.0.borrow_mut());
                    if let Lam(LambdaRef(y), h) = f_ref {
                        *y.borrow_mut() = Link(LambdaBox(g.0.clone()));
                        h.do_link();
                        *self = LambdaBox(h.0.clone());
                        return true;
                    }
                }
                if f.eval_onestep() {
                    return true;
                } else {
                    g.eval_onestep()
                }
            }
            _ => false,
        }
    }
    pub fn composition(self, expr_box_2: Self) -> Self {
        App(self, expr_box_2).wrap()
    }
    pub fn compose(&mut self, expr_box_2: Self) {
        *self = App(LambdaBox(self.0.clone()), expr_box_2).wrap();
    }
    /// return (abstaction, move out)
    pub fn abstr(self, ptr: LambdaRef<T>) -> (Self, Self) {
        let check_pass = self.check_ref(&ptr).unwrap_or(false);
        let carrier = Self::default();
        if check_pass {
            carrier.0.swap(&ptr.0);
            (Lam(ptr, self).wrap(), carrier)
        } else {
            (Lam(Self::default().get_ref(), self).wrap(), carrier)
        }
    }

    /// check if the pointer point to free varible in pression. None -> not found, Some(true)-> Ok, Some(false)-> found binding var
    pub fn check_ref(&self, pointer: &LambdaRef<T>) -> Option<bool> {
        let expr_box = &self.0;
        if Rc::ptr_eq(&pointer.0, expr_box) {
            Some(true)
        } else {
            let expr = &*expr_box.borrow();
            match expr {
                App(f, g) => {
                    if let Some(b) = f.check_ref(pointer) {
                        Some(b)
                    } else {
                        g.check_ref(pointer)
                    }
                }
                //not a binding varible
                Lam(LambdaRef(x), f) => {
                    if Rc::ptr_eq(&pointer.0, x) {
                        Some(false)
                    } else {
                        f.check_ref(pointer)
                    }
                }
                DuplRef(f) => LambdaBox(f.0.clone()).check_ref(pointer),
                Link(f) => f.check_ref(pointer),
                DuplLink(f, g) => {
                    if let Some(b) = f.check_ref(pointer) {
                        Some(b)
                    } else {
                        g.check_ref(pointer)
                    }
                }
                _ => None,
            }
        }
    }
    /// |x| x
    pub fn id_factory() -> Self {
        let (x, x_r) = Var.wrap_ref();
        x.abstr(x_r).0
    }
    /// B x y z = x (y z)
    pub fn b_factory() -> Self {
        let (x, x_r) = Var.wrap_ref();
        let (y, y_r) = Var.wrap_ref();
        let (z, z_r) = Var.wrap_ref();
        let expr = x.composition(y.composition(z));
        let expr = expr.abstr(z_r).0;
        let expr = expr.abstr(y_r).0;
        let expr = expr.abstr(x_r).0;
        expr
    }
    ///C x y z = x z y
    pub fn c_factory() -> Self {
        let (x, x_r) = Var.wrap_ref();
        let (y, y_r) = Var.wrap_ref();
        let (z, z_r) = Var.wrap_ref();
        let expr = x.composition(z).composition(y);
        let expr = expr.abstr(z_r).0;
        let expr = expr.abstr(y_r).0;
        let expr = expr.abstr(x_r).0;
        expr
    }
    pub fn k_factory() -> Self {
        let (x, x_r) = Var.wrap_ref();
        let (_y, y_r) = Var.wrap_ref();
        let expr = x.abstr(y_r).0;
        let expr = expr.abstr(x_r).0;
        expr
    }
    pub fn w_factory() -> Self {
        let (x, x_r) = Var.wrap_ref();
        let (y, y_r) = Var.wrap_ref();
        let expr = x.composition(y).composition(DuplRef(y_r.clone()).wrap());
        println!("{}", LambdaBox(y_r.0.clone()).get_ref() == y_r);
        let expr = expr.abstr(y_r).0;
        let expr = expr.abstr(x_r).0;
        expr
    }
    pub fn new_const(t: T) -> Self {
        Con(t).wrap()
    }
}
impl<T> PartialEq for LambdaRef<T> {
    fn eq(&self, LambdaRef(other): &Self) -> bool {
        let LambdaRef(this) = self;
        Rc::ptr_eq(this, other)
    }
}
impl<T> Eq for LambdaRef<T> {}
impl<T> Clone for LambdaRef<T> {
    fn clone(&self) -> Self {
        LambdaRef(self.0.clone())
    }
}
impl<T> std::hash::Hash for LambdaRef<T> {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: std::hash::Hasher,
    {
        hasher.write_usize(Rc::as_ptr(&self.0) as usize);
    }
}
impl<T: fmt::Display> LambdaBox<T> {
    fn fmt_context(
        &self,
        ref_map: &mut HashMap<LambdaRef<T>, i32>,
        index: &mut Box<i32>,
    ) -> String {
        let expr_box = self.0.clone();
        let expr = &*expr_box.borrow();
        match expr {
            Var => {
                if let Some(j) = ref_map.get(&self.get_ref()) {
                    j.to_string()
                } else {
                    **index += 1;
                    (**index - 1).to_string()
                }
            }
            Con(s) => s.to_string(),
            Lam(x, f) => {
                let c_index = **index;
                ref_map.insert(x.clone(), c_index);
                **index += 1;

                format!(
                    "|{}|.{}",
                    // LAMBDA,
                    c_index,
                    f.fmt_context(ref_map, index)
                )
            }
            App(f, g) => {
                let f_fmt = match &*f.0.borrow() {
                    Lam(_, _) => format!("({})", f.fmt_context(ref_map, index)),
                    _ => format!("{}", f.fmt_context(ref_map, index)),
                };
                match &*g.0.borrow() {
                    App(h, i) => format!(
                        "{} ({} {})",
                        f_fmt,
                        h.fmt_context(ref_map, index),
                        i.fmt_context(ref_map, index)
                    ),
                    _ => format!("{} {}", f_fmt, g.fmt_context(ref_map, index)),
                }
            }
            Link(f1) => f1.fmt_context(ref_map, index),
            DuplRef(f2) => LambdaBox(f2.0.clone()).fmt_context(ref_map, index),
            DuplLink(a, _) => a.fmt_context(ref_map, index),
        }
    }
    pub fn gen_mino(&self) -> LambdaMino<T> {
        let mut mino = self.gen_mino_context(self.get_ref(), &mut HashMap::new(), (0, 0), (-1, 0));
        mino.update_link();
        mino
    }
    ///generate a LambdaMino, start at the given position
    fn gen_mino_context(
        &self,
        sq_ref: LambdaRef<T>,
        ref_map: &mut HashMap<LambdaRef<T>, LambdaRef<T>>,
        pos: MinoPos,
        target: MinoPos,
    ) -> LambdaMino<T> {
        let expr_box = self.0.clone();
        let expr = &*expr_box.borrow();
        match expr {
            Var => {
                let mut mino = LambdaMino::default();
                if let Some(link) = ref_map.get(&self.get_ref()) {
                    let sq = LambdaSquare {
                        pos,
                        target,
                        sq_type: MLink(link.clone(), (0, 0).into()),
                    };
                    mino.squares.insert(sq_ref, sq);
                }
                mino
            }
            Con(s) => {
                let mut mino = LambdaMino::default();
                let sq = LambdaSquare {
                    pos,
                    target,
                    sq_type: MCon(s.to_string()),
                };
                mino.squares.insert(sq_ref, sq);
                mino.up_convex.insert(pos.0, pos.1);
                mino.down_convex.insert(pos.0, pos.1);
                mino.width = 1;
                mino.height = 1;
                mino.skew_width_l = 1;
                mino.skew_width_r = 1;
                mino.skew_height = 2;
                mino
            }
            Lam(x, f) => {
                ref_map.insert(x.clone(), self.get_ref());
                let mut mino = f.gen_mino_context(f.get_ref(), ref_map, (pos.0 + 1, pos.1), pos);
                let sq = LambdaSquare {
                    pos,
                    target,
                    sq_type: MLam,
                };
                mino.squares.insert(sq_ref, sq);
                mino.up_convex.insert(pos.0, pos.1);
                mino.down_convex.insert(pos.0, pos.1);
                mino.width += 1;
                mino.height = max(mino.height, 1);

                mino.skew_width_l += 1;
                mino.skew_width_r = max(mino.skew_width_r - 1, 1);
                mino.skew_height += 1;
                mino
            }
            App(f, g) => {
                let mut mino = f.gen_mino_context(f.get_ref(), ref_map, (0, 0), (-1, 0));

                mino.app_combine(g.gen_mino_context(g.get_ref(), ref_map, (0, 0), (-1, 0)));
                mino.move_mino(pos, target);
                let sq = LambdaSquare {
                    pos,
                    target,
                    sq_type: MApp,
                };
                mino.squares.insert(self.get_ref(), sq);
                mino
            }
            Link(f) => f.gen_mino_context(sq_ref, ref_map, pos, target),
            DuplRef(f) => LambdaBox(f.0.clone()).gen_mino_context(sq_ref, ref_map, pos, target),
            DuplLink(a, _) => a.gen_mino_context(sq_ref, ref_map, pos, target),
        }
    }
}

/// Notice x>0 toward left
type MinoPos = (i32, i32);
#[derive(Debug)]
pub enum LambdaSqType<T> {
    MCon(String),
    MApp,
    MLam,
    MLink(LambdaRef<T>, RefCell<MinoPos>),
}
impl<T: fmt::Display> fmt::Display for LambdaSqType<T> {
    fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MCon(s) => write!(fm, "{}", s.to_string()),
            MApp => write!(fm, "@"),
            MLam => write!(fm, "/"),
            MLink(_, _) => write!(fm, ""),
        }
    }
}
use crate::lambda::LambdaSqType::*;
#[derive(Debug)]
pub struct LambdaSquare<T> {
    pub pos: MinoPos,
    pub target: MinoPos,
    pub sq_type: LambdaSqType<T>,
}

#[derive(Debug)]
pub struct LambdaMino<T> {
    pub squares: HashMap<LambdaRef<T>, LambdaSquare<T>>,
    up_convex: HashMap<i32, i32>,
    down_convex: HashMap<i32, i32>,
    pub width: i32,
    pub height: i32,
    // width and height in rotation 45 degree
    pub skew_width_l: i32,
    pub skew_width_r: i32,
    pub skew_height: i32,
}
impl<T> LambdaMino<T> {
    fn update_link(&mut self) {
        self.squares.iter().for_each(|(_, sq)| {
            if let MLink(lk_ref, lk_pos) = &sq.sq_type {
                if let Some(lk_sq) = self.squares.get(lk_ref) {
                    lk_pos.replace(lk_sq.pos);
                }
            }
        })
    }
    ///move a mino at (0,0)
    fn move_mino(&mut self, offset: MinoPos, target: MinoPos) {
        self.squares.iter_mut().for_each(|(_, sq)| {
            let tg = sq.target;
            sq.target = (tg.0 + offset.0, tg.1 + offset.1);
            let pos = sq.pos;
            if pos == (0, 0) {
                sq.target = target
            }
            sq.pos = (pos.0 + offset.0, pos.1 + offset.1);
        });

        let up_convex = self
            .up_convex
            .iter()
            .map(|(x, y)| (x + offset.0, y + offset.1))
            .collect();
        self.up_convex = up_convex;
        let down_convex = self
            .down_convex
            .iter()
            .map(|(x, y)| (x + offset.0, y + offset.1))
            .collect();
        self.down_convex = down_convex;
    }
    ///combine two mino to a new square at (0,0)
    fn app_combine(&mut self, mut other: Self) {
        self.move_mino((1, 0), (0, 0));
        self.up_convex.entry(0).or_insert(0);
        self.down_convex.entry(0).or_insert(0);
        self.width += 1;
        self.height = max(self.height, 1);
        //get the height move for the other mino
        let diff_map = other
            .down_convex
            .iter()
            .map(|(d_x, d_y)| self.up_convex.get(d_x).unwrap_or(&0) - d_y);
        let diff = 1 + diff_map.max().unwrap_or(0);
        other.move_mino((0, diff), (0, 0));

        self.squares.extend(other.squares);
        other.up_convex.iter().for_each(|(x, y)| {
            let y_1 = *self.up_convex.entry(*x).or_insert(*y);
            if y_1 < *y {
                self.up_convex.insert(*x, *y);
            }
        });
        other.down_convex.iter().for_each(|(x, y)| {
            let y_1 = *self.down_convex.entry(*x).or_insert(*y);
            if y_1 > *y {
                self.down_convex.insert(*x, *y);
            }
        });
        self.width = max(self.width, other.width);
        self.height = max(self.height, other.height + diff);
        self.skew_width_l = self
            .down_convex
            .iter()
            .map(|(x, y)| x - y)
            .max()
            .unwrap_or(0)
            + 1;
        self.skew_width_r = self.up_convex.iter().map(|(x, y)| y - x).max().unwrap_or(0) + 1;
        self.skew_height = self.up_convex.iter().map(|(x, y)| y + x).max().unwrap_or(0) + 2;
    }
}
impl<T: fmt::Display> fmt::Display for LambdaBox<T> {
    fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ref_map = HashMap::new();
        let mut index = Box::new(0);
        write!(fm, "{}", self.fmt_context(&mut ref_map, &mut index))
    }
}
impl<T> Default for LambdaMino<T> {
    fn default() -> Self {
        LambdaMino {
            squares: HashMap::new(),
            up_convex: HashMap::new(),
            down_convex: HashMap::new(),
            width: 0,
            height: 0,
            skew_width_l: 0,
            skew_width_r: 0,
            skew_height: 0,
        }
    }
}
impl<T> Default for LambdaBox<T> {
    fn default() -> Self {
        Var.wrap()
    }
}
impl<T> Default for LamExpr<T> {
    fn default() -> Self {
        Var
    }
}

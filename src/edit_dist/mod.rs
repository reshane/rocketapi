mod vec_2d;

use std::cmp::min;
use crate::edit_dist::vec_2d::Vec2D;

pub fn edit_dist(a: &String, b: &String) -> usize {
    let len = (a.len() + 1) * (b.len() + 1);
    let mut mem: Vec2D<usize> = Vec2D::new(
        vec![0; len],
        a.len() + 1,
        b.len() + 1
    );
    for i in 0..(a.len() + 1) {
        *mem.index_mut(i, 0) = i;
    }
    for j in 0..(b.len() + 1) {
        *mem.index_mut(0, j) = j;
    }
    for i in 1..(a.len() + 1) {
        for j in 1..(b.len() + 1) {
            if a.chars().nth(i-1) == b.chars().nth(j-1) {
                *mem.index_mut(i, j) = *mem.index(i-1, j-1);
            } else {
                let insertion = 1 + mem.index(i, j-1);
                let deletion = 1 + mem.index(i-1, j);
                let replacement = 1 + mem.index(i-1, j-1);

                *mem.index_mut(i, j) = min(insertion, min(deletion, replacement));
            }
        }
    }
    *mem.index(a.len(), b.len())
}

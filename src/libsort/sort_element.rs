use ggez::{
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};

use crate::INIT_WINDOW_SIZE;

#[derive(Default)]
pub enum SortState {
    #[default]
    UNSORTED,
    SORTED,
    SELECTED,
}

impl SortState {
    pub fn get_color(&self) -> Color {
        match self {
            Self::UNSORTED => Color::WHITE,
            Self::SORTED => Color::GREEN,
            Self::SELECTED => Color::RED,
        }
    }
}

#[derive(Clone)]
/// Contains `mesh` and `rect` which mainly represents the bars drawn and also the coordinates
/// Gets sorted as a whole.
pub struct SortElement {
    /// Mesh that gets drawn
    pub mesh: graphics::Mesh,
    /// Part that will get sorted
    /// NOTE: use zero coordinates, otherwise the position will get added to the mesh as well
    pub rect: graphics::Rect,
    // FIXME: implement color change, currently it looks like we would have to replace the mesh with a
    // new one, that has the updated color.
}

impl SortElement {
    pub fn new(
        ctx: &mut Context,
        i: usize,
        elem_value: f32,
        max_value: f32,
        no_rects: u32,
    ) -> GameResult<Self> {
        const WIDTH: f32 = 10.;
        const X_OFFSET: f32 = 10.;

        let max_width = no_rects as f32 * WIDTH;
        let x_scale: f32 = INIT_WINDOW_SIZE.0 / max_width;
        let y_scale: f32 = INIT_WINDOW_SIZE.1 / max_value;

        let l_state = SortState::default();

        // we need to reverse the height, since origin is at top left
        let l_rectangle = graphics::Rect::new(
            X_OFFSET * x_scale * (i as f32),
            0.,
            10. * x_scale,
            -elem_value * y_scale,
        );

        let l_object = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            l_rectangle,
            l_state.get_color(),
        )?;

        Ok(Self {
            mesh: l_object,
            rect: l_rectangle,
        })
    }

    pub fn get_sort_value(&self) -> f32 {
        -self.rect.h
    }
}

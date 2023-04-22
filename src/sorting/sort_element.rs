use ggez::{
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};

use rand::{self, Rng};

const MAX_VALUE: f32 = 750.0;

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

// TODO: implement color change, currently it looks like we would have to replace the mesh with a
// new one, that has the updated color.
pub struct SortElement {
    /// Mesh that gets drawn
    pub(crate) mesh: graphics::Mesh,
    /// Part that will get sorted
    pub(crate) rect: graphics::Rect,
    /// Holds sorting state
    pub(crate) state: SortState,
}

impl SortElement {
    pub fn new(ctx: &mut Context, i: usize) -> GameResult<Self> {
        const SCALE: f32 = 0.75;
        const X_OFFSET: f32 = 10.;
        let mut rng = rand::thread_rng();

        let l_state = SortState::default();

        let mut l_rectangle = graphics::Rect::new(0., 100., 10., 0.);

        // we need to reverse the height, since origin is at top left
        l_rectangle.h = -rng.gen_range(0.0..MAX_VALUE);
        l_rectangle.x += X_OFFSET + i as f32 * l_rectangle.w * SCALE;

        let l_object = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            l_rectangle,
            l_state.get_color(),
        )?;

        Ok(Self {
            mesh: l_object,
            rect: l_rectangle,
            state: l_state,
        })
    }

    pub fn get_height(&self) -> f32 {
        -self.rect.h
    }
}

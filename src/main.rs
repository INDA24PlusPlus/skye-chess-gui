
use std::borrow::Borrow;

use chess_lib::*;
use ggez::{
    event,
    glam::*,
    graphics::{self, Color, Rect},
    Context, GameResult,
};

struct MainState {
    pos_x: f32,
    board: [graphics::Mesh;64],
    game: chess_lib::Game
}

impl MainState {
    fn new(ctx: &mut Context)-> GameResult<MainState> {
        let a=(ctx.gfx.drawable_size().0.min(ctx.gfx.drawable_size().1)-30.0).max(0.0)/8.0;
        let b=graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect { x: 15.0, y: 10.0, w: a, h: a }, Color::GREEN)?;
        let mut board:[graphics::Mesh;64]=std::array::from_fn(|i| b.clone());
        let mut gme=chess_lib::start();
        let mut idx=0.0;
        let mut idx2=0.0;
        for ind in 0..64{

            board[ind]=graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect { x: 15.0+(idx%8.0)*a, y: 15.0+(idx2*a), w: a, h: a }, if ((idx%8.0)+idx2)%2.0==0.0 { Color::GREEN } else { Color::WHITE })?;
            idx+=1.0;
            if idx%8.0==0.0{
                idx2+=1.0;
            }
        }
        Ok(MainState {pos_x: 0.0, board:board, game:gme})
        
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        for sq in self.board.clone(){

        canvas.draw(&sq, Vec2::new(self.pos_x, 0.0));

        }

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

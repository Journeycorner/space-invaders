use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::space_invaders::{JellyFishAlien, ARENA_HEIGHT, JELLY_FISH_ALIEN_HEIGHT};

pub struct AliensSystem {
  pub x_moves: i32,
}

impl<'s> System<'s> for AliensSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, JellyFishAlien>,
    Read<'s, InputHandler<String, String>>,
  );

  fn run(&mut self, (mut transforms, aliens, input): Self::SystemData) {
    for (alien, transform) in (&aliens, &mut transforms).join() {
      if self.x_moves < 80 {
        self.x_moves += 1;
        transform.translate_x(1.0);
      } else {
        transform.translate_x(-self.x_moves as f32);
        self.x_moves = 0;
        transform.translate_y(-20.0);
      }
    }
  }
}

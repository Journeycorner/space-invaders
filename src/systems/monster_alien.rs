use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::space_invaders::{MonsterAlien, ARENA_HEIGHT, JELLY_FISH_ALIEN_HEIGHT};

pub enum Direction {
  Right,
  Left,
}

impl Default for Direction {
  fn default() -> Self {
    Direction::Right
  }
}

pub struct MonsterAlienSystem {
  pub x_moves: i32,
  pub time_since_move: i32,
  pub direction: Direction,
}

impl Default for MonsterAlienSystem {
  fn default() -> Self {
    MonsterAlienSystem {
      x_moves: 0,
      time_since_move: 0,
      direction: Default::default(),
    }
  }
}

impl<'s> System<'s> for MonsterAlienSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    WriteStorage<'s, MonsterAlien>,
    Read<'s, Time>,
  );

  fn run(&mut self, (mut transforms, mut aliens, time): Self::SystemData) {
    let mut flip = false;
    // detect direction flip
    for (_, transform) in (&mut aliens, &mut transforms).join() {
      match &self.direction {
        Direction::Right => {
          if transform.translation()[0] >= 420.0 {
            self.direction = Direction::Left;
            flip = true;
            break;
          }
        }
        Direction::Left => {
          if transform.translation()[0] <= 80.0 {
            self.direction = Direction::Right;
            flip = true;
            break;
          }
        }
      }
    }

    // jump down with all aliens
    if flip {
      for (_, transform) in (&mut aliens, &mut transforms).join() {
        transform.translate_y(-30.0);
      }
    }

    for (mut alien, transform) in (&mut aliens, &mut transforms).join() {
      alien.time_since_move += time.delta_seconds();
      if alien.time_since_move > 1.0 {
        alien.time_since_move = 0.0;
        match &self.direction {
          Direction::Right => {
            transform.translate_x(6.0);
          }
          Direction::Left => {
            transform.translate_x(-6.0);
          }
        }
      }
    }
  }
}

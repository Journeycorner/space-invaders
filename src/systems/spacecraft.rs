use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::{VirtualKeyCode};

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::space_invaders::{Spacecraft, ARENA_HEIGHT, SPACECRAFT_HEIGHT};

pub struct SpacecraftSystem;

impl<'s> System<'s> for SpacecraftSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Spacecraft>,
    Read<'s, InputHandler<String, String>>,
  );

  fn run(&mut self, (mut transforms, spacecrafts, input): Self::SystemData) {
    for (_, transform) in (&spacecrafts, &mut transforms).join() {
      if let Some(mv_amount) = input.axis_value("spacecraft") {
        if mv_amount != 0.0 {
          let scaled_amount = 1.2 * mv_amount as f32;
          transform.translate_x(scaled_amount);
        }
      }
      // TODO move key binding to bindings_config.ron
      if input.key_is_down(VirtualKeyCode::Space) {
        println!("Firing weapons!");
      }
    }
  }
}

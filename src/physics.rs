
//! Trying to module physics using state research.

use UpdateDelta = state::UpdateDelta;

/// The difference in time from one frame to the next.
pub struct DeltaTime(f64);

pub type Position = [f64, ..3];
pub type Velocity = [f64, ..3];

/// Object in motion under no force.
pub struct NoForce {
    /// The position of object.
    pub pos: Position,
    /// The velocity of object.
    pub vel: Velocity,
}

impl UpdateDelta<DeltaTime> for NoForce {
    #[inline(always)]
    fn update(&mut self, &DeltaTime(dt): &DeltaTime) {
        for i in range(0u, 3) {
            self.pos[i] += self.vel[i] * dt;
        }
    }
}


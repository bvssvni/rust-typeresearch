
//! Objects that move influenced by acceleration.

use physics::{Property, DynamicalProperties};
use physics::{Position, Velocity, Acceleration};
use physics::DeltaTime;
use state::UpdateDelta;

/// Moves point with acceleration.
pub struct WithAcceleration {
    /// The position of object.
    pub pos: Position,
    /// The velocity of object.
    pub vel: Velocity,
    /// The acceleration on object.
    pub acc: Acceleration,
}

impl UpdateDelta<DeltaTime> for WithAcceleration {
    #[inline(always)]
    fn update(&mut self, &DeltaTime(dt): &DeltaTime) {
        for i in range(0u, 3) {
            let vel_next = self.vel[i] + self.acc[i] * dt;
            self.pos[i] += 0.5 * dt * (self.vel[i] + vel_next);
        }
    }
}

impl DynamicalProperties for WithAcceleration {
    #[inline(always)]
    fn get_vec3<'a>(&'a self, prop: Property) -> Option<&'a [f64, ..3]> {
        match prop {
            Position => Some(&self.pos),
            Velocity => Some(&self.vel),
            Acceleration => Some(&self.acc),
            _ => None,
        }
    }

    #[inline(always)]
    fn get_mut_vec3<'a>(&'a mut self, prop: Property) -> Option<&'a mut [f64, ..3]> {
        match prop {
            Position => Some(&mut self.pos),
            Velocity => Some(&mut self.vel),
            Acceleration => Some(&mut self.acc),
            _ => None,
        }
    }

    #[inline(always)]
    fn get_f64(&self, _prop: Property) -> Option<f64> {
        None
    }

    #[inline(always)]
    fn get_mut_f64<'a>(&'a mut self, _prop: Property) -> Option<&'a mut f64> {
        None
    }
}

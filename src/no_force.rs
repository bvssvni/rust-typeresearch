
//! Objects that are moving under no influence of force.
//! Such objects move in a straight line.

use physics::{Property, DynamicalProperties};
use physics::{Position, Velocity};
use physics::DeltaTime;
use state::UpdateDelta;

/// Object in motion under no force.
pub struct NoForce {
    /// The position of object.
    pub pos: Position,
    /// The velocity of object.
    pub vel: Velocity,
}

impl DynamicalProperties for NoForce {
    fn get_vec3<'a>(&'a self, prop: Property) -> Option<&'a [f64, ..3]> {
        match prop {
            Position => Some(&self.pos),
            Velocity => Some(&self.vel),
            _ => None,
        }
    }

    fn get_mut_vec3<'a>(&'a mut self, prop: Property) -> Option<&'a mut [f64, ..3]> {
        match prop {
            Position => Some(&mut self.pos),
            Velocity => Some(&mut self.vel),
            _ => None,
        }
    }

    fn get_f64(&self, _prop: Property) -> Option<f64> {
        None
    }

    fn get_mut_f64<'a>(&'a mut self, _prop: Property) -> Option<&'a mut f64> {
        None
    }
}

impl UpdateDelta<DeltaTime> for NoForce {
    #[inline(always)]
    fn update(&mut self, &DeltaTime(dt): &DeltaTime) {
        for i in range(0u, 3) {
            self.pos[i] += self.vel[i] * dt;
        }
    }
}

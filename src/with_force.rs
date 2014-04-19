
//! Objects that move under influence by force.

use physics::{Property, DynamicalProperties};
use physics::{Position, Velocity, Force, InvMass};
use physics::DeltaTime;
use state::UpdateDelta;

/// Moves with force.
pub struct WithForce {
    /// The position.
    pub pos: Position,
    /// The velocity.
    pub vel: Velocity,
    /// The force.
    pub force: Force,
    /// The inverse mass.
    pub inv_mass: InvMass,
}

impl UpdateDelta<DeltaTime> for WithForce {
    fn update(&mut self, &DeltaTime(dt): &DeltaTime) {
        for i in range(0u, 3) {
            let acc = self.force[i] * self.inv_mass;
            let vel_next = self.vel[i] + acc * dt;
            self.pos[i] += 0.5 * dt * (self.vel[i] + vel_next);
        }
    }
}

impl DynamicalProperties for WithForce {
    fn get_vec3<'a>(&'a self, prop: Property) -> Option<&'a [f64, ..3]> {
        match prop {
            Position => Some(&self.pos),
            Velocity => Some(&self.vel),
            Force => Some(&self.force),
            _ => None,
        }
    }

    fn get_mut_vec3<'a>(&'a mut self, prop: Property) -> Option<&'a mut [f64, ..3]> {
        match prop {
            Position => Some(&mut self.pos),
            Velocity => Some(&mut self.vel),
            Force => Some(&mut self.force),
            _ => None,
        }
    }

    fn get_f64(&self, prop: Property) -> Option<f64> {
        match prop {
            InvMass => Some(self.inv_mass),
            _ => None,
        }
    }

    fn get_mut_f64<'a>(&'a mut self, prop: Property) -> Option<&'a mut f64> {
        match prop {
            InvMass => Some(&mut self.inv_mass),
            _ => None,
        }
    }
}

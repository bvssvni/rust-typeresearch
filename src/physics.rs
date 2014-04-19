
//! Trying to module physics using state research.

use UpdateDelta = state::UpdateDelta;

/// The difference in time from one frame to the next.
pub struct DeltaTime(f64);

pub type Position = [f64, ..3];
pub type Velocity = [f64, ..3];
pub type Acceleration = [f64, ..3];
pub type Force = [f64, ..3];
pub type Mass = f64;
pub type InvMass = f64;

/// Gets the position.
pub trait PositionProperty {
    /// Get a readonly position.
    fn pos<'a>(&'a self) -> &'a Position;
    /// Get a mutable position.
    fn pos_mut<'a>(&'a mut self) -> &'a mut Position;
}

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

impl PositionProperty for NoForce {
    #[inline(always)]
    fn pos<'a>(&'a self) -> &'a Position {
        &self.pos
    }

    #[inline(always)]
    fn pos_mut<'a>(&'a mut self) -> &'a mut Position {
        &mut self.pos
    }
}

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

impl PositionProperty for WithAcceleration {
    fn pos<'a>(&'a self) -> &'a Position {
        &self.pos
    }

    fn pos_mut<'a>(&'a mut self) -> &'a mut Position {
        &mut self.pos
    }
}

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

impl PositionProperty for WithForce {
    fn pos<'a>(&'a self) -> &'a Position {
        &self.pos
    }

    fn pos_mut<'a>(&'a mut self) -> &'a mut Position {
        &mut self.pos
    }
}


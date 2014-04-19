
//! Trying to module physics using state research.

pub mod no_force;
pub mod with_acceleration;
pub mod with_force;

/// The difference in time from one frame to the next.
pub struct DeltaTime(f64);

pub type Position = [f64, ..3];
pub type Velocity = [f64, ..3];
pub type Acceleration = [f64, ..3];
pub type Force = [f64, ..3];
pub type Mass = f64;
pub type InvMass = f64;

/// Set of properties that physical objects can have.
pub enum Property {
    /// Position [f64, ..3].
    Position,
    /// Velocity [f64, ..3].
    Velocity,
    /// Acceleration [f64, ..3].
    Acceleration,
    /// Force [f64, ..3].
    Force,
    /// Mass f64.
    Mass,
    /// Inverse mass f64.
    InvMass,
}

/// Implemented on types that can access properties dynamically.
pub trait DynamicalProperties {
    /// Gets a readonly vector.
    fn get_vec3<'a>(&'a self, prop: Property) -> Option<&'a [f64, ..3]>;
    /// Gets a mutable vector.
    fn get_mut_vec3<'a>(&'a mut self, prop: Property) -> Option<&'a mut [f64, ..3]>;
    /// Gets a scalar.
    fn get_f64(&self, prop: Property) -> Option<f64>;
    /// Gets a mutable scalar.
    fn get_mut_f64<'a>(&'a mut self, prop: Property) -> Option<&'a mut f64>;
}




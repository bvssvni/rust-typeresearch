
//! This is an experiment trying to model touch.
//!
//! The target resolution of the modeling is everyday language.
//! A successful model will be able to 'compute' touch logic.
//! 
//! A computer can not really touch anything without sensors.
//! This gives the first impression that modeling touch is unusable.
//! Instead let us imagine that the computer controls the sensation.
//! The computer does not feel, so it does not know what the
//! sensation should be like under different circumstances.
//! Therefore we can program a computer to give the right sensation.
//!
//! ### N-dimensional semanthic space
//!
//! It is difficult to come up with an accurate description of 'soft'.
//! Yet this is a concept that is shared among many people.
//!
//! One way to solve this problem is mapping to a N-dimensional
//! semanthic space, where the number of dimensions are unknown.
//! Each word associated with touch is a point in that space.
//! To describe a sensation, each word is given a weight.
//! 
//! We do not model the N-dimensional space in this library,
//! because it might change depending on system.
//! Instead it is sufficient to assign each word a weight.
//! The user of the library can then decide how to transform.
//!
//! ### Opposite words
//!
//! Some words are opposites, like 'hot' and 'cold'.
//! This is necessary to avoid flattening the space.
//! We do not need words that are in the middle,
//! because we can just assign the same weight to each end.
//!
//! ### Human anatomy
//!
//! Humans have the ability feel touch through their entire skin.
//! This means one can feel a touch with some part of the body,
//! and another kind of touch with another part.
//! In that respect it becomes to complex to describe the state
//! of the whole touching experience.
//!
//! To simplify we assume that a surface or a certain kind of interaction
//! has a specific touch experience, which is independent of how
//! much or which part of the skin one uses to touch.
//!
//! ### Associated words with touch experience
//!
//! There are words that are associated with the touch experience
//! that is not a directly result of the touch interaction.
//! These are words like 'solid' and 'fragile'
//! or 'elastic' and 'rigid'.
//!
//! These associated experiences are left out to reduce the scope of the library. 

/// Contains a list of words associated with distinct touch experiences.
#[deriving(Clone, Eq, TotalOrd, TotalEq, Ord, Hash)]
pub enum Touch {
    /// Cold.
    Cold,
    /// Hard.
    Hard,
    /// Hot.
    Hot,
    /// Rough.
    Rough,
    /// Smooth.
    Smooth,
    /// Soft.
    Soft,
    /// Warm.
    Warm,
}

impl Touch {
    /// Creates a simple touch sensation.
    pub fn simple(touch: Touch) -> Vec<(Touch, f64)> {
        vec!((touch, 1.0))
    }
}

/// Implemented by all touchable objects.
pub trait Touchable {
    /// Returns a list of values that can be transformed.
    /// All values that are not in the list has weight 0.
    ///
    /// Returns Err if not sufficient information.
    /// These are cases when a touchable object
    /// gives different touch experiences based on internal state.
    /// Without sufficient information, it can not compute the experience.
    ///
    /// The vector should be sorted in order to perform Boolean algebra. 
    fn touch(&self) -> Result<Vec<(Touch, f64)>, ~str>;
}


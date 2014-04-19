
//! A different way of thinking about states.
//!
//! All objects are stored in a state.
//!
//! You can either manually create states for different types,
//! or you can use a `StateMachine` to move between states of same type.

use HashMap = collections::HashMap;

pub mod state_machine;

/// Contains objects.
pub struct State<Object> {
    objects: HashMap<uint, Object>,
}

impl<T> State<T> {
    /// Moves an object from one state to another.
    #[inline(always)]
    pub fn move_to(&mut self, to: &mut State<T>, obj: uint) {
        let pop = self.objects.pop(&obj);
        match pop {
            None => {},
            Some(val) => { to.objects.insert(obj, val); }
        }
    }

    /// Removes object from state.
    #[inline(always)]
    pub fn pop(&mut self, obj: uint) -> Option<T> {
        self.objects.pop(&obj)
    }

    /// Returns a readonly pointer to object.
    #[inline(always)]
    pub fn get<'a>(&'a self, obj: uint) -> &'a T {
        self.objects.get(&obj)
    }

    /// Returns a mutable pointer to object.
    #[inline(always)]
    pub fn get_mut<'a>(&'a mut self, obj: uint) -> &'a mut T {
        self.objects.get_mut(&obj)
    }
}

/// Updates on delta information.
///
/// For any type implementing this trait,
/// there is a `State<T>` and `StateMachine<T>`
/// which also implements this trait.
pub trait UpdateDelta<D> {
    /// Updates all the objects with delta information.
    fn update(&mut self, delta: &D);
}

impl<T: UpdateDelta<D>, D>
UpdateDelta<D> for State<T> {
    fn update(&mut self, delta: &D) {
        for (_, obj) in self.objects.mut_iter() {
            obj.update(delta);
        }
    }
}


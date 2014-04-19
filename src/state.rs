
//! A different way of thinking about states.
//!
//! All objects are stored in a state.

use HashMap = collections::HashMap;

/// Keeps track of the current state of objects.
pub struct StateMachine<Object> {
    current: State<CurrentState<Object>>,
    states: State<State<Object>>,
}

impl<T> StateMachine<T> {
    /// Returns the current value of object by looking up in correct state.
    #[inline(always)]
    pub fn val<'a>(&'a self, obj: uint) -> &'a T {
        let &CurrentState{ state: state } = self.current.objects.get(&obj);
        self.states.objects.get(&state).objects.get(&obj)
    }
}

/// Stores the current state of an object.
struct CurrentState<Object> {
    state: uint,
}

/// Contains objects.
pub struct State<Object> {
    objects: HashMap<uint, Object>,
}

impl<T> State<T> {
    /// Moves an object from one state to another.
    pub fn move_to(&mut self, to: &mut State<T>, obj: uint) {
        let pop = self.objects.pop(&obj);
        match pop {
            None => {},
            Some(val) => { to.objects.insert(obj, val); }
        }
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

impl<T: UpdateDelta<D>, D>
UpdateDelta<D> for StateMachine<T> {
    fn update(&mut self, delta: &D) {
        for (_, state) in self.states.objects.mut_iter() {
            state.update(delta);
        }
    }
}


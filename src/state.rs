
//! A different way of thinking about states.
//!
//! All objects are stored in a state.

use HashMap = collections::HashMap;

pub type UpdateFunction<T, D> = fn (obj: &mut T, delta: &D);

/// Keeps track of the current state of objects.
/// Also keeps an update function per state.
/// This is because otherwise all states require same logic.
pub struct StateMachine<Object, Delta> {
    current: State<CurrentState<Object>>,
    states: State<State<Object>>,
    update_functions: State<UpdateFunction<Object, Delta>>
}

impl<T, D> StateMachine<T, D> {
    /// Adds a new state.
    pub fn add_state(
            &mut self, 
            state_id: uint,
            state: State<T>, 
            update: Option<UpdateFunction<T, D>>
    ) {
        self.states.objects.insert(state_id, state);
        match update {
            None => {},
            Some(fun) => { self.update_functions.objects.insert(state_id, fun); }
        }
    }

    /// Adds a new object.
    pub fn add_object(
            &mut self,
            state_id: uint,
            obj_id: uint,
            obj: T) {
        self.current.objects.insert(obj_id, CurrentState { state: state_id });
        self.states.objects.get_mut(&state_id).objects.insert(obj_id, obj);
    }

    /// Returns the current value of object by looking up in correct state.
    #[inline(always)]
    pub fn val<'a>(&'a self, obj: uint) -> &'a T {
        let &CurrentState { state: state } = self.current.objects.get(&obj);
        self.states.objects.get(&state).objects.get(&obj)
    }

    /// Returns the current `state_id` of an object.
    pub fn state_id(&self, obj: uint) -> uint {
        let &CurrentState { state: state } = self.current.objects.get(&obj);
        state
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
UpdateDelta<D> for StateMachine<T, D> {
    fn update(&mut self, delta: &D) {
        for (id_state, state) in self.states.objects.mut_iter() {
            match self.update_functions.objects.find(id_state) {
                    None => {},
                    Some(fun) => {
                        for (_, obj) in state.objects.mut_iter() {
                            (*fun)(obj, delta);
                        }
                    }
            }
        }
    }
}


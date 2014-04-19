//! Logic for state machine.

use state::State;
use state::UpdateDelta;

pub type UpdateFunction<T, D> = fn (obj: &mut T, delta: &D);

/// A unique identifier for object.
pub struct ObjectId(uint);

/// A unique identifier for state.
pub struct StateId(uint);

/// Keeps track of the current state of objects.
/// Also keeps an update function per state.
/// This is because otherwise all states require same logic.
pub struct StateMachine<Object, Delta> {
    current: State<StateId>,
    states: State<State<Object>>,
    update_functions: State<UpdateFunction<Object, Delta>>
}

impl<T, D> StateMachine<T, D> {
    /// Adds a new state.
    pub fn add_state(
            &mut self, 
            StateId(state_id): StateId,
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
            StateId(state_id): StateId,
            ObjectId(obj_id): ObjectId,
            obj: T) {
        self.current.objects.insert(obj_id, StateId(state_id));
        self.states.objects.get_mut(&state_id).objects.insert(obj_id, obj);
    }

    /// Returns a readonly pointer to object.
    #[inline(always)]
    pub fn get<'a>(&'a self, ObjectId(obj): ObjectId) -> &'a T {
        let &StateId(state) = self.current.objects.get(&obj);
        self.states.objects.get(&state).objects.get(&obj)
    }

    /// Returns a mutable pointer to object.
    #[inline(always)]
    pub fn get_mut<'a>(&'a mut self, ObjectId(obj): ObjectId) -> &'a mut T {
        let &StateId(state) = self.current.objects.get(&obj);
        self.states.objects.get_mut(&state).objects.get_mut(&obj)
    }

    /// Returns the current `state_id` of an object.
    pub fn state_id(&self, ObjectId(obj): ObjectId) -> StateId {
        *self.current.objects.get(&obj)
    }

    /// Returns the current state of an object.
    pub fn state<'a>(&'a self, ObjectId(obj): ObjectId) -> &'a State<T> {
        let &StateId(state) = self.current.objects.get(&obj);
        self.states.objects.get(&state)
    }
    
    /// Moves an object to a new state.
    pub fn move_to(&mut self, StateId(to): StateId, ObjectId(obj): ObjectId) {
        let &StateId(state) = self.current.objects.get(&obj);
        let pop = self.states.objects.get_mut(&state).objects.pop(&obj);
        match pop {
            None => {},
            Some(val) => { self.states.objects.get_mut(&to).objects.insert(obj, val); }
        }
    }

    /// Removes an object from state machine.
    pub fn pop(&mut self, ObjectId(obj): ObjectId) -> Option<T> {
        let &StateId(state) = self.current.objects.get(&obj);
        self.states.objects.get_mut(&state).objects.pop(&obj)
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


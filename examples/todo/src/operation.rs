use iced::{
    advanced::widget::{operate, Id, Operation},
    Task,
};

pub fn swap_modify_states<Message, State, Modify>(t1: Id, t2: Id, modify: Modify) -> Task<Message>
where
    Message: Send + 'static,
    State: Clone + Send + 'static,
    Modify: Fn(&State, &State) -> State + Clone + Send + 'static,
{
    operate(swap_modify_states_operation(t1, t2, modify))
}

pub fn swap_modify_states_operation<T, State, Modify>(
    t1: Id,
    t2: Id,
    modify: Modify,
) -> impl Operation<T>
where
    State: Clone + Send + 'static,
    Modify: Fn(&State, &State) -> State + Clone + Send + 'static,
{
    struct FindTargets<State, Modify>
    where
        State: Clone + Send + 'static,
        Modify: Fn(&State, &State) -> State + Clone + Send + 'static,
    {
        t1: Id,
        t2: Id,
        modify: Modify,
        t1_state: Option<State>,
        t2_state: Option<State>,
    }

    impl<T, State, Modify> Operation<T> for FindTargets<State, Modify>
    where
        State: Clone + Send + 'static,
        Modify: Fn(&State, &State) -> State + Clone + Send + 'static,
    {
        fn container(
            &mut self,
            _id: Option<&Id>,
            _bounds: iced::Rectangle,
            operate_on_children: &mut dyn FnMut(&mut dyn Operation<T>),
        ) {
            if self.t1_state.is_some() && self.t2_state.is_some() {
                return;
            }
            operate_on_children(self);
        }

        fn custom(&mut self, state: &mut dyn std::any::Any, id: Option<&Id>) {
            if self.t1_state.is_some() && self.t2_state.is_some() {
                return;
            }
            if let Some(state) = state.downcast_mut::<State>() {
                match id {
                    Some(id) => {
                        if id == &self.t1 {
                            self.t1_state = Some(state.clone());
                        } else if id == &self.t2 {
                            self.t2_state = Some(state.clone());
                        }
                    }
                    None => (),
                }
            }
        }

        fn finish(&self) -> iced::advanced::widget::operation::Outcome<T> {
            if self.t1_state.is_none() || self.t2_state.is_none() {
                iced::advanced::widget::operation::Outcome::None
            } else {
                iced::advanced::widget::operation::Outcome::Chain(Box::new(SwapModify {
                    t1: self.t1.clone(),
                    t2: self.t2.clone(),
                    modify: self.modify.clone(),
                    t1_state: self.t1_state.clone().unwrap(),
                    t2_state: self.t2_state.clone().unwrap(),
                    swapped_t1: false,
                    swapped_t2: false,
                }))
            }
        }
    }

    struct SwapModify<State, Modify>
    where
        State: Clone + 'static,
        Modify: Fn(&State, &State) -> State + Clone + 'static,
    {
        t1: Id,
        t2: Id,
        modify: Modify,
        t1_state: State,
        t2_state: State,
        swapped_t1: bool,
        swapped_t2: bool,
    }

    impl<T, State, Modify> Operation<T> for SwapModify<State, Modify>
    where
        State: Clone + Send + 'static,
        Modify: Fn(&State, &State) -> State + Clone + Send + 'static,
    {
        fn container(
            &mut self,
            _id: Option<&Id>,
            _bounds: iced::Rectangle,
            operate_on_children: &mut dyn FnMut(&mut dyn Operation<T>),
        ) {
            if self.swapped_t1 && self.swapped_t2 {
                return;
            }
            operate_on_children(self);
        }

        fn custom(&mut self, state: &mut dyn std::any::Any, id: Option<&Id>) {
            if self.swapped_t1 && self.swapped_t2 {
                return;
            }
            if let Some(state) = state.downcast_mut::<State>() {
                match id {
                    Some(id) => {
                        if id == &self.t1 {
                            *state = (self.modify)(state, &self.t2_state);
                            self.swapped_t1 = true;
                        } else if id == &self.t2 {
                            *state = (self.modify)(state, &self.t1_state);
                            self.swapped_t2 = true;
                        }
                    }
                    None => (),
                }
            }
        }

        fn finish(&self) -> iced::advanced::widget::operation::Outcome<T> {
            iced::advanced::widget::operation::Outcome::None
        }
    }

    FindTargets {
        t1,
        t2,
        modify,
        t1_state: None,
        t2_state: None,
    }
}

pub(crate) trait Minimax<State, Action>
where
    State: Clone,
{
    fn utility(&self, state: &State) -> i32;
    fn terminal(&self, state: &State) -> bool;
    fn actions(&self, state: &State) -> Vec<Action>;
    fn result(&self, state: &State, action: Action) -> State;

    // Minimax will handle 3 possible values as result of the game:
    // 1 Player 1 wins
    // -1 Player 2 wins
    // 0 Draw
    // This information will be used to do optimizations
    fn min_value(&self, state: &State) -> i32 {
        if self.terminal(state) {
            return self.utility(state);
        }

        let mut v = i32::MAX;
        for action in self.actions(state) {
            v = v.min(self.max_value(&self.result(state, action)));
            if v == -1 {
                return v;
            }
        }

        v
    }
    fn max_value(&self, state: &State) -> i32 {
        if self.terminal(state) {
            return self.utility(state);
        }

        let mut v = i32::MIN;
        for action in self.actions(state) {
            v = v.max(self.min_value(&self.result(state, action)));
            if v == 1 {
                return v;
            }
        }

        v
    }
}

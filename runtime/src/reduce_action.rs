use tree_sitter_language::Symbol;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct ReduceAction {
    pub count: u32,
    pub symbol: Symbol,
    pub dynamic_precedence: i32,
    pub production_id: u16,
}
pub(crate) type ReduceActionSet = Vec<ReduceAction>;
pub(crate) fn ts_reduce_action_set_add(set: &mut ReduceActionSet, new_action: ReduceAction) {
    if !set
        .iter()
        .any(|a| a.symbol == new_action.symbol && a.count == new_action.count)
    {
        set.push(new_action);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_action_wins() {
        let mut set = vec![];
        ts_reduce_action_set_add(
            &mut set,
            ReduceAction {
                symbol: 1,
                count: 2,
                dynamic_precedence: 3,
                production_id: 4,
            },
        );
        ts_reduce_action_set_add(
            &mut set,
            ReduceAction {
                symbol: 1,
                count: 2,
                dynamic_precedence: 9,
                production_id: 8,
            },
        );
        assert_eq!(set.len(), 1);
        assert_eq!(set[0].dynamic_precedence, 3);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BattleAction {
    Fight,
    Defend,
    Act,
    Spare,
    Item,
    /// Internally there's apparently a sprite for this as text, not used in
    /// battle
    Tech,
}

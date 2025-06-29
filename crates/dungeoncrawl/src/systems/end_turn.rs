use legion::system;

use crate::state::TurnState;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    *turn_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonasterTurn,
        TurnState::MonasterTurn => TurnState::AwaitingInput,
    }
}

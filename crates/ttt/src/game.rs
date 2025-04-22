use crate::Board;
use crate::BoardIndex;
use crate::Sym;
use std::fmt::Display;

/// Represents the game state, holds information about current player and the state of the board
pub struct Game {
    board: Board,
    turn: Sym,
}

/// An Error emitted when attempting to play in a position that's already filled
#[derive(Debug, Clone)]
pub struct GamePositionAlreadyFull(BoardIndex);

impl Display for GamePositionAlreadyFull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid board index {}, already full", self.0)
    }
}

impl Game {
    /// Place a symbol at the given index, will fail if index was played at previously
    pub fn play_turn(&mut self, idx: BoardIndex) -> Result<(), GamePositionAlreadyFull> {
        // NOTE: this return type is so stupid
        if self.board[idx].is_some() {
            return Err(GamePositionAlreadyFull(idx));
        }

        self.board[idx] = Some(self.turn);
        self.turn.swap();
        Ok(())
    }
}

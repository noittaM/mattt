use crate::Board;
use crate::BoardIndex;
use crate::Sym;
use std::fmt::Display;

/// Represents the game state, holds information about current player and the state of the board
#[derive(Default)]
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
    /// Place a symbol at the given index
    /// # Errors
    /// if index was played at previously
    pub fn play_turn(&mut self, idx: BoardIndex) -> Result<(), GamePositionAlreadyFull> {
        // NOTE: this return type is so stupid
        if self.board[idx].is_some() {
            return Err(GamePositionAlreadyFull(idx));
        }

        self.board[idx] = Some(self.turn);
        self.turn.swap();
        Ok(())
    }

    /// Get winner or None if match is not over
    #[must_use]
    pub fn game_over(&self) -> Option<Sym> {
        // FIXME: WHAT THE FUCK
        let board: &[bool] = &self.board.get().map(|x| match x {
            Some(s) => match s {
                Sym::Circle => true,
                Sym::Cross => false,
            },
            None => false,
        });

        if (board[0] && board[1] && board[2])
            || (board[3] && board[4] && board[5])
            || (board[6] && board[7] && board[8])
            || (board[0] && board[3] && board[6])
            || (board[1] && board[4] && board[7])
            || (board[2] && board[5] && board[8])
            || (board[0] && board[4] && board[8])
            || (board[2] && board[4] && board[6])
        {
            return Some(Sym::Circle);
        }

        let board: &[bool] = &self.board.get().map(|x| match x {
            Some(s) => match s {
                Sym::Circle => false,
                Sym::Cross => true,
            },
            None => false,
        });

        if (board[0] && board[1] && board[2])
            || (board[3] && board[4] && board[5])
            || (board[6] && board[7] && board[8])
            || (board[0] && board[3] && board[6])
            || (board[1] && board[4] && board[7])
            || (board[2] && board[5] && board[8])
            || (board[0] && board[4] && board[8])
            || (board[2] && board[4] && board[6])
        {
            return Some(Sym::Cross);
        }
        None
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.board.get().len() {
            #[allow(clippy::cast_possible_truncation)]
            match self.board[BoardIndex::try_from(i as u8).unwrap()] {
                Some(s) => write!(f, " {s} ")?,
                None => write!(f, "   ")?,
            }
            if i == 8 {
                return Ok(());
            }
            if i % 3 == 2 {
                write!(f, "\n-----------\n")?;
            } else {
                write!(f, "|")?;
            }
        }
        Ok(())
    }
}

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
    /// Places symbol at the given index
    ///
    /// # Errors
    ///
    /// Retruns [`GamePositionAlreadyFull`]
    pub fn play_turn(&mut self, idx: BoardIndex) -> Result<(), GamePositionAlreadyFull> {
        // FIXME: Consider returning something useful like if game is over or not after the move.
        // NOTE: this return type is so stupid
        if self.board[idx].is_some() {
            return Err(GamePositionAlreadyFull(idx));
        }

        self.board[idx] = Some(self.turn);
        self.turn.swap();
        Ok(())
    }

    /// Returns the player who won the game if it has finished. Otherwise, returns `None`.
    pub fn has_winner(&self) -> Option<Sym> {
        // FIXME: A macro would go hard here, mayhaps.
        const LINES: [[BoardIndex; 3]; 8] = unsafe {
            [
                // Lines across:
                [
                    BoardIndex::new_unchecked(0),
                    BoardIndex::new_unchecked(1),
                    BoardIndex::new_unchecked(2),
                ],
                [
                    BoardIndex::new_unchecked(3),
                    BoardIndex::new_unchecked(4),
                    BoardIndex::new_unchecked(5),
                ],
                [
                    BoardIndex::new_unchecked(6),
                    BoardIndex::new_unchecked(7),
                    BoardIndex::new_unchecked(8),
                ],
                // Lines down:
                [
                    BoardIndex::new_unchecked(0),
                    BoardIndex::new_unchecked(3),
                    BoardIndex::new_unchecked(6),
                ],
                [
                    BoardIndex::new_unchecked(1),
                    BoardIndex::new_unchecked(4),
                    BoardIndex::new_unchecked(7),
                ],
                [
                    BoardIndex::new_unchecked(2),
                    BoardIndex::new_unchecked(5),
                    BoardIndex::new_unchecked(8),
                ],
                //The two diagonals:
                [
                    BoardIndex::new_unchecked(0),
                    BoardIndex::new_unchecked(4),
                    BoardIndex::new_unchecked(8),
                ],
                [
                    BoardIndex::new_unchecked(2),
                    BoardIndex::new_unchecked(4),
                    BoardIndex::new_unchecked(6),
                ],
            ]
        };

        fn check_line(board: &Board, line: [BoardIndex; 3]) -> bool {
            // FIXME: Might be a prettier way? Search through `Option` in std.
            board[line[0]].is_some()
                && board[line[0]] == board[line[1]]
                && board[line[1]] == board[line[2]]
        }

        for line in LINES {
            if check_line(&self.board, line) {
                // Should always be `Some` since that is checked in `check_line`
                return self.board[line[0]];
            }
        }

        None
    }

    // Get winner or None if match is not over
    // #[must_use]
    // pub fn game_over(&self) -> Option<Sym> {
    //     // FIXME: WHAT THE FUCK
    //     let board: &[bool] = &self.board.get().map(|x| match x {
    //         Some(s) => match s {
    //             Sym::Circle => true,
    //             Sym::Cross => false,
    //         },
    //         None => false,
    //     });
    //
    //     let a = self.board[0u8.try_into().unwrap()];
    //
    //     if (board[0] && board[1] && board[2])
    //         || (board[3] && board[4] && board[5])
    //         || (board[6] && board[7] && board[8])
    //         || (board[0] && board[3] && board[6])
    //         || (board[1] && board[4] && board[7])
    //         || (board[2] && board[5] && board[8])
    //         || (board[0] && board[4] && board[8])
    //         || (board[2] && board[4] && board[6])
    //     {
    //         return Some(Sym::Circle);
    //     }
    //
    //     let board: &[bool] = &self.board.get().map(|x| match x {
    //         Some(s) => match s {
    //             Sym::Circle => false,
    //             Sym::Cross => true,
    //         },
    //         None => false,
    //     });
    //
    //     if (board[0] && board[1] && board[2])
    //         || (board[3] && board[4] && board[5])
    //         || (board[6] && board[7] && board[8])
    //         || (board[0] && board[3] && board[6])
    //         || (board[1] && board[4] && board[7])
    //         || (board[2] && board[5] && board[8])
    //         || (board[0] && board[4] && board[8])
    //         || (board[2] && board[4] && board[6])
    //     {
    //         return Some(Sym::Cross);
    //     }
    //     None
    // }
}

// FIXME: Should this function really go here?
// Please move to a newtype wrapper or a separate method in another crate.
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

// TODO: Unit tests plssss
#[cfg(test)]
mod tests {
    use super::*;
}

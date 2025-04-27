use std::error::Error;
use std::fmt::Display;
use std::ops::Index;
use std::ops::IndexMut;

/// Represents one of two symbols used for playing tic-tac-toe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Sym {
    /// A "cross" or "X".
    #[default]
    Cross,
    /// A "circle" or "O".
    Circle,
}

impl Sym {
    /// Swaps held symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ttt::Sym;
    /// let mut sym = Sym::Cross;
    ///
    /// sym.swap();
    /// assert_eq!(sym, Sym::Circle);
    ///
    /// sym.swap();
    /// assert_eq!(sym, Sym::Cross);
    /// ```
    pub fn swap(&mut self) {
        match self {
            Sym::Cross => *self = Sym::Circle,
            Sym::Circle => *self = Sym::Cross,
        }
    }
}

impl Display for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sym::Cross => write!(f, "X"),
            Sym::Circle => write!(f, "O"),
        }
    }
}

/// Represents the 3x3 grid on which tic-tac-toe is played on.
///
/// Does not provide any additional logic e.g. preventing invalid states.
#[derive(Debug, Clone, Default)]
pub struct Board([Option<Sym>; 9]);

impl Board {
    /// Returns the underlying game grid.
    #[must_use]
    pub fn get(&self) -> &[Option<Sym>; 9] {
        &self.0
    }
}

impl Index<BoardIndex> for Board {
    type Output = Option<Sym>;

    fn index(&self, index: BoardIndex) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl IndexMut<BoardIndex> for Board {
    fn index_mut(&mut self, index: BoardIndex) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}

/// An index of a cell in a tic-tac-toe grid.
///
/// Guaranteed to be in the range of `0..=8`.
///
/// Indexes the grid left-to-right, top-to-bottom such that the top-left cell is indexed with 0 and
/// the bottom-right is indexed with 8.
///
/// # Examples
///
/// ```
/// # use ttt::BoardIndex;
/// let Ok(index) = BoardIndex::try_from(2) else {
///     panic!("Error converting to BoardIndex")
/// };
///
/// assert_eq!(u8::from(index), 2);
/// ```
///
/// The following code demonstrates an invalid conversion:
/// ```
/// # use ttt::BoardIndex;
/// let index_invalid = BoardIndex::try_from(18);
///
/// assert!(index_invalid.is_err());
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardIndex(u8);

impl BoardIndex {
    // NOTE: Added these functions to use them in const contexts.

    /// Constructs a new instance of [`BoardIndex`]
    ///
    /// # Errors
    ///
    /// Returns [`BoardIndexOutOfBoundsError`] if `value` is out of bounds.
    pub const fn new(value: u8) -> Result<Self, BoardIndexOutOfBoundsError> {
        if value > 8 {
            Err(BoardIndexOutOfBoundsError(value))
        } else {
            unsafe { Ok(Self::new_unchecked(value)) }
        }
    }

    /// Constructs a new instance of [`BoardIndex`] without performing the bounds check.
    ///
    /// # Safety
    ///
    /// If `val` is an invalid index, indexing [`Board`] will panic.
    #[must_use]
    pub const unsafe fn new_unchecked(value: u8) -> Self {
        Self(value)
    }
}

impl Display for BoardIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for BoardIndex {
    type Error = BoardIndexOutOfBoundsError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 8 {
            Err(Self::Error::from(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<BoardIndex> for u8 {
    fn from(value: BoardIndex) -> Self {
        value.0
    }
}

/// Error emitted when attempting to perform an invalid [`BoardIndex`] conversion.
#[derive(Debug, Clone)]
pub struct BoardIndexOutOfBoundsError(u8);

impl Display for BoardIndexOutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid index: {}. A board index must be in the range of 0..9.",
            self.0
        )
    }
}

impl From<u8> for BoardIndexOutOfBoundsError {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Error for BoardIndexOutOfBoundsError {}

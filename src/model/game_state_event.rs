use super::{ClueSet, ClueWithAddress, Deduction, Difficulty, TimerState};
use crate::model::{GameBoard, GameStats};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClueSelection {
    pub clue: ClueWithAddress,
    pub is_focused: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PuzzleCompletionState {
    Incomplete,
    Correct(GameStats),
    Incorrect,
}

#[derive(Debug)]
pub enum GameStateEvent {
    HistoryChanged {
        history_index: usize,
        history_length: usize,
    },
    GridUpdate(GameBoard),
    ClueStatusUpdate {
        horizontal_hidden_tiles: Vec<usize>,
        vertical_hidden_tiles: Vec<usize>,
    },
    CellHintHighlight(Deduction),
    HintUsageChanged(u32),
    TimerStateChanged(TimerState),
    PuzzleSubmissionReadyChanged(bool),
    PuzzleCompleted(PuzzleCompletionState),
    ClueHintHighlight(Option<ClueWithAddress>),
    ClueSetUpdate(Rc<ClueSet>, Difficulty),
    ClueSelected(Option<ClueSelection>),
}

impl GameStateEvent {}

use super::{Clue, ClueSet, ClueWithGrouping, Difficulty, TimerState};
use crate::model::{GameBoard, GameStats};
use std::rc::Rc;

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
    CellHintHighlight {
        cell: (usize, usize),
        variant: char,
    },
    HintUsageChanged(u32),
    TimerStateChanged(TimerState),
    PuzzleSubmissionReadyChanged(bool),
    PuzzleSuccessfullyCompleted(PuzzleCompletionState),
    ClueHintHighlight {
        clue_with_grouping: ClueWithGrouping,
    },
    ClueSetUpdate(Rc<ClueSet>, Difficulty),
    ClueFocused(Option<Clue>),
}

impl GameStateEvent {}

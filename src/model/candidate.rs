use serde::{Deserialize, Serialize};

use super::Tile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum CandidateState {
    Available,
    Eliminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Candidate {
    pub tile: Tile,
    pub state: CandidateState,
}

impl Candidate {
    pub fn new(tile: Tile) -> Self {
        Self {
            tile,
            state: CandidateState::Available,
        }
    }

    pub fn from_bool(row: usize, variant: char, b: bool) -> Self {
        let tile = Tile::new(row, variant);
        if b {
            Self {
                tile,
                state: CandidateState::Available,
            }
        } else {
            Self {
                tile,
                state: CandidateState::Eliminated,
            }
        }
    }
}

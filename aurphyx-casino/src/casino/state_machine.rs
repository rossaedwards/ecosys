use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Waiting,
    Playing,
    Paused,
    Finished,
    Error,
}

pub struct GameStateMachine {
    state: GameState,
}

impl GameStateMachine {
    pub fn new() -> Self {
        Self {
            state: GameState::Waiting,
        }
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn start(&mut self) -> Result<()> {
        match self.state {
            GameState::Waiting | GameState::Paused => {
                self.state = GameState::Playing;
                Ok(())
            }
            _ => Err(Error::Game("Cannot start game from current state".to_string())),
        }
    }

    pub fn pause(&mut self) -> Result<()> {
        match self.state {
            GameState::Playing => {
                self.state = GameState::Paused;
                Ok(())
            }
            _ => Err(Error::Game("Cannot pause game from current state".to_string())),
        }
    }

    pub fn finish(&mut self) -> Result<()> {
        match self.state {
            GameState::Playing => {
                self.state = GameState::Finished;
                Ok(())
            }
            _ => Err(Error::Game("Cannot finish game from current state".to_string())),
        }
    }

    pub fn reset(&mut self) {
        self.state = GameState::Waiting;
    }
}


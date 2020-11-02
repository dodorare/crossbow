use core::fmt;

use bevy::prelude::*;

/// Component to tag an entity that used in state's
pub struct ForStates {
    pub states: Vec<GameState>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameState {
    MainMenu,
    ThreeDScene,
    TwoDScene,
    Audio,
    Explorer,
    About,
}

#[derive(Debug)]
pub struct RunState {
    pub game_state: GameStateFsm<GameState>,
}

impl RunState {
    pub fn new(start: GameState) -> RunState {
        RunState {
            game_state: GameStateFsm::new(start),
        }
    }
}

pub fn run_state_fsm_system(mut run_state: ResMut<RunState>) {
    run_state.game_state.update();
}

pub fn state_despawn_system(
    mut commands: Commands,
    run_state: ResMut<RunState>,
    mut query: Query<(Entity, &ForStates)>,
) {
    for (entity, for_states) in &mut query.iter() {
        if run_state.game_state.exiting_one_of(&for_states.states)
            && !run_state
                .game_state
                .transiting_to_one_of(&for_states.states)
        {
            commands.despawn(entity);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum FsmTransition {
    Exit,
    Enter,
    None,
}

/// Game state finite-state machine (FSM).
/// Serves for managing state transitions.
#[derive(Debug)]
pub struct GameStateFsm<T: PartialEq + Eq + Copy + fmt::Debug> {
    transition: FsmTransition,
    current: Option<T>,
    next: Option<T>,
    prev: Option<T>,
}

impl<T: PartialEq + Eq + Copy + fmt::Debug> GameStateFsm<T> {
    pub fn new(start: T) -> GameStateFsm<T> {
        GameStateFsm {
            transition: FsmTransition::Enter,
            current: None,
            next: Some(start),
            prev: None,
        }
    }

    pub fn is(&self, state: T) -> bool {
        self.current == Some(state)
    }

    pub fn exiting_one_of(&self, states: &[T]) -> bool {
        self.transition == FsmTransition::Exit && states.contains(&self.current.unwrap())
    }

    pub fn transiting_to_one_of(&self, states: &[T]) -> bool {
        self.next
            .map(|next| states.contains(&next))
            .unwrap_or(false)
    }

    pub fn entering(&self, state: T) -> bool {
        self.transition == FsmTransition::Enter && self.next == Some(state)
    }

    pub fn entering_not_from(&self, state: T, from: T) -> bool {
        self.transition == FsmTransition::Enter
            && self.next == Some(state)
            && self.prev != Some(from)
    }

    pub fn transit_to(&mut self, state: T) {
        self.next = Some(state);
    }

    /// Called every frame to update the phases of transitions.
    /// A transition requires 3 frames: exit current, enter next, current=next
    pub fn update(&mut self) {
        if self.next.is_some() {
            match self.transition {
                FsmTransition::Exit => {
                    // We have exited current state, we can enter the new one
                    self.prev = self.current;
                    self.current = None;
                    self.transition = FsmTransition::Enter;
                }
                FsmTransition::Enter => {
                    // We have entered the new one it is now current
                    self.current = self.next;
                    self.transition = FsmTransition::None;
                    self.next = None;
                }
                FsmTransition::None => {
                    // This is new request to go to the next state, exit the current one first
                    self.transition = FsmTransition::Exit;
                }
            }
        }
    }
}

use bonsaidb::core::{
    api::{Api, Infallible},
    schema::{ApiName, Qualified},
};
use serde::{Deserialize, Serialize};

/// Set the current choice.
#[derive(Serialize, Deserialize, Debug)]
pub struct SetChoice(pub Choice);

impl Api for SetChoice {
    type Response = ChoiceSet;

    type Error = Infallible;

    fn name() -> ApiName {
        ApiName::private("set-choice")
    }
}

/// Set the current tell.
#[derive(Serialize, Deserialize, Debug)]
pub struct SetTell(pub Choice);

impl Api for SetTell {
    type Response = ChoiceSet;

    type Error = Infallible;

    fn name() -> ApiName {
        ApiName::private("set-tell")
    }
}

/// Our choice has been set.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChoiceSet(pub Choice);

/// A player's choice in the game.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Choice {
    GoOut,
    StayIn,
}

/// The server has set up our player record.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Welcome {
    pub player_id: u64,
    pub happiness: f32,
}

impl Api for Welcome {
    type Response = Self;

    type Error = Infallible;

    fn name() -> ApiName {
        ApiName::private("welcome")
    }
}

/// A round is pending.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoundPending {
    pub seconds_remaining: u32,
    pub number_of_players: u32,
    pub current_rank: u32,
    pub number_of_tells: u32,
    pub tells_going_out: u32,
}

impl Api for RoundPending {
    type Response = Self;

    type Error = Infallible;

    fn name() -> ApiName {
        ApiName::private("round-pending")
    }
}

/// A round has finished.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoundComplete {
    /// The player's happiness has gone up this round.
    pub won: bool,
    pub happiness: f32,
    pub current_rank: u32,
    pub number_of_players: u32,
    pub number_of_liars: u32,
    pub number_of_tells: u32,
}

impl Api for RoundComplete {
    type Response = Self;

    type Error = Infallible;

    fn name() -> ApiName {
        ApiName::private("round-complete")
    }
}

/// Converts a `percent` to its nearest whole number.
pub fn whole_percent(percent: f32) -> u32 {
    (percent * 100.).round() as u32
}

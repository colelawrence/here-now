use bonsaidb::core::{
    document::{CollectionDocument, Emit},
    schema::{Collection, CollectionViewSchema, Schema, View, ViewMapResult},
};
use minority_game_shared::Choice;
use serde::{Deserialize, Serialize};

#[derive(Schema, Debug)]
#[schema(authority = "minority-game", name = "game", collections = [Player])]
pub enum GameSchema {}

#[derive(Collection, Default, Debug, Serialize, Deserialize, Clone)]
#[collection(authority = "minority-game", name = "player", views = [PlayerByScore])]
pub struct Player {
    pub choice: Option<Choice>,
    #[serde(default)]
    pub tell: Option<Choice>,
    pub stats: PlayerStats,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerStats {
    pub happiness: f32,
    pub times_went_out: u32,
    pub times_stayed_in: u32,
    #[serde(default)]
    pub times_lied: u32,
    #[serde(default)]
    pub times_told_truth: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            happiness: 0.5,
            times_went_out: 0,
            times_stayed_in: 0,
            times_lied: 0,
            times_told_truth: 0,
        }
    }
}

impl PlayerStats {
    pub fn score(&self) -> u32 {
        let total_games = self.times_stayed_in + self.times_went_out;
        (self.happiness * total_games as f32) as u32
    }
}

#[derive(View, Debug, Clone)]
#[view(collection = Player, name = "by-score", key = u32, value = PlayerStats)]
pub struct PlayerByScore;

impl CollectionViewSchema for PlayerByScore {
    type View = Self;

    fn version(&self) -> u64 {
        2
    }

    fn map(
        &self,
        player: CollectionDocument<<Self::View as View>::Collection>,
    ) -> ViewMapResult<Self::View> {
        player
            .header
            .emit_key_and_value(player.contents.stats.score(), player.contents.stats)
    }
}

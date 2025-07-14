use crate::*;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum GameScenes {
	MainMenuScene,
	FactionSelectScene,
	IdleGameScene,
	LevelUpScene,
	TierUpScene,
	LastFactionWinScene
}
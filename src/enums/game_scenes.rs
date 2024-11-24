use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum GameScenes {
	MainMenuScene,
	FactionSelectScene,
	IdleGameScene,
	LevelUpScene,
	TierUpScene
}
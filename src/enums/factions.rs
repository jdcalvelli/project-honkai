#[turbo::serialize]
#[derive(PartialEq, Copy)]
pub enum Factions {
    Green,
    Orange,
    Purple,
    NoFaction,
}

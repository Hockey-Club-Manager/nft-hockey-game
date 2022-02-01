mod game;
mod cell;
mod timer;
mod user;
mod player;

#[cfg(test)]
mod tests {
    use crate::player::{FieldPlayer, FieldPlayerStats, Player};
    use crate::player::PlayerPosition::Center;
    use crate::player::PlayerRole::Shooter;

    #[test]
    fn it_works() {
        let stats: FieldPlayerStats = FieldPlayerStats::new(125, 124, 144, 0, 200);


        let player: FieldPlayer = FieldPlayer::new(false, Center, Shooter, 0, stats);

        assert_eq!(player.stats.skating, 125, "not eq");
    }
}

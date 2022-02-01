mod game;
mod cell;
mod timer;
mod user;
mod player;

#[cfg(test)]
mod tests {
    use crate::player::{Player, PlayerStats};
    use crate::player::PlayerPosition::Center;
    use crate::player::PlayerRole::Shooter;

    #[test]
    fn it_works() {
        let stats: PlayerStats = PlayerStats::new(125, 124, 144, 0, 200);


        let player: Player = Player::new(false, Center, Shooter, 0, stats);
        println!("{}", player.stats.skating);
    }
}

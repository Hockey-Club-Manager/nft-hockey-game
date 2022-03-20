use crate::*;
use crate::team::Goalies;

#[ext_contract(ext_manage_team)]
pub trait ExtManageTeam{
    fn get_teams(&mut self,
                 account_id_1: AccountId,
                 account_id_2: AccountId) -> (TeamMetadata, TeamMetadata);

    fn get_owner_team(&mut self, account_id: AccountId) -> TeamMetadata;

    fn insert_nft_field_players(&mut self, fives: Vec<(Fives, Vec<(PlayerPosition, TokenId)>)>);

    fn insert_nft_goalies(&mut self, goalies: Vec<(Goalies, TokenId)>);
}

#[ext_contract(ext_self)]
pub trait ExtThis {
    fn on_get_teams(&mut self,
                    opponent_id: AccountId,
                    account_id: AccountId,
                    config: GameConfig,
                    #[callback] teams: (TeamMetadata, TeamMetadata)) -> GameId;
}
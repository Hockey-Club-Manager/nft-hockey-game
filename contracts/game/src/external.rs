use crate::*;

#[ext_contract(ext_manage_team)]
pub trait ExtManageTeam{
    fn get_teams(&mut self,
                 account_id_1: AccountId,
                 account_id_2: AccountId) -> (TeamMetadata, TeamMetadata);

    fn get_owner_team(&self, account_id: AccountId) -> TeamMetadata;
}

#[ext_contract(ext_self)]
pub trait ExtThis {
    fn on_get_teams(&mut self,
                    opponent_id: AccountId,
                    account_id: AccountId,
                    config: GameConfig,
                    #[callback] teams: (TeamMetadata, TeamMetadata)) -> GameId;

    fn on_get_team(&mut self,
                   account_id: AccountId,
                   deposit: Balance,
                   config: GameConfig,
                   #[callback] team: TeamMetadata);
}
use crate::*;

#[ext_contract(this_contract)]
pub trait Callbacks {
    fn on_get_teams(&mut self,
                    opponent_id: AccountId,
                    account_id: AccountId,
                    config: GameConfig,
                    #[callback_result] call_result: Result<(TeamMetadata, TeamMetadata), PromiseError>
    ) -> bool;

    fn on_get_team(&mut self,
                   account_id: AccountId,
                   deposit: Balance,
                   config: GameConfig,
                   #[callback_result] call_result: Result<TeamMetadata, PromiseError>
    ) -> bool;
}

#[ext_contract(ext_manage_team)]
pub trait ExtManageTeam{
    fn get_teams(&mut self,
                 account_id_1: AccountId,
                 account_id_2: AccountId
    ) -> (TeamMetadata, TeamMetadata);

    fn get_owner_team(&self, account_id: AccountId) -> TeamMetadata;
}
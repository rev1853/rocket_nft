use cosmwasm_schema::cw_serde;
use cosmwasm_std::Response;
use cw721_base::ContractError;

#[cw_serde]
pub struct CreateNftResponse {
    pub res: Response
}

#[cw_serde]
pub struct EmptyResponse {

}
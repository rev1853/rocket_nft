use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;
use cw721::Expiration;
use cw721_base::{MintMsg, QueryMsg as Cw721QueryMsg};
use cw721_base::ExecuteMsg as Cw721ExecuteMsg;
use crate::Extension;

#[cw_serde]
pub enum QueryMsg {
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    AllOperators {
        owner: String,
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    NumTokens {},
    ContractInfo {},
    NftInfo {
        token_id: String,
    },
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    Minter {},
}

impl From<QueryMsg> for Cw721QueryMsg {
    fn from(value: QueryMsg) -> Self {
        match value {
            QueryMsg::AllNftInfo {token_id, include_expired} => Cw721QueryMsg::AllNftInfo {token_id, include_expired},
            QueryMsg::AllTokens {limit, start_after} => Cw721QueryMsg::AllTokens {limit, start_after},
            QueryMsg::OwnerOf {token_id, include_expired} => Cw721QueryMsg::OwnerOf {token_id, include_expired},
            QueryMsg::Minter {} => Cw721QueryMsg::Minter {},
            QueryMsg::AllOperators {
                limit,
                start_after,
                include_expired,
                owner
            } => Cw721QueryMsg::AllOperators {limit, start_after, include_expired, owner},
            QueryMsg::Approval {
                spender,
                token_id,
                include_expired
            } => Cw721QueryMsg::Approval {spender,token_id,include_expired},
            QueryMsg::Approvals {include_expired, token_id} => Cw721QueryMsg::Approvals {token_id, include_expired},
            QueryMsg::ContractInfo {} => Cw721QueryMsg::ContractInfo {},
            QueryMsg::NftInfo {token_id} => Cw721QueryMsg::NftInfo {token_id},
            QueryMsg::NumTokens {} => Cw721QueryMsg::NumTokens {},
            QueryMsg::Tokens {
                start_after,
                owner,
                limit
            } => Cw721QueryMsg::Tokens {start_after,owner,limit},
            _ => unreachable!("Cannot convert {:?} to Msg", value),
        }
    }
}
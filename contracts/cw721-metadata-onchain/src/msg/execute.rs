use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;
use cw721::Expiration;
use cw721_base::{MintMsg, QueryMsg as Cw721QueryMsg};
use cw721_base::ExecuteMsg as Cw721ExecuteMsg;
use crate::Extension;

#[cw_serde]
pub enum ExecuteMsg {
    TransferNft { recipient: String, token_id: String },
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    Revoke { spender: String, token_id: String },
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    RevokeAll { operator: String },
    Mint(MintMsg<Extension>),
    Burn { token_id: String },

    /// custom msg
    CreateNft {
        token_id: String,
        token_uri: Option<String>,
        owner: String,
        extension: Extension
    },
}

impl From<ExecuteMsg> for Cw721ExecuteMsg<Extension> {
    fn from(value: ExecuteMsg) -> Self {
        match value {
            ExecuteMsg::Approve {
                spender,
                token_id,
                expires
            } => Cw721ExecuteMsg::Approve {spender,token_id,expires},
            ExecuteMsg::ApproveAll {expires,operator} => Cw721ExecuteMsg::ApproveAll {operator,expires},
            ExecuteMsg::TransferNft {recipient,token_id} => Cw721ExecuteMsg::TransferNft {recipient,token_id},
            ExecuteMsg::SendNft {
                msg,
                contract,
                token_id
            } => Cw721ExecuteMsg::SendNft {msg,contract,token_id},
            ExecuteMsg::Burn {token_id} => Cw721ExecuteMsg::Burn {token_id},
            ExecuteMsg::Mint(msg) => Cw721ExecuteMsg::Mint(msg),
            ExecuteMsg::Revoke {spender,token_id} => Cw721ExecuteMsg::Revoke {spender,token_id},
            ExecuteMsg::RevokeAll {operator} => Cw721ExecuteMsg::RevokeAll {operator},
            _ => unreachable!("Cannot convert {:?} to Execute", value)
        }
    }
}
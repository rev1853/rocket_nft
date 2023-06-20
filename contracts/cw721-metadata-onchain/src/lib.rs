pub mod contract;
pub mod responses;
pub mod msg;

use cosmwasm_std::Empty;
pub use cw721_base::{ContractError, InstantiateMsg, MinterResponse};

use crate::contract::Metadata;

pub type Extension = Option<Metadata>;
pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;

// #[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::{entry_point};
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw721::Cw721Execute;
    use cw721_base::MintMsg;
    use crate::msg::execute::ExecuteMsg;
    use crate::msg::query::QueryMsg;

    // This is a simple type to let us handle empty extensions

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        Cw721MetadataContract::default().instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let contract = Cw721MetadataContract::default();
        match msg {
            ExecuteMsg::CreateNft {
                token_uri,
                token_id,
                owner,
                extension
            } => {
                let mint_msg: MintMsg<Extension> = MintMsg {
                    token_id,
                    token_uri,
                    owner,
                    extension
                };
                let result = contract.mint(deps, env, info, mint_msg);
                match result {
                    Ok(res) => Ok(res),
                    Err(err) => Err(err),
                }
            }
            _ => contract.execute(deps, env, info, msg.into())
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            _ => Cw721MetadataContract::default().query(deps, env, msg.into())
        }
    }
}

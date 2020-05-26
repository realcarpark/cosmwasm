use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, CosmosMsg, HumanAddr, QueryRequest};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    ReflectMsg {
        msgs: Vec<CosmosMsg<CustomMsgWrapper>>,
    },
    ChangeOwner {
        owner: HumanAddr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Owner {},
    // this will call out to CustomQuery::Capitalize
    ReflectCustom { text: String },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerResponse {
    pub owner: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// CustomMsgWrapper is an override of CosmosMsg::Custom to show this works and can be extended in the contract
pub struct CustomMsgWrapper {
    pub route: String,
    pub msg_data: CustomMsg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CustomMsg {
    Debug(String),
    Raw(Binary),
}

impl Into<CosmosMsg<CustomMsgWrapper>> for CustomMsgWrapper {
    fn into(self) -> CosmosMsg<CustomMsgWrapper> {
        CosmosMsg::Custom(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// CustomQueryWrapper is an override of QueryRequest::Custom to show this works and can be extended in the contract
pub struct CustomQueryWrapper {
    pub route: String,
    pub query_data: CustomQuery,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CustomQuery {
    Ping {},
    Capital { text: String },
}

// TODO: do we want to standardize this somehow for all?
impl Into<QueryRequest<CustomQueryWrapper>> for CustomQueryWrapper {
    fn into(self) -> QueryRequest<CustomQueryWrapper> {
        QueryRequest::Custom(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
// All return values of CustomQueryWrapper are CustomResponse
pub struct CustomResponse {
    pub msg: String,
}

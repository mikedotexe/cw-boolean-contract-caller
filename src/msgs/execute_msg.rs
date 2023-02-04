use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    MakeCroncatTask {
        croncat_factory_address: String,
        boolean_address: String
    }
}

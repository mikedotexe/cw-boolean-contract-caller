use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("ERR_REPLY_ERROR|{code:?}|{msg:?}")]
    ReplyError { code: u64, msg: String },

    #[error("{code:?}|{msg:?}")]
    CustomError { code: String, msg: String },

    #[error("ERR_UNKNOWN_REPLY|Unknown reply ID: {id:?}")]
    UnknownReplyID { id: u64 },
}

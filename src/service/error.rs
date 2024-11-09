use crate::constant::base_error::BaseError;
pub const ERR_INVALID_TOKEN: BaseError = BaseError("invalid token");
pub const ERR_USER_NOT_FOUND: BaseError = BaseError("user not found");

pub const ERR_USER_EXISTED: BaseError = BaseError("user existed");

pub const ERR_INVALID_PARAM: BaseError = BaseError("invalid param");

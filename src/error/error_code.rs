macro_rules! status_error_codes {
    // 匹配多个元组，生成多个常量
    (
        $(
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        $(
            // 为每个元组生成一个常量定义
            pub const $konst: (i16, &str) = ($num, $phrase);
        )+
    }
}

status_error_codes! {
    (-1, SERVER_ERROR, "server error");
    (-2, INVALID_REQUEST, "invalid request");
    (-3, INVALID_PARAM, "invalid param");
    (-4, INVALID_TOKEN, "invalid token");
    (-5, INVALID_SIGN, "invalid sign");
    (-6, INVALID_METHOD, "invalid method");
    (-7, INVALID_URL, "invalid url");
    (-8, INVALID_CONTENT_TYPE, "invalid content type");
    (-9, USER_NOT_LOGIN, "user not logged in");
    (-10, USER_EXIST, "user already exists");
    (-11, USER_NOT_EXIST, "user does not exist");
    (-12, PASSWORD_ERROR, "password error");
    (-13, INVALID_SYMBOL, "invalid symbol");
    (-14, USER_NOT_FOUND, "user not found");
    (-15, INSUFFICIENT_BALANCE, "insufficient balance");
    (-16, INVITE_CODE_NOT_FOUND, "invite code not found");
}

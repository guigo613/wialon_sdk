use super::{
    templates::*,
    response_json::*
};

impl WialonResponse for Login {
    fn svc() -> &'static str {
        TOKEN_LOGIN
    }
}

impl WialonResponse for TokenUpdate {
    fn svc() -> &'static str {
        TOKEN_UPDATE
    }
}

impl WialonResponse for Vec<TokenList> {
    fn svc() -> &'static str {
        TOKEN_LIST
    }
}

impl WialonResponse for SearchItem {
    fn svc() -> &'static str {
        CORE_SEARCH_ITEM
    }
}

impl WialonResponse for SearchItems {
    fn svc() -> &'static str {
        CORE_SEARCH_ITEMS
    }
}

impl WialonResponse for Vec<UpdateDataFlags> {
    fn svc() -> &'static str {
        CORE_UPDATE_DATA_FLAGS
    }
}

impl WialonResponse for AvlEvts {
    fn svc() -> &'static str {
        ""
    }
}

impl WialonResponse for Logout {
    fn svc() -> &'static str {
        CORE_LOGOUT
    }
}
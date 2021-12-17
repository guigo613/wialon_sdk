mod flags;
mod params;
mod wialon;
mod prelude;
mod templates;
mod response_json;

mod external {
    pub use serde::{Deserialize, Serialize, de::DeserializeOwned};
    pub use serde_json::Value;
}

use std::{
    error::Error as StdError
};
use templates::*;
use reqwest::blocking as reqwest;
use regex::Regex;

pub use self::{
    wialon::Wialon,
    flags::{
        Units,
        UnitsFlag
    },
    response_json::{
        UpdateDataFlags,
        WialonResponse,
        TokenUpdate,
        SearchItems,
        SearchItem,
        TokenList,
        AvlEvts,
        Logout,
        Login,
        Error,
        Evts,
    },
    params::{
        SearchItems as ParamsSearchItems,
        SearchItem as ParamsSearchItem,
        UpdateDataFlags as ParamsDataFlags,
        TraitDataFlags
    },
    external::Value
};

type GenResult<T> = Result<T, Box<dyn StdError>>;
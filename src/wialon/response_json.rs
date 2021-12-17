use super::{
    external::*,
    flags::Units
};
use std::{
    fmt::{
        self,
        Debug,
        Display
    },
    net::Ipv4Addr,
    error::Error as StdError,
    collections::HashMap
};

pub trait WialonResponse: DeserializeOwned + Debug {
    fn svc() -> &'static str;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    pub host: Ipv4Addr,
    pub eid: String,
    pub gis_sid: String,
    pub au: String,
    pub tm: u32,
    pub wsdk_version: String,
    pub base_url: String,
    pub hw_gw_ip: Ipv4Addr,
    pub hw_gw_dns: String,
    pub gis_search: String,
    pub gis_render: String,
    pub gis_geocode: String,
    pub gis_routing: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenUpdate {
    pub h: String,
	pub app: String,
	pub at: u32,
	pub ct: u32,
	pub dur: u32,
	pub fl: isize,
	pub items: Vec<i32>,
	pub p: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenList {
    pub h: String,
	pub app: String,
	pub at: u32,
	pub ct: u32,
	pub dur: u32,
	pub fl: isize,
	pub ll: u32,
	pub ttl: u32,
	pub items: Vec<i32>,
	pub p: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchItem {
    pub item: HashMap<String, String>,
	pub flags: isize
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct SearchSpec {
    pub itemsType: String,
    pub propName: String,
    pub propValueMask: String,
    pub sortType: String,
    pub propType: String
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct SearchItems {
    pub searchSpec: SearchSpec,
	pub dataFlags: Units,
	pub totalItemsCount: u32,
	pub indexFrom: u32,
	pub indexTo: u32,
	pub items: Vec<HashMap<String, Value>>		
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateDataFlags {
    pub i: i32,                                         // ID
    pub d: Option<HashMap<String, Value>>,              // other fields
    pub f: i32                                          // applied data flags
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Evts {
    pub i: i32, 	                                    // item ID
    pub t: String, 	                                    // event type: m - message, u - update, d - delete
    pub d: Option<HashMap<String, Value>>	            // description of event, depends on event type
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AvlEvts {
	pub tm: u32,                                        // server time
	pub events: Vec<Evts>                               // events
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Logout {
    pub error: i32
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Error {
    pub error: i32
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.error);

        Ok(())
    }
}

impl StdError for Error {}
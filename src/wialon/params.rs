use super::{
    external::*,
    flags::Units
};

pub const VOID: &str = "{}";

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct Login {
    pub token: String,
    pub operateAs: Option<String>,
    pub fl: Option<u32>
}

impl Login {
    pub fn new(token: String, fl: Option<u32>, operate_as: Option<String>) -> Self {
        Self {
            token,
            fl,
            operateAs: operate_as
        }
    }
}

#[derive(Serialize)]
pub struct SearchItem {
    pub id: i32,		
    pub flags: Units,
}

impl SearchItem {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            flags: Default::default(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct Spec {
    pub itemsType: String,	
    pub propName: String,	
    pub propValueMask: String,	
    pub sortType: String,
    pub propType: Option<String>,
    pub or_logic: bool
}

impl Default for Spec {
    fn default() -> Self {
        Self {
            itemsType: "avl_unit".to_string(),	
            propName: "sys_name".to_string(),	
            propValueMask: "*".to_string(),	
            sortType: "sys_name".to_string(),
            propType: None,
            or_logic: false
        }
    }
}

#[derive(Serialize)]
pub struct SearchItems {
    pub spec: Spec,
    pub force: u32,			
    pub flags: Units,			
    pub from: u32,			
    pub to: u32
}

impl Default for SearchItems {
    fn default() -> Self {
        Self {
            spec: Default::default(),
            force: 1,
            flags: Default::default(),
            from: 0,
            to: 0
        }
    }
}

#[derive(Serialize, Default, Debug)]
pub struct UpdateDataFlags {
    pub spec: Vec<DataFlags>,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum UpdateData {
    Long(u32),
    Array(Vec<u32>)
}

#[derive(Serialize, Debug)]
pub struct DataFlags {
    pub r#type: String,
    pub data: UpdateData,
    pub flags: Units,	
    pub mode: u32,
    pub max_items: Option<u32>
}

impl DataFlags {
    fn new_id(id: u32, flags: Units, mode: u32) -> Self {
        Self {
            r#type: "id".to_owned(),
            data: UpdateData::Long(id),
            flags: flags,
            mode,
            max_items: None
        }
    }

    fn new_col(col: Vec<u32>, flags: Units, mode: u32) -> Self {
        Self {
            r#type: "col".to_owned(),
            data: UpdateData::Array(col),
            flags: flags,
            mode,
            max_items: None
        }
    }
}

pub trait TraitDataFlags<T> {
    fn add(&mut self, id: T, flags: Units);
    fn remove(&mut self, id: T, flags: Units);
}

impl TraitDataFlags<u32> for UpdateDataFlags {
    fn add(&mut self, id: u32, flags: Units) {
        self.spec.push(DataFlags::new_id(id, flags, 0))
    }
    
    fn remove(&mut self, id: u32, flags: Units) {
        self.spec.push(DataFlags::new_id(id, flags, 2))
    }
}

impl TraitDataFlags<Vec<u32>> for UpdateDataFlags {
    fn add(&mut self, col: Vec<u32>, flags: Units) {
        self.spec.push(DataFlags::new_col(col, flags, 0))
    }

    fn remove(&mut self, col: Vec<u32>, flags: Units) {
        self.spec.push(DataFlags::new_col(col, flags, 2))
    }
}
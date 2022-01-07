use super::external::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Units(u32);

impl Units {
    pub fn change(&mut self, value: UnitsFlag, checked: bool) {
        if checked { self.0 |= value as u32 } else { self.0 &= u32::MAX - value as u32 }
    }

    pub fn check(&self, value: UnitsFlag) -> bool {
        self.0 | value as u32 == self.0
    }

    pub fn flag(&self) -> u32 {
        self.0
    }
}

impl Default for Units {
    fn default() -> Self {
        Self(1)
    }
}

pub enum UnitsFlag {
    BaseFlag                = 0x000001,
    CustomProperties        = 0x000002,
    BillingProperties       = 0x000004,
    CustomFields            = 0x000008,
    Image                   = 0x000010,
    Messages                = 0x000020,
    Guid                    = 0x000040,
    AdministrativeFields    = 0x000080,
    AdvancedProperties      = 0x000100,
    CurrentMoment           = 0x000200,
    LastMessagePosition     = 0x000400,
    Sensors                 = 0x001000,
    Counters                = 0x002000,
    Maintenance             = 0x008000,
    UnitConfiguration       = 0x020000,
    AllCommands             = 0x080000,
    MessageParameters       = 0x100000,
    UnitConnectionStatus    = 0x200000,
    Position                = 0x400000,
    ProfileFields           = 0x800000,
    All                     = 0xFAB7FF,
}
use crate::resources::*;
use strum_macros::EnumIter;
use serde::{Deserialize, Serialize};

#[derive(EnumIter, Clone, Serialize, Deserialize)]
pub enum StructureClass {
    Farm,
    Camp,
    Mine,
    Settlement,
    Fortress,
    MillitaryQuarter,
    AdminQuarter,
    MerchantQuarter,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Structure {
    pub count: u32,
    pub name: String,
    pub class: StructureClass,
    pub cost: Payment,
    pub revenue: Payment,
}

impl Structure {
    pub fn default(class: StructureClass) -> Self {
        match class {
            StructureClass::Farm => Structure {
                count: 1,
                name: "Ферма".to_string(),
                class: StructureClass::Farm,
                cost: Payment {
                    building_materials: 30,
                    money: 100,
                    economic_fragment: 2,
                    ..Payment::new()
                },
                revenue: Payment {
                    population: -5,
                    food: 20,
                    ..Payment::new()
                }
            },
            StructureClass::Camp => Structure {
                count: 1,
                name: "Лагерь".to_string(),
                class: StructureClass::Camp,
                cost: Payment {
                    building_materials: 15,
                    money: 200,
                    economic_fragment: 2,
                    ..Payment::new()
                },
                revenue: Payment {
                    population: -5,
                    building_materials: 10,
                    ..Payment::new()
                }
            },
            StructureClass::Mine => Structure {
                count: 1,
                name: "Шахта".to_string(),
                class: StructureClass::Mine,
                cost: Payment {
                    building_materials: 10,
                    money: 300,
                    economic_fragment: 3,
                    ..Payment::new()
                },
                revenue: Payment {
                    population: -3,
                    metal: 10,
                    ..Payment::new()
                }
            },
            StructureClass::Settlement => Structure {
                count: 1,
                name: "Поселение".to_string(),
                class: StructureClass::Settlement,
                cost: Payment {
                    building_materials: 45,
                    money: 650,
                    metal: 5,
                    military_fragment: 1,
                    political_fragment: 2,
                    economic_fragment: 5,
                    ..Payment::new()
                },
                revenue: Payment {
                    population: -10,
                    money: 50,
                    military_fragment: 1,
                    political_fragment: 1,
                    economic_fragment: 1,
                    ..Payment::new()
                }
            },
            StructureClass::Fortress => Structure {
                count: 1,
                name: "Крепость".to_string(),
                class: StructureClass::Fortress,
                cost: Payment {
                    building_materials: 30,
                    money: 400,
                    metal: 15,
                    military_fragment: 2,
                    economic_fragment: 2,
                    ..Payment::new()
                },
                revenue: Payment {
                    population: -3,
                    money: -50,
                    military_fragment: 1,
                    ..Payment::new()
                }
            },
            StructureClass::MillitaryQuarter => Structure {
                count: 1,
                name: "Военный квартал".to_string(),
                class: StructureClass::MillitaryQuarter,
                cost: Payment {
                    building_materials: 15,
                    money: 400,
                    metal: 5,
                    economic_fragment: 2,
                    ..Payment::new()
                },
                revenue: Payment {
                    population: -3,
                    military_fragment: 1,
                    ..Payment::new()
                }
            },
            StructureClass::AdminQuarter => Structure {
                count: 1,
                name: "Админ. квартал".to_string(),
                class: StructureClass::MillitaryQuarter,
                cost: Payment {
                    building_materials: 15,
                    money: 400,
                    metal: 5,
                    economic_fragment: 2,
                    ..Payment::new()
                },
                revenue: Payment {
                    population: -3,
                    political_fragment: 1,
                    ..Payment::new()
                }
            },
            StructureClass::MerchantQuarter => Structure {
                count: 1,
                name: "Торговый квартал".to_string(),
                class: StructureClass::MillitaryQuarter,
                cost: Payment {
                    building_materials: 15,
                    money: 400,
                    metal: 5,
                    economic_fragment: 2,
                    ..Payment::new()
                },
                revenue: Payment {
                    population: -3,
                    economic_fragment: 1,
                    ..Payment::new()
                }
            }
        }
    }
}

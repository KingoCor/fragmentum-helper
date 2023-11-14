use crate::resources::*;
use strum_macros::EnumIter;

#[derive(EnumIter,Clone)]
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

#[derive(Clone)]
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
                    building_materials: Resource::BuildingMaterials(30),
                    money: Resource::Money(100),
                    economic_fragment: Fragment::Economic(2),
                    ..Payment::new()
                },
                revenue: Payment {
                    population: Resource::Population(-5),
                    food: Resource::Food(20),
                    ..Payment::new()
                }
            },
            StructureClass::Camp => Structure {
                count: 1,
                name: "Лагерь".to_string(),
                class: StructureClass::Camp,
                cost: Payment {
                    building_materials: Resource::BuildingMaterials(15),
                    money: Resource::Money(200),
                    economic_fragment: Fragment::Economic(2),
                    ..Payment::new()
                },
                revenue: Payment {
                    population: Resource::Population(-5),
                    food: Resource::BuildingMaterials(10),
                    ..Payment::new()
                }
            },
            StructureClass::Mine => Structure {
                count: 1,
                name: "Шахта".to_string(),
                class: StructureClass::Mine,
                cost: Payment {
                    building_materials: Resource::BuildingMaterials(10),
                    money: Resource::Money(300),
                    economic_fragment: Fragment::Economic(3),
                    ..Payment::new()
                },
                revenue: Payment {
                    population: Resource::Population(-3),
                    food: Resource::Metal(10),
                    ..Payment::new()
                }
            },
            StructureClass::Settlement => Structure {
                count: 1,
                name: "Поселение".to_string(),
                class: StructureClass::Settlement,
                cost: Payment {
                    building_materials: Resource::BuildingMaterials(45),
                    money: Resource::Money(650),
                    metal: Resource::Metal(5),
                    military_fragment: Fragment::Military(1),
                    political_fragment: Fragment::Political(2),
                    economic_fragment: Fragment::Economic(5),
                    ..Payment::new()
                },
                revenue: Payment {
                    population: Resource::Population(-10),
                    money: Resource::Money(50),
                    military_fragment: Fragment::Military(1),
                    political_fragment: Fragment::Political(1),
                    economic_fragment: Fragment::Economic(1),
                    ..Payment::new()
                }
            },
            StructureClass::Fortress => Structure {
                count: 1,
                name: "Крепость".to_string(),
                class: StructureClass::Fortress,
                cost: Payment {
                    building_materials: Resource::BuildingMaterials(30),
                    money: Resource::Money(400),
                    metal: Resource::Metal(15),
                    military_fragment: Fragment::Military(2),
                    economic_fragment: Fragment::Economic(2),
                    ..Payment::new()
                },
                revenue: Payment {
                    population: Resource::Population(-3),
                    money: Resource::Money(-50),
                    military_fragment: Fragment::Military(1),
                    ..Payment::new()
                }
            },
            StructureClass::MillitaryQuarter => Structure {
                count: 1,
                name: "Военный квартал".to_string(),
                class: StructureClass::MillitaryQuarter,
                cost: Payment {
                    building_materials: Resource::BuildingMaterials(15),
                    money: Resource::Money(400),
                    metal: Resource::Metal(5),
                    economic_fragment: Fragment::Economic(2),
                    ..Payment::new()
                },
                revenue: Payment {
                    population: Resource::Population(-3),
                    military_fragment: Fragment::Military(1),
                    ..Payment::new()
                }
            },
            StructureClass::AdminQuarter => Structure {
                count: 1,
                name: "Админ. квартал".to_string(),
                class: StructureClass::MillitaryQuarter,
                cost: Payment {
                    building_materials: Resource::BuildingMaterials(15),
                    money: Resource::Money(400),
                    metal: Resource::Metal(5),
                    economic_fragment: Fragment::Economic(2),
                    ..Payment::new()
                },
                revenue: Payment {
                    population: Resource::Population(-3),
                    political_fragment: Fragment::Political(1),
                    ..Payment::new()
                }
            },
            StructureClass::MerchantQuarter => Structure {
                count: 1,
                name: "Торговый квартал".to_string(),
                class: StructureClass::MillitaryQuarter,
                cost: Payment {
                    building_materials: Resource::BuildingMaterials(15),
                    money: Resource::Money(400),
                    metal: Resource::Metal(5),
                    economic_fragment: Fragment::Economic(2),
                    ..Payment::new()
                },
                revenue: Payment {
                    population: Resource::Population(-3),
                    economic_fragment: Fragment::Economic(1),
                    ..Payment::new()
                }
            }
        }
    }
}

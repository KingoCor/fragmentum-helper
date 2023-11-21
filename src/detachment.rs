use std::{ops::{Add,Mul}, collections::HashMap, cmp::{PartialEq,Eq}, iter::Sum};
use crate::resources::*;

use crate::aspects::AspectClass;

pub enum DetachmentClass {
    LightInfantry,
    HeavyInfantry,
    LightCavalry,
    HeavyCavalry,
    LightSupport,
    HeavySupport
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Stats {
    pub health: i32,
    pub collision: i32,
    pub scouting: i32,
    pub maneuver: i32,
    pub mobility: i32,
    pub moral: i32,
    pub protection_d: i32,
    pub protection_m: i32,
    pub damage_d: i32,
    pub damage_m: i32
}

impl Add for Stats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Stats {
            health: self.health+rhs.health,
            collision: self.collision+rhs.collision,
            scouting: self.scouting+rhs.scouting,
            maneuver: self.maneuver+rhs.maneuver,
            mobility: self.mobility+rhs.mobility,
            moral: self.moral+rhs.moral,
            protection_d: self.protection_d+rhs.protection_d,
            protection_m: self.protection_m+rhs.protection_m,
            damage_d: self.damage_d+rhs.damage_d,
            damage_m: self.damage_m+rhs.damage_m
        }
    }
}

impl Sum for Stats {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut stats = Stats::new();
        for i in iter {
            stats=stats+i;
        }
        stats
    }
}

impl Mul<i32> for Stats {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Stats {
            health: self.health*rhs,
            collision: self.collision*rhs,
            scouting: self.scouting*rhs,
            maneuver: self.maneuver*rhs,
            mobility: self.mobility*rhs,
            moral: self.moral*rhs,
            protection_d: self.protection_d*rhs,
            protection_m: self.protection_m*rhs,
            damage_d: self.damage_d*rhs,
            damage_m: self.damage_m*rhs
        }
    }
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            health: 0,
            collision: 0,
            scouting: 0,
            maneuver: 0,
            mobility: 0,
            moral: 0,
            protection_d: 0,
            protection_m: 0,
            damage_d: 0,
            damage_m: 0
        }
    }

    pub fn one() -> Self {
        Stats {
            health: 1,
            collision: 1,
            scouting: 1,
            maneuver: 1,
            mobility: 1,
            moral: 1,
            protection_d: 1,
            protection_m: 1,
            damage_d: 1,
            damage_m: 1 
        }
    }

    pub fn default() -> Self {
        Stats {
            health: 5,
            collision: 5,
            scouting: 5,
            maneuver: 5,
            mobility: 7,
            moral: 5,
            protection_d: 0,
            protection_m: 0,
            damage_d: 0,
            damage_m: 3
        }
    }

    pub fn get_by_detachment_class(class: DetachmentClass) -> Self {
        match class {
            DetachmentClass::LightInfantry => Stats::default(),
            DetachmentClass::HeavyInfantry => Stats::default() + Stats {
                collision: 1,
                scouting: -2,
                maneuver: -1,
                mobility: -1,
                protection_d: 2,
                protection_m: 1,
                ..Stats::new()
            },
            DetachmentClass::LightCavalry => Stats::default() + Stats {
                maneuver: 2,
                mobility: 2,
                protection_d: -2,
                ..Stats::new()
            },
            DetachmentClass::HeavyCavalry => Stats::default() + Stats {
                collision: 3,
                scouting: -1,
                maneuver: 1,
                mobility: -3,
                protection_m: 1,
                ..Stats::new()
            },
            DetachmentClass::LightSupport => Stats::default() + Stats {
                collision: -2,
                damage_m: -2,
                damage_d: 2,
                ..Stats::new()
            },
            DetachmentClass::HeavySupport => Stats::default() + Stats {
                scouting: -1,
                maneuver: -2,
                protection_d: 2,
                collision: 1,
                mobility: -2,
                damage_m: -3,
                damage_d: 4,
                ..Stats::new()
            },

        }
    }

    pub fn get_aspect_bonus(class: AspectClass) -> Self {
        match class {
            AspectClass::Strong => Stats {
                collision: 1,
                damage_m: 2,
                ..Stats::new()
            },
            AspectClass::Hardy => Stats {
                maneuver: 1,
                health: 1,
                mobility: 2,
                ..Stats::new()
            },
            AspectClass::Large => Stats {
                scouting: -2,
                maneuver: -1,
                protection_d: -1,
                collision: 3,
                moral: 1,
                health: 2,
                mobility: 1,
                damage_m: 2,
                ..Stats::new()
            },
            AspectClass::Resilient => Stats {
                protection_d: 2,
                protection_m: 1,
                moral: 2,
                health: 4,
                ..Stats::new()
            },
            AspectClass::Sturdy => Stats {
                protection_d: 2,
                protection_m: 1,
                moral: 1,
                ..Stats::new()
            },
            AspectClass::FastAndAgile => Stats {
                scouting: 2,
                maneuver: 2,
                protection_m: 1,
                mobility: 1,
                damage_m: 1,
                damage_d: 1,
                ..Stats::new()
            },
            AspectClass::DevelopedFeelings => Stats {
                scouting: 3,
                damage_d: 1,
                ..Stats::new()
            },
            AspectClass::Smart => Stats {
                scouting: 1,
                maneuver: 1,
                moral: -1,
                damage_d: 1,
                ..Stats::new()
            },
            AspectClass::Fearless => Stats {
                collision: 1,
                moral: 3,
                ..Stats::new()
            },
            AspectClass::CultOfPower => Stats {
                maneuver: 1,
                collision: 1,
                moral: 1,
                health: 1,
                ..Stats::new()
            },

            _ => Stats::new()
        }
    }

    pub fn get_equipment_cost(&self) -> Payment {
        Payment {
            money: (
                    self.health
                    +self.collision
                    +self.scouting
                    +self.maneuver
                    +self.mobility
                    +self.moral
                    +self.protection_d
                ).pow(2)*5
                +self.damage_d.pow(2)*5
                +self.damage_m.pow(2)*5,
            metal: self.protection_m.pow(2)
                +self.damage_d/2
                +self.damage_m/2,
            food: self.mobility.pow(2)*2,
            ..Payment::new()
        }
    } 

    pub fn to_string(&self) -> String {
        format!(
            "Здоровье: {}\nСшибка: {}\nРазведка: {}\nМанёвр: {}\nМобильность: {}\nМораль: {}\nЗащида д/б: {}\nЗащида б/б: {}\nУрон д/б: {}\nУрон б/б: {}",
            self.health,
            self.collision,
            self.scouting,
            self.maneuver,
            self.mobility,
            self.moral,
            self.protection_d,
            self.protection_m,
            self.damage_d,
            self.damage_m
        )
    }

    pub fn to_string_short(&self) -> String {
        let mut out = "".to_string();
        if self.health!=0 { out += &format!("Здоровье: {}, ", self.health).to_string(); }
        if self.collision!=0 { out += &format!("Сшибка: {}, ", self.collision).to_string(); }
        if self.scouting!=0 { out += &format!("Разведка: {}, ", self.scouting).to_string(); }
        if self.maneuver!=0 { out += &format!("Манёвр: {}, ", self.maneuver).to_string(); }
        if self.mobility!=0 { out += &format!("Мобильность: {}, ", self.mobility).to_string(); }
        if self.moral!=0 { out += &format!("Мораль: {}, ", self.moral).to_string(); }
        if self.protection_d!=0 { out += &format!("Защита д/б: {}, ", self.protection_d).to_string(); }
        if self.protection_m!=0 { out += &format!("Защита б/б: {}, ", self.protection_m).to_string(); }
        if self.damage_d!=0 { out += &format!("Урон д/б: {}, ", self.damage_d).to_string(); }
        if self.damage_m!=0 { out += &format!("Урон б/б: {}", self.damage_m).to_string(); }
        if out.len()!=0 {
            if &out[out.len()-1..]==" " {
                out = out[..out.len()-2].to_string();
            }
        }
        out
    }
}

pub struct Detachment {
    pub class: DetachmentClass,
    pub base_stats: Stats,
    pub aspect_bonuses: Vec<AspectClass>,
    pub bonuses: HashMap<String,Stats>,
    pub equipment: HashMap<String,Stats>
}

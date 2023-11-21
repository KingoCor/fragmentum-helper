use serde::{Deserialize, Serialize};
use std::{ops::Add, iter::Sum};

#[derive(Clone, Serialize, Deserialize)]
pub enum  Fragment {
    Military(i32),
    Political(i32),
    Economic(i32)
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Resource {
    Population(i32),
    BuildingMaterials(i32),
    Money(i32),
    Food(i32),
    Metal(i32),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Payment {
    pub population: i32,
    pub building_materials: i32,
    pub money: i32,
    pub food: i32,
    pub metal: i32,
    pub military_fragment: i32,
    pub political_fragment: i32,
    pub economic_fragment: i32
}

impl Add for Payment {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Payment {
            population: self.population + rhs.population,
            building_materials: self.building_materials + rhs.building_materials,
            money: self.money + rhs.money,
            food: self.food + rhs.food,
            metal: self.metal + rhs.metal,
            military_fragment: self.military_fragment+rhs.military_fragment,
            political_fragment: self.political_fragment+rhs.political_fragment,
            economic_fragment: self.economic_fragment+rhs.economic_fragment
        }
    }
}

impl Sum for Payment {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut payment = Payment::new();
        for i in iter {
            payment=payment+i;
        }
        payment
    }
}

impl Payment {
    pub const fn default() -> Self {
        Payment { 
            population: 36, 
            building_materials: 45, 
            money: 1900, 
            food: 85, 
            metal: 10, 
            military_fragment: 5, 
            political_fragment: 5, 
            economic_fragment: 5 
        }
    }

    pub const fn new() -> Self {
        Payment { 
            population: 0, 
            building_materials: 0, 
            money: 0, 
            food: 0, 
            metal: 0, 
            military_fragment: 0, 
            political_fragment: 0, 
            economic_fragment: 0 
        }
    }
}

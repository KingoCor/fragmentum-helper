#[derive(Clone)]
pub enum  Fragment {
    Military(i32),
    Political(i32),
    Economic(i32)
}

#[derive(Clone)]
pub enum Resource {
    Population(i32),
    BuildingMaterials(i32),
    Money(i32),
    Food(i32),
    Metal(i32),
}

#[derive(Clone)]
pub struct Payment {
    pub population: Resource,
    pub building_materials: Resource,
    pub money: Resource,
    pub food: Resource,
    pub metal: Resource,
    pub military_fragment: Fragment,
    pub political_fragment: Fragment,
    pub economic_fragment: Fragment
}

impl Payment {
    pub const fn new() -> Self {
        Payment { 
            population: Resource::Population(0), 
            building_materials: Resource::BuildingMaterials(0), 
            money: Resource::Money(0), 
            food: Resource::Food(0), 
            metal: Resource::Metal(0), 
            military_fragment: Fragment::Military(0), 
            political_fragment: Fragment::Political(0), 
            economic_fragment: Fragment::Economic(0) 
        }
    }
}

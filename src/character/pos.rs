use bevy_ecs::component::Component;


#[derive(Clone, Copy, PartialEq, Component, Debug, Default)]
pub struct CharacterPos {
    pub x : f64,
    pub y : f64,
    pub z : f64
}

impl CharacterPos {
    pub const ZERO : Self = Self { x : 0.0, y : 0.0, z : 0.0 };
}


#[derive(Clone, Copy, PartialEq, Component, Debug, Default)]
pub struct CharacterRot {
    pub pitch    : f64,
    pub yaw      : f64,
    pub head_yaw : f64
}

impl CharacterRot {
    pub const ZERO : Self = Self { pitch : 0.0, yaw : 0.0, head_yaw : 0.0 };
}


#[derive(Clone, Copy, PartialEq, Component, Debug, Default)]
pub struct CharacterVel {
    pub x : f64,
    pub y : f64,
    pub z : f64
}

impl CharacterVel {
    pub const ZERO : Self = Self { x : 0.0, y : 0.0, z : 0.0 };
}

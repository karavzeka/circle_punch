#[derive(Serialize, Deserialize, Debug)]
pub struct MapScheme {
    #[serde(default)]
    pub spike_templates: Vec<SpikeTemplate>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpikeTemplate {
    pub tile_point_1: Option<Position>,
    pub tile_point_2: Option<Position>,
    pub certain_point_1: Option<Position>,
    pub certain_point_2: Option<Position>,
    pub normal: Position,
}

// TODO collapse it with Position from command.rs
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
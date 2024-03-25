use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Wheel{
    pub  front_left:f32,
    pub  front_right:f32,
    pub  rear_left:f32,
    pub  rear_right:f32,
}

impl Wheel {
    pub fn serialize(value:&Wheel)->String
    {
        serde_json::to_string(value).unwrap()
    }
    pub fn deserialize(str_value:String)->Wheel
    {
        let result:Wheel = serde_json::from_str(&str_value).unwrap();
        result
    }
}

#[derive(Serialize, Deserialize)]
pub struct Vector{
    pub  x:f32,
    pub  y:f32,
    pub  z:f32,
}

impl Vector {
    pub fn serialize(value:&Vector)->String
    {
        serde_json::to_string(value).unwrap()
    }
    pub fn deserialize(str_value:String)->Vector
    {
        let result:Vector = serde_json::from_str(&str_value).unwrap();
        result
    }
}
#[derive(Serialize, Deserialize)]
pub struct CmdVel{
    pub  x:f32,
    pub  y:f32,
    pub  rotation_power:f32,
}

impl CmdVel {
    pub fn serialize(value:&CmdVel)->String
    {
        serde_json::to_string(value).unwrap()
    }
    pub fn deserialize(str_value:String)->CmdVel
    {
        let result:CmdVel = serde_json::from_str(&str_value).unwrap();
        result
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameCon{
    pub  left_x:f32,
    pub  left_y:f32,
    pub  right_x:f32,
    pub  right_y:f32,
    pub  circle:f32,
    pub  cross:f32,
    pub  cube:f32,
    pub  triangle:f32,
    pub  up_key:f32,
    pub  down_key:f32,
    pub  right_key:f32,
    pub  left_key:f32,
    pub  r1:f32,
    pub  r2:f32,
    pub  l1:f32,
    pub  l2:f32,
}

impl GameCon {
    pub fn serialize(value:&GameCon)->String
    {
        serde_json::to_string(value).unwrap()
    }
    pub fn deserialize(str_value:String)->GameCon
    {
        let result:GameCon = serde_json::from_str(&str_value).unwrap();
        result
    }
}
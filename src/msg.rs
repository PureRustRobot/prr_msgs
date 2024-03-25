use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Motor{
    pub  power:f32,
}

impl Motor {
    pub fn serialize(value:&Motor)->String
    {
        serde_json::to_string(value).unwrap()
    }
    pub fn deserialize(str_value:String)->Motor
    {
        let result:Motor = serde_json::from_str(&str_value).unwrap();
        result
    }
}

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
    pub  circle:bool,
    pub  cross:bool,
    pub  cube:bool,
    pub  triangle:bool,
    pub  up_key:bool,
    pub  down_key:bool,
    pub  right_key:bool,
    pub  left_key:bool,
    pub  r1:bool,
    pub  r2:bool,
    pub  l1:bool,
    pub  l2:bool,
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

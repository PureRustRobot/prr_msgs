use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Wheel{
    pub  front_left:f32,
    pub  front_right:f32,
    pub  rear_left:f32,
    pub  rear_right:f32,
}
pub fn serialize_wheel(value:&Wheel)->String
{
    serde_json::to_string(value).unwrap()
}
pub fn deserialize_wheel(str_value:String)->Wheel
{
     let result:Wheel = serde_json::from_str(&str_value).unwrap();
    result
}
#[derive(Serialize, Deserialize)]
pub struct Vector{
    pub  x:f32,
    pub  y:f32,
    pub  z:f32,
}
pub fn serialize_vector(value:&Vector)->String
{
    serde_json::to_string(value).unwrap()
}
pub fn deserialize_vector(str_value:String)->Vector
{
     let result:Vector = serde_json::from_str(&str_value).unwrap();
    result
}
#[derive(Serialize, Deserialize)]
pub struct CmdVel{
    pub  x:f32,
    pub  y:f32,
    pub  rotation_power:f32,
}
pub fn serialize_cmdvel(value:&CmdVel)->String
{
    serde_json::to_string(value).unwrap()
}
pub fn deserialize_cmdvel(str_value:String)->CmdVel
{
     let result:CmdVel = serde_json::from_str(&str_value).unwrap();
    result
}
#[derive(Serialize, Deserialize)]
pub struct JoyStick{
    pub  left_x:f32,
    pub  left_y:f32,
    pub  right_x:f32,
    pub  right_y:f32,
}
pub fn serialize_joystick(value:&JoyStick)->String
{
    serde_json::to_string(value).unwrap()
}
pub fn deserialize_joystick(str_value:String)->JoyStick
{
     let result:JoyStick = serde_json::from_str(&str_value).unwrap();
    result
}
#[derive(Serialize, Deserialize)]
pub struct Angular{
    pub  x:f32,
    pub  y:f32,
    pub  z:f32,
}
pub fn serialize_angular(value:&Angular)->String
{
    serde_json::to_string(value).unwrap()
}
pub fn deserialize_angular(str_value:String)->Angular
{
     let result:Angular = serde_json::from_str(&str_value).unwrap();
    result
}
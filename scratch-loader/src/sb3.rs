use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

pub type Num = f64;

macro_rules! impl_deser_for_tuple_struct {
    ($n: ident ( $($i: tt => $t: ty), * )) => {
        impl<'de> Deserialize<'de> for $n {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>
            {
                let tuple: (
                    $(
                        $t
                    ), * ,
                ) = Deserialize::deserialize(deserializer)?;
                Ok($n(
                    $(
                        tuple.$i
                    ), *
                ))
            }
        }
    };
}

#[derive(Debug)]
pub struct Sb3File {
    pub resources: HashMap<String, String>,
    pub project: Project,
}

impl Sb3File {
    pub fn new(resources: HashMap<String, String>, project: Project) -> Self {
        Self { resources, project }
    }
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub targets: Vec<Target>,
    pub extensions: Vec<String>,
    pub meta: ProjectMeta,
}

#[derive(Debug, Deserialize)]
pub struct ProjectMeta {
    pub semver: String,
    pub vm: String,
    pub agent: String,
}

#[derive(Debug, Deserialize)]
pub struct Target {
    #[serde(rename = "isStage")]
    pub is_stage: bool,
    pub name: String,
    pub variables: HashMap<String, Variable>,
    pub lists: HashMap<String, List>,
    pub broadcasts: HashMap<String, Broadcast>,
    #[serde(flatten)]
    pub blocks: Blocks,
    pub costumes: Vec<Costume>,
    #[serde(rename = "currentCostume")]
    pub current_costume: usize,
    pub sounds: Vec<Sound>,
    pub volume: Num,
    #[serde(rename = "layerOrder")]
    pub layer_order: usize,
    #[serde(flatten)]
    pub position: Position,
    #[serde(default)]
    pub size: Num,
    #[serde(default)]
    pub direction: Num,
    #[serde(default)]
    pub draggable: bool,
    #[serde(rename = "rotationStyle", default)]
    pub rotation_style: RotationStyle,
}

#[derive(Debug, Deserialize)]
pub enum RotationStyle {
    #[serde(rename = "left-right")]
    LeftRight,
    #[serde(rename = "don't rotate")]
    NoRotation,
    #[serde(other)]
    AllAround,
}

impl Default for RotationStyle {
    fn default() -> Self {
        Self::AllAround
    }
}

#[derive(Debug, Deserialize)]
pub struct Position {
    #[serde(default)]
    pub x: Num,
    #[serde(default)]
    pub y: Num,
}

// todo! complete this placeholder
#[derive(Debug, Deserialize)]
pub struct Sound {}

#[derive(Debug, Deserialize)]
pub struct Costume {
    pub name: String,
    #[serde(flatten)]
    pub data_format: ImageFormat,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub md5ext: String,
    #[serde(rename = "rotationCenterX")]
    pub center_x: Num,
    #[serde(rename = "rotationCenterY")]
    pub center_y: Num,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "dataFormat")]
pub enum ImageFormat {
    #[serde(rename = "png")]
    ImagePNG,
    #[serde(rename = "svg")]
    ImageSVG,
}

#[derive(Debug)]
pub struct Variable(pub String, pub ScratchValue);

impl_deser_for_tuple_struct!(Variable(0 => String, 1 => ScratchValue));

#[derive(Debug)]
pub struct List(pub String, pub Vec<(ScratchValue, ScratchValue)>);

impl_deser_for_tuple_struct!(List(0 => String, 1 => Vec<(ScratchValue, ScratchValue)>));

pub type Broadcast = String;

#[derive(Debug, Deserialize)]
pub struct Blocks {
    pub blocks: HashMap<String, Block>,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub opcode: String,
    pub next: Option<String>,
    pub parent: Option<String>,
    pub inputs: HashMap<String, Input>,
    pub fields: HashMap<String, Field>,
    #[serde(rename = "topLevel")]
    pub top_level: bool,
    // ignore shadow and position
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Input {
    // 1 -> shadow & 2 -> no_shadow
    NoHidden(u8, InputType),
    // always 3
    Hidden(u8, InputType, InputType),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum InputType {
    Block(String),
    NumOrStr(u8, ScratchValue),
    BroadcastOrVarOrList(u8, String, String),
    TopVarOrList(u8, String, String, Num, Num),
}

#[derive(Debug)]
pub struct Field(pub String, pub Option<String>);

impl_deser_for_tuple_struct!(Field(0 => String, 1 => Option<String>));

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum ScratchValue {
    Str(String),
    Num(Num),
    Bool(bool),
}

impl ScratchValue {
    pub fn force_num(&self) -> Num {
        match self {
            Self::Num(num) => *num,
            Self::Bool(true) => 1.0,
            _ => 0.0,
        }
    }

    #[allow(illegal_floating_point_literal_pattern)]
    pub fn force_bool(&self) -> bool {
        match self {
            Self::Bool(bool) => *bool,
            Self::Str(s) if s.as_str() == "" => false,
            Self::Num(0.0) => false,
            _ => true,
        }
    }

    pub fn force_str(&self) -> String {
        match self {
            Self::Num(num) => num.to_string(),
            Self::Bool(bool) => bool.to_string(),
            Self::Str(str) => str.clone(),
        }
    }
}

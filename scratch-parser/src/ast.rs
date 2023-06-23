use std::collections::HashMap;

use scratch_loader::sb3::ScratchValue;

#[derive(Debug)]
pub struct ParsedScratchProject {
    pub resources: HashMap<ResourcePath, String>,
    pub sprites: Vec<Sprite>,
    pub bakground: Background,
}

static mut RESOURCE_PATH_ID: usize = 0;

#[derive(Debug, PartialEq, Eq)]
pub struct ResourcePath {
    id: usize,
}

impl ResourcePath {
    pub fn new() -> Self {
        unsafe {
            RESOURCE_PATH_ID += 1;
        }
        Self {
            id: unsafe { RESOURCE_PATH_ID },
        }
    }

    pub fn clone(&self) -> Self {
        Self { id: self.id }
    }

    pub fn js_name(&self) -> String {
        format!("${}", self.id)
    }
}

#[derive(Debug)]
pub struct Sprite {
    pub name: String,
    pub variables: HashMap<String, ScratchValue>,
    pub lists: HashMap<String, Vec<ScratchValue>>,
    pub blocks: Vec<BlockItem>,
}

pub type Variable = (ResourcePath, ScratchValue);
pub type List = (ResourcePath, Vec<ScratchValue>);

#[derive(Debug)]
pub struct Background {
    pub name: String,
    pub variables: HashMap<String, Variable>,
    pub lists: HashMap<String, List>,
    pub broadcasts: HashMap<String, ResourcePath>,
    pub blocks: Vec<BlockItem>,
    // actually, sratch identify a definion by its display name?!
    pub definions: HashMap<String, ResourcePath>,
}

#[derive(Debug)]
pub enum BlockItem {
    EvWhenGreenFlagCliked(BlockStack),
    EvWhenKeyPressed(KeyId /* I'm not sure for this */, BlockStack),
    EWhenRecieveBroadcast(ResourcePath, BlockStack),
    /* Def(ResourcePath, DefArgs, BlockStack), */
    // ...
}

#[derive(Debug)]
pub struct BlockStack {
    blocks: Vec<Block>,
}

#[derive(Debug)]
pub enum KeyId {
    A,
    B,
    C,
    D, // ...
}

#[derive(Debug)]
pub enum Block {
    LlScratchValue(ScratchValue),
    MotionMove(Box<Block>),
    // ...
}

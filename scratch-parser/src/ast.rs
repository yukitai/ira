use std::{collections::HashMap, rc::Rc};

use scratch_loader::sb3::ScratchValue;

#[derive(Debug)]
pub struct ParsedScratchProject {
    pub resources: HashMap<ResourcePath, String>,
    pub sprites: Vec<Sprite>,
    pub background: Background,
    pub extensions: Vec<String>,
}

static mut RESOURCE_PATH_ID: usize = 0;

#[derive(Clone, PartialEq, Eq)]
pub struct ResourcePath {
    id: usize,
    name: Rc<String>,
}

impl std::hash::Hash for ResourcePath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl std::fmt::Debug for ResourcePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resource(`{}`#{})", self.name, self.id)
    }
}

impl ResourcePath {
    pub fn new(name: Rc<String>) -> Self {
        unsafe {
            RESOURCE_PATH_ID += 1;
        }
        Self {
            id: unsafe { RESOURCE_PATH_ID },
            name,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &Rc<String> {
        &self.name
    }

    pub fn js_name(&self) -> String {
        format!("${}", self.id)
    }
}

#[derive(Debug)]
pub struct Sprite {
    pub name: String,
    pub variables: HashMap<String, Variable>,
    pub lists: HashMap<String, List>,
    pub blocks: Vec<BlockItem>,
    pub definions: HashMap<String, ResourcePath>,
}

pub type Variable = (ResourcePath, ScratchValue);
pub type List = (ResourcePath, Vec<(ScratchValue, ScratchValue)>);

#[derive(Debug)]
pub struct Background {
    // pub name: String,
    pub variables: HashMap<String, Variable>,
    pub lists: HashMap<String, List>,
    pub broadcasts: HashMap<String, ResourcePath>,
    pub blocks: Vec<BlockItem>,
    // actually, sratch identify a definion by its display name?!
    pub definions: HashMap<String, ResourcePath>,
}

#[derive(Debug)]
pub enum BlockItem {
    EvWhenGreenFlagClicked(BlockStack),
    EvWhenKeyPressed(KeyId /* I'm not sure for this */, BlockStack),
    EWhenRecieveBroadcast(ResourcePath, BlockStack),
    /* Def(ResourcePath, DefArgs, BlockStack), */
    // ...
}

#[derive(Debug)]
pub struct BlockStack {
    blocks: Vec<Block>,
}

impl BlockStack {
    pub fn new(blocks: Vec<Block>) -> Self {
        Self { blocks }
    }
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
    LlBroadcast(ResourcePath),
    LlVar(ResourcePath),
    LlList(ResourcePath),
    BlockStack(BlockStack),
    MotionMove(Box<Block>),
    ControlForever(Box<Block>),
    OperatorAdd(Box<Block>, Box<Block>),
    PenClear,
    // ...
}

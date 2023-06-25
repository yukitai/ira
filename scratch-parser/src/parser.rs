use std::{collections::HashMap, fmt::Display, rc::Rc};

use colored::Colorize;
use scratch_loader::sb3::{
    Block as Sb3Block, Blocks, Broadcast, Input, InputType, List as Sb3List, Sb3File, ScratchValue,
    Target, Variable as Sb3Variable,
};

use crate::ast::{
    Background, Block, BlockItem, BlockStack, List, ParsedScratchProject, ResourcePath, Sprite,
    Variable,
};

#[derive(Debug)]
pub enum ParseSb3Error {
    InvaildSb3InputFormat,
    CannotResolveVariableOrList(String),
    Unsupported(String),
}

impl Display for ParseSb3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseSb3Error::InvaildSb3InputFormat => write!(f, "invaild block input format"),
            ParseSb3Error::CannotResolveVariableOrList(name) => {
                write!(f, "cannot resolve variable or list `{}`", name)
            }
            ParseSb3Error::Unsupported(msg) => write!(f, "unsupported: {}", msg),
        }
    }
}

pub struct Sb3FormatParser {
    src: Sb3File,
    proj: ParsedScratchProject,
}

impl Sb3FormatParser {
    pub fn new(src: Sb3File) -> Self {
        Self {
            src,
            proj: ParsedScratchProject {
                resources: HashMap::new(),
                sprites: Vec::new(),
                background: Background {
                    variables: HashMap::new(),
                    lists: HashMap::new(),
                    broadcasts: HashMap::new(),
                    blocks: Vec::new(),
                    definions: HashMap::new(),
                },
                extensions: Vec::new(),
            },
        }
    }

    pub fn parse(mut self) -> Result<ParsedScratchProject, ParseSb3Error> {
        Self::move_resources(&mut self.proj, self.src.resources)?;
        for target in self.src.project.targets {
            if target.is_stage {
                Self::parse_stage(&mut self.proj, target)?;
            } else {
                Self::parse_sprite(&mut self.proj, target)?;
            }
        }
        Ok(self.proj)
    }

    fn move_resources(
        proj: &mut ParsedScratchProject,
        resources: HashMap<String, String>,
    ) -> Result<(), ParseSb3Error> {
        for (id, content) in resources {
            let path = ResourcePath::new(Rc::new(id));
            proj.resources.insert(path, content);
        }
        Ok(())
    }

    fn parse_stage(proj: &mut ParsedScratchProject, stage: Target) -> Result<(), ParseSb3Error> {
        proj.background.variables = Self::parse_variables(stage.variables)?;
        proj.background.lists = Self::parse_lists(stage.lists)?;
        let mut definions = HashMap::new();
        proj.background.blocks = Self::parse_blocks(
            &proj.background,
            &proj.background.variables,
            &proj.background.lists,
            stage.blocks,
            &mut definions,
        )?;
        proj.background.definions = definions;
        proj.background.broadcasts = Self::parse_broadcasts(stage.broadcasts)?;
        Ok(())
    }

    fn parse_sprite(proj: &mut ParsedScratchProject, sprite: Target) -> Result<(), ParseSb3Error> {
        let name = sprite.name;
        let variables = Self::parse_variables(sprite.variables)?;
        let lists = Self::parse_lists(sprite.lists)?;
        let mut definions = HashMap::new();
        let blocks = Self::parse_blocks(
            &proj.background,
            &variables,
            &lists,
            sprite.blocks,
            &mut definions,
        )?;
        let sprite = Sprite {
            name,
            variables,
            lists,
            blocks,
            definions,
        };
        proj.sprites.push(sprite);
        Ok(())
    }

    fn parse_variables(
        variables: HashMap<String, Sb3Variable>,
    ) -> Result<HashMap<String, Variable>, ParseSb3Error> {
        Ok(variables
            .into_iter()
            .map(|(k, v)| (k, (ResourcePath::new(Rc::new(v.0)), v.1)))
            .collect())
    }

    fn parse_lists(
        lists: HashMap<String, Sb3List>,
    ) -> Result<HashMap<String, List>, ParseSb3Error> {
        Ok(lists
            .into_iter()
            .map(|(k, v)| (k, (ResourcePath::new(Rc::new(v.0)), v.1)))
            .collect())
    }

    fn parse_broadcasts(
        broadcasts: HashMap<String, Broadcast>,
    ) -> Result<HashMap<String, ResourcePath>, ParseSb3Error> {
        Ok(broadcasts
            .into_iter()
            .map(|(k, v)| (k, ResourcePath::new(Rc::new(v))))
            .collect())
    }

    fn parse_blocks(
        background: &Background,
        variable: &HashMap<String, Variable>,
        list: &HashMap<String, List>,
        blocks: Blocks,
        definions: &mut HashMap<String, ResourcePath>,
    ) -> Result<Vec<BlockItem>, ParseSb3Error> {
        let mut items = Vec::new();
        for (_, b) in blocks.blocks.iter().filter(|(_, b)| b.top_level) {
            match b.opcode.as_str() {
                "event_whenflagclicked" => {
                    let bs = Self::parse_block_stack(background, variable, list, &blocks, b)?;
                    items.push(BlockItem::EvWhenGreenFlagClicked(bs));
                }
                _ => { /* should return an Err here */ }
            }
        }
        Ok(items)
    }

    fn parse_block_stack_from_here(
        background: &Background,
        variable: &HashMap<String, Variable>,
        list: &HashMap<String, List>,
        blocks: &Blocks,
        tl_block: &Sb3Block,
    ) -> Result<BlockStack, ParseSb3Error> {
        let mut bs = Vec::new();
        let mut block_p = tl_block;
        bs.push(Self::parse_block(
            blocks, background, variable, list, block_p,
        )?);
        while let Some(next) = &block_p.next {
            let block = &blocks.blocks[next.as_str()];
            block_p = block;
            bs.push(Self::parse_block(
                blocks, background, variable, list, block_p,
            )?);
        }
        Ok(BlockStack::new(bs))
    }

    fn parse_block_stack(
        background: &Background,
        variable: &HashMap<String, Variable>,
        list: &HashMap<String, List>,
        blocks: &Blocks,
        tl_block: &Sb3Block,
    ) -> Result<BlockStack, ParseSb3Error> {
        let mut bs = Vec::new();
        let mut block_p = tl_block;
        while let Some(next) = &block_p.next {
            let block = &blocks.blocks[next.as_str()];
            block_p = block;
            bs.push(Self::parse_block(
                blocks, background, variable, list, block_p,
            )?);
        }
        Ok(BlockStack::new(bs))
    }

    fn parse_block(
        blocks: &Blocks,
        background: &Background,
        variable: &HashMap<String, Variable>,
        list: &HashMap<String, List>,
        block: &Sb3Block,
    ) -> Result<Block, ParseSb3Error> {
        match block.opcode.as_str() {
            "motion_movesteps" => Ok(Block::MotionMove(Box::new(Self::parse_input(
                blocks,
                background,
                variable,
                list,
                &block.inputs["STEPS"],
            )?))),
            "pen_clear" => Ok(Block::PenClear),
            "control_forever" => Ok(Block::ControlForever(Box::new(Self::parse_input(
                blocks,
                background,
                variable,
                list,
                &block.inputs["SUBSTACK"],
            )?))),
            "operator_add" => Ok(Block::OperatorAdd(
                Box::new(Self::parse_input(
                    blocks,
                    background,
                    variable,
                    list,
                    &block.inputs["NUM1"],
                )?),
                Box::new(Self::parse_input(
                    blocks,
                    background,
                    variable,
                    list,
                    &block.inputs["NUM2"],
                )?),
            )),
            _ => {
                println!(
                    "{}: unsupported block opcode `{}` (rewrite -> {})",
                    "warning".bright_yellow(),
                    block.opcode,
                    "0".bold().italic(),
                );
                Ok(Block::LlScratchValue(ScratchValue::Num(0.0)))
            } // placeholder
        }
    }

    fn parse_input(
        blocks: &Blocks,
        background: &Background,
        variable: &HashMap<String, Variable>,
        list: &HashMap<String, List>,
        input: &Input,
    ) -> Result<Block, ParseSb3Error> {
        match input {
            Input::NoHidden(_, real) | Input::Hidden(_, real, _) => match real {
                InputType::NumOrStr(4..=10, val) => Ok(Block::LlScratchValue(val.clone())),
                InputType::BroadcastOrVarOrList(11, _, id) => Ok(Block::LlBroadcast(
                    background.broadcasts[id.as_str()].clone(),
                )),
                InputType::BroadcastOrVarOrList(12, name, id)
                | InputType::TopVarOrList(12, name, id, _, _) => Ok(Block::LlVar(
                    Self::lookup_variable(background, variable, name, id)?,
                )),
                InputType::BroadcastOrVarOrList(13, name, id)
                | InputType::TopVarOrList(13, name, id, _, _) => Ok(Block::LlList(
                    Self::lookup_list(background, list, name, id)?,
                )),
                InputType::Block(id) => Ok(Block::BlockStack(Self::parse_block_stack_from_here(
                    background,
                    variable,
                    list,
                    blocks,
                    &blocks.blocks[id.as_str()],
                )?)),
                _ => Err(ParseSb3Error::InvaildSb3InputFormat),
            },
        }
    }

    fn lookup_variable(
        background: &Background,
        variable: &HashMap<String, Variable>,
        name: &String,
        id: &str,
    ) -> Result<ResourcePath, ParseSb3Error> {
        if let Some(var) = variable.get(id) {
            return Ok(var.0.clone());
        }
        if let Some(var) = background.variables.get(id) {
            return Ok(var.0.clone());
        }
        Err(ParseSb3Error::CannotResolveVariableOrList(name.clone()))
    }

    fn lookup_list(
        background: &Background,
        list: &HashMap<String, List>,
        name: &String,
        id: &str,
    ) -> Result<ResourcePath, ParseSb3Error> {
        if let Some(lst) = list.get(id) {
            return Ok(lst.0.clone());
        }
        if let Some(lst) = background.lists.get(id) {
            return Ok(lst.0.clone());
        }
        Err(ParseSb3Error::CannotResolveVariableOrList(name.clone()))
    }
}

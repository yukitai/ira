use zip::ZipArchive;

use crate::sb3::Sb3File;

use std::{collections::HashMap, fmt::Display, fs, io::Read};

use colored::Colorize;

#[derive(Debug)]
pub enum Sb3LoaderError {
    MissProjectJson,
    UnableReadFile,
    UnableExtractFile,
    InvaildProjectJsonFormat,
}

impl Display for Sb3LoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissProjectJson => write!(f, "missing `project.json`"),
            Self::UnableExtractFile => write!(f, "unable to extract file"),
            Self::UnableReadFile => write!(f, "unable to read file, does the file exist?"),
            Self::InvaildProjectJsonFormat => write!(f, "invaild `project.json` format"),
        }
    }
}

trait HandleSb3LoaderError<T> {
    fn handle(self, err: Sb3LoaderError) -> Result<T, Sb3LoaderError>;
}

impl<T, E: std::fmt::Debug> HandleSb3LoaderError<T> for Result<T, E> {
    fn handle(self, err: Sb3LoaderError) -> Result<T, Sb3LoaderError> {
        match self {
            Ok(val) => Ok(val),
            Err(_e) => {
                // eprintln!("{:?}", _e);
                Err(err)
            }
        }
    }
}

pub fn load(src: &str) -> Result<Sb3File, Sb3LoaderError> {
    let file = fs::File::open(src).handle(Sb3LoaderError::UnableReadFile)?;
    let mut archieve = ZipArchive::new(file).handle(Sb3LoaderError::UnableExtractFile)?;
    let mut resources = HashMap::new();
    let mut project = None;
    for id in 0..archieve.len() {
        let mut file = archieve
            .by_index(id)
            .handle(Sb3LoaderError::UnableExtractFile)?;
        let fname = file.name();
        println!("  {} `{}`", "Extracting".bright_green(), fname);
        let mut data = String::with_capacity(file.size() as usize);
        if fname == "project.json" {
            file.read_to_string(&mut data).unwrap();
            let pjson =
                serde_json::from_str(&data).handle(Sb3LoaderError::InvaildProjectJsonFormat)?;
            project = Some(pjson);
        } else {
            // resource
            let fname = fname.to_string();
            file.read_to_string(&mut data).unwrap();
            resources.insert(fname, data);
        }
    }
    if let Some(project) = project {
        Ok(Sb3File::new(resources, project))
    } else {
        Err(Sb3LoaderError::MissProjectJson)
    }
}

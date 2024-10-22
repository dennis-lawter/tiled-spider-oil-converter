use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TmxError {
    #[error("File not found")]
    FileNotFound(#[from] io::Error),
    #[error("TMX file was malformed")]
    TmxMalformed(#[from] serde_xml_rs::Error),
}

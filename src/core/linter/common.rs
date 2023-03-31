use crate::core::config::FluffConfig;
use crate::core::errors::{SQLBaseError, SQLTemplaterError};
use crate::core::parser::segments::base::BaseSegment;
use crate::core::templaters::base::TemplatedFile;
use std::collections::HashMap;

/// Rule Tuple object for describing rules.
#[derive(Debug, PartialEq, Clone)]
pub struct RuleTuple {
    code: String,
    name: String,
    description: String,
    groups: Vec<String>,
    aliases: Vec<String>,
}

/// Parsed version of a 'noqa' comment.
#[derive(Debug, PartialEq, Clone)]
pub struct NoQaDirective {
    /// Source line number
    line_no: u32,
    /// Affected rule names
    rules: Option<Vec<String>>,
    /// "enable", "disable", or "None"
    action: Option<String>,
}

/// An object to store the result of a templated file/string.
///
/// This is notable as it's the intermediate state between what happens
/// in the main process and the child processes when running in parallel mode.
#[derive(Debug, PartialEq, Clone)]
pub struct RenderedFile {
    pub templated_file: TemplatedFile,
    pub templater_violations: Vec<SQLTemplaterError>,
    pub config: FluffConfig,
    time_dict: HashMap<String, f64>,
    pub(crate) f_name: String,
    encoding: String,
    source_str: String,
}

/// An object to store the result of parsing a string.
#[derive(Debug, PartialEq, Clone)]
pub struct ParsedString {
    tree: Option<BaseSegment>,
    violations: Vec<SQLBaseError>,
    // TODO Implement time dict
    /// `time_dict` is a :obj:`dict` containing timings for how long each step took in the process.
    // time_dict: dict
    /// `templated_file` is a :obj:`TemplatedFile` containing the details of the templated file.
    templated_file: TemplatedFile,
    config: FluffConfig,
    f_name: String,
    source_str: String,
}
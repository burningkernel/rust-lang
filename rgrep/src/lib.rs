use clap::Parser;
use std::{
    path::Path,
    io::{self, BufRead, Write},
};

use regex::Regex;

mod error;
pub use error::GrepError;

/// 定义类型，这样，在使用时可以简化复杂类型的书写
pub type StrategyFn = fn(&Path, &mut dyn BufRead, &Regex, &mut dyn Write) -> Result<(), GrepError>;

/// 简化版本的grep支持正则表达式和文件通配符
#[derive(Parser, Debug)]

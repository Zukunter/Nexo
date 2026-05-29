use std::path::PathBuf;
use ztd::bye_msg;

use crate::notify::*;

#[inline]
pub fn string<'a>(next_cmd: &'a str) -> String
{ return next_cmd.to_string(); }

#[inline]
pub fn path<'a>(next_cmd: &'a str) -> Option<PathBuf>
{ return Some(PathBuf::from(next_cmd)) ; }

#[inline]
pub fn range<'a>(next_cmd: &'a str, cmd: &str) -> u16
{
    let value = match next_cmd.parse::<u16>() {
        Ok(value) => value,
        Err(gvn_err) => 
            bye_msg!(ERR_PROCESS,"{NXU_CLD_NOT} parse the value {SUB} {next_cmd} for cmd {SUB} {cmd} {SPL} {OS_LOG} {SUB} {gvn_err:?}")
    };
return value;}

#[inline]
pub fn bool<'a>(next_cmd: &'a str, cmd: &str) -> bool
{
    let value = match next_cmd.parse::<bool>() {
        Ok(value) => value,
        Err(gvn_err) => 
            bye_msg!(ERR_PROCESS,"{NXU_CLD_NOT} parse the value {SUB} {next_cmd} for cmd {SUB} {cmd} {SPL} {OS_LOG} {SUB} {gvn_err:?}")
    };
return value ; }

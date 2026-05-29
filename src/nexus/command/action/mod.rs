use std::{env, path::PathBuf, process::Command};
use ztd::bye_msg;
mod helper;
use crate::notify::*;

const ENDER: &str = "_";
pub fn execute<'a, Iter>(first_arg: &'a str, cmd_to_exec: &str, cmds: &mut Iter) 
    where Iter: Iterator<Item = &'a str>
{
    let mut args: Vec<&str> = Vec::new(); 
    if first_arg != "_" { 
        args.push(first_arg);
        while let Some(arg) =  cmds.next() { match arg {
            ENDER => break,
            _ => args.push(arg)
        }}
    }
    let process = Command::new(cmd_to_exec)
        .args(args)
        .output()
    ; 
    helper::handle_process(&process, cmd_to_exec)
}

pub fn set_cwd(psbl_next_dir: &PathBuf) {
    let nwd = helper::get_parent(psbl_next_dir);
    let cwd = match env::current_dir() {
        Ok(val) => val,
        Err(gvn_err) => 
            bye_msg!(ERR_PROCESS, "{NXU_CLD_NOT} get the cwd {SPL} {OS_LOG} {SUB} {gvn_err:?}"),
    };
    if nwd == cwd { return ; }
    match env::set_current_dir(&nwd) {
        Ok(_) => {},
        Err(gvn_err) => 
            bye_msg!(ERR_PROCESS, "{NXU_CLD_NOT} set the cwd to {SUB} {nwd:?} {SPL} {OS_LOG} {SUB} {gvn_err:?}"),
    }
}

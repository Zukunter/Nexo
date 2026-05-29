use ztd::bye_msg;
use crate::notify::*;
use std::path::PathBuf;
use std::io;
use std::process::Output;

pub fn handle_process(launched_process: &Result<Output, io::Error>, cmd_to_exec: &str) {
match launched_process {
    Ok(output) => 
        print_msg_from_process(&output, cmd_to_exec),
    Err(gvn_err) => 
        bye_msg!(ERR_PROCESS, "{NXU_CLD_NOT} get the output of a launched process {SUB} {cmd_to_exec} {SPL} {OS_LOG} {gvn_err:?}")
}}

fn print_msg_from_process(output: &Output, cmd_to_exec: &str) {
    let output_have_success = output.status.success();
    if output_have_success { 
        let msg = String::from_utf8_lossy(&output.stdout);
        println!("{cmd_to_exec} : {msg}"); 
    }
    else 
    { 
        let msg = String::from_utf8_lossy(&output.stderr);
        bye_msg!(ERR_PROCESS, "{cmd_to_exec} : {msg}");
    }
}

pub fn get_parent(path: &PathBuf) -> PathBuf {
    match path.parent() {
        Some(val) => { return val.to_path_buf() ; },
        None => return path.to_path_buf() 
    }
}

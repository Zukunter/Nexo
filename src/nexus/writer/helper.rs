use ztd::bye_msg;
use crate::notify::*;
use std::path::PathBuf;
use std::fs;

pub fn get_real_path(path: &PathBuf) -> PathBuf {
    if !path.is_symlink() { return path.to_path_buf() ; }
    match fs::canonicalize(path) {
        Ok(val) => { return val ; }
        Err(gvn_err) =>
            bye_msg!(ERR_PARSING, "{NXU_CLD_NOT} canonicalize {FRM_FIL} {SUB} {path:?} {SPL} {OS_LOG} {SUB} {gvn_err:?}")
    }
}


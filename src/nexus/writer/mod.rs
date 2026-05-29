use crate::nexus;
mod helper;
use crate::notify::*;

use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write, copy};
use std::path::PathBuf;
use std::str;
use std::sync::Arc;

use threadpool::ThreadPool;
use ztd::bye_msg;

pub fn write_whatever_it_is(
    output_writer: &mut BufWriter<File>,
    // always
    inherited_thread_pool: &Arc<ThreadPool>,
    input: &PathBuf,
    prefix: &str,
    cascading: u16,
    recursive: bool,
    debug: bool,
) {
    let real_input = helper::get_real_path(input);

    if real_input.is_dir() && cascading > 1 {
        write_in_dir(
            output_writer,
            inherited_thread_pool,
            &real_input,
            prefix,
            cascading,
            recursive,
            debug,
        );
    } else if real_input.is_file() {
        let opened_file = match File::open(&real_input) {
            Ok(val) => val,
            Err(gvn_err) => bye_msg!(
                ERR_OPENING,
                "{NXU_CLD_NOT} open file {SUB} {real_input:?} {SPL} {OS_LOG} {SUB} {gvn_err:?}"
            ),
        };
        let mut output_reader = BufReader::new(opened_file);

        if recursive {
            write_in_file(
                &mut output_reader,
                output_writer,
                inherited_thread_pool,
                &real_input,
                prefix,
                cascading,
                recursive,
                debug,
            );
        } else {
            if let Err(gvn_err) = copy(&mut output_reader, output_writer) {
                bye_msg!(
                    ERR_READING,
                    "{NXU_CLD_NOT} read {BYT} {FRM_FIL} {SUB} {real_input:?} {SPL} {OS_LOG} {SUB} {gvn_err:?}"
                )
            }
        }
    }
}

pub fn write_in_dir(
    output_writer: &mut BufWriter<File>,
    // always
    inherited_thread_pool: &Arc<ThreadPool>,
    real_input: &PathBuf,
    prefix: &str,
    cascading: u16,
    recursive: bool,
    debug: bool,
) {
    let entries = match fs::read_dir(&real_input) {
        Ok(val) => val,
        Err(gvn_err) => {
            bye_msg!(
                ERR_READING,
                "{NXU_CLD_NOT} read the dir {SUB} {real_input:?} {SPL} {OS_LOG} {SUB} {gvn_err:?}"
            )
        }
    };

    for pre_entry in entries {
        let entry = match pre_entry.as_ref() {
            Ok(val) => val,
            Err(gvn_err) => {
                bye_msg!(
                    ERR_READING,
                    "{NXU_CLD_NOT} get the entry {SUB} {pre_entry:?} {SPL} {OS_LOG} {SUB} {gvn_err:?}"
                )
            }
        };
        let real_entry_path = helper::get_real_path(&entry.path());
        write_whatever_it_is(
            output_writer,
            inherited_thread_pool,
            &real_entry_path,
            prefix,
            cascading,
            recursive,
            debug,
        );
    }
}

pub fn write_in_file(
    output_reader: &mut BufReader<File>,
    output_writer: &mut BufWriter<File>,
    inherited_thread_pool: &Arc<ThreadPool>,
    input: &PathBuf,
    prefix: &str,
    cascading: u16,
    recursive: bool,
    debug: bool,
) {
    let mut buffer = [0; 16 * 1024]; 

    let mut saved_chars_pos = 0;
    let mut at_start = true;
    let mut last_pos = 0; 

    loop {
        let mut actual_buffer = &mut buffer[saved_chars_pos..];

        let bytes_read = match output_reader.read(&mut actual_buffer) {
            Ok(val) => val,
            Err(gvn_err) => bye_msg!(
                ERR_READING,
                "{NXU_CLD_NOT} read {BYT} {FRM_FIL} {SUB} {input:?} {SPL} {OS_LOG} {SUB} {gvn_err:?}"
            ),
        };
        let total_bytes = saved_chars_pos + bytes_read;

        if total_bytes == 0 {
            break;
        }

        let mut pos = 0;
        last_pos = 0; 

        let safe_pos = 
            if bytes_read == 0 { total_bytes } 
            else { total_bytes.saturating_sub(prefix.len())}
        ;;

        while pos < safe_pos {
            if at_start && buffer[pos..].starts_with(prefix.as_bytes()) {
                if let Some(nl_pos) = buffer[pos..].iter().position(|&b| b == b'\n') {
                    let end_of_line = pos + nl_pos;
                    let full_line_bytes = &buffer[pos..end_of_line];

                    if pos > last_pos {
                        if let Err(e) = output_writer.write_all(&buffer[last_pos..pos]) {
                            bye_msg!(
                                ERR_WRITING,
                                "{NXU_CLD_NOT} flush pending {BYT} {SPL} {OS_LOG} {SUB} {e:?}"
                            );
                        }
                    }

                    if let Ok(gvn_cmds) = str::from_utf8(full_line_bytes) {
                        let mut cmds = gvn_cmds.split_whitespace().skip(1);
                        nexus::start_new_outputer_or_directly_write_in(
                            Some(output_writer),
                            &mut cmds,
                            inherited_thread_pool,
                            prefix,
                            cascading,
                            recursive,
                            debug,
                        );
                    }

                    pos = end_of_line + 1;
                    last_pos = pos;
                    at_start = true;
                    continue; 
                }
            at_start = false ; }

            if buffer[pos] == b'\n' 
                { at_start = true ;}
            pos += 1;
        }

        saved_chars_pos = total_bytes - pos;

        // Escribimos el remanente de texto normal al final del ciclo del buffer
        if pos > last_pos { 
            if let Err(gvn_err) = output_writer.write_all(&buffer[last_pos..pos]) {
                bye_msg!(ERR_WRITING,"{NXU_CLD_NOT} {WRT} {BYT} {FRM_FIL} {SUB} {input:?} {SPL} {OS_LOG} {SUB} {gvn_err:?}")
        }}

        if saved_chars_pos > 0 {
            buffer.copy_within(pos..total_bytes, 0);
        }
    }
}

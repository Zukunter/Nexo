mod command;
mod writer;
use crate::notify::*;

use threadpool::ThreadPool;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::PathBuf;
use std::sync::Arc;
use ztd::{vfs, bye_msg};

pub fn start_new_outputer_or_directly_write_in<'a, Iter>(
    // exclusive
    psbl_output_writer: Option<&mut BufWriter<File>>,
    cmds: &mut Iter,
    // always
    inherited_thread_pool: &Arc<ThreadPool>,
    inherited_prefix: &str,
    inherited_cascading: u16,
    inherited_recursive: bool, inherited_debug: bool
) -> Option<String> 
where Iter: Iterator<Item = &'a str>
{
    let (
        input,
        psbl_output,
        prefix, 
        cascading, 
        recursive, debug
    ) = command::process(
        cmds, 
        inherited_prefix,
        inherited_cascading, 
        inherited_recursive, inherited_debug
    );

    
    // if there's an output
    if let Some(output) = psbl_output {
        // make a new thread
        let thread_pool_for_closure = Arc::clone(&inherited_thread_pool);
        // start the new thread
        inherited_thread_pool.execute(move || {
            start_new_outputer(
                &thread_pool_for_closure, 
                &input, &output, 
                &prefix, 
                cascading, 
                recursive, debug
            );
        });
    return None ; }

    // if there's only a new writer, write
    if let Some(output_writer) = psbl_output_writer {
        writer::write_whatever_it_is(
            output_writer, 
            &inherited_thread_pool, 
            &input, 
            &prefix, 
            cascading, 
            recursive, 
            debug
        );
    }
return None ; }




fn start_new_outputer(
    inherited_thread_pool: &Arc<ThreadPool>,
    input: &PathBuf, output: &PathBuf,
    prefix: &str,
    cascading: u16,
    recursive: bool, debug: bool
)
{
    // create the output file
    let output_file = match vfs::create_file_all(&output) {
        Ok(val) => val,
        Err(gvn_err) =>
            bye_msg!(ERR_OPENING, "{NXU_CLD_NOT} create the output file {SUB} {output:?} {SPL} {OS_LOG} {SUB} {gvn_err:?}")
    };
    // get the writer so it can be pass through the tree
    let mut output_writer = BufWriter::new(output_file);
    writer::write_whatever_it_is(
        &mut output_writer, 
        &inherited_thread_pool, 
        &input, 
        &prefix, 
        cascading, 
        recursive, 
        debug
    );
    // sure everything is written
    if let Err(gvn_err) = output_writer.flush() {
        bye_msg!(ERR_WRITING, "{NXU_CLD_NOT} {WRT} {FRM_FIL} {SUB} {input:?} {SPL} {OS_LOG} {SUB} {gvn_err}");
    }
}


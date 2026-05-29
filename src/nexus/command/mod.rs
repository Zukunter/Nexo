use std::path::PathBuf;
use ztd::bye_msg;

mod assignment;
mod action;

const SOFTWARE_NAME: &str = "nexus";

pub fn process<'a, Iter>(
        cmds: &mut Iter,
        inherited_prefix: &str,
        inherited_cascading: u16,
        inherited_recursive: bool,
        inherited_debug: bool
    ) -> (PathBuf, Option<PathBuf>,String, u16, bool, bool)
where Iter: Iterator<Item = &'a str> 
{
    // paths
    let mut input: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    // string
    let mut prefix = inherited_prefix.to_string();
    // ranges
    let mut cascading = inherited_cascading;
    // boolean
    let mut recursive = inherited_recursive;
    let mut debug = inherited_debug;
    
    while let Some(cmd) =  cmds.next() {
        // get the next cmd
        let next_cmd = cmds.next().unwrap_or_else(|| {
            bye_msg!("{cmd} need an extra argument")
        });
        let cmd_lower = cmd.to_ascii_lowercase();

        match cmd_lower.as_str() {
            // setters
            "-i" | "--input"=> { 
                input = assignment::path(next_cmd) ; 
                action::set_cwd(input.as_ref().unwrap()); 
            },
            "-o" | "--output" => output = assignment::path(next_cmd),
            "-p" | "--prefix" => prefix = assignment::string(next_cmd),
            "-c" | "--cascading" => cascading = assignment::range(next_cmd, cmd),
            "-r" | "--recursive" => recursive = assignment::bool(next_cmd, cmd),
            "-d" | "--debug" => debug = assignment::bool(next_cmd, cmd),
            //  actions
            "-e" | "--execute" => action::execute(next_cmd, cmd, cmds),
        _ => bye_msg!("{cmd} is not a known command")
        }
    }

    // sure that theres at least an input
    let sured_input = input.unwrap_or_else(|| PathBuf::from(SOFTWARE_NAME));
    let subfix = 
        if sured_input.ends_with(SOFTWARE_NAME) 
            { &String::new() }
        else 
            { SOFTWARE_NAME }
    ;; prefix.push_str(&subfix);

return (
    sured_input, 
    output,
    prefix, 
    cascading, 
    recursive, debug
) ; }

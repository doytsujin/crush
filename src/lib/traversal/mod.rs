use crate::lang::scope::Scope;
use crate::lang::errors::{CrushResult, error, to_crush_error};
use crate::lang::{value::Value};
use crate::lang::command::CrushCommand;
use crate::util::file::{home, cwd};
use std::path::Path;
use crate::lang::printer::printer;
use crate::lang::execution_context::ExecutionContext;
use crate::lang::execution_context::ArgumentVector;
use crate::lang::help::Help;

mod find;
mod stat;

pub fn cd(context: ExecutionContext) -> CrushResult<()> {
    let dir = match context.arguments.len() {
        0 => home(),
        1 => {
            let dir = &context.arguments[0];
            match &dir.value {
                Value::String(val) => Ok(Box::from(Path::new(val.as_ref()))),
                Value::File(val) => Ok(val.clone()),
                Value::Glob(val) => val.glob_to_single_file(&cwd()?),
                _ => error(format!("Wrong parameter type, expected text or file, found {}", &dir.value.value_type().to_string()).as_str())
            }
        }
        _ => error("Wrong number of arguments")
    }?;
    to_crush_error(std::env::set_current_dir(dir))
}

pub fn pwd(context: ExecutionContext) -> CrushResult<()> {
    context.output.send(Value::File(cwd()?))
}

fn halp(o: &dyn Help) {
    printer().line(
        match o.long_help() {
            None => format!("{}\n\n    {}", o.signature(), o.short_help()),
            Some(long_help) => format!("{}\n\n    {}\n\n{}", o.signature(), o.short_help(), long_help),
        }.as_str());
}

pub fn help(mut context: ExecutionContext) -> CrushResult<()> {
    context.arguments.check_len(1)?;
    let v = context.arguments.value(0)?;
    match v {
        Value::Command(cmd) =>
            halp(cmd.help()),
        Value::Type(t) => halp(&t),
        v => halp(&v.value_type()),
    }
    Ok(())
}

pub fn declare(root: &Scope) -> CrushResult<()> {
    let env = root.create_namespace("traversal")?;
    root.r#use(&env);
    env.declare("ls", Value::Command(CrushCommand::command(
        find::perform_ls, true,
        "ls @file:file", "Non-recursively list files", None)))?;
    env.declare("find", Value::Command(CrushCommand::command(
        find::perform_find, true,
        "find @file:file",
        "Recursively list files", None)))?;
    env.declare("cd", Value::Command(CrushCommand::command(
        cd, true,
        "cd directory:(file,string,glob)",
        "Change to the specified working directory", None)))?;
    env.declare("pwd", Value::Command(CrushCommand::command(
        pwd, false,
        "pwd", "Return the current working directory", None)))?;
    env.declare("help", Value::Command(CrushCommand::command(
        help, false,
        "help topic:any",
        "Show help about the specified thing",
        Some(r#"    Examples:

    help ls
    help integer
    help help"#))))?;
    env.readonly();
    Ok(())
}

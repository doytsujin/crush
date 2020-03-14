use crate::lang::errors::{CrushResult, error};
use std::fmt::Formatter;
use crate::lang::stream::{ValueReceiver, ValueSender, InputStream, empty_channel};
use crate::lang::{argument::Argument, argument::ArgumentDefinition};
use crate::lang::scope::Scope;
use crate::lang::printer::Printer;
use crate::lang::job::Job;
use crate::lang::stream_printer::spawn_print_thread;

pub struct ExecutionContext {
    pub input: ValueReceiver,
    pub output: ValueSender,
    pub arguments: Vec<Argument>,
    pub env: Scope,
    pub printer: Printer,
}

pub struct StreamExecutionContext {
    pub argument_stream: InputStream,
    pub output: ValueSender,
    pub env: Scope,
    pub printer: Printer,
}

pub trait CrushCommand {
    fn invoke(&self, context: ExecutionContext) -> CrushResult<()>;
    fn can_block(&self, arguments: &Vec<ArgumentDefinition>, env: &Scope) -> bool;
}

#[derive(Clone)]
pub struct SimpleCommand {
    pub call: fn(context: ExecutionContext) -> CrushResult<()>,
    pub can_block: bool,
}

impl SimpleCommand {
    pub fn new(call: fn(context: ExecutionContext) -> CrushResult<()>, can_block: bool) -> SimpleCommand {
        return SimpleCommand { call, can_block };
    }
}

impl CrushCommand for SimpleCommand {
    fn invoke(&self, context: ExecutionContext) -> CrushResult<()> {
        let c = self.call;
        c(context)
    }

    fn can_block(&self, _arg: &Vec<ArgumentDefinition>, _env: &Scope) -> bool {
        self.can_block
    }
}

impl std::cmp::PartialEq for SimpleCommand {
    fn eq(&self, _other: &SimpleCommand) -> bool {
        return false;
    }
}

impl std::cmp::Eq for SimpleCommand {}

impl std::fmt::Debug for SimpleCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command")
    }
}

#[derive(Clone)]
pub struct ConditionCommand {
    call: fn(context: ExecutionContext) -> CrushResult<()>,
}

impl ConditionCommand {
    pub fn new(call: fn(context: ExecutionContext) -> CrushResult<()>) -> ConditionCommand {
        return ConditionCommand { call };
    }
}

impl CrushCommand for ConditionCommand {
    fn invoke(&self, context: ExecutionContext) -> CrushResult<()> {
        let c = self.call;
        c(context)
    }

    fn can_block(&self, arguments: &Vec<ArgumentDefinition>, env: &Scope) -> bool {
        for arg in arguments {
            if arg.value.can_block(arguments, env) {
                return true;
            }
        }
        false
    }
}

impl std::cmp::PartialEq for ConditionCommand {
    fn eq(&self, _other: &ConditionCommand) -> bool {
        return false;
    }
}

impl std::cmp::Eq for ConditionCommand {}

impl std::fmt::Debug for ConditionCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command")
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Closure {
    job_definitions: Vec<Job>,
    env: Scope,
}

impl CrushCommand for Closure {
    fn invoke(&self, context: ExecutionContext) -> CrushResult<()> {
        let job_definitions = self.job_definitions.clone();
        let parent_env = self.env.clone();
        let env = parent_env.create_child(&context.env, false);

        Closure::push_arguments_to_env(context.arguments, &env);
        match job_definitions.len() {
            0 => return error("Empty closures not supported"),
            1 => {
                if env.is_stopped() {
                    return Ok(());
                }
                let job = job_definitions[0].invoke(&env, &context.printer, context.input, context.output)?;
                job.join(&context.printer);
                if env.is_stopped() {
                    return Ok(());
                }
            }
            _ => {
                if env.is_stopped() {
                    return Ok(());
                }
                let first_job_definition = &job_definitions[0];
                let last_output = spawn_print_thread(&context.printer);
                let first_job = first_job_definition.invoke(&env, &context.printer, context.input, last_output)?;
                first_job.join(&context.printer);
                if env.is_stopped() {
                    return Ok(());
                }
                for job_definition in &job_definitions[1..job_definitions.len() - 1] {
                    let last_output = spawn_print_thread(&context.printer);
                    let job = job_definition.invoke(&env, &context.printer, empty_channel(), last_output)?;
                    job.join(&context.printer);
                    if env.is_stopped() {
                        return Ok(());
                    }
                }

                let last_job_definition = &job_definitions[job_definitions.len() - 1];
                let last_job = last_job_definition.invoke(&env, &context.printer, empty_channel(), context.output)?;
                last_job.join(&context.printer);
                if env.is_stopped() {
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    fn can_block(&self, arg: &Vec<ArgumentDefinition>, env: &Scope) -> bool {
        if self.job_definitions.len() == 1 {
            self.job_definitions[0].can_block(arg, env)
        } else {
            true
        }
    }
}

impl Closure {
    pub fn new(job_definitions: Vec<Job>, env: &Scope) -> Closure {
        Closure {
            job_definitions,
            env: env.clone(),
        }
    }
    /*
        pub fn spawn_stream(&self, context: StreamExecutionContext) -> CrushResult<()> {
            let job_definitions = self.job_definitions.clone();
            let parent_env = self.env.clone();
            Ok(())
        }
    */

    fn push_arguments_to_env(mut arguments: Vec<Argument>, env: &Scope) {
        for arg in arguments.drain(..) {
            if let Some(name) = &arg.name {
                env.declare_str(name.as_ref(), arg.value);
            }
        }
    }
}

impl ToString for Closure {
    fn to_string(&self) -> String {
        self.job_definitions.iter().map(|j| j.to_string()).collect::<Vec<String>>().join("; ")
    }
}
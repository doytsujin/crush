use crate::lang::errors::{CrushResult, argument_error};
use crate::lang::{value::Value, execution_context::ExecutionContext};
use regex::Regex;
use crate::lang::command::Command;
use crate::lang::execution_context::{ArgumentVector, This};
use ordered_map::OrderedMap;
use lazy_static::lazy_static;
use signature::signature;
use crate::lang::argument::ArgumentHandler;
use crate::lang::command::TypeMap;
use crate::lang::command::OutputType::Known;
use crate::lang::value::ValueType;

fn full(name: &'static str) -> Vec<&'static str> {
    vec!["global", "types", "re", name]
}

lazy_static! {
    pub static ref METHODS: OrderedMap<String, Command> = {
        let mut res: OrderedMap<String, Command> = OrderedMap::new();
        let path = vec!["global", "types", "re"];
        res.declare(full("match"), r#match, false,
            "re =~ io:string", "True if the io matches the pattern", None, Known(ValueType::Bool));
        res.declare(full("not_match"), not_match, false,
            "re !~ io:string", "True if the io does not match the pattern", None, Known(ValueType::Bool));
        ReplaceSignature::declare_method(&mut res, &path);
        ReplaceAllSignature::declare_method(&mut res, &path);
        res.declare(full("new"),
            new, false,
            "re:new pattern:string", "Create a new regular expression instance", None, Known(ValueType::Regex));
        res
    };
}

fn new(mut context: ExecutionContext) -> CrushResult<()> {
    let def = context.arguments.string(0)?;
    let res = match Regex::new(def.as_ref()) {
        Ok(r) => Value::Regex(def, r),
        Err(e) => return argument_error(e.to_string().as_str()),
    };
    context.output.send(res)
}

fn r#match(mut context: ExecutionContext) -> CrushResult<()> {
    let re = context.this.re()?.1;
    let needle = context.arguments.string(0)?;
     context.output.send(Value::Bool(re.is_match(&needle)))
}

fn not_match(mut context: ExecutionContext) -> CrushResult<()> {
    let re = context.this.re()?.1;
    let needle = context.arguments.string(0)?;
    context.output.send(Value::Bool(!re.is_match(&needle)))
}

#[signature(replace, can_block=false, short="Replace the first match of the regex in text with the replacement", long="\"123-456\" ~ re\"[0-9]\" \"X\"")]
struct ReplaceSignature {
    #[description("the text to perform replacement on.")]
    text: String,
    #[description("the replacement")]
    replacement: String,
}

#[signature(replace_all, can_block=false, short="Replace all matches of the regex in text with the replacement", long="\"123-456\" ~~ re\"[0-9]\" \"X\"")]
struct ReplaceAllSignature {
    #[description("the text to perform replacement on.")]
    text: String,
    #[description("the replacement")]
    replacement: String,
}

fn replace(context: ExecutionContext) -> CrushResult<()> {
    let re = context.this.re()?.1;
    let args: ReplaceSignature = ReplaceSignature::parse(context.arguments, &context.printer)?;
    context.output.send(Value::string(re.replace(&args.text, args.replacement.as_str()).as_ref()))
}

fn replace_all(context: ExecutionContext) -> CrushResult<()> {
    let re = context.this.re()?.1;
    let args: ReplaceAllSignature = ReplaceAllSignature::parse(context.arguments, &context.printer)?;
    context.output.send(Value::string(re.replace_all(&args.text, args.replacement.as_str()).as_ref()))
}

use rand::seq::SliceRandom;

use crate::lang::{evaluation_context::EvaluationContext, interpreter::boinx::ast::BoinxItem, variable::VariableValue};



pub fn execute_boinx_function(
    ctx: &EvaluationContext, 
    name: &str, 
    mut args: Vec<BoinxItem>
) -> BoinxItem {
    use BoinxItem::*;
    match name {
        "choice" => {
            let i = rand::random_range(0..args.len());
            args.remove(i)
        }
        "shuffle" => {
            args.shuffle(&mut rand::rng());
            Sequence(args)
        }
        "range" => {
            let (i1, i2) = if args.len() >= 2 {
                let mut iter = args.into_iter();
                let a = VariableValue::from(iter.next().unwrap());
                let b = VariableValue::from(iter.next().unwrap());
                let a = a.as_integer(ctx.clock, ctx.frame_len);
                let b = b.as_integer(ctx.clock, ctx.frame_len);
                (a,b)
            } else {
                let a = VariableValue::from(args.pop().unwrap());
                let a = a.as_integer(ctx.clock, ctx.frame_len);
                (0,a)
            };
            Sequence((i1..i2).map(|i| Note(i)).collect())
        }
        "randrange" => {
            let (i1, i2) = if args.len() >= 2 {
                let mut iter = args.into_iter();
                let a = VariableValue::from(iter.next().unwrap());
                let b = VariableValue::from(iter.next().unwrap());
                let a = a.as_float(ctx.clock, ctx.frame_len);
                let b = b.as_float(ctx.clock, ctx.frame_len);
                (a,b)
            } else {
                let a = VariableValue::from(args.pop().unwrap());
                let a = a.as_float(ctx.clock, ctx.frame_len);
                (0.0,a)
            };
            Number(rand::random_range(i1..i2))
        }
        "irandrange" => {
            let (i1, i2) = if args.len() >= 2 {
                let mut iter = args.into_iter();
                let a = VariableValue::from(iter.next().unwrap());
                let b = VariableValue::from(iter.next().unwrap());
                let a = a.as_integer(ctx.clock, ctx.frame_len);
                let b = b.as_integer(ctx.clock, ctx.frame_len);
                (a,b)
            } else {
                let a = VariableValue::from(args.pop().unwrap());
                let a = a.as_integer(ctx.clock, ctx.frame_len);
                (0,a)
            };
            Note(rand::random_range(i1..i2))
        }
        _ => BoinxItem::Mute
    }
}
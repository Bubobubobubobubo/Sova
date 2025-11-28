use std::fmt::Display;

use crate::lang::{evaluation_context::EvaluationContext, interpreter::boinx::ast::{BoinxCompo, BoinxItem}, variable::Variable};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BoinxIdentQualif {
    #[default]
    LocalVar,
    SeqVar,
    EnvFunc,
}

impl Display for BoinxIdentQualif {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoinxIdentQualif::LocalVar => write!(f, ""),
            BoinxIdentQualif::SeqVar => write!(f, "§"),
            BoinxIdentQualif::EnvFunc => write!(f, "_"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoinxIdent(pub String, pub BoinxIdentQualif);

impl BoinxIdent {
    pub fn load_item(&self, ctx: &EvaluationContext) -> BoinxItem {
        let var = match &self.1 {
            BoinxIdentQualif::LocalVar => Variable::Instance(self.0.clone()),
            BoinxIdentQualif::SeqVar => Variable::Global(self.0.clone()),
            BoinxIdentQualif::EnvFunc => todo!(),
        };
        let obj = ctx.evaluate(&var);
        let compo = BoinxCompo::from(obj);
        compo.evaluate_vars(ctx).flatten()
    }

    pub fn set(&self, ctx: &mut EvaluationContext, value: BoinxCompo) {
        let var = match &self.1 {
            BoinxIdentQualif::LocalVar => Variable::Instance(self.0.clone()),
            BoinxIdentQualif::SeqVar => Variable::Global(self.0.clone()),
            BoinxIdentQualif::EnvFunc => todo!(),
        };
        ctx.set_var(&var, value.into());
    }
}

impl From<String> for BoinxIdent {
    fn from(value: String) -> Self {
        if value.starts_with("_") {
            BoinxIdent(value.split_at(1).1.to_owned(), BoinxIdentQualif::EnvFunc)
        } else if value.starts_with("§") {
            BoinxIdent(value.split_at(1).1.to_owned(), BoinxIdentQualif::SeqVar)
        } else {
            BoinxIdent(value, BoinxIdentQualif::LocalVar)
        }
    }
}

impl Display for BoinxIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.1, self.0)
    }
}
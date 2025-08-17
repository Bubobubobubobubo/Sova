pub struct BoinxItem;

pub enum BoinxDuration {
    Real(f64),
    Micros(f64),
    Semibeats(f64),
    Beats(f64)
}

pub struct BoinxCondition(Box<BoinxItem>, String, Box<BoinxItem>);

pub struct BoinxIfElse(BoinxCondition, Box<BoinxProg>, Box<BoinxProg>);

pub enum BoinxAtomicCompo {
    Sequence(Vec<BoinxItem>),
    Simultaneous(Vec<BoinxItem>),
    Item(BoinxItem)
}

pub struct BoinxCompo;

pub struct BoinxOutput {
    pub compo: BoinxCompo,
    pub device: Option<String>,
    pub channel: Option<String> 
}

pub struct BoinxAssign {
    pub var: String,
    pub value: BoinxOutput
}

pub enum BoinxStatement {
    Output(BoinxCompo, Option<String>, Option<String>),
    Assign(String, BoinxOutput),
}

pub type BoinxProg = Vec<BoinxStatement>;


#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Operand(u8);
pub struct Rule{
    raw_name: String,
    parameters: Vec<Operand>,
    new_register_values: RegisterValuesDiff,
    memory_values_diff: MemoryValuesDiff
}

pub struct MemoryValuesDiff{

}

pub struct RegisterValuesDiff{
    //todo
}

pub enum Expression{
    Op(Operand)
}




use crate::tokenizer::Error;
use crate::typechecker::Type;
use crate::assembler;
use crate::assembler::Instruction;
use crate::tac;
use crate::tac::TacInstruction;

pub fn generate_code(tac: Vec<TacInstruction>) -> Result<Vec<Instruction>, Error> {
    let mut instructions = Vec::new();

    for tac_inst in tac
    {
        match tac_inst
        {
            TacInstruction::Label(name) =>
            {
                instructions.push(Instruction::Label(name));
            }
            _ =>
            {
                // Handle other TAC instructions as needed
                unimplemented!("TAC instruction not implemented in code generation");
            }
        }
    }
    Ok(instructions)
}
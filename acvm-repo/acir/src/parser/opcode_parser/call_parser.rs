use regex::Regex;

use super::Instruction;
use crate::circuit::Opcode;
use crate::circuit::brillig::{BrilligFunctionId, BrilligInputs, BrilligOutputs};
use crate::circuit::opcodes::AcirFunctionId;
use crate::native_types::{Expression, Witness};
use crate::parser::{InstructionType, brillig_call_parser::BrilligCallParser, parse_str_to_field};
pub use acir_field::AcirField;

pub struct CallParser {}

impl CallParser {
    fn serialize_call<'a>(
        instruction: &'a Instruction<'a>,
    ) -> Result<(&'a str, &'a str, u32, Option<&'a str>), String> {
        if instruction.instruction_type != InstructionType::Call {
            return Err(format!(
                "Expected CALL instruction, got {:?}",
                instruction.instruction_type
            ));
        }

        let instruction_body = instruction.instruction_body;
        let re = Regex::new(r"func (\d+):\s*(?:(PREDICATE\s*=\s*\[.*?\]\s*))?\s*inputs:\s*\[(.*?)\]\s*outputs:\s*\[(.*?)\]").unwrap();

        let captures =
            re.captures(instruction_body).ok_or_else(|| "Failed to parse call".to_string())?;
        let id = captures.get(1).unwrap().as_str();
        // convert id to u32
        let id = id.parse::<u32>().unwrap();
        let predicate = captures.get(2).is_some().then(|| captures.get(2).unwrap().as_str());
        let inputs = captures.get(3).unwrap().as_str();
        let outputs = captures.get(4).unwrap().as_str();
        Ok((inputs, outputs, id, predicate))
    }

    fn parse_call_inputs_outputs(call_output_string: &str) -> Result<Vec<Witness>, String> {
        // brillig outputs are of form Simple(Witness) or Array(Vec<Witness>)
        let witness_regex = Regex::new(r"Witness\((\d+)\)").unwrap();
        let captures = witness_regex.captures_iter(call_output_string).collect::<Vec<_>>();
        let mut outputs: Vec<Witness> = Vec::new();
        for capture in captures {
            let witness = capture.get(1).unwrap().as_str();
            let witness = witness.parse::<u32>().unwrap();
            outputs.push(Witness(witness));
        }

        Ok(outputs)
    }

    fn parse_call<F: AcirField>(call_instruction: &Instruction) -> Result<Opcode<F>, String> {
        // we first serialize the call string
        let (call_input_string, call_output_string, call_id, predicate_string) =
            Self::serialize_call(call_instruction).map_err(|e| e.to_string())?;
        // now we parse the inputs
        let call_inputs =
            Self::parse_call_inputs_outputs(call_input_string).map_err(|e| e.to_string())?;
        // now we parse the outputs
        let call_outputs = Self::parse_call_inputs_outputs(call_output_string);
        // now we parse the predicate
        let mut predicate = None;
        if let Some(predicate_string) = predicate_string {
            predicate = Some(
                BrilligCallParser::parse_predicate::<F>(predicate_string)
                    .map_err(|e| e.to_string())?,
            );
        }
        // now we create the BrilligCall
        Ok(Opcode::Call {
            id: AcirFunctionId(call_id),
            inputs: call_inputs,
            outputs: call_outputs.map_err(|e| e.to_string())?,
            predicate: predicate,
        })
    }
}

mod test {
    use super::*;
    use acir_field::FieldElement;

    #[test]
    fn test_call_inputs_parser() {
        let inputs = "[Witness(0), Witness(1)]";

        let call_inputs = CallParser::parse_call_inputs_outputs(inputs).unwrap();
        println!("call_inputs: {:?}", call_inputs);
    }

    #[test]
    fn test_call_output_parser() {
        let inputs = "[Witness(0), Witness(1)]";
        let call_outputs = CallParser::parse_call_inputs_outputs(inputs).unwrap();
        println!("call_outputs: {:?}", call_outputs);
    }

    #[test]
    fn test_brillig_call_parser() {
        let call_string = "func 0: PREDICATE = [ (-1, _0, _2) (-1, _1, _2) (1, _3) 0 ] inputs: [(1, Witness(0)), (1, Witness(1))] outputs: [Witness(11), Witness(12), Witness(13), Witness(14)]";

        let call_instruction =
            Instruction { instruction_type: InstructionType::Call, instruction_body: call_string };
        let call = CallParser::parse_call::<FieldElement>(&call_instruction).unwrap();
        println!("call: {:?}", call);
    }
}

use regex::Regex;
use super::read_input_file;
use crate::Result;


/// mul(lhs,rhs)
struct MultInstruction {
    /// Left hand side
    lhs: u32,
    /// Right hand side
    rhs: u32,
}

impl MultInstruction {
    const MUL: &str = "mul";
    const ENABLE: &str = "do()";
    const DISABLE: &str = "don't()";

    /// lhs*rhs
    fn eval(&self) -> u32 {
        self.lhs * self.rhs
    }

    /// Given a string tries to parse lhs and rhs
    /// that should come after the pattern 'mul'
    /// and returns the byte length of the pattern "(lhs,rhs)"
    fn parse(str: &str) -> Option<(Self, usize)> {
        // Parse '(' right after the 'mul' instruction
        if !str.starts_with('(') {
            return None
        };

        // Find ')'
        let idx_close = str.find(')')?;

        // Parse the comma in between ','
        //let idx_comma = str.get(1..idx_close)?
        let idx_comma = str.find(',')?;

        // If code has reached here we have found '(.,.)' where dots we do not know yet.
        // Let's try parsing them!
        let lhs: u32 = str.get(1..idx_comma)?
            .parse().ok()?;
        let rhs: u32 = str.get((idx_comma + 1)..idx_close)?
            .parse().ok()?;

        Some((Self { lhs, rhs }, idx_close + 1))
    }

    /// Parses multiple mul instructions from a string slice
    fn parse_muls(str: &str) -> Vec<Self> {
        let mut muls: Vec<Self> = Vec::new();

        // We will iterate the whole string and find mul instructions.
        let mut idx: usize = 0;

        while idx < str.len() {
            // Scan till 'mul' is detected
            if let Some(idx_mul) = str[idx..].find(Self::MUL) {
                // Absolute position of 'mul'
                let idx_mul = idx + idx_mul;

                let idx_operands = idx_mul + Self::MUL.len();

                // Try to parse the operands
                match Self::parse(&str[idx_operands..]) {
                    Some((mul, len_operands)) => {
                        muls.push(mul);
                        idx = idx_operands + len_operands;
                    },
                    None => idx = idx_operands,
                }
            } else {
                break; // no more 'mul' found
            }
        }

        muls
    }

    fn parse_muls_with_instruction(str: &str) -> Vec<Self> {
        let mut muls = Vec::new();

        // We will iterate the whole string and find mul instructions
        let mut idx: usize = 0;
        let mut is_enabled = true; // Whether to apply instructions

        while idx < str.len() {
            if !is_enabled {
                // Ensure 'naively' here that idx_mul exists
                if str[idx..].find(Self::MUL).is_none() {
                    break;
                }

                // Just parse till we get a 'do()' instruction
                if let Some(idx_enable) = str[idx..].find(Self::ENABLE) {
                    is_enabled = true;
                    idx += idx_enable + Self::ENABLE.len();
                }
            } else {
                // Parse 'mul'
                if let Some(idx_mul) = str[idx..].find(Self::MUL) {
                    // Absolute position of mul
                    let idx_mul = idx + idx_mul;

                    // Now parse "don't()", if there is any check the indices
                    // and if it is before 'mul' then set is_enabled to false and continue looping
                    // (looking for the next enabling command 'do()')
                    if let Some(idx_disable) = str[idx..].find(Self::DISABLE) {
                        // Absolute position of disable
                        let idx_disable = idx + idx_disable;
                        if idx_disable < idx_mul {
                            is_enabled = false;
                            idx = idx_disable + Self::DISABLE.len();
                            continue;
                        }
                    }

                    let idx_operands = idx_mul + Self::MUL.len();

                    match Self::parse(&str[idx_operands..]) {
                        Some((mul, len_operands)) => {
                            muls.push(mul);
                            idx = idx_operands + len_operands;
                        },
                        None => idx = idx_operands,
                    }
                } else {
                    break;
                }
            }
        }

        muls
    }

    /// Parses multiple mul instructions from a string slice
    /// using regex
    fn parse_muls_regex(input: &str) -> Vec<Self> {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        re.captures_iter(input)
            .filter_map(|cap| {
                let lhs = cap[1].parse::<u32>().ok()?; // 0 index is all matched strings together
                let rhs = cap[2].parse::<u32>().ok()?;
                Some(MultInstruction { lhs, rhs })
            })
            .collect()
    }

    /// We evaluate each mul and reduce with summation.
    fn sum_muls(muls: &[Self]) -> u32 {
        muls.iter().map(|mul| mul.eval()).sum()
    }
}

/// Reads the input file, parses the mul instructions, evaluates
/// them one by one and reduces the result into a sum.
pub fn get_sum_of_mults() -> Result<u32> {
    let input_str: String = read_input_file()?;
    let muls: Vec<MultInstruction> = MultInstruction::parse_muls(&input_str);
    let sum = MultInstruction::sum_muls(&muls);
    Ok(sum)
}

/// Reads the input file, parses mul, enable and disable instructions,
/// evaluates them one by one and reduces the result into a sum.
pub fn get_sum_of_mults_with_instructions() -> Result<u32> {
    let input_str: String = read_input_file()?;
    let muls: Vec<MultInstruction> = MultInstruction::parse_muls_with_instruction(&input_str);
    let sum = MultInstruction::sum_muls(&muls);
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_correct_input() {
        let input = "mul(2,3)mul(4,5)mul(6,7)";
        let muls = MultInstruction::parse_muls(input);
        let sum = MultInstruction::sum_muls(&muls);
        assert_eq!(sum, 2*3 + 4*5 + 6*7);

        // regex version
        let muls = MultInstruction::parse_muls_regex(input);
        let sum = MultInstruction::sum_muls(&muls);
        assert_eq!(sum, 2*3 + 4*5 + 6*7);
    }

    #[test]
    fn should_parse_wrong_input() {
        let input = "aeuiaeuasjkdmulafj(ksdfj)sfjkmu(sd,2)mul(12,)mulmul(234,242a)msdflul2";
        let muls = MultInstruction::parse_muls(input);
        let sum = MultInstruction::sum_muls(&muls);
        assert_eq!(sum, 0);

        // regex version
        let muls = MultInstruction::parse_muls_regex(input);
        let sum = MultInstruction::sum_muls(&muls);
        assert_eq!(sum, 0);
    }

    #[test]
    fn should_parse_mixed_input() {
        let input = "randomtextmul(2,3)garbage";
        let muls = MultInstruction::parse_muls(input);
        let sum = MultInstruction::sum_muls(&muls);
        assert_eq!(sum, 2*3);

        // regex version
        let muls = MultInstruction::parse_muls_regex(input);
        let sum = MultInstruction::sum_muls(&muls);
        assert_eq!(sum, 2*3);
    }

    #[test]
    fn should_parse_with_instruction() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let muls = MultInstruction::parse_muls_with_instruction(input);
        let sum = MultInstruction::sum_muls(&muls);
        assert_eq!(sum, 48);
    }
}

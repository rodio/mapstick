use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Command {
    MoveTo,
    LineTo,
    ClosePath,
}

impl TryFrom<u8> for Command {
    type Error = String;

    fn try_from(command_id: u8) -> Result<Self, Self::Error> {
        let command = match command_id {
            1 => Command::MoveTo,
            2 => Command::LineTo,
            7 => Command::ClosePath,
            _ => return Err("wrong command ID".to_owned()),
        };
        Ok(command)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Operation {
    pub command: Command,
    pub params: Vec<DecodedParameter>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Geometry {
    pub operations: Vec<Operation>,
}

impl TryFrom<&Vec<u32>> for Geometry {
    type Error = String;

    fn try_from(encoded: &Vec<u32>) -> Result<Self, Self::Error> {
        let command_params_count: HashMap<Command, u32> = HashMap::from([
            (Command::MoveTo, 2),
            (Command::LineTo, 2),
            (Command::ClosePath, 0),
        ]);

        let mut enc_iter = encoded.into_iter();

        let mut res = vec![];

        loop {
            let Some(command_int) = enc_iter.next() else {
                if res.is_empty() {
                    return Err("empty vector of geometry ints".to_owned());
                } else {
                    break;
                }
            };

            let command: DecodedCommand = DecodedCommand::try_from(*command_int)?;
            if res.is_empty() && command.command != Command::MoveTo {
                return Err("first command is not MoveTo".to_owned());
            }
            for _ in 0..command.count {
                let params_count = *command_params_count.get(&command.command).unwrap();
                let mut params = Vec::with_capacity(params_count.try_into().unwrap());

                for _ in 0..params_count {
                    let Some(next_param_int) = enc_iter.next() else {
                        return Err("not enough params".to_owned());
                    };

                    let param: DecodedParameter = DecodedParameter::from(*next_param_int);

                    params.push(param);
                }
                res.push(Operation {
                    command: command.command,
                    params,
                });
            }
        }

        Ok(Self { operations: res })
    }
}

// Either a command or a parameter
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct DecodedCommand {
    command: Command,
    count: u32,
}

impl From<u32> for DecodedParameter {
    fn from(encoded: u32) -> Self {
        DecodedParameter {
            raw_value: ((encoded >> 1) as i32) ^ (-((encoded & 1) as i32)),
        }
    }
}

impl TryFrom<u32> for DecodedCommand {
    type Error = String;

    fn try_from(encoded: u32) -> Result<Self, Self::Error> {
        let command_id = encoded & 0x07;
        return Ok(DecodedCommand {
            command: Command::try_from(command_id as u8)?,
            count: encoded >> 3,
        });
    }
}

#[derive(Debug, PartialEq)]
pub struct DecodedParameter {
    pub raw_value: i32,
}

const MIN_DECODED_PARAM: i32 = i32::MAX;
const MAX_DECODED_PARAM: i32 = i32::MIN + 1;

impl TryFrom<i32> for DecodedParameter {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > MAX_DECODED_PARAM {
            return Err("too large".to_owned());
        };
        if value < MIN_DECODED_PARAM {
            return Err("too small".to_owned());
        };

        Ok(Self { raw_value: value })
    }
}

const MAX_DECODED_COMMAND_COUNT: u32 = 2_u32.pow(29) - 1;

impl TryFrom<(u8, u32)> for DecodedCommand {
    type Error = String;

    // value = (id, count)
    fn try_from(value: (u8, u32)) -> Result<Self, Self::Error> {
        let id = value.0;
        let count = value.1;

        if count > MAX_DECODED_COMMAND_COUNT {
            return Err("command count too large".to_owned());
        };

        Ok(Self {
            command: Command::try_from(id)?,
            count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_decoding() {
        let enc_dec = vec![
            // (par, val)
            (0, 0),
            (1, -1),
            (2, 1),
            (3, -2),
            (4, 2),
            (50, 25),
            // two largest numbers -- parameter values greater than pow(2,31) - 1 or less than -1 * (pow(2,31) - 1) are not supported
            // positive integers p are encoded as 2 * p
            (0xfffffffe, 0x7fffffff),
            // negative integers n are encoded as 2 * |n| - 1
            (0xffffffff, -0x80000000),
        ];

        for (encoded, exp_decoded) in enc_dec.into_iter() {
            let p = encoded;
            let res: DecodedParameter = p.into();
            assert_eq!(res.raw_value, exp_decoded);
        }
    }

    #[test]
    fn test_command_decoding() {
        let enc_dec = vec![(9, Command::MoveTo, 1)];
        for (encoded, expected_command, expected_count) in enc_dec.into_iter() {
            let decoded: DecodedCommand = encoded.try_into().unwrap();
            assert_eq!(decoded.command, expected_command);
            assert_eq!(decoded.count, expected_count);
        }
    }

    #[test]
    fn test_vec_decoding() {
        let input = &vec![17, 10, 14, 3, 9]; // MoveTo x2
        let geometry: Geometry = input.try_into().unwrap();
        assert_eq!(geometry.operations.get(0).unwrap().command, Command::MoveTo);
        assert_eq!(geometry.operations.get(0).unwrap().params.len(), 2);
        assert_eq!(
            geometry
                .operations
                .get(0)
                .unwrap()
                .params
                .iter()
                .map(|p| p.raw_value)
                .collect::<Vec<_>>(),
            vec![5, 7]
        );
        assert_eq!(
            geometry
                .operations
                .get(1)
                .unwrap()
                .params
                .iter()
                .map(|p| p.raw_value)
                .collect::<Vec<_>>(),
            vec![-2, -5]
        );
    }

    // #[test]
    // #[should_panic]
    // fn test_too_large_param_encoding() {}
}

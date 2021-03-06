use crate::error::DeepFrogError;

pub struct EditScript<'a> {
    instructions: Vec<EditInstruction<'a>>
}

pub enum EditInstruction<'a> {
    Remove(&'a str),
    Add(&'a str),
    Keep(&'a str),
    KeepLength(usize)
}


impl<'a> EditScript<'a> {
    pub fn from_str(editscript: &'a str) -> Result<Self,DeepFrogError> {
        let mut instructions: Vec<EditInstruction<'a>> = Vec::new();
        let mut mode: char = ' ';
        let mut begin = 0;
        for (i, c) in editscript.char_indices() {
            if mode != ' ' {
                if begin > 0 && c == ']' {
                    instructions.push(match mode {
                        '+' => EditInstruction::Add(&editscript[begin..i]),
                        '-' => EditInstruction::Remove(&editscript[begin..i]),
                        '=' => {
                            let ss = &editscript[begin..i];
                            if ss.chars().nth(0) == Some('#') && ss[1..].parse::<usize>().is_ok() {
                                EditInstruction::KeepLength(ss[1..].parse::<usize>().unwrap())
                            } else {
                                EditInstruction::Keep(ss)
                            }
                        },
                        _ => return Err(DeepFrogError::OtherError("Parsing editscript failed (invalid mode), should not happen!".to_string()))
                    });
                    //reset
                    begin = 0;
                    mode = ' ';
                }
                if c == '[' {
                    begin = i+1;
                }
            }
            if c == '+' || c == '-' || c == '=' {
                mode = c;
            }
        }
        Ok(EditScript {
            instructions: instructions
        })
    }
}

///applies the edit rule (as generated by sesdiff) to convert a word from into its lemma
pub fn compute_lemma(word: &str, editscript: &str) -> Result<String,DeepFrogError> {
    if editscript == "0" || editscript.is_empty() {
        //no change
        return Ok(word.to_string());
    }
    let editscript = EditScript::from_str(editscript)?;
    let mut head: String = word.clone().to_string();
    let mut tail = String::new();

    for instr in editscript.instructions.iter() {
        let headchars = head.chars().count();
        match instr {
            EditInstruction::Remove(suffix) => {
                let suffixchars = suffix.chars().count();
                if suffixchars > headchars {
                    return Err(DeepFrogError::OtherError(format!("Rule does not match current word, suffix is longer than head (unable to remove suffix {})", suffix)));
                }
                let foundsuffix: String = head.chars().skip(headchars - suffixchars).take(suffixchars).collect();
                if foundsuffix.as_str() != *suffix {
                    return Err(DeepFrogError::OtherError(format!("Rule does not match current word (unable to find and remove suffix {})", suffix)));
                }
                head = head.chars().take(headchars - suffixchars).collect();
            },
            EditInstruction::Add(s) => {
                head += s;
            },
            EditInstruction::KeepLength(keeplength) => {
                if *keeplength > headchars {
                    return Err(DeepFrogError::OtherError(format!("Rule does not match current word, length to keep is longer than head")));
                }
                tail = head.chars().skip(headchars - *keeplength).take(*keeplength).collect::<String>() + tail.as_str();
                head = head.chars().take(headchars - *keeplength).collect();
            },
            EditInstruction::Keep(suffix) => {
                let suffixchars = suffix.chars().count();
                if suffixchars > headchars {
                    return Err(DeepFrogError::OtherError(format!("Rule does not match current word, suffix is longer than head (unable to keep suffix {})", suffix)));
                }
                let foundsuffix: String = head.chars().skip(headchars - suffixchars).take(suffixchars).collect();
                if foundsuffix.as_str() != *suffix {
                    return Err(DeepFrogError::OtherError(format!("Rule does not match current word (unable to find and keep suffix {})", suffix)));
                }
                tail = head.chars().skip(headchars - suffixchars).take(suffixchars).collect::<String>() + tail.as_str();
                head = head.chars().take(headchars - suffixchars).collect();
            },
        }

    }
    head += tail.as_str();
    Ok(head)
}


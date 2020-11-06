use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Line {
    pub label: Option<String>,
    pub instruction: Option<String>,
    pub comment: Option<String>,
}

impl Default for Line {
    fn default() -> Self {
        Line {
            label: None,
            instruction: None,
            comment: None,
        }
    }
}
impl From<&str> for Line {
    fn from(line: &str) -> Self {
        let r = Regex::new(
            r"^\s*((?P<label>[^:;]*?)\s*:)?\s*(?P<instruction>[^+-;][^;]+?)?\s*(;(?P<comment>.+))?$",
        )
        .unwrap();
        let c = r.captures(line).unwrap();
        Self {
            label: c.name("label").map(|x| x.as_str().into()),
            instruction: c.name("instruction").map(|x| x.as_str().into()),
            comment: c.name("comment").map(|x| x.as_str().into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;

    macro_rules! testfrom {
        ($input:expr, $($a:ident : $b:expr),*) => {
            assert_eq!(Line::from($input),
            Line {
                $( $a: Some($b.into()), )* ..Default::default()
            });
        };
    }

    #[test]
    fn test_creation_from_strings() {
        testfrom!("",);
        testfrom!(": ", label:"");
        testfrom!(":  instr", label:"", instruction:"instr");
        testfrom!("label:", label:"label");
        testfrom!("op", instruction:"op");
        testfrom!(";comm", comment:"comm");
        testfrom!("next: lda ($24),x ; load from the table",
            label:"next", 
            instruction:"lda ($24),x", 
            comment:" load from the table");
        testfrom!("beq :+", instruction:"beq :+");
    }

    #[test]
    fn test_empty_space_removal() {
        testfrom!("  a:", label:"a");
        testfrom!("  a  :", label:"a");
        testfrom!("a  :", label:"a");
        testfrom!("a:", label:"a");
        testfrom!(":", label:"");
        testfrom!(";  co", comment:"  co");
        testfrom!(";co", comment:"co");
        //testfrom!(";co  ", comment:"co");
    }
}

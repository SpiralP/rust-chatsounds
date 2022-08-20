use nom::{branch::alt, bytes::complete::tag, IResult};

use super::{parse_args, ModifierTrait};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SelectModifier {
    /// 0-indexed
    pub select: u32,
}

impl ModifierTrait for SelectModifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        // 1-indexed
        // input = "select(2)"
        let (input, _) = alt((tag("select"), tag("choose")))(input)?;
        let (input, args) = parse_args(input)?;

        let mut modifier = SelectModifier::default();

        if let Some(select) = args.get(0).copied().unwrap_or(None) {
            modifier.select = (select.min(u32::MAX as f32) as u32).max(1) - 1;
        }

        Ok((input, modifier))
    }
}

#[test]
fn test_select_modifier() {
    assert_eq!(
        SelectModifier::parse("select(2)").unwrap().1,
        SelectModifier { select: 1 }
    );

    assert_eq!(
        SelectModifier::parse("select(0)").unwrap().1,
        SelectModifier { select: 0 }
    );

    assert_eq!(
        SelectModifier::parse("select(1)").unwrap().1,
        SelectModifier { select: 0 }
    );

    assert_eq!(
        SelectModifier::parse("select(4294967295)").unwrap().1,
        SelectModifier { select: 4294967294 }
    );

    assert_eq!(
        SelectModifier::parse("select(4294967296)").unwrap().1,
        SelectModifier { select: 4294967294 }
    );
}

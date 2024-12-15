use std::{cmp, ops::Range};

use day19::*;
use miette;

fn main() -> miette::Result<()> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../advent_of_code_input/2023/",
        env!("CARGO_PKG_NAME"),
        "/input.txt"
    ));
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

#[derive(Debug, Clone)]
struct RangePart {
    x: Range<i64>,
    m: Range<i64>,
    a: Range<i64>,
    s: Range<i64>,
}

fn process(input: &str) -> Result<i64, AocError> {
    let (workflows, _) = parse(input)?;

    let mut pending = Vec::new();
    pending.push((
        WorkflowRuleTarget::Next("in"),
        RangePart {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        },
    ));

    let mut count = 0;
    while !pending.is_empty() {
        let (workflow_target, mut range_part) = pending.pop().unwrap();

        let rule_name = match workflow_target {
            WorkflowRuleTarget::Accepted => {
                // println!("Match: {range_part:?}");
                count += (range_part.x.end - range_part.x.start)
                    * (range_part.m.end - range_part.m.start)
                    * (range_part.a.end - range_part.a.start)
                    * (range_part.s.end - range_part.s.start);
                continue;
            }
            WorkflowRuleTarget::Rejected => continue,
            WorkflowRuleTarget::Next(name) => name,
        };

        let workflow = workflows.get(rule_name).unwrap();

        workflow.iter().try_for_each(|rule| match rule.cmp {
            None => {
                pending.push((rule.target, range_part.clone()));
                None
            }
            Some(cmp) => {
                // Prepare the split
                let mut pass_part = range_part.clone();
                let mut fail_part = range_part.clone();

                // Get the address of the ranges we'll need to modify
                let (pass_range, fail_range) = match cmp.field {
                    WorkflowRuleField::X => (&mut pass_part.x, &mut fail_part.x),
                    WorkflowRuleField::M => (&mut pass_part.m, &mut fail_part.m),
                    WorkflowRuleField::A => (&mut pass_part.a, &mut fail_part.a),
                    WorkflowRuleField::S => (&mut pass_part.s, &mut fail_part.s),
                };

                // Split the range
                (*pass_range, *fail_range) = split_range(&pass_range, cmp.cmp, cmp.value);

                if !pass_range.is_empty() {
                    // push the successful part to be processed with the new
                    // target workflow
                    pending.push((rule.target, pass_part))
                }
                if fail_range.is_empty() {
                    // The whole range passed, there is no need to process the
                    // next rules
                    None
                } else {
                    // Process the fail part with the next rule
                    range_part = fail_part;
                    Some(())
                }
            }
        });
    }

    Ok(count)
}

// Split the range into two parts: `.0` is the range that pass the condition
// and `.1` is the one that fails it
fn split_range(
    range: &Range<i64>,
    ordering: cmp::Ordering,
    value: i64,
) -> (Range<i64>, Range<i64>) {
    if ordering == cmp::Ordering::Less {
        ((range.start)..value, value..range.end)
    } else {
        ((value + 1)..(range.end), (range.start)..(value + 1))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() -> miette::Result<()> {
        let input = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
        assert_eq!(process(input).unwrap(), 167409079868000);

        Ok(())
    }
}

use day19::*;
use miette;

fn main() -> miette::Result<()> {
    let input = include_str!("input.txt");
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str) -> Result<i64, AocError> {
    let (workflows, parts) = parse(input)?;

    let output = parts
        .into_iter()
        .filter_map(|part| {
            let mut next_workflow = "in";
            loop {
                let workflow = workflows.get(next_workflow).unwrap();
                let target = workflow
                    .iter()
                    .find_map(|rule| match rule.cmp {
                        None => Some(rule.target),
                        Some(cmp) => match cmp.field {
                            WorkflowRuleField::X => {
                                (part.x.cmp(&cmp.value) == cmp.cmp).then(|| rule.target)
                            }
                            WorkflowRuleField::M => {
                                (part.m.cmp(&cmp.value) == cmp.cmp).then(|| rule.target)
                            }
                            WorkflowRuleField::A => {
                                (part.a.cmp(&cmp.value) == cmp.cmp).then(|| rule.target)
                            }
                            WorkflowRuleField::S => {
                                (part.s.cmp(&cmp.value) == cmp.cmp).then(|| rule.target)
                            }
                        },
                    })
                    .unwrap();
                match target {
                    WorkflowRuleTarget::Accepted => return Some(part.x + part.m + part.a + part.s),
                    WorkflowRuleTarget::Rejected => return None,
                    WorkflowRuleTarget::Next(next) => next_workflow = next,
                }
            }
        })
        .sum();

    Ok(output)
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
        assert_eq!(process(input).unwrap(), 19114);

        Ok(())
    }
}

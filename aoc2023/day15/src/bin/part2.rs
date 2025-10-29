use std::collections::BTreeMap;

use day15::*;

fn main() -> miette::Result<()> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../input/2023/",
        env!("CARGO_PKG_NAME"),
        "/input.txt"
    ));
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal: usize,
}

fn process(input: &str) -> Result<usize, AocError> {
    let steps = parse(input)?;

    let mut boxes = BTreeMap::<usize, Vec<Lens>>::new();
    steps.into_iter().for_each(|step| {
        let lens_list = boxes.entry(step.hash).or_default();
        let existing_lens = lens_list.iter().position(|lens| lens.label == step.label);
        match step.op {
            Operation::Add(focal) => {
                match existing_lens {
                    Some(index) => {
                        if let Some(lens) = lens_list.get_mut(index) {
                            *lens = Lens {
                                label: step.label.clone(),
                                focal,
                            };
                        }
                    }
                    None => {
                        lens_list.push(Lens {
                            label: step.label.clone(),
                            focal,
                        });
                    }
                };
            }
            Operation::Remove => {
                if let Some(index) = existing_lens {
                    lens_list.remove(index);
                }
            }
        };
    });

    let output = boxes
        .into_iter()
        .map(|(box_no, lens_list)| {
            lens_list
                .into_iter()
                .enumerate()
                .map(|(lens_no, lens)| (box_no + 1) * (lens_no + 1) * lens.focal)
                .sum::<usize>()
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
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(process(input).unwrap(), 145);

        Ok(())
    }

    #[test]
    fn test_box_no() -> miette::Result<()> {
        assert_eq!(aoc_hash("rn"), 0);
        assert_eq!(aoc_hash("qp"), 1);
        assert_eq!(aoc_hash("cm"), 0);
        assert_eq!(aoc_hash("pc"), 3);
        assert_eq!(aoc_hash("ot"), 3);
        assert_eq!(aoc_hash("ab"), 3);

        Ok(())
    }
}

use common::error::AdventError;
use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<usize, AdventError> {
    let (presents, regions) = parse(content)?;

    let result = regions
        .into_iter()
        .filter(|region| {
            // We expect a trick to simplify, to avoid the very hard problem
            // of packing:
            // - either a region large enough that it can accept the presents
            // in a regular grid
            // - or region's area is smaller than the total area of the presents,
            // i.e it's too small even for a perfect packing (packing without
            // any gap anywhere)
            let grid_area = (region.0.0 / 3) * (region.0.1 / 3);
            let present_count = region.1.iter().sum::<usize>();
            if grid_area >= present_count {
                // The region is big enough for grid packing
                // So it's definitely possible to fit the present
                return true;
            }

            let region_area = region.0.0 * region.0.1;
            let presents_area = region
                .1
                .iter()
                .enumerate()
                .map(|(idx, count)| presents[idx] * count)
                .sum::<usize>();
            if region_area < presents_area {
                // The region is too small for the number of presents
                // So it's impossible to fit the present
                return false;
            }

            // There might be a solution. If this triggers, we either need
            // better heuristics than the above, or we actually need to
            // implement packing.
            panic!("no easy answer")
        })
        .count();

    Ok(result)
}

#[cfg(test)]
mod test {

    // cspell:disable
    const _INPUT_SAMPLE: &[u8] = br#"
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;
    // cspell:enable

    // No test because the sample contains "maybe" regions while our input
    // contains only "definitely-not" and "easily-yes" regions.
    // So the sample would require an actual packing algorithm but our input
    // does not
}

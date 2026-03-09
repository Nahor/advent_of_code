use std::collections::HashMap;

use day25::*;

use petgraph::graph::UnGraph;
use rustworkx_core::connectivity::stoer_wagner_min_cut;

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

fn process(input: &'_ str) -> Result<usize, AocError<'_>> {
    let component_map = parse(input)?;

    let mut graph = UnGraph::new_undirected();
    let mut node_map = HashMap::new();
    component_map.iter().for_each(|(src, vec)| {
        let src_idx = *node_map.entry(src).or_insert_with(|| graph.add_node(src));
        vec.iter().for_each(|dst| {
            let dst_idx = *node_map.entry(dst).or_insert_with(|| graph.add_node(dst));
            graph.add_edge(src_idx, dst_idx, 1);
        });
    });

    let min_cut_res = stoer_wagner_min_cut(&graph, |_| Ok::<i32, AocError>(1));
    let (cuts, partition) = min_cut_res.unwrap().unwrap();
    println!("cuts: {cuts}");
    println!("count: {}", partition.len());
    println!("remain: {}", node_map.len() - partition.len());

    // {
    //     let mut file = File::create("base.dot").unwrap();
    //     let _ = write!(file, "{:?}", Dot::new(&graph));
    // }

    // let condensed = condensation(graph, false);
    // {
    //     let mut file = File::create("condensed.dot").unwrap();
    //     let _ = write!(file, "{:?}", Dot::new(&condensed));
    // }

    let output = partition.len() * (node_map.len() - partition.len());
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() -> miette::Result<()> {
        let input = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";
        assert_eq!(process(input)?, 54);

        Ok(())
    }
}

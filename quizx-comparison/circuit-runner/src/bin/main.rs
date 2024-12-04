use circuit_runner::converter::convert_poc_graph_to_quizx;
use quizx::decompose_tri::*;
use quizx::graph::*;
use quizx::hash_graph::*;
use std::env;
use std::path::Path;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Missing file path");
    let file_path = Path::new(file_path);

    let dot_string = std::fs::read_to_string(file_path).expect("Failed to read file");
    let dot_string = dot_string.replace("\r", "");
    // println!("received string:\n{}", dot_string);

    let mut poc_graph: Graph = GraphLike::from_dot(&dot_string);

    println!("Graph loaded!");

    let mut quizx_graph = convert_poc_graph_to_quizx(&mut poc_graph);

    let mut outputs: Vec<V> = vec![];
    for v in quizx_graph.clone().vertices() {
        if quizx_graph.vertex_type(v) == VType::B {
            outputs.push(v);
        }
    }
    quizx_graph.set_outputs(outputs);

    println!(
        "----------------------------------------------------------------------------------------"
    );
    // graphviz-friendly string representation of the graph
    // println!("string of final diagram:\n{}", quizx_graph.to_dot());

    let time = Instant::now();

    quizx::simplify::clifford_simp(&mut quizx_graph);
    let mut d = TriDecomposer::new();
    d.add_decomp(Decompositions::ZSpider(ZSpiderDecomp::new()));
    d.add_decomp(Decompositions::Tri(TriDecomp::new(3)));

    let (_returned_statevec, returned_num_terms) = d.decomp_sequential_statevec(quizx_graph);

    // println!("Statevec: {:#?}", returned_statevec);

    println!("Number of terms in decomposition: {}", returned_num_terms);

    println!("Runtime: {}", time.elapsed().as_secs_f64());
}

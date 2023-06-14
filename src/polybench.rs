use dace::ast::Node;
use std::rc::Rc;



fn trmm_trace(M: usize, N:usize) -> Rc<Node> {

    let i_loop_ref = Node::new_single_loop("i", 0, M as i32);
    let j_loop_ref = Node::new_single_loop("j", 0, N as i32);
    let k_loop_ref = Node::new_single_loop("k", Node::get_lb(&i_loop_ref).unwrap() + 1, M as i32);

    // B[i * N + j] += A[k * M + i] * B[k * N + j];
    let a_ref = Node::new_ref("A", vec![N, M], |ijk| {
        vec![ijk[2] as usize, ijk[0] as usize]
    });
    let b1_ref = Node::new_ref("B", vec![M, N], |ijk| {
        vec![ijk[2] as usize, ijk[1] as usize]
    });
    let b2_ref = Node::new_ref("B", vec![M, N], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize]
    });

    Node::extend_loop_body(&k_loop_ref, &a_ref);
    Node::extend_loop_body(&k_loop_ref, &b1_ref);
    Node::extend_loop_body(&k_loop_ref, &b2_ref);

    // B[i * N + j] = alpha * B[i * N + j];
    let b3_ref = Node::new_ref("B", vec![M, N], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize]
    });
    Node::extend_loop_body(&j_loop_ref, &b3_ref);
    Node::extend_loop_body(&j_loop_ref, &k_loop_ref);

    Node::extend_loop_body(&i_loop_ref, &j_loop_ref);

    i_loop_ref
}

#[cfg(test)]
mod tests {
    use super::*;
    fn trmm_trace_test() {
        let M = 1024;
        let N = 1024;

        let ast = trmm_trace(M, N);
        assert_eq!(ast.node_count(), 7);
    }
}

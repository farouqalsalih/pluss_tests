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

pub fn heat_3d(m: usize, n: usize) -> Rc<Node> {
    // n: usize is array dim
    let ubound = n as i32; // loop bound
    let tsteps = m as i32; // steps bound
    
    let t_loop_ref = Node::new_single_loop("t", 0, tsteps);
    let i_loop_ref_1 = Node::new_single_loop("i_1", 0, ubound);
    let j_loop_ref_1 = Node::new_single_loop("j_1", 0, ubound);
    let k_loop_ref_1 = Node::new_single_loop("k_1", 0, ubound);


    let s_ref_a_1 = Node::new_ref("A1", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_a_2 = Node::new_ref("A2", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_a_3 = Node::new_ref("A3", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_a_4 = Node::new_ref("A4", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_a_5 = Node::new_ref("A5", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_a_6 = Node::new_ref("A6", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_a_7 = Node::new_ref("A7", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_a_8 = Node::new_ref("A8", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_a_9 = Node::new_ref("A9", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_a_10 = Node::new_ref("A10", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });

    let s_ref_b = Node::new_ref("B", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });

    Node::extend_loop_body(&k_loop_ref_1, &s_ref_a_1);
    Node::extend_loop_body(&k_loop_ref_1, &s_ref_a_2);
    Node::extend_loop_body(&k_loop_ref_1, &s_ref_a_3);
    Node::extend_loop_body(&k_loop_ref_1, &s_ref_a_4);
    Node::extend_loop_body(&k_loop_ref_1, &s_ref_a_5);
    Node::extend_loop_body(&k_loop_ref_1, &s_ref_a_6);
    Node::extend_loop_body(&k_loop_ref_1, &s_ref_a_7);
    Node::extend_loop_body(&k_loop_ref_1, &s_ref_a_8);
    Node::extend_loop_body(&k_loop_ref_1, &s_ref_a_9);
    Node::extend_loop_body(&k_loop_ref_1, &s_ref_a_10);
    Node::extend_loop_body(&k_loop_ref_1, &s_ref_b);

    Node::extend_loop_body(&j_loop_ref_1, &k_loop_ref_1);
    Node::extend_loop_body(&i_loop_ref_1, &j_loop_ref_1);


    let i_loop_ref_2 = Node::new_single_loop("i_2", 0, ubound);
    let j_loop_ref_2 = Node::new_single_loop("j_2", 0, ubound);
    let k_loop_ref_2 = Node::new_single_loop("k_2", 0, ubound);


    let s_ref_b_1 = Node::new_ref("B1", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_b_2 = Node::new_ref("B1", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_b_3 = Node::new_ref("B1", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_b_4 = Node::new_ref("B1", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_b_5 = Node::new_ref("B1", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_b_6 = Node::new_ref("B1", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_b_7 = Node::new_ref("B1", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_b_8 = Node::new_ref("B1", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_b_9 = Node::new_ref("B1", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_b_10 = Node::new_ref("B1", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });
    let s_ref_a = Node::new_ref("A", vec![n, n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize, ijk[2] as usize]
    });


    Node::extend_loop_body(&k_loop_ref_2, &s_ref_b_1);
    Node::extend_loop_body(&k_loop_ref_2, &s_ref_b_2);
    Node::extend_loop_body(&k_loop_ref_2, &s_ref_b_3);
    Node::extend_loop_body(&k_loop_ref_2, &s_ref_b_4);
    Node::extend_loop_body(&k_loop_ref_2, &s_ref_b_5);
    Node::extend_loop_body(&k_loop_ref_2, &s_ref_b_6);
    Node::extend_loop_body(&k_loop_ref_2, &s_ref_b_7);
    Node::extend_loop_body(&k_loop_ref_2, &s_ref_b_8);
    Node::extend_loop_body(&k_loop_ref_2, &s_ref_b_9);
    Node::extend_loop_body(&k_loop_ref_2, &s_ref_b_10);
    Node::extend_loop_body(&k_loop_ref_2, &s_ref_a);

    Node::extend_loop_body(&j_loop_ref_2, &k_loop_ref_2);
    Node::extend_loop_body(&i_loop_ref_2, &j_loop_ref_2);


    Node::extend_loop_body(&t_loop_ref, &i_loop_ref_1);
    Node::extend_loop_body(&t_loop_ref, &i_loop_ref_2);

    t_loop_ref
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

    #[test]
    fn heat_3d_test() {
        let mm = heat_3d(5, 100);
        assert_eq!(mm.node_count(), 29);
    }
}

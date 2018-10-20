
fn eliminate_h(dom_view: &mut [u8], i: usize) {
    let row = i * 9;
    for offset in 0..9 {
        dom_view[row+offset] = 0;
    }
}

fn eliminate_v(dom_view: &mut [u8], j: usize) {
    let col = j;
    for offset in (0..9).map(|x| {x*9}).into_iter() {
        dom_view[col+offset] = 0;
    }
}

fn eliminate_b(dom_view: &mut [u8], i: usize, j: usize) {
    let blk = i / 3 * 3 * 9 + j / 3 * 3;
    for &offset in [0, 1, 2, 9, 10, 11, 18, 19, 20].iter() {
        dom_view[blk+offset] = 0;
    }
}

fn count_possibilities(dom: &[u8], out: &mut u8, i: usize, j: usize) {
    let mut s = 0u8;
    let pt = i * 9 + j;
    for offset in (0..9).map(|x| {x*81}).into_iter() {
        s += dom[pt+offset];
    }
    *out = s;
}

fn debug_view(v: &[u8]) {
    for i in 0..9 {
        let r = &v[i*9..(i+1)*9];
        println!("{} {} {} {} {} {} {} {} {}", r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], r[8]);
    }
}

fn main() {
    let mut x = vec![
        0u8, 0, 7, 0, 0, 0, 3, 0, 2,
        2, 0, 0, 0, 0, 5, 0, 1, 0,
        0, 0, 0, 8, 0, 1, 4, 0, 0,
        0, 1, 0, 0, 9, 6, 0, 0, 8,
        7, 6, 0, 0, 0, 0, 0, 4, 9,
        0, 0, 0, 0, 0, 0 ,0, 0, 0,
        0, 0, 0, 1, 0, 3, 0, 0, 0,
        8, 0, 1, 0, 6, 0, 0, 0, 0,
        0, 0, 0 ,7, 0, 0, 0 ,6, 3
    ];

    let mut dom = vec![1u8; 9*9*9];

    for i in 0..9 {
        for j in 0..9 {
            let v = x[i*9+j];
            if v>0 {
                let idx = (v - 1) as usize;
                let view = &mut dom[idx*81..(idx + 1)*81];
                eliminate_h(view, i);
                eliminate_v(view, j);
                eliminate_b(view, i, j);
            }
        }
    }

    let mut P = vec![0; 81];

    for i in 0..9 {
        for j in 0..9 {
            count_possibilities(&dom, &mut P[i*9+j], i, j);
        }
    }

    // let k = 1-1;
    // debug_view(&dom[(k*81)..((k+1)*81)]);

    debug_view(&P);

}

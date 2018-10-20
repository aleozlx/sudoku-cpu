
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

fn dom_get(dom: &[u8], i: usize, j: usize) -> u8 {
    let pt = i * 9 + j;
    for v in 0..9 {
        if dom[pt+v*81] > 0 {
            return (v + 1) as u8;
        }
    }
    unreachable!();
}

fn count_h(dom_view: &[u8], out: &mut u8, i: usize) {
    let mut s = 0u8;
    let row = i * 9;
    for offset in 0..9 {
        s += dom_view[row+offset];
    }
    *out = s;
}

fn dom_get_h(dom_view: &[u8], i: usize) -> usize {
    let row = i * 9;
    for j in 0..9 {
        if dom_view[row+j] > 0 {
            return j;
        }
    }
    unreachable!();
}

fn count_v(dom_view: &[u8], out: &mut u8, j: usize) {
    let mut s = 0u8;
    let col = j;
    for offset in (0..9).map(|x| {x*9}).into_iter() {
        s += dom_view[col+offset];
    }
    *out = s;
}

fn dom_get_v(dom_view: &[u8], j: usize) -> usize {
    let col = j;
    for i in 0..9 {
        if dom_view[i*9+col] > 0 {
            return i;
        }
    }
    unreachable!();
}

fn count_b(dom_view: &[u8], out: &mut u8, blk: usize) {
    let mut s = 0u8;
    for &offset in [0, 1, 2, 9, 10, 11, 18, 19, 20].iter() {
        s += dom_view[blk+offset];
    }
    *out = s;
}

fn dom_get_b(dom_view: &[u8], blk: usize) -> (usize, usize) {
    for &offset in [0, 1, 2, 9, 10, 11, 18, 19, 20].iter() {
        let ptr = blk+offset;
        if dom_view[ptr] > 0 {
            return (ptr / 9, ptr % 9);
        }
    }
    unreachable!();
}

fn dom_zero(dom: &mut [u8], i: usize, j: usize) {
    let pt = i * 9 + j;
    for offset in (0..9).map(|x| {x*81}).into_iter() {
        dom[pt+offset] = 0;
    }
}

fn debug_view(v: &[u8]) {
    println!("=========");
    for i in 0..9 {
        let r = &v[i*9..(i+1)*9];
        println!("{} {} {} {} {} {} {} {} {}", r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], r[8]);
    }
    println!("=========");
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

    let n: usize = x.iter().map(|i| if *i==0 {1} else {0}).sum();

    let mut dom = vec![1u8; 9*9*9];

    for _k in 0..n {
        for i in 0..9 {
            for j in 0..9 {
                let v = x[i*9+j];
                if v>0 {
                    dom_zero(&mut dom, i, j);
                    let idx = (v - 1) as usize;
                    let view = &mut dom[idx*81..(idx + 1)*81];
                    eliminate_h(view, i);
                    eliminate_v(view, j);
                    eliminate_b(view, i, j);
                }
            }
        }

        let mut mat_p = vec![0; 81];

        for i in 0..9 {
            for j in 0..9 {
                let out = &mut mat_p[i*9+j];
                count_possibilities(&dom, out, i, j);

                if *out == 1 {
                    let v = dom_get(&dom, i, j);
                    println!("[{}, {}] ← {} (constraint)", i+1, j+1, v);
                    x[i*9+j] = v;
                }
            }
        }

        for idx in 0..9 {
            for i in 0..9 {
                let out = &mut mat_p[idx*9+i];
                let view = &dom[idx*81..(idx + 1)*81];
                count_h(view, out, i);

                if *out == 1 {
                    let j = dom_get_h(view, i);
                    let v = (idx + 1) as u8;
                    println!("[{}, {}] ← {} (h scan)", i+1, j+1, v);
                    x[i*9+j] = v;
                }
            }
        }

        for idx in 0..9 {
            for j in 0..9 {
                let out = &mut mat_p[idx*9+j];
                let view = &dom[idx*81..(idx + 1)*81];
                count_v(view, out, j);

                if *out == 1 {
                    let i = dom_get_v(view, j);
                    let v = (idx + 1) as u8;
                    println!("[{}, {}] ← {} (v scan)", i+1, j+1, v);
                    x[i*9+j] = v;
                }
            }
        }

        for idx in 0..9 {
            for (i_blk, &blk) in [0, 3, 6, 27, 30, 33, 54, 57, 60].iter().enumerate() {
                let out = &mut mat_p[idx*9+i_blk];
                let view = &dom[idx*81..(idx + 1)*81];
                count_b(view, out, blk);

                if *out == 1 {
                    let (i, j) = dom_get_b(view, blk);
                    let v = (idx + 1) as u8;
                    println!("[{}, {}] ← {} (blk)", i+1, j+1, v);
                    x[i*9+j] = v;
                }
            }
        }

        
    }

    // let k = 3-1;
    // debug_view(&dom[(k*81)..((k+1)*81)]);
    debug_view(&x);
        // debug_view(&mat_p);
    

}

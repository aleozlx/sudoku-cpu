
fn eliminate_h(dom_view: &mut [u8], idx: usize, i: usize, j: usize) {
    let row = i * 9;
    for offset in 0..9 {
        dom_view[row+offset] = 0;
    }
}

fn eliminate_v(dom_view: &mut [u8], idx: usize, i: usize, j: usize) {
    let col = j;
    for &offset in [0, 9, 18, 27, 36, 45, 54, 63, 72].iter() {
        dom_view[col+offset] = 0;
    }
}

fn eliminate_b(dom_view: &mut [u8], idx: usize, i: usize, j: usize) {
    let blk = i / 3 * 3 * 9 + j / 3 * 3;
    for &offset in [0, 1, 2, 9, 10, 11, 18, 19, 20].iter() {
        dom_view[blk+offset] = 0;
    }
}

fn debug_view(dom_view: &[u8]) {
    for i in 0..9 {
        let r = &dom_view[i*9..(i+1)*9];
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
                eliminate_h(view, idx, i, j);
                eliminate_v(view, idx, i, j);
                eliminate_b(view, idx, i, j);
            }
        }
    }

    debug_view(&dom[(1*81)..(2*81)]);

}

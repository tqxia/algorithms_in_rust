use rand::Rng;

pub fn quick_sort<T: PartialOrd>(v: &mut Vec<T>) -> &Vec<T> {
    shuffle(v);
    sort(v, 0, v.len()-1);
    v
}

fn shuffle<T>(v: &mut Vec<T>) {
    let mut gen = rand::thread_rng();
    for i in 0..v.len() {
        v.swap(i, gen.gen_range(0, i+1));
    }
}

fn sort<T: PartialOrd>(v: &mut Vec<T>, lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let mid = partition(v, lo, hi);

    if mid > 0 {
        sort(v, lo, mid-1);
    }
    sort(v, mid+1, hi);
}

// sort2 implements quick sort using 3-way partitioning 
// fn sort2<T: PartialOrd>(v: &mut Vec<T>, lo: usize, hi: usize) {
//     // TODO
// }

fn partition<T: PartialOrd>(v: &mut Vec<T>, lo: usize, hi: usize) -> usize {
    // let pivot = v[lo];
    let mut i = lo+1;
    let mut j = hi;

    loop {
        while i <= hi && v[i] < v[lo] {
            i += 1;
        }
        
        while j > lo && v[j] > v[lo] {
            j -= 1;
        }

        if i >= j {
            break;
        }

        v.swap(i, j);
        i += 1;
        j -= 1;
    }

    v.swap(lo, j);
    j
}

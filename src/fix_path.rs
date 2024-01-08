use i_float::fix_float::FIX_FRACTION_BITS;
use i_float::fix_vec::FixVec;

pub type FixPath = Vec<FixVec>;

pub trait FixPathExtension {
    fn area(&self) -> i64;
    fn unsafe_area(&self) -> i64;
    fn is_convex(&self) -> bool;
    fn contains(&self, point: FixVec) -> bool;
    fn remove_degenerates(&mut self);
    fn removed_degenerates(&self) -> FixPath;
}

impl FixPathExtension for FixPath {

    fn area(&self) -> i64 {
        self.unsafe_area() >> (FIX_FRACTION_BITS + 1)
    }

    fn unsafe_area(&self) -> i64 {
        let n = self.len();
        let mut p0 = self[n - 1];
        let mut area: i64 = 0;

        for p1 in self.iter() {
            area += p1.unsafe_cross_product(p0);
            p0 = *p1;
        }

        area
    }

    fn is_convex(&self) -> bool {
        let n = self.len();
        if n <= 2 {
            return true;
        }

        let p0 = self[n - 2];
        let mut p1 = self[n - 1];
        let mut e0 = p1 - p0;

        let mut sign: i64 = 0;
        for p in self.iter() {
            let p2 = *p;
            let e1 = p2 - p1;
            let cross = e1.unsafe_cross_product(e0).signum();
            if cross == 0 {
                let dot = e1.unsafe_dot_product(e0);
                if dot == -1 {
                    return false
                }
            } else {
                if sign == 0 {
                    sign = cross
                } else if sign != cross {
                    return false
                }
            }

            e0 = e1;
            p1 = p2;
        }

        true
    }
    
    fn contains(&self, point: FixVec) -> bool {
        let n = self.len();
        let mut is_contain = false;
        let mut b = self[n - 1];
        for i in 0..n {
            let a = self[i];
            let is_in_range = (a.y > point.y) != (b.y > point.y);
            if is_in_range {
                let dx = b.x - a.x;
                let dy = b.y - a.y;
                let sx = (point.y - a.y) * dx / dy + a.x;
                if point.x < sx {
                    is_contain = !is_contain;
                }
            }
            b = a;
        }
        
        is_contain
    }

    fn remove_degenerates(&mut self) {
        if self.len() < 3 {
            self.clear();
            return
        }
        
        if !has_degenerates(&self) {
            return
        }
        
        let clean = filter(&self);

        self.splice(.., clean);
    }

    fn removed_degenerates(&self) -> FixPath {
        if self.len() < 3 {
            return vec![FixVec::ZERO; 0]
        }
        
        if !has_degenerates(&self) {
            return self.clone()
        }
        
        filter(&self)
    }

}

fn has_degenerates(path: &FixPath) -> bool {
    let count = path.len();
    let mut p0 = path[count - 2];
    let p1 = path[count - 1];
    
    let mut v0 = p1 - p0;
    p0 = p1;
    
    for pi in path.iter() {
        let vi = *pi - p0;
        let prod = vi.unsafe_cross_product(v0);
        if prod == 0 {
            return true
        }
        v0 = vi;
        p0 = *pi;
    }

    return false
}

fn filter(path: &FixPath) -> FixPath {
    let mut n = path.len();
    let mut nodes: Vec<Node> = vec![Node { next: 0, index: 0, prev: 0 }; n];
    let mut validated: Vec<bool> = vec![false; n];
    
    let mut i0 = n - 2;
    let mut i1 = n - 1;
    for i2 in 0..n {
        nodes[i1] = Node{ next: i2, index: i1, prev: i0};
        i0 = i1;
        i1 = i2;
    }

    let mut first: usize = 0;
    let mut node = nodes[first];
    let mut i = 0;
    while i < n {
        if validated[node.index] {
            node = nodes[node.next];
            continue
        }
        
        let p0 = path[node.prev];
        let p1 = path[node.index];
        let p2 = path[node.next];

        if (p1 - p0).unsafe_cross_product(p2 - p1) == 0 {
            n -= 1;
            if n < 3 {
                return vec![FixVec::ZERO; 0]
            }

            // remove node
            nodes[node.prev].next = node.next;
            nodes[node.next].prev = node.prev;

            if node.index == first {
                first = node.next
            }

            node = nodes[node.prev];
            
            if validated[node.prev] {
                i -= 1;
                validated[node.prev] = false
            }
            
            if validated[node.next] {
                i -= 1;
                validated[node.next] = false
            }
            
            if validated[node.index] {
                i -= 1;
                validated[node.index] = false
            }
        } else {
            validated[node.index] = true;
            i += 1;
            node = nodes[node.next];
        }
    }
    
    let mut buffer = vec![FixVec::ZERO; n];
    node = nodes[first];
    for j in 0..n {
        buffer[j] = path[node.index];
        node = nodes[node.next];
    }

    return buffer
}


#[derive(Clone, Copy)]
struct Node {
    next: usize,
    index: usize,
    prev: usize
}

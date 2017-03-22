pub struct Heap<T> {
    vec: Vec<T>,
    pub size: usize
}

fn parent(i: usize) -> usize { (i-1)/2 }
fn left(i: usize) -> usize { 2*i + 1 }
fn right(i: usize) -> usize { 2*i + 2 }

impl<T: Ord + Copy> Heap<T> {
    pub fn new() -> Self {
        Heap::<T> {
            vec: vec!(),
            size: 0
        }
    }
    pub fn from_vec(vec: Vec<T>) -> Self {
        let size = vec.len();
        let mut heap = Heap::<T> {
            vec: vec,
            size: size
        };
        heap.build_max_heap();
        heap
    }

    pub fn get_vec(&self) -> Vec<T> { self.vec.clone() }

    pub fn insert(&mut self, elem: T) {
        self.size = self.size + 1;
        let size = self.size;
        self.vec[size - 1] = elem;
        self.increase_key(size - 1, elem);
    }

    pub fn increase_key(&mut self, i: usize, elem: T) {
        assert!(elem >= self.vec[i], "New value is smaller than current value");
        let mut i = i;
        self.vec[i] = elem;
        while i > 0 && self.vec[parent(i)] < self.vec[i] {
            let temp = self.vec[i];
            self.vec[i] = self.vec[parent(i)];
            self.vec[parent(i)] = temp;
            i = parent(i);
        }
    }

    pub fn remove_max(&mut self) -> T {
        self.size -= 1;
        let largest = self.vec[0];
        self.vec[0] = self.vec[self.size];
        self.vec[self.size] = largest;
        if self.size != 0 { self.max_heapify(0) }
        return largest;
    }

    fn isleaf(&self, i: usize) -> bool { i >= self.size/2 && i < self.size }

    fn max_heapify(&mut self, i: usize) {
        let l = left(i);
        let r = right(i);
        let mut largest = if l < self.size && self.vec[l] > self.vec[i] { l } else { i };
        if r < self.size && self.vec[r] > self.vec[largest] { largest = r; }
        if largest != i {
            let temp = self.vec[i];
            self.vec[i] = self.vec[largest];
            self.vec[largest] = temp;
            self.max_heapify(largest);
        }
    }


    fn build_max_heap(&mut self) {
        for i in (0..(self.size/2)).rev() {
            self.max_heapify(i);
        }
    }

}

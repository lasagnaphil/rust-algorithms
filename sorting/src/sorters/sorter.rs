use ds::heap::Heap;

pub trait Sorter {
    fn sort<T: Ord + Copy>(&self, vec: Vec<T>) -> Vec<T>;
}

pub struct HeapSort {}
impl Sorter for HeapSort {
    fn sort<T: Ord + Copy>(&self, vec: Vec<T>) -> Vec<T> {
        let mut heap = Heap::<T>::from_vec(vec);
        let sorted: Vec<T> = (0..heap.size).map(|x| heap.remove_max()).collect();
        sorted
    }
}

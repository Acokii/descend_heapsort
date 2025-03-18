pub mod heap {
    pub struct Heap {
        pub arr: Vec<i32>, // Pakai Vec biar fleksibel
        pub size: usize,
    }

    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    fn left(i: usize) -> usize {
        if i == 0 {
            return 1;
        }else {
            return 2 * i + 1;
        }
    }

    fn right(i: usize) -> usize {
        if i == 0 {
            return 2;
        }else {
           return 2 * i + 2;
        }
    }

    pub fn minHeapify(heap: &mut Heap, i: usize) {
        let l = left(i);
        let r = right(i);
        let mut largest: usize ;

        if l < heap.size && heap.arr[l] < heap.arr[i] {
            largest = l;
        }else {
            largest = i;
        }

        if r < heap.size && heap.arr[r] < heap.arr[largest] {
            largest = r;
        }

        if largest != i {
            heap.arr.swap(i, largest); // Langsung pakai swap bawaan Rust
            minHeapify(heap, largest);
        }
    }

    pub fn build_minheap(heap: &mut Heap){
        let length = heap.arr.len()/2;
        for i in (0..length).rev()  {
            minHeapify(heap, i);
        }
    }

    pub fn heap_sort(heap: &mut Heap){
        build_minheap(heap);
        for a in (1..heap.arr.len()).rev() {
            heap.arr.swap(0, a);
            heap.size -= 1;
            minHeapify(heap, 0);
        }
    }
}

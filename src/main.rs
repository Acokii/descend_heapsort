mod lib;

fn main() {
    let arr: Vec<i32> = vec![1,4,10,14,7,9,3,2,8,16];
    println!("Original array: {:?}",arr);
    let mut alen = arr.len();
    let mut heap : lib::heap::Heap = lib::heap::Heap{arr: arr, size: alen};
    lib::heap::heap_sort(&mut heap);
    println!("after sort: {:?}", heap.arr); 
}

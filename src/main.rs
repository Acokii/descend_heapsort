mod lib;

fn main() {
    let mut arr: Vec<i32> = Vec::new();
    arr.push(1);
    arr.push(4);
    arr.push(10);
    arr.push(14);
    arr.push(7);
    arr.push(9);
    arr.push(3);
    arr.push(2);
    arr.push(8);
    arr.push(16);
    println!("Original array: {:?}",arr);
    let mut alen = arr.len();
    let mut heap : lib::heap::Heap = lib::heap::Heap{arr: arr, size: alen};
    lib::heap::heap_sort(&mut heap);
    heap.arr.reverse();
    println!("after sort: {:?}", heap.arr); 
}

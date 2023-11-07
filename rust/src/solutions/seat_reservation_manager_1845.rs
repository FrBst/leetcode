use std::{collections::{HashMap, BinaryHeap}, cmp::Reverse};

struct SeatManager {
    free_seats: BinaryHeap<Reverse<i32>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {

    fn new(n: i32) -> Self {
        SeatManager {
            free_seats: {
                let mut heap = BinaryHeap::with_capacity(n as usize);
                for i in 1..n + 1 {
                    heap.push(Reverse(i));
                } 
                heap
            }
        }
    }
    
    fn reserve(&mut self) -> i32 {
        self.free_seats.pop().unwrap().0

    }
    
    fn unreserve(&mut self, seat_number: i32) {
        self.free_seats.push(Reverse(seat_number));
    }
}
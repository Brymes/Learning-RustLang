// OWNERSHIP
/* Ownership is a set of rules governing memory management in rust.
all data on the stack must have known fixed size. unknown and unfixed size must be pushed to the heap.
heap returns a pointer to the unorganized data's address in memory.
the pointer to the heap is fixed, and can be stored in the stack.
pushing to stack is faster than allocating to the heap.
accessing data in the heap is slower than in the stack.
OWNERSHIP RULES
1. each value in rust has an owner.
2. there can only be one owner at a time.
3. when the owner leaves the scope, the value is dropped.
*/
fn main() { 
    println!("Hello, world!");
}

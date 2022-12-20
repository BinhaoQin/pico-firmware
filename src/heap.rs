use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use defmt::error;

#[global_allocator]
static HEAP: CortexMHeap = CortexMHeap::empty();

pub const HEAP_SIZE: usize = 1024;

pub fn initialize_heap() {
    use core::mem::MaybeUninit;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

#[alloc_error_handler]
pub fn out_of_memory(_: Layout) -> ! {
    error!("system out of memory, halt!");
    loop {}
}

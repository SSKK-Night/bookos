struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

struct SimpleAllocator {
    head: ListNode,
}

fn align_addr(addr: usize, align: usize) -> usize {
    (addr + align - 1) / align * align
}

impl SimpleAllocator {
    unsafe fn add_new_node(&mut self, start_addr: usize, size: usize) {
        let end_addr = start_addr + usize;
        let aligned_addr = align_addr(start_addr, core::mem::align_of::<ListNode>());

        let size = end_addr - aligned_addr;
    
        if size < core::mem::size_of::<ListNode>() {
            return
        }

        let new_area_ptr = aligned_addr as *mut ListNode;
        (*new_area_ptr).size = size;
        (*new_area_ptr).next = self.head.next.take();

        self.head.next = Some(&mut *new_area_ptr);
    }
}

impl listNode {
    const fn new(size: unsize) -> Self {
        Self {
            size,
            next: None,
        }
    }
}

impl SimpleAllocator {
    const fn new() -> Self {
        Self {
            head: ListNode::new(0),
        }
    }
}
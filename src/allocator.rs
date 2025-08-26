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

impl SimpleAllocator {
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        self.add_new_node(ptr as usize, layout.size());
    }

    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();
        let mut current = &mut self.head;
        // 空きリストを先頭から見ていく
        while let Some(ref mut node) = current.next {
            let start_addr = node.start_addr();
            let aligned_addr = align_addr(start_addr, align);
            let end_addr = node.end_addr();
            let node_size = end_addr - align_addr;
            if node_size < size {
                current = current.next.as_mut().unwrap();
            } else {
                // 十分な大きさのノードを見つけたのでリストからノードを削除する
                let next = current.next.take();
                let result = aligned_addr as *mut u8;
                current.next = next.unwrap().next.take();
                // 余っている領域があるならば空きリスト領域に追加しなおす
                self.add_new_node(start_addr, align_addr - start_addr);
                self.add_new_node(aligned_addr + size, end_addr - (aligned_addr + size));

                return result;
            }
        }

        return core::ptr::null_mut();
    }
}

unsafe impl GlobalAlloc for Mutex<SimpleAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(ptr, layout);
    }
}
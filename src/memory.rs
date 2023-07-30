use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
    PhysAddr,
};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::structures::paging::{FrameAllocator, OffsetPageTable, PhysFrame, Size4KiB};

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges
            .flat_map(|r| r.step_by(4096));
        frame_addresses
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

// Returns a mutable reference to the active level 4 table.
unsafe fn active_level_4_table(
    physical_memory_offset: VirtAddr
) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

// pub unsafe fn translate_addr(
//     addr: VirtAddr,
//     physical_memory_offset: VirtAddr
// ) -> Option<PhysAddr> {
//     translate_addr_inner(addr, physical_memory_offset)
// }
//
// fn translate_addr_inner(
//     addr: VirtAddr,
//     physical_memory_offset: VirtAddr
// ) -> Option<PhysAddr> {
//     use x86_64::structures::paging::page_table::FrameError;
//     use x86_64::registers::control::Cr3;
//
//     let (level_4_table_frame, _) = Cr3::read();
//
//     let table_indexes = [
//         addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index(),
//     ];
//     let mut frame = level_4_table_frame;
//
//     for &index in &table_indexes {
//         let virt = physical_memory_offset + frame.start_address().as_u64();
//         let table_ptr: *const PageTable = virt.as_ptr();
//         let table = unsafe {&*table_ptr};
//
//         let entry = &table[index];
//         frame = match entry.frame() {
//             Ok(frame) => frame,
//             Err(FrameError::FrameNotPresent) => return None,
//             Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
//         }
//     }
//
//     Some(frame.start_address() + u64::from(addr.page_offset()))
// }
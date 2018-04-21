//! Interrupt description and set-up code.
/*
use ::segmentation::SegmentSelector;
use Ring;
use descriptor::*;
use paging::VAddr;

pub use ::irq::*;

/// An interrupt gate descriptor.
///
/// See Intel manual 3a for details, specifically section "6.14.1 64-Bit Mode
/// IDT" and "Figure 6-7. 64-Bit IDT Gate Descriptors".
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    /// Lower 16 bits of ISR.
    pub base_lo: u16,
    /// Segment selector.
    pub selector: SegmentSelector,
    /// This must always be zero.
    pub ist_index: u8,
    /// Flags.
    pub struct: Flags,
    /// The upper 48 bits of ISR (the last 16 bits must be zero).
    pub base_hi: u64,
    /// Must be zero.
    pub reserved1: u16,
}

pub enum Type {
    InterruptGate,
    TrapGate,
}

impl Type {
    pub fn pack(self) -> Flags {
        match self {
            Type::InterruptGate => FLAGS_TYPE_SYS_NATIVE_INTERRUPT_GATE,
            Type::TrapGate => FLAGS_TYPE_SYS_NATIVE_TRAP_GATE,
        }
    }
}

impl IdtEntry {
    /// A "missing" IdtEntry.
    ///
    /// If the CPU tries to invoke a missing interrupt, it will instead
    /// send a General Protection fault (13), with the interrupt number and
    /// some other data stored in the error code.
    pub const MISSING: IdtEntry = IdtEntry {
        base_lo: 0,
        selector: SegmentSelector::from_raw(0),
        ist_index: 0,
        flags: Flags::BLANK,
        base_hi: 0,
        reserved1: 0,
    };

    /// Create a new IdtEntry pointing at `handler`, which must be a function
    /// with interrupt calling conventions.  (This must be currently defined in
    /// assembly language.)  The `gdt_code_selector` value must be the offset of
    /// code segment entry in the GDT.
    ///
    /// The "Present" flag set, which is the most common case.  If you need
    /// something else, you can construct it manually.
    pub fn new(
        handler: VAddr,
        gdt_code_selector: SegmentSelector,
        dpl: Ring,
        ty: Type,
        ist_index: u8,
    ) -> IdtEntry {
        assert!(ist_index < 0b1000);
        IdtEntry {
            base_lo: ((handler.as_usize() as u64) & 0xFFFF) as u16,
            base_hi: handler.as_usize() as u64 >> 16,
            selector: gdt_code_selector,
            ist_index: ist_index,
            flags: Flags::from_priv(dpl) | ty.pack() | FLAGS_PRESENT,
            reserved1: 0,
        }
    }
}

*/
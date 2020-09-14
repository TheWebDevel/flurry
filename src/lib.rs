/// The largest possible table capacity.  This value must be
/// exactly 1<<30 to stay within Java array allocation and indexing
/// bounds for power of two table sizes, and is further required
/// because the top two bits of 32bit hash fields are used for
/// control purposes.
const MAXIMUM_CAPACITY: usize = 1 << 30;

/// The default initial table capacity.  Must be a power of 2
/// (i.e., at least 1) and at most `MAXIMUM_CAPACITY`.
const DEFAULT_CAPACITY: usize = 16;

/// The load factor for this table. Overrides of this value in
/// constructors affect only the initial table capacity.  The
/// actual floating point value isn't normally used -- it is
/// simpler to use expressions such as `n - (n >>> 2)` for
/// the associated resizing threshold.
const LOAD_FACTOR: f64 = 0.75;

/// Minimum number of rebinnings per transfer step. Ranges are
/// subdivided to allow multiple resizer threads.  This value
/// serves as a lower bound to avoid resizers encountering
/// excessive memory contention.  The value should be at least
/// `DEFAULT_CAPACITY`.
const MIN_TRANSFER_STRIDE: usize = 16;

/// The number of bits used for generation stamp in `size_ctl`.
/// Must be at least 6 for 32bit arrays.
const RESIZE_STAMP_BITS: usize = 16;

/// The maximum number of threads that can help resize.
/// Must fit in `32 - RESIZE_STAMP_BITS` bits.
const MAX_RESIZERS: usize = (1 << (32 - RESIZE_STAMP_BITS)) - 1;

/// The bit shift for recording size stamp in `size_ctl`.
const RESIZE_STAMP_SHIFT: usize = 32 - RESIZE_STAMP_BITS;

mod node;

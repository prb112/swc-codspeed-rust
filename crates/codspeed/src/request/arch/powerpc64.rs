use std::arch::asm;

pub type Value = u64;

#[inline(always)]
pub unsafe fn send_client_request(default: Value, args: &[Value; 6]) -> Value {
    let mut value = default;
    asm!(
        // PowerPC equivalent rotations
        // Using r12 for the equivalent of x12 in ARM
        "rotldi 12, 12, 61", // Rotate left by 61 bits (equivalent to right by 3)
        "rotldi 12, 12, 51", // Rotate left by 51 bits (equivalent to right by 13)
        "rotldi 12, 12, 13", // Rotate left by 13 bits (equivalent to right by 51)
        "rotldi 12, 12, 3",  // Rotate left by 3 bits (equivalent to right by 61)
        // PowerPC equivalent of orr instruction
        "or 10, 10, 10",
        in("r4") args.as_ptr(),
        inlateout("r3") value,
    );
    value
}

// Made with Bob

use core::arch::asm;
use std::ptr::write;

const STACK_SIZE: isize = 48; // 48 bytes

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
}

fn hello() {
    println!("Waking up a new stack!");
    loop {}
}

//rsp is the stack pointer
unsafe fn gt_switch(new: *const ThreadContext) {
    asm!(
    "mov rsp, [{0} + 0x00]",
    "ret",
    in(reg) &new
    );
}

fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = [0u8; STACK_SIZE as usize];
    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(STACK_SIZE);
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        write(sb_aligned.offset(-16) as *mut u64, hello as u64);
        ctx.rsp = sb_aligned as u64;
        gt_switch(&mut ctx);

        for i in 0..STACK_SIZE {
            println!(
                "mem: {}, val: {}",
                sb_aligned.offset(-i as isize) as usize,
                *sb_aligned.offset(-i as isize)
            )
        }
    }
}

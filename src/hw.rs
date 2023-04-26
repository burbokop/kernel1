
pub fn timer_tick() -> u64 {
    #[cfg(target_arch = "x86")]
    unsafe {
        core::arch::x86::_rdtsc()
    }

    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_rdtsc()
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        let mut ticks: u64;
        core::arch::asm!("mrs {}, cntvct_el0", out(reg) ticks);
        ticks
    }
}
/*
unsigned read_pit_count(void) {
	unsigned count = 0;
 
	// Disable interrupts
	cli();
 
	// al = channel in bits 6 and 7, remaining bits clear
	outb(0x43,0b0000000);
 
	count = inb(0x40);		// Low byte
	count |= inb(0x40)<<8;		// High byte
 
	return count;
}
 */
pub fn ttt() -> u64 {
    /* 
    unsafe {
        let mut aa: u8;
        core::arch::asm!(
            "cli",
            "out 0x43, 0b0000000",
            "in al,0x40", out("al") aa);
        return aa as u64;
    }
    */
    0
}

pub fn timer_freq() -> u64 {
    #[cfg(any(target_arch = "x86_64"))]
    unsafe {
        /*
        let start = timer_tick();
        nanosleep(1, 2);
        let end = timer_tick();
        end - start
        */
        let mut freq: u64 = 3 * 1000 * 1000 * 1000;

        /*
        core::arch::asm!("
            ;__get_speed__:
            ;first do a cpuid command, with eax=1
                mov  eax,1
                cpuid
                test edx,byte 0x10      ; test bit #4. Do we have TSC ?
                jz   detect_end         ; no ?, go to detect_end
                ;wait until the timer interrupt has been called.
                mov  ebx, ~[irq0_count]
 
            ;__wait_irq0__:
 
                cmp  ebx, ~[irq0_count]
                jz   wait_irq0
                rdtsc                   ; read time stamp counter
                mov  ~[tscLoDword], eax
                mov  ~[tscHiDword], edx
                add  ebx, 2             ; Set time delay value ticks.
                ; remember: so far ebx = ~[irq0]-1, so the next tick is
                ; two steps ahead of the current ebx ;)
 
            ;__wait_for_elapsed_ticks__:
 
                cmp  ebx, ~[irq0_count] ; Have we hit the delay?
                jnz  wait_for_elapsed_ticks
                rdtsc
                sub eax, ~[tscLoDword]  ; Calculate TSC
                sbb edx, ~[tscHiDword]
                ; f(total_ticks_per_Second) =  (1 / total_ticks_per_Second) * 1,000,000
                ; This adjusts for MHz.
                ; so for this: f(100) = (1/100) * 1,000,000 = 10000
                mov ebx, 10000
                div ebx
                ; ax contains measured speed in MHz
                mov {}, ax
        ", out(reg) freq);
        */
        freq
    };

    #[cfg(target_arch = "aarch64")]
    unsafe {
        let mut freq: u64;
        core::arch::asm!("mrs {}, cntfrq_el0", out(reg) freq);
        freq
    }

    3 * 1000 * 1000 * 1000
}

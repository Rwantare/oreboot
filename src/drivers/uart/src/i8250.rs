//use core::ops;
use model::*;
use register::mmio::{ReadOnly, ReadWrite};
use register::{register_bitfields, Field}; // How to use -> https://github.com/tock/tock/tree/master/libraries/tock-register-interface

#[repr(C)]
pub struct RegisterBlock {
    uart: ReadWrite<u8, UART::Register>,
}

pub struct I8250<'a> {
    base: usize,
    _baud: u32,
    d: &'a mut dyn Driver,
}

// it is possible that trying to make this work is a fool's errand but
// ... would be nice if the deref used the Driver in the 8250 ... dream on.
// impl ops::Deref for I8250 {
//     type Target = RegisterBlock;

//     fn deref(&self) -> &Self::Target {
//         unsafe { &*self.ptr() }
//     }
// }

impl<'a> I8250<'a> {
    // why is base a usize? for mmio 8250.
    pub fn new(base: usize, _baud: u32, d: &'a mut dyn Driver) -> I8250<'a> {
        I8250 { base: base, _baud: _baud, d: d }
    }

    /// Returns a pointer to the register block
    // fn ptr(&self) -> *const RegisterBlock {
    //     self.base as *const _
    // }

    /// Poll the status register until the specified field is set to the given value.
    /// Returns false iff it timed out.
    //    fn poll_status(&self, bit: Field<u8, LS::Register>, val: bool) -> bool {
    fn poll_status(&self, mask: u8, val: u8) -> bool {
        // Timeout after a few thousand cycles to prevent hanging forever.
        for _ in 0..100_000 {
            let mut s = [0; 1];
            self.d.pread(&mut s, self.base + 5).unwrap();
            if s[0] & mask == val {
                return true;
            }
        }
        return false;
    }
}

#[allow(dead_code)]
impl<'a> Driver for I8250<'a> {
    const IER: usize = 0x01; // Interrupt Enable Register            0b0001 RW
    const IIR: usize = 0x02; // Interrupt Identification Register    0b0010 R
    const FCR: usize = 0x02; // FIFO Control Register                0b0010 W
    const LCR: usize = 0x03; // Line Control Register                0b0011 RW
    const MCR: usize = 0x04; // Modem Control Register               0b0100 RW
    const MCR_DMA_EN: usize = 0x04;
    const MCR_TX_DFR: usize = 0x08;
    const DLL: usize = 0x00; // Divisor Latch Low Byte               0      RW
    const DLH: usize = 0x01; // Divisor Latch High Byte              0x0001 RW
    const LSR: usize = 0x05; // Line Status Register                 0x0101 R
    const MSR: usize = 0x06; // Modem Status Register                0x0110 R
    const SCR: usize = 0x07; // Scratch Register                     0x0111 RW
    const DLAB: u8 = 0x80; // Divisor Latch Access Bit             0x1000 RW

    // TODO: properly use the register crate.
    fn init(&mut self) -> Result<()> {
        const FIFOENABLE: u8 = 1;

        const EIGHTN1: u8 = 3; //?

        let mut s: [u8; 1] = [0u8; 1];
        self.d.pwrite(&s, self.base + IER).unwrap();
        //outb(0x0, base_port + UART8250_IER);

        /* Enable FIFOs */
        //outb(&s, base_port + fcr);
        s[0] = FIFOENABLE;
        self.d.pwrite(&s, self.base + FCR).unwrap();

        /* assert DTR and RTS so the other end is happy */
        // 3 wires don't care.
        //outb(UART8250_MCR_DTR | UART8250_MCR_RTS, base_port + UART8250_MCR);

        /* DLAB on */
        // so we can set baud rate.
        s[0] = DLAB | EIGHTN1;
        self.d.pwrite(&s, self.base + LCR).unwrap();

        /* Set Baud Rate Divisor. 12 ==> 9600 Baud */
        // 1 for 115200
        s[0] = 1;
        self.d.pwrite(&s, self.base + DLL).unwrap();
        s[0] = 0;
        self.d.pwrite(&s, self.base + DLH).unwrap();
        //outb(divisor & 0xFF,   base_port + UART8250_DLL);
        //outb((divisor >> 8) & 0xFF,    base_port + UART8250_DLM);

        /* Set to 3 for 8N1 */
        s[0] = EIGHTN1;
        self.d.pwrite(&s, self.base + LCR).unwrap();
        //        outb(CONFIG_TTYS0_LCS, base_port + UART8250_LCR);
        // /* disable all interrupts */
        // self.ie.set(0u8);
        // /* Enable dLAB */
        // self.lc.write(LC::DivisorLatchAccessBit::BaudRate);
        // // Until we know the clock rate the divisor values are kind of
        // // impossible to know. Throw in a phony value.
        // self.lc.write(LC::WLEN::WLEN_8);
        // // TOdO: what are these bits. how do we write them.
        // self.fc.set(0xc7);
        // self.mc.set(0x0b);
        // self.lc.write(LC::DivisorLatchAccessBit::Normal);
        Ok(())
    }

    fn pread(&self, data: &mut [u8], _offset: usize) -> Result<usize> {
        for c in data.iter_mut() {
            let mut s = [0u8; 1];
            while !self.poll_status(IER, 1) {}
            self.d.pread(&mut s, self.base).unwrap();
            *c = s[0];
        }
        Ok(data.len())
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        for (_i, &c) in data.iter().enumerate() {
            // Poll the status for long enough to let a char out; then push it out anyway.
            while !self.poll_status(0x20, 0x20) && !self.poll_status(0x40, 0x40) {}
            let mut s = [0u8; 1];
            s[0] = c;
            self.d.pwrite(&s, self.base).unwrap();
        }
        Ok(data.len())
    }

    fn shutdown(&mut self) {}
}

// // TODO: bitfields
register_bitfields! {
    u8,
    THR [ // Transmitter Holding Buffer
        //TODO
    ],
    RBR [ // Transmitter Receiver Buffer
        //TODO
    ],
    DLL [ //    Divisor Latch Bytes Low
        //TODO
    ],
    IER [ //    Interrupt Enable Register
        RECEIVED_DATA_AVAILABLE OFFSET(0) NUMBITS(1) [],
        THR_EMPTY               OFFSET(1) NUMBITS(1) [],
        RECEIVER_LINE_STATUS    OFFSET(2) NUMBITS(1) [],
        MODEM_STATUS            OFFSET(3) NUMBITS(1) [],
        SLEEP_MODE              OFFSET(4) NUMBITS(1) [],
        LOW_POWER_MODE          OFFSET(5) NUMBITS(1) [],
        RESERVED                OFFSET(6) NUMBITS(1) [],
    ],
    DLH [ //Divisor Latch Bytes High
        //TODO
    ],
    IIR [
        INTERRUPT_PENDING OFFSET(0) NUMBITS(1),
        GROUP_ONE OFFSET(1) NUMBITS(3) [
            ModemStatus = 0,
            THREmpty =1,
            ReceivedDataAvailable = 2,
            ReceiverLineStatus = 3,
            TimeoutInterruptPending = 6,
        ],
        
    ],
    FCR [
        //TODO
    ],
    LCR [
        //TODO
    ],
    MCR [
        //TODO
    ],
    LSR [
        //TODO
    ],
    MSR [
        //TODO
    ],
    SCR [
        //TODO
    ],
}

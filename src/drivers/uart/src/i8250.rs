use core::ops;
use core::mem::transmute;
use model::*;
use register::mmio::{ReadOnly, ReadWrite, WriteOnly};
use register::register_bitfields; // How to use -> https://github.com/tock/tock/tree/master/libraries/tock-register-interface

pub struct I8250<'a> {
    base: usize,
    _baud: u32,
    d: &'a mut dyn Driver,
    blocks: RegisterBlockSelect<'a>,
}

impl<'a> I8250<'a> {
    // why is base a usize? for mmio 8250.
    pub fn new(base: usize, _baud: u32, d: &'a mut dyn Driver) -> I8250<'a> {
        // Avert your eyes,
        // i8250 has overloaded registers at the same address depending on whether you're reading, writing, or DLAB flag is set
        // Therefore, the three registerblocks share a base address
        let blocks = unsafe { RegisterBlockSelect { main: transmute::<usize, &MainRegisterBlock>(base), aux: transmute::<usize, &AuxRegisterBlock>(base) } };
        I8250 { base, _baud, d, blocks }
    }

    /// Poll the status register until the specified field is set to the given value.
    /// Returns false iff it timed out.
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

impl<'a> ops::Deref for I8250<'a> {
  type Target = RegisterBlockSelect<'a>;
  fn deref(&self) -> &Self::Target {
    &self.blocks
  }
}

pub struct RegisterBlockSelect<'a> {
    main: &'a MainRegisterBlock,
    aux: &'a AuxRegisterBlock,
}

#[repr(C)]
pub struct MainRegisterBlock {
    rbr_thr: ReadWrite<u8, RBR::Register>,  /* Receiver Buffer & Transmitter Holding Buffer depending on read or write */
    ier: ReadWrite<u8, IER::Register>, /* Interrupt Enable Register */
    fcr: WriteOnly<u8, FCR::Register>, /* FIFO Control Register */
    lcr: ReadWrite<u8, LCR::Register>, /* Line Control Register */
    mcr: ReadWrite<u8, MCR::Register>, /* Modem Control Register */
    lsr: ReadOnly<u8, LSR::Register>,  /* Line Status Register */
    msr: ReadOnly<u8, MSR::Register>,  /* Modem Status Register */
    scr: ReadWrite<u8, SCR::Register>, /* Scratch Register */
}

#[repr(C)]
pub struct AuxRegisterBlock {
    dll: ReadWrite<u8, DLL::Register>, /* Divisor Latch Low Byte */
    dlh: ReadWrite<u8, DLH::Register>, /* Divisor Latch High Byte */
    iir: ReadOnly<u8, IIR::Register>,  /* Interrupt Identification Register */
}

/*
How to use
Normally:

some8250struct.regName.set(< your inputs here>);
              ^Deref coerces this method call to go to the Registerblock instead of the driver struct

The special way
some8250struct.regBlock.regName.modify(<your inputs here>)
              ^dref1   ^dref2
              deref1 redirects to a struct with 3 reg blocks
              deref2 gives you a register block
*/

#[allow(dead_code)]
impl<'a> Driver for I8250<'a> {
    // TODO: properly use the register crate.
    fn init(&mut self) -> Result<()> {
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
            while !self.poll_status(0x01, 1) {}
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

//Register bitfield syntax at: https://github.com/tock/tock/tree/master/libraries/tock-register-interface
// Bitfields are defined as:
// name OFFSET(shift) NUMBITS(num) [ /* optional values */ ]
register_bitfields! {
    u8,
    // THR [ //  Transmitter Holding Buffer   WRITE ONLY
    //   DATA OFFSET(0) NUMBITS(8) []
    // ],

    RBR [ //  Transmitter Receiver Buffer  READ ONLY but combined with THR in this driver
      DATA OFFSET(0) NUMBITS(8) []
    ],

    DLL[ //  Divisor Latch Bytes Low
      BYTES OFFSET(0) NUMBITS(8) []
    ],

    DLH [ //  Divisor Latch Bytes High
      BYTES OFFSET(0) NUMBITS(8) []
    ],

    IER [ //  Interrupt Enable Register
      RECEIVED_DATA_AVAILABLE 0,
      THR_EMPTY               1,
      RECEIVER_LINE_STATUS    2,
      MODEM_STATUS            3,
      SLEEP_MODE              4, // 16750 Only
      LOW_POWER_MODE          5 // 16750 Only
    ],
//  Interrupt Identification Register
    IIR [
        INTERRUPT_PENDING OFFSET(0) NUMBITS(1) [],
        GENERAL OFFSET(1) NUMBITS(3) [
            ModemStatus = 0,
            ThrEmpty = 1,
            ReceivedDataAvailable = 2,
            ReceiverLineStatus = 3, // Overflow on RBR
            TimeoutInterruptPending = 6
        ],
        // FIFO_ENABLED_16750 OFFSET(5) NUMBITS(1) [],
        FIFO OFFSET(6) NUMBITS(2) [
          NoFifo = 0,
          NotFunctioning = 2,
          Enabled = 3
        ]
    ],

    FCR [ // FIFO Control Register WRITE ONLY
      FIFO_ENABLE       OFFSET(0) NUMBITS(1) [],
      CLR_RECEIVE       OFFSET(1) NUMBITS(1) [],
      CLR_TRANSMIT      OFFSET(2) NUMBITS(1) [],
      DMA_MODE          OFFSET(3) NUMBITS(1) [],
      FIFO_64B_ENABLE   OFFSET(5) NUMBITS(1) [],
      TRIGGER_THRESHOLD OFFSET(6) NUMBITS(2) [ // FIFO Buffer size alert threshold
        Max     = 0,  // 1 byte
        High    = 1,  // 4
        Medium  = 2,  // 8
        Low     = 3   // 14
      ]
    ],

    LCR [ // Line Control Register
      WORD_LENGTH OFFSET(0) NUMBITS(2) [
        Five = 0,
        Six = 1,
        Seven = 2,
        Eight = 3
      ],
      EXTEND_STOP_BIT OFFSET(2) NUMBITS(1) [],
      PARITY OFFSET(3) NUMBITS(3) [
        None  = 0b000,
        Odd   = 0b001,
        Even  = 0b011,
        Mark  = 0b101,
        Space = 0b111
      ],
      SET_BREAK OFFSET(6) NUMBITS(1) [],
      DLAB OFFSET(7) NUMBITS(1) []
    ],

    MCR [ // Modem Control Register
      DTR 0,  // Data Terminal Ready
      RTS 1,  // Request To Send
      AUX_OUT_1 2,
      AUX_OUT_2 3,
      LOOPBACK 4
    ],

    LSR [ // Line Status Register
      DATA_READY    0,
      OVERRUN_ERR   1,
      PARITY_ERR    2,
      FRAMING_ERR   3,
      BREAK         4,        // Break interrupt received i.e. line dead
      THR_EMPTY     5,
      DHR_EMPTY     6,        // Empty data holding registers
      RECEIVED_FIFO_ERR 7     // General
    ],

    MSR [ // Modem Status Register
      CLEAR_TO_SEND OFFSET(4) NUMBITS(1),
      DATA_SET_READY  OFFSET(5) NUMBITS(1),
      RING_INDICATOR  OFFSET(6) NUMBITS(1),
      CARRIER_DETECT  OFFSET(7) NUMBITS(1)
    ],

    SCR [ // Scratch Register
      DATA OFFSET(0) NUMBITS(8)
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * UART pushing into a vec, copied from log.rs because rust doesn't let you
     * "use" sibling modules in unit tests
     */
    extern crate heapless; // v0.4.x
    use heapless::consts::*;
    use heapless::Vec;
    pub struct TestLog<'a> {
        dat: &'a mut Vec<u8, U1024>,
    }

    impl<'a> TestLog<'a> {
        pub fn new(v: &'a mut Vec<u8, U1024>) -> TestLog {
            TestLog { dat: v }
        }
    }

    impl<'a> Driver for TestLog<'a> {
        fn init(&mut self) -> Result<()> {
            Ok(())
        }

        fn pread(&self, _data: &mut [u8], _offset: usize) -> Result<usize> {
            return Ok(0);
        }

        fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
            for (_, &c) in data.iter().enumerate() {
                self.dat.push(c).unwrap();
            }
            Ok(data.len())
        }

        fn shutdown(&mut self) {}
    }

    const IER: usize = 0x01; // Interrupt Enable Register            0b0001 RW
    const FCR: usize = 0x02; // FIFO Control Register                0b0010 W
    const LCR: usize = 0x03; // Line Control Register                0b0011 RW
    const DLL: usize = 0x00; // Divisor Latch Low Byte               0      RW
    const DLH: usize = 0x01; // Divisor Latch High Byte              0x0001 RW
    const DLAB: usize = 0x80; // Divisor Latch Access Bit             0x1000 RW

    #[test]
    fn uart_driver_inits_correctly() {
        let mut vec = Vec::<u8, U1024>::new();
        let log = &mut TestLog::new(&mut vec);

        let test_uart = &mut I8250::new(0, 0, log);
        test_uart.init().unwrap();

        assert_eq!(1, vec[IER]); //Interrupts enabled
        assert_eq!(1, 1 & vec[FCR]); // FIFOs enabled

        /* DLAB gets set, baud is non-zero, FIFOs enabled */
        // assert_eq!(0x1 & vec[DLL], 1);    // Baud rate set to 115200
    }
}

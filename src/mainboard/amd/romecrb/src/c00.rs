/// Write 32 bits to port
unsafe fn outb(port: u16, val: u8) {
    llvm_asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(val));
}

fn one(w: &mut impl core::fmt::Write, a: u8, v: u8) {
    write!(w, "{:x}:{:x} ", a, v).unwrap();
    unsafe {
        outb(0xc00, a);
        outb(0xc01, v);
    }
}
pub fn c00(w: &mut impl core::fmt::Write) {
    one(w, 0x00, 0x1f);
    one(w, 0x01, 0x1f);
    one(w, 0x02, 0x1f);
    one(w, 0x03, 0x1f);
    one(w, 0x04, 0x1f);
    one(w, 0x05, 0x1f);
    one(w, 0x06, 0x1f);
    one(w, 0x07, 0x1f);
    one(w, 0x08, 0xfa);
    one(w, 0x09, 0x91);
    one(w, 0x0a, 0x00);
    one(w, 0x0b, 0x00);
    one(w, 0x0c, 0x1f);
    one(w, 0x0d, 0x1f);
    one(w, 0x0e, 0x1f);
    one(w, 0x0f, 0x1f);
    one(w, 0x10, 0x1f);
    one(w, 0x11, 0x1f);
    one(w, 0x12, 0x1f);
    one(w, 0x13, 0x00);
    one(w, 0x14, 0x1f);
    one(w, 0x15, 0x1f);
    one(w, 0x16, 0x1f);
    one(w, 0x17, 0x1f);
    one(w, 0x18, 0x1f);
    one(w, 0x19, 0x1f);
    one(w, 0x1a, 0x1f);
    one(w, 0x1b, 0x00);
    one(w, 0x1c, 0x00);
    one(w, 0x1d, 0x00);
    one(w, 0x1e, 0x00);
    one(w, 0x1f, 0x00);
    one(w, 0x20, 0x1f);
    one(w, 0x21, 0x1f);
    one(w, 0x22, 0x1f);
    one(w, 0x23, 0x1f);
    one(w, 0x24, 0x00);
    one(w, 0x25, 0x00);
    one(w, 0x26, 0x00);
    one(w, 0x27, 0x00);
    one(w, 0x28, 0x00);
    one(w, 0x29, 0x00);
    one(w, 0x2a, 0x00);
    one(w, 0x2b, 0x00);
    one(w, 0x2c, 0x00);
    one(w, 0x2d, 0x00);
    one(w, 0x2e, 0x00);
    one(w, 0x2f, 0x00);
    one(w, 0x30, 0x1f);
    one(w, 0x31, 0x1f);
    one(w, 0x32, 0x1f);
    one(w, 0x33, 0x00);
    one(w, 0x34, 0x1f);
    one(w, 0x35, 0x1f);
    one(w, 0x36, 0x00);
    one(w, 0x37, 0x00);
    one(w, 0x38, 0x00);
    one(w, 0x39, 0x00);
    one(w, 0x3a, 0x00);
    one(w, 0x3b, 0x00);
    one(w, 0x3c, 0x00);
    one(w, 0x3d, 0x00);
    one(w, 0x3e, 0x00);
    one(w, 0x3f, 0x00);
    one(w, 0x40, 0x00);
    one(w, 0x41, 0x1f);
    one(w, 0x42, 0x1f);
    one(w, 0x43, 0x1f);
    one(w, 0x44, 0x00);
    one(w, 0x45, 0x00);
    one(w, 0x46, 0x00);
    one(w, 0x47, 0x00);
    one(w, 0x48, 0x00);
    one(w, 0x49, 0x00);
    one(w, 0x4a, 0x00);
    one(w, 0x4b, 0x00);
    one(w, 0x4c, 0x00);
    one(w, 0x4d, 0x00);
    one(w, 0x4e, 0x00);
    one(w, 0x4f, 0x00);
    one(w, 0x50, 0x1f);
    one(w, 0x51, 0x1f);
    one(w, 0x52, 0x1f);
    one(w, 0x53, 0x1f);
    one(w, 0x54, 0x00);
    one(w, 0x55, 0x00);
    one(w, 0x56, 0x00);
    one(w, 0x57, 0x00);
    one(w, 0x58, 0x00);
    one(w, 0x59, 0x00);
    one(w, 0x5a, 0x00);
    one(w, 0x5b, 0x00);
    one(w, 0x5c, 0x00);
    one(w, 0x5d, 0x00);
    one(w, 0x5e, 0x00);
    one(w, 0x5f, 0x00);
    one(w, 0x60, 0x1f);
    one(w, 0x61, 0x1f);
    one(w, 0x62, 0x1f);
    one(w, 0x63, 0x00);
    one(w, 0x64, 0x00);
    one(w, 0x65, 0x00);
    one(w, 0x66, 0x00);
    one(w, 0x67, 0x00);
    one(w, 0x68, 0x00);
    one(w, 0x69, 0x00);
    one(w, 0x6a, 0x00);
    one(w, 0x6b, 0x00);
    one(w, 0x6c, 0x00);
    one(w, 0x6d, 0x00);
    one(w, 0x6e, 0x00);
    one(w, 0x6f, 0x00);
    one(w, 0x70, 0x1f);
    one(w, 0x71, 0x1f);
    one(w, 0x72, 0x1f);
    one(w, 0x73, 0x1f);
    one(w, 0x74, 0x03);
    one(w, 0x75, 0x04);
    one(w, 0x76, 0x1f);
    one(w, 0x77, 0x1f);
    one(w, 0x78, 0x1f);
    one(w, 0x79, 0x1f);
    one(w, 0x7a, 0x00);
    one(w, 0x7b, 0x00);
    one(w, 0x7c, 0x00);
    one(w, 0x7d, 0x00);
    one(w, 0x7e, 0x00);
    one(w, 0x7f, 0x00);
    one(w, 0x80, 0x10);
    one(w, 0x81, 0x11);
    one(w, 0x82, 0x12);
    one(w, 0x83, 0x13);
    one(w, 0x84, 0x14);
    one(w, 0x85, 0x15);
    one(w, 0x86, 0x16);
    one(w, 0x87, 0x17);
    one(w, 0x88, 0x00);
    one(w, 0x89, 0x00);
    one(w, 0x8a, 0x00);
    one(w, 0x8b, 0x00);
    one(w, 0x8c, 0x1f);
    one(w, 0x8d, 0x1f);
    one(w, 0x8e, 0x1f);
    one(w, 0x8f, 0x1f);
    one(w, 0x90, 0x1f);
    one(w, 0x91, 0x1f);
    one(w, 0x92, 0x1f);
    one(w, 0x93, 0x00);
    one(w, 0x94, 0x1f);
    one(w, 0x95, 0x1f);
    one(w, 0x96, 0x1f);
    one(w, 0x97, 0x10);
    one(w, 0x98, 0x1f);
    one(w, 0x99, 0x1f);
    one(w, 0x9a, 0x10);
    one(w, 0x9b, 0x00);
    one(w, 0x9c, 0x00);
    one(w, 0x9d, 0x00);
    one(w, 0x9e, 0x00);
    one(w, 0x9f, 0x00);
    one(w, 0xa0, 0x1f);
    one(w, 0xa1, 0x1f);
    one(w, 0xa2, 0x1f);
    one(w, 0xa3, 0x1f);
    one(w, 0xa4, 0x00);
    one(w, 0xa5, 0x00);
    one(w, 0xa6, 0x00);
    one(w, 0xa7, 0x00);
    one(w, 0xa8, 0x00);
    one(w, 0xa9, 0x00);
    one(w, 0xaa, 0x00);
    one(w, 0xab, 0x00);
    one(w, 0xac, 0x00);
    one(w, 0xad, 0x00);
    one(w, 0xae, 0x00);
    one(w, 0xaf, 0x00);
    one(w, 0xb0, 0x1f);
    one(w, 0xb1, 0x1f);
    one(w, 0xb2, 0x1f);
    one(w, 0xb3, 0x00);
    one(w, 0xb4, 0x1f);
    one(w, 0xb5, 0x1f);
    one(w, 0xb6, 0x00);
    one(w, 0xb7, 0x00);
    one(w, 0xb8, 0x00);
    one(w, 0xb9, 0x00);
    one(w, 0xba, 0x00);
    one(w, 0xbb, 0x00);
    one(w, 0xbc, 0x00);
    one(w, 0xbd, 0x00);
    one(w, 0xbe, 0x00);
    one(w, 0xbf, 0x00);
    one(w, 0xc0, 0x00);
    one(w, 0xc0, 0x00);
    one(w, 0xc2, 0x1f);
    one(w, 0xc3, 0x05);
    one(w, 0xc4, 0x00);
    one(w, 0xc5, 0x00);
    one(w, 0xc6, 0x00);
    one(w, 0xc7, 0x00);
    one(w, 0xc8, 0x00);
    one(w, 0xc9, 0x00);
    one(w, 0xca, 0x00);
    one(w, 0xcb, 0x00);
    one(w, 0xcc, 0x00);
    one(w, 0xcd, 0x00);
    one(w, 0xce, 0x00);
    one(w, 0xcf, 0x00);
    one(w, 0xd0, 0x1f);
    one(w, 0xd1, 0x1f);
    one(w, 0xd2, 0x1f);
    one(w, 0xd3, 0x1f);
    one(w, 0xd4, 0x00);
    one(w, 0xd5, 0x00);
    one(w, 0xd6, 0x00);
    one(w, 0xd7, 0x00);
    one(w, 0xd8, 0x00);
    one(w, 0xd9, 0x00);
    one(w, 0xda, 0x00);
    one(w, 0xdb, 0x00);
    one(w, 0xdc, 0x00);
    one(w, 0xdd, 0x00);
    one(w, 0xde, 0x00);
    one(w, 0xdf, 0x00);
    one(w, 0xe0, 0x1f);
    one(w, 0xe1, 0x1f);
    one(w, 0xe2, 0x07);
    one(w, 0xe3, 0x00);
    one(w, 0xe4, 0x00);
    one(w, 0xe5, 0x00);
    one(w, 0xe6, 0x00);
    one(w, 0xe7, 0x00);
    one(w, 0xe8, 0x00);
    one(w, 0xe9, 0x00);
    one(w, 0xea, 0x00);
    one(w, 0xeb, 0x00);
    one(w, 0xec, 0x00);
    one(w, 0xed, 0x00);
    one(w, 0xee, 0x00);
    one(w, 0xef, 0x00);
    one(w, 0xf0, 0x0a);
    one(w, 0xf1, 0x0b);
    one(w, 0xf2, 0x04);
    one(w, 0xf3, 0x06);
    one(w, 0xf4, 0x03);
    one(w, 0xf5, 0x04);
    one(w, 0xf6, 0x0e);
    one(w, 0xf7, 0x0f);
    one(w, 0xf8, 0x05);
    one(w, 0xf9, 0x0c);
    one(w, 0xfa, 0x00);
    one(w, 0xfb, 0x00);
    one(w, 0xfc, 0x00);
    one(w, 0xfd, 0x00);
    one(w, 0xfe, 0x00);
    one(w, 0xff, 0x00);
    one(w, 0xff, 0x00);
}

#![feature(lang_items)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn entry() {
    init_graphic();
    let vram_address:u32 = 0x06000000;
    // 1 点目 (118, 80)
    let white:u16 = convert_u16_color(0x1F, 0x1F, 0x1F);
    let mut offset: u32 = ((80 * 240) + 118) as u32;
    // 1 ドット 2 バイト使用することに注意
    let mut vram: *mut u16 = (vram_address + (offset * 2)) as *mut u16;
    unsafe {
        *vram = white;
    }
    // 2 点目 (120, 80)
    let green:u16 = convert_u16_color(0x00, 0x1F, 0x00);
    offset = ((80 * 240) + 120) as u32;
    vram = (vram_address + (offset * 2)) as *mut u16;
    unsafe {
        *vram = green;
    }
    // 3 点目 (122, 80)
    let red:u16 = convert_u16_color(0x1F, 0x00, 0x00);
    offset = ((80 * 240) + 122) as u32;
    vram = (vram_address + (offset * 2)) as *mut u16;
    unsafe {
        *vram = red;
    }
    loop {}
}

fn init_graphic() {
    let ioram_address: u32 = 0x04000000;
    let video_mode: *mut u8 = ioram_address as *mut u8;
    let bg: *mut u8 = (ioram_address + 1) as *mut u8;
    unsafe {
        *video_mode = 0x03; // mode 3
        *bg = 0x04; // BG2
    }
}

fn convert_u16_color(r:u8, g:u8, b:u8) -> u16{
    return (((b & 0x1F) as u16) << 10) +
        (((g & 0x1F) as u16) << 5) +
        (r & 0x1F) as u16;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
pub extern fn eh_personality() {}

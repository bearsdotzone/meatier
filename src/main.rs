#![no_std]
#![no_main]

use agb::{println, syscall::*};
use core::fmt::Write;
use gba::prelude::*;

// #[panic_handler]
// fn panic_handler(_: &core::panic::PanicInfo) -> ! {
//     #[cfg(debug_assertions)]
//     if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
//         writeln!(logger, "[info]").ok();
//     }
//     loop {}
// }

#[no_mangle]
extern "C" fn main() -> ! {
    DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
    IE.write(IrqBits::VBLANK);
    IME.write(true);

    let buffer0 = DisplayControl::new()
        .with_video_mode(VideoMode::_5)
        .with_show_bg2(true)
        .with_show_frame1(false);

    let buffer1 = DisplayControl::new()
        .with_video_mode(VideoMode::_5)
        .with_show_bg2(true)
        .with_show_frame1(true);

    DISPCNT.write(buffer0);

    let r = 60;
    let r_2 = r * r;

    let mut sweep: Fixed<i32, 8> = Fixed::<i32, 8>::from_raw(80 - r);

    let mut frame_count = 0;

    loop {
        VBlankIntrWait();

        let mut page = VideoMode5Frame::_0;

        if frame_count % 2 == 1 {
            page = VideoMode5Frame::_1;
            DISPCNT.write(buffer0)
        } else {
            DISPCNT.write(buffer1)
        }

        for i in 80 - r..=80 + r {
            let x: i32 = i;
            let y: i32 = sqrt((r_2 - (x - 80).pow(2)).abs());
            let y_round = usize::try_from(y).unwrap();

            let mut color_write = Color::RED;

            if Fixed::<i32, 8>::from_raw(i) > sweep.sub(Fixed::<i32, 8>::from_raw(3))
                && Fixed::<i32, 8>::from_raw(i) < sweep.add(Fixed::<i32, 8>::from_raw(3))
            {
                color_write = Color::BLUE;
            }

            page.row_col(y_round + 64, usize::try_from(i).unwrap())
                .write(color_write);
            page.row_col(64 - y_round, usize::try_from(i).unwrap())
                .write(color_write);
        }

        sweep += Fixed::<i32, 8>::from_raw(1).div(Fixed::<i32, 8>::from_raw(75));
        if sweep > Fixed::<i32, 8>::from_raw(r + 80) {
            sweep = Fixed::<i32, 8>::from_raw(80 - r);
        }
        frame_count += 1;
        frame_count %= 2;
    }
}

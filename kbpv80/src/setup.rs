use crate::hal;
use crate::leds::Leds;
use core::mem::MaybeUninit;
use hal::ckcu::CkcuExt;
use hal::gpio::{gpioa::PA4, AF0};
use hal::gpio::{Afio, GpioExt, Input, Output, Pin, PullUp, PushPull};
use hal::pac;
use hal::time::RateExtU32;
use hal::usb::{Peripheral, UsbBus};
use keyberon::matrix::Matrix;
use rtic_monotonics::systick::Systick;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;

pub const MTRX_C: usize = 8;
pub const MTRX_R: usize = 15;

pub type MyUsbBus = UsbBus<Peripheral<PA4<Output<PushPull>, AF0>>>;
pub type MyMatrix = Matrix<Pin<Input<PullUp>>, Pin<Output<PushPull>>, MTRX_C, MTRX_R>;

#[inline(always)]
pub fn setup(
    dp: pac::Peripherals,
    cp: cortex_m::Peripherals,
    usb_bus: &'static mut MaybeUninit<UsbBusAllocator<MyUsbBus>>,
) -> (
    keyberon::Class<'static, MyUsbBus, Leds>,
    UsbDevice<'static, MyUsbBus>,
    MyMatrix,
) {
    let ckcu = dp.CKCU.constrain(dp.RSTCU);
    let _clocks = ckcu
        .configuration
        .use_hse(8.MHz())
        .ck_sys(144u32.MHz())
        .hclk(72u32.MHz())
        .ck_usb(48u32.MHz())
        .freeze();

    let token = rtic_monotonics::create_systick_token!();
    Systick::start(cp.SYST, 72_000_000, token);

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();
    let gpioe = dp.GPIOE.split();
    let mut afio = Afio::new(dp.AFIO);

    let dppu = gpioa.pa4.into_output_push_pull();

    let leds = Leds::new(gpioa.pa5, gpioa.pa6);
    let usb = Peripheral { usb: dp.USB, dppu };

    let usb_bus: &'static _ = usb_bus.write(UsbBus::new(usb));
    let usb_class = keyberon::new_class(usb_bus, leds);
    let usb_dev = keyberon::new_device(usb_bus);

    #[rustfmt::skip]
    let matrix = Matrix::new(
        [
            gpioa.pa0.into_input_pull_up().downgrade(),
            gpioa.pa1.into_input_pull_up().downgrade(),
            gpioa.pa2.into_input_pull_up().downgrade(),
            gpioa.pa3.into_input_pull_up().downgrade(),
            gpiob.pb4.into_alternate_af1(&mut afio).into_input_pull_up().downgrade(),
            gpiob.pb5.into_alternate_af1(&mut afio).into_input_pull_up().downgrade(),
            gpiob.pb6.into_alternate_af1(&mut afio).into_input_pull_up().downgrade(),
            gpiob.pb7.into_input_pull_up().downgrade(),
        ],
        [
            gpioc.pc0.into_output_push_pull().downgrade(),
            gpioc.pc1.into_output_push_pull().downgrade(),
            gpioc.pc11.into_output_push_pull().downgrade(),
            gpioc.pc12.into_output_push_pull().downgrade(),
            gpioc.pc13.into_output_push_pull().downgrade(),
            gpioc.pc14.into_output_push_pull().downgrade(),
            gpioc.pc15.into_output_push_pull().downgrade(),
            gpiod.pd9.into_output_push_pull().downgrade(),
            gpiod.pd10.into_output_push_pull().downgrade(),
            gpiod.pd11.into_output_push_pull().downgrade(),
            gpiod.pd12.into_output_push_pull().downgrade(),
            gpiod.pd13.into_output_push_pull().downgrade(),
            gpioe.pe5.into_output_push_pull().downgrade(),
            gpioe.pe6.into_output_push_pull().downgrade(),
            gpioe.pe7.into_output_push_pull().downgrade(),
        ],
    );

    (usb_class, usb_dev, matrix.unwrap())
}

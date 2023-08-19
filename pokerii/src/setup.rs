use crate::hal;
use core::mem::MaybeUninit;
use hal::ckcu::CkcuExt;
use hal::gpio::{Pin, Afio, GpioExt, Input, Output, PullUp, PushPull};
use hal::pac;
use hal::time::RateExtU32;
use hal::usb::{Peripheral, UsbBus};
use keyberon::matrix::Matrix;
use rtic_monotonics::systick::Systick;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;

pub const MTRX_C: usize = 8;
pub const MTRX_R: usize = 9;

pub type MyUsbBus = UsbBus<Peripheral>;
pub type MyMatrix = Matrix<Pin<Input<PullUp>>, Pin<Output<PushPull>>, MTRX_C, MTRX_R>;

#[inline(always)]
pub fn setup(
    dp: pac::Peripherals,
    cp: cortex_m::Peripherals,
    usb_bus: &'static mut MaybeUninit<UsbBusAllocator<MyUsbBus>>,
) -> (
    keyberon::Class<'static, MyUsbBus, ()>,
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
    let mut afio = Afio::new(dp.AFIO);

    let usb = Peripheral { usb: dp.USB };
    let usb_bus: &'static _ = usb_bus.write(UsbBus::new(usb));
    let usb_class = keyberon::new_class(usb_bus, ());
    let usb_dev = keyberon::new_device(usb_bus);

    #[rustfmt::skip]
    let matrix = Matrix::new(
        [
            gpioc.pc9.into_input_pull_up().downgrade(),
            gpioa.pa5.into_input_pull_up().downgrade(),
            gpioa.pa6.into_input_pull_up().downgrade(),
            gpioa.pa7.into_input_pull_up().downgrade(),
            gpioc.pc11.into_input_pull_up().downgrade(),
            gpioc.pc13.into_alternate_af1(&mut afio).into_input_pull_up().downgrade(),
            gpioc.pc14.into_alternate_af1(&mut afio).into_input_pull_up().downgrade(),
            gpioc.pc15.into_alternate_af1(&mut afio).into_input_pull_up().downgrade(),
        ],
        [
            gpioc.pc4.into_output_push_pull().downgrade(),
            gpiob.pb6.into_output_push_pull().downgrade(),
            gpiod.pd0.into_output_push_pull().downgrade(),
            gpiob.pb11.into_output_push_pull().downgrade(),
            gpioa.pa11.into_alternate_af1(&mut afio).into_output_push_pull().downgrade(),
            gpioa.pa0.into_output_push_pull().downgrade(),
            gpioa.pa1.into_output_push_pull().downgrade(),
            gpioa.pa2.into_output_push_pull().downgrade(),
            gpioa.pa3.into_output_push_pull().downgrade(),
        ],
    );

    (usb_class, usb_dev, matrix.unwrap())
}

# ht32f1yyy keyboard firmware

This repository contains rust firmware for keyboards
with a Holtek `ht32f1yyy` MCU.  

Currently supported keyboards:
- ikbc New Poker II
- KB Paradise v80

It is based on:
- [ht32f1yyy-hal][] for the hardware abstraction layer
- [rtic][] as RTOS/concurrency framework
- [keyberon][] for scanning the keyboard matrix and implementing the HID communication

[ht32f1yyy-hal]: https://github.com/ht32-rs/ht32f1yyy-hal
[rtic]: https://rtic.rs
[keyberon]: https://github.com/TeXitoi/keyberon

## License

[MIT]

[MIT]: ./LICENSE

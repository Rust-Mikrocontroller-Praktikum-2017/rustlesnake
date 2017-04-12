#![feature(lang_items)]
#![feature(const_fn)]
#![feature(alloc, collections)]

#![no_std]
#![no_main]

#[macro_use]
extern crate stm32f7_discovery as stm32f7;

// init routines for .data and .bss
extern crate r0;
extern crate alloc;

#[macro_use]
extern crate collections;

mod renderer;
mod snake;
mod randomizer;

use stm32f7::{system_clock, sdram, lcd, i2c, touch, board, embedded};

use randomizer::RNG;
use renderer::*;
use snake::*;

const LCD_WIDTH: u16 = 480;
const LCD_HEIGHT: u16 = 272;
const LCD_AXIS_X: u16 = 64;
const LCD_AXIS_Y: u16 = 64;

const GRID_WIDTH: u16 = 40;
const GRID_HEIGHT: u16 = 22;

const GAME_SPEED: usize = 250;
const MAX_GAME_SPEED: usize = 50;
const SCORE_SPEED_MODIFIER: usize = 5;

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static __DATA_LOAD: u32;
        static __DATA_END: u32;
        static mut __DATA_START: u32;
        static mut __BSS_START: u32;
        static mut __BSS_END: u32;
    }

    let data_load = &__DATA_LOAD;
    let data_start = &mut __DATA_START;
    let data_end = &__DATA_END;
    let bss_start = &mut __BSS_START;
    let bss_end = &__BSS_END;

    r0::init_data(data_start, data_end, data_load);
    r0::zero_bss(bss_start, bss_end);

    stm32f7::heap::init();

    main(board::hw());
}

#[inline(never)]
fn main(hw: board::Hardware) -> ! {
    let board::Hardware {
        rcc,
        pwr,
        flash,
        fmc,
        ltdc,
        gpio_a,
        gpio_b,
        gpio_c,
        gpio_d,
        gpio_e,
        gpio_f,
        gpio_g,
        gpio_h,
        gpio_i,
        gpio_j,
        gpio_k,
        i2c_3,
        ..
    } = hw;

    use embedded::interfaces::gpio::{self, Gpio};

    let mut gpio = Gpio::new(gpio_a,
                             gpio_b,
                             gpio_c,
                             gpio_d,
                             gpio_e,
                             gpio_f,
                             gpio_g,
                             gpio_h,
                             gpio_i,
                             gpio_j,
                             gpio_k);


    system_clock::init(rcc, pwr, flash);

    // enable all gpio ports
    rcc.ahb1enr
        .update(|r| {
            r.set_gpioaen(true);
            r.set_gpioben(true);
            r.set_gpiocen(true);
            r.set_gpioden(true);
            r.set_gpioeen(true);
            r.set_gpiofen(true);
            r.set_gpiogen(true);
            r.set_gpiohen(true);
            r.set_gpioien(true);
            r.set_gpiojen(true);
            r.set_gpioken(true);
        });

    // configure led pin as output pin
    let led_pin = (gpio::Port::PortI, gpio::Pin::Pin1);
    let mut led = gpio.to_output(led_pin,
                                 gpio::OutputType::PushPull,
                                 gpio::OutputSpeed::Low,
                                 gpio::Resistor::NoPull)
        .expect("led pin already in use");

    // turn led on
    led.set(true);

    let restart_button_pin = (gpio::Port::PortI, gpio::Pin::Pin11);
    let restart_button = gpio.to_input(restart_button_pin, gpio::Resistor::NoPull)
        .expect("button pin already in use");
    let mut restart_pressed = false;
    let mut restart_pressed_old = false;

    sdram::init(rcc, fmc, &mut gpio);
    let mut lcd = lcd::init(ltdc, rcc, &mut gpio);

    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);



    let mut rng = RNG { seed: (system_clock::ticks() as u16) };
    let mut game = Game::new(Renderer::new(LcdExt::new(&mut lcd)),
                             &mut rng,
                             GRID_WIDTH,
                             GRID_HEIGHT);
    game.init_game();

    let mut last_frame_ticks = system_clock::ticks();

    loop {
        let ticks = system_clock::ticks();

        restart_pressed = restart_button.get();

        if restart_pressed && !restart_pressed_old {
            game.restart()
        }

        let direction = tap(&mut i2c_3);
        game.update_direction(direction);

        let frame_time = if (game.get_score() as usize) * SCORE_SPEED_MODIFIER < GAME_SPEED {
            max(GAME_SPEED - (game.get_score() as usize) * SCORE_SPEED_MODIFIER,
                MAX_GAME_SPEED)
        } else {
            MAX_GAME_SPEED
        };

        if ticks - last_frame_ticks > frame_time {
            game.step();
            last_frame_ticks = ticks;
        }

        restart_pressed_old = restart_pressed;
    }
}

fn tap(mut i2c: &mut i2c::I2C) -> MoveDirection {
    let mut direction = MoveDirection::None;
    if let Ok(touches) = touch::touches(&mut i2c) {
        if touches.len() > 0 {
            // Only use primary touch
            let touch = touches[0];
            let x = touch.x;
            let y = touch.y;

            let left = x < LCD_AXIS_X;
            let top = y < LCD_AXIS_Y;
            let right = x >= LCD_WIDTH - LCD_AXIS_X;
            let bottom = y >= LCD_HEIGHT - LCD_AXIS_Y;

            let hor = !(top || bottom);
            let ver = !(left || right);

            if ver ^ hor {
                direction = if hor {
                    if left {
                        MoveDirection::Left
                    } else {
                        MoveDirection::Right
                    }
                } else {
                    if top {
                        MoveDirection::Up
                    } else {
                        MoveDirection::Down
                    }
                }
            }
        }
    }
    direction
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
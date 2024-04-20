#![no_main]
#![no_std]

// Importing HAL and PAC
use stm32f1xx_hal::pac;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

// Setup global logger and panic behavior if using `defmt`
use defmt_rtt as _;  // Initialize defmt over RTT
use panic_probe as _; // Handle panics with panic-probe

#[entry]
fn main() -> ! {
    // Take ownership of the device's peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Optional: Setup the device clock and configure GPIO pins, etc.
    // let rcc = dp.RCC.constrain();
    // let gpioa = dp.GPIOA.split(&mut rcc.apb2);

    // Print "Hello, world!" to semihosting console
    hprintln!("Hello, world!");

    // An infinite loop to prevent the program from ending
    loop {
        // Your application logic here
    }
}

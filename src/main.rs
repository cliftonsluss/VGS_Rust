#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

use embedded_hal::serial::Read;



#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
		pins.d9.into_output();		
		
		let tc1 = dp.TC1;

		/*
		/	Configure inverted fast pmw mode on timer 1
		/	When WGM11, 12, 13 are set timer1 Waveform Generation Mode
		/ is configured as Fast PWM with TOP counter value determined 
		/ by ICR1 and OCR1x updated at the bottom of the count. 
		/ TOV1 (overflow) flag is set on TOP in this configuration. 
		/ See pg 109 ATmega328P [DATSHEET]
		/ 
		/	To accompish this we set the WGM11 and 12 bits on TCCR1A and	
		/	WGM13 with bits(0b10) and bits(0b11) respectively.
		/
		/	TCCR1A COMnx selects compare output mode dependent upon
		/	selected PWM mode. In this case with mode set as fast
		/ we would set both COM1A1 and COM1A0 bits on TCCR1A to 
		/ achieve the desired compare output mode of "Set OC1A on 
		/ compare match, clear OC1A at BOTTOM" to configure inverting
		/ mode. The avr_device crate provides a writer for setting the
		/ registers associated with a given WGM. com1a().match_clear()
		/ sets the COM1A1 and COM1A1 bits to configure compare output 
		/ mode to 'clear at BOTTOM after compare match' or 
		/ 'inverted mode'.    
		/
		/	Finally we feed the write method a closure setting the
		/	prescaler for timer 1 with no prescaling (direct()).
		/
		*/
		tc1.tccr1a
				.write(|w| w.wgm1().bits(0b10).com1a().match_clear());
		tc1.tccr1b
				.write(|w| w.wgm1().bits(0b11).cs1().direct());			
	
		
		//ICR1 = _NTSC_CYCLES_SCANLINE;
		tc1.icr1.write(|w| unsafe { w.bits(1015) });
	
		// 16 cycles per microsecond * time for horz sync - 1	
		//OCR1A = _CYCLES_HORZ_SYNC; (16 * 4.7) -1
		tc1.ocr1a.write(|w| unsafe { w.bits(74) });


		//TIMSK1 = _BV(TOIE1);
		//tc1.timsk1.write(|w| w.toie1().set_bit());


    loop {
    }
}

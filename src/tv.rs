use avr_progmem::progmem;
use arduino_hal::prelude::*;
use panic_halt as _;
use embedded_hal::serial::Read;

const WMAX: usize = 128;
const HMAX: usize = 96;
const AMAX: usize = WMAX*HMAX;
const SCN_SZ: usize = 12288;

const TCCR1A: *mut u8 = 0x80 as *mut u8;    
const TCCR1B: *mut u8 = 0x81 as *mut u8;
const TCCR2A: *mut u8 = 0xB0 as *mut u8;
const TCCR2B: *mut u8 = 0xB1 as *mut u8;

//TCCR1A = 


progmem! { pub static progmem SCREEN: [u8;SCN_SZ] = [0;SCN_SZ]; }


pub struct Thing<'a> {
	aptr: &'a [u8],
}

pub	fn build_thing(aptr: &[u8]) -> Thing {
		Thing { aptr }
}


struct TVout_vid<'a> {
	scanline: u16,
	frames: u32,
	start_render: u8,
	lines_frame: u16,
	vres: u8,
	hres: u8,
	output_delay: u8,
	vscale_const: u8,
	vsync_end: u8,
	screen: &'a [u8],
}

	

struct Cursor {
	x: u8,
	y: u8,
}

struct Screen {
	w: usize,
	h: usize,
	field: [usize; AMAX]
}


pub struct Tv<'a>{
	mode: u8,
	cursor: Cursor,
  scrn_ptr: &'a [u8],					
} 




//pub fn build_Tv(mode: u8, w: usize, h :usize) -> Tv {
//	if w*h < AMAX {
//		let screen = Screen { w, h, field: [0; AMAX] };
//		let cursor = Cursor { x:0, y:0};
//		Tv { mode, cursor, screen,}
//		} else {
//				panic!("Defined screen size exceeds AMAX");
//		}
//}


/*impl Tv{
	
	fn begin(mut self, mode: u8, w: usize, h: usize) {
		self.mode = mode;
		self.cursor.x = 0;
		self.cursor.y = 0;
		if w*h < AMAX {
			self.screen.w = w;
			self.screen.h = h;
		} else {
				panic!("Defined screen size exceeds AMAX");
		}
		Tv::render_setup();
	}
	fn render_setup() {
		let f = 0; // render_setup stub
	}
} 
*/






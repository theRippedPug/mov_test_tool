use rppal::gpio::{Error, Gpio, OutputPin};
use std::thread::sleep;
use std::time::Duration;
pub struct StraightLineMotion {
	ticks: i64,
	intervial_in_ms: u64,
}
pub struct DiagonalMotion {
	x: i8,
	y: i8,
	ticks: u64,
	intervial_in_ms: u64,
}
pub enum MotorMotion {
	XAxis(StraightLineMotion),
	YAxis(StraightLineMotion),
	Diagonal(DiagonalMotion),
}

pub struct RawMotor {
	motor_pins: [OutputPin; 4],
	motor_state: u8,
}

impl RawMotor {
	pub fn new(p1: u8, p2: u8, p3: u8, p4: u8) -> Result<RawMotor, Error> {
		let motor_pins = [
			Gpio::new()?.get(p1)?.into_output(),
			Gpio::new()?.get(p2)?.into_output(),
			Gpio::new()?.get(p3)?.into_output(),
			Gpio::new()?.get(p4)?.into_output(),
		];

		Ok(RawMotor {
			motor_pins,
			motor_state: 0 as u8,
		})
	}

	pub fn move_motor_by(&mut self, ticks: i64, pausetime_in_ms: u64) {
		let dur = Duration::from_millis(pausetime_in_ms);
		if ticks > 0 {
			for _ in 0..ticks {
				self.tick_front();
				sleep(dur);
			}
		} else {
			let ticks = -ticks;
			for _ in 0..ticks {
				self.tick_back();
				sleep(dur);
			}
		}
	}

	pub fn tick_front(&mut self) {
		match self.motor_state {
			1 => {
				self.motor_pins[0].set_high();
				self.motor_pins[1].set_low();
				self.motor_pins[2].set_low();
				self.motor_pins[3].set_low();

				self.motor_state = 2;
			}
			2 => {
				self.motor_pins[0].set_high();
				self.motor_pins[1].set_high();
				self.motor_pins[2].set_low();
				self.motor_pins[3].set_low();

				self.motor_state = 3;
			}
			3 => {
				self.motor_pins[0].set_low();
				self.motor_pins[1].set_high();
				self.motor_pins[2].set_low();
				self.motor_pins[3].set_low();

				self.motor_state = 4;
			}
			4 => {
				self.motor_pins[0].set_low();
				self.motor_pins[1].set_high();
				self.motor_pins[2].set_high();
				self.motor_pins[3].set_low();

				self.motor_state = 5;
			}
			5 => {
				self.motor_pins[0].set_low();
				self.motor_pins[1].set_low();
				self.motor_pins[2].set_high();
				self.motor_pins[3].set_low();

				self.motor_state = 6;
			}
			6 => {
				self.motor_pins[0].set_low();
				self.motor_pins[1].set_low();
				self.motor_pins[2].set_high();
				self.motor_pins[3].set_high();

				self.motor_state = 7;
			}
			7 => {
				self.motor_pins[0].set_low();
				self.motor_pins[1].set_low();
				self.motor_pins[2].set_low();
				self.motor_pins[3].set_high();

				self.motor_state = 8;
			}
			8 => {
				self.motor_pins[0].set_high();
				self.motor_pins[1].set_low();
				self.motor_pins[2].set_low();
				self.motor_pins[3].set_high();

				self.motor_state = 1;
			}

			_ => {}
		}
	}

	pub fn tick_back(&mut self) {
		match self.motor_state {
			1 => {
				self.motor_pins[0].set_high();
				self.motor_pins[1].set_low();
				self.motor_pins[2].set_low();
				self.motor_pins[3].set_low();

				self.motor_state = 8;
			}
			2 => {
				self.motor_pins[0].set_high();
				self.motor_pins[1].set_high();
				self.motor_pins[2].set_low();
				self.motor_pins[3].set_low();

				self.motor_state = 1;
			}
			3 => {
				self.motor_pins[0].set_low();
				self.motor_pins[1].set_high();
				self.motor_pins[2].set_low();
				self.motor_pins[3].set_low();

				self.motor_state = 2;
			}
			4 => {
				self.motor_pins[0].set_low();
				self.motor_pins[1].set_high();
				self.motor_pins[2].set_high();
				self.motor_pins[3].set_low();

				self.motor_state = 3;
			}
			5 => {
				self.motor_pins[0].set_low();
				self.motor_pins[1].set_low();
				self.motor_pins[2].set_high();
				self.motor_pins[3].set_low();

				self.motor_state = 4;
			}
			6 => {
				self.motor_pins[0].set_low();
				self.motor_pins[1].set_low();
				self.motor_pins[2].set_high();
				self.motor_pins[3].set_high();

				self.motor_state = 5;
			}
			7 => {
				self.motor_pins[0].set_low();
				self.motor_pins[1].set_low();
				self.motor_pins[2].set_low();
				self.motor_pins[3].set_high();

				self.motor_state = 6;
			}
			8 => {
				self.motor_pins[0].set_high();
				self.motor_pins[1].set_low();
				self.motor_pins[2].set_low();
				self.motor_pins[3].set_high();

				self.motor_state = 7;
			}

			_ => {}
		}
	}
}

pub struct DualAxisCtrl {
	motor_x: RawMotor,
	motor_y: RawMotor,
}

impl DualAxisCtrl {
	pub fn new(motor_x_pins: [u8; 4], motor_y_pins: [u8; 4]) -> Result<DualAxisCtrl, Error> {
		let motor_x = RawMotor::new(
			motor_x_pins[0],
			motor_x_pins[1],
			motor_x_pins[2],
			motor_x_pins[3],
		)?;
		let motor_y = RawMotor::new(
			motor_y_pins[0],
			motor_y_pins[1],
			motor_y_pins[2],
			motor_y_pins[3],
		)?;

		Ok(DualAxisCtrl { motor_x, motor_y })
	}

	pub fn move_em(&mut self, motion: MotorMotion) {
		match motion {
			MotorMotion::XAxis(details) => {
				let dur = Duration::from_millis(details.intervial_in_ms);
				if details.ticks > 0 {
					for _ in 0..details.ticks {
						self.motor_x.tick_front();
						sleep(dur);
					}
				} else {
					let ticks = -details.ticks;
					for _ in 0..ticks {
						self.motor_x.tick_back();
						sleep(dur);
					}
				}
			},
			MotorMotion::YAxis(details) =>{
				let dur = Duration::from_millis(details.intervial_in_ms);
				if details.ticks > 0 {
					for _ in 0..details.ticks {
						self.motor_y.tick_front();
						sleep(dur);
					}
				} else {
					let ticks = -details.ticks;
					for _ in 0..ticks {
						self.motor_y.tick_back();
						sleep(dur);
					}
				}
			},
			MotorMotion::Diagonal(details) => {
				let dur = Duration::from_millis(details.intervial_in_ms);
				for _ in 0..details.ticks{
					match details.x{
						1 =>{
							self.motor_x.tick_front()
						}
						-1 =>{
							self.motor_x.tick_back()
						}
						_ =>{}
					}
					match details.y{
						1 =>{
							self.motor_y.tick_front()
						}
						-1 =>{
							self.motor_y.tick_back()
						}
						_ =>{}
					}
					sleep(dur);
				}
			}
		}
	}
}

use super::motor_driver;
use std::sync::Arc;
use std::thread;
use std::time;
use parking_lot::{RwLock};
use rppal::gpio::Error;


pub struct MotorServer{
	handle1: Arc<RwLock<i32>>,
	handle2: Arc<RwLock<i32>>
}
impl MotorServer{
	pub fn new(pins_motor1:(u8,u8,u8,u8), pins_motor2:(u8,u8,u8,u8))-> Result<MotorServer, Error>{
		let lock1 = Arc::new(RwLock::new(0_i32));
		let lock2 = Arc::new(RwLock::new(0_i32));

		let mut motor1 = motor_driver::RawMotor::new(pins_motor1.0, pins_motor1.1, pins_motor1.2, pins_motor1.3).unwrap();
		let mut motor2 = motor_driver::RawMotor::new(pins_motor2.0, pins_motor2.1, pins_motor2.2, pins_motor2.3).unwrap();

		let motor1_handle = lock1.clone();
		let motor2_handle = lock2.clone();

		thread::spawn(move ||{
			loop{
				let motion_val = motor1_handle.read();
				thread::sleep(time::Duration::from_millis(motion_val.abs() as u64));
				if motion_val.eq(&0_i32) {

				}
				if motion_val.is_negative(){
					motor1.tick_back()
				}else{
					motor1.tick_front()
				}
			}
		});
		thread::spawn(move ||{
			loop{
				let motion_val = motor2_handle.read();
				thread::sleep(time::Duration::from_millis(motion_val.abs() as u64));
				if motion_val.eq(&0_i32) {
					thread::sleep(time::Duration::from_millis(3));
				}
				if motion_val.is_negative(){
					motor2.tick_back()
				}else{
					motor2.tick_front()
				}
			}
		});

		Ok(MotorServer{handle1: lock1, handle2:lock2})
	}
	pub fn 
}
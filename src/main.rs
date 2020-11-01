#![allow(dead_code)]
mod event;
pub mod motor_driver;
mod motor_server;

use event::{Event, Events};
use gilrs::{Axis, Button, Gilrs};
use std::{
	error::Error,
	io::{self, Write},
};
use termion::{
	cursor::Goto, event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen,
};
use tui::{
	backend::TermionBackend,
	layout::{Constraint, Direction, Layout},
	style::{Color, Style},
	widgets::{
		canvas::{Canvas, Line, MapResolution, Rectangle},
		Block, Borders,
	},
	Terminal,
};
use motor_driver::{RawMotor};

fn main() -> Result<(), Box<dyn Error>> {
	//setup stdio
	let stdout = io::stdout().into_raw_mode()?;
	let stdout = MouseTerminal::from(stdout);
	let stdout = AlternateScreen::from(stdout);
	let backend = TermionBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	// Setup event handlers
	let mut events = Events::new();

	let mut gilrs = Gilrs::new().unwrap();
	let mut active_gamepad = None;
	// Examine new events

	let mut pos_tup: (f64, f64) = (0.0, 0.0);

	// let mut motor1 = RawMotor::new(5, 6, 13, 19)?;
	// let mut motor2 = RawMotor::new(12, 16, 20, 21)?;

	let mov_server = motor_server::MotorServer::new((6,5,12,19), (12,16,20,21));
	loop {
		while let Some(gilrs::Event { id, event, time }) = gilrs.next_event() {
			active_gamepad = Some(id);
		}
		if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
			let val1 = match gamepad.axis_data(Axis::LeftStickX) {
				Some(X) => X.value(),
				None => 0.0,
			};
			let val2 = match gamepad.axis_data(Axis::LeftStickY) {
				Some(X) => X.value(),
				None => 0.0,
			};

			pos_tup = (((val2 + 1.0) * 50.0) as f64, (((val1 + 1.0) * 50.0) as f64));
		}

		let line1 = Line {
			color: Color::DarkGray,
			x1: 0.0,
			x2: 100.0,
			y1: pos_tup.0,
			y2: pos_tup.0,
		};
		let line2 = Line {
			color: Color::DarkGray,
			x1: pos_tup.1,
			x2: pos_tup.1,
			y1: 0.0,
			y2: 100.0,
		};
/*
		//for motor1
		if pos_tup.0 >0.75 {
			motor1.tick_front();
		}else if pos_tup.0 <0.25 {
			motor1.tick_front();
		}

		//for motor2
		if pos_tup.1 >0.75 {
			motor2.tick_front();
		}else if pos_tup.1 <0.25 {
			motor2.tick_front();
		}
*/
		

		// Draw UI
		terminal.draw(|mut f| {
			let canvas = Canvas::default()
				.block(Block::default().borders(Borders::ALL).title("Input"))
				.paint(|ctx| {
					ctx.draw(&line1);
					ctx.draw(&line2)
				})
				.x_bounds([0.0, 100.0])
				.y_bounds([0.0, 100.0]);
			f.render_widget(canvas, f.size());
		})?;

		if let Event::Input(input) = events.next()? {
			if let Key::Char('q') = input {
				break;
			}
		}
	}

	Ok(())
}

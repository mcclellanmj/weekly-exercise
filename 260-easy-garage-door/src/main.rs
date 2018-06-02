#[derive(Debug)]
enum DoorState {
	Open,
	Closed,
	Opening,
	Closing,
	Stopped(Box<DoorState>),
	BlockedOpen(Box<DoorState>),
	EmergencyOpening(Box<DoorState>)
}

#[derive(Debug)]
enum Event {
	ButtonClicked,
	CycleCompleted,
	BlockDetected,
	BlockCleared
}

fn next_state(current_state: DoorState, event: Event) -> DoorState {
	match event {
		Event::ButtonClicked => match current_state {
			DoorState::Open => DoorState::Closing,
			DoorState::Closed => DoorState::Opening,
			DoorState::Opening | DoorState::Closing => DoorState::Stopped(Box::new(current_state)),
			DoorState::Stopped(x) => {
				let last_state = *x;

				match last_state {
					DoorState::Opening => DoorState::Closing,
					DoorState::Closing => DoorState::Opening,
					x => panic!(format!("Unexpected stop state, stopped in [{:?}] but only expected Opening and Closing", x))
				}
			},
			x @ DoorState::BlockedOpen(_) | x @ DoorState::EmergencyOpening(_) => x
		},
		Event::CycleCompleted => match current_state {
			DoorState::Opening => DoorState::Open,
			DoorState::Closing => DoorState::Closed,
			DoorState::EmergencyOpening(x) => DoorState::BlockedOpen(x),
			_ => panic!(format!("Current state is [{:?}] but tried to cycle complete which is not possible", current_state))
		},
		_ => panic!("Not Implemented")
	}
}

fn string_to_event(input: &str) -> Event {
	match input {
		"button_clicked" => Event::ButtonClicked,
		"cycle_complete" => Event::CycleCompleted,
		x => panic!(format!("Unknown event type for [{}]", x))
	}
}

fn main() {
	let events = vec!("button_clicked", "cycle_complete", "button_clicked", "button_clicked", "button_clicked", "button_clicked", "button_clicked", "cycle_complete");
	let events_as_enum = events.iter().map(|x| string_to_event(x));

	let mut current_state = DoorState::Closed;

	for event in events_as_enum {
		current_state = next_state(current_state, event);
	}

    println!("Final state was [{:?}]", current_state);
}

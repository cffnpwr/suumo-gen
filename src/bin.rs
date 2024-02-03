use suumo_gen::{SuumoElement, SuumoState};

fn main() {
    let mut suumo_state = SuumoState::new();

    loop {
        let element = SuumoElement::new();
        print!("{}", element);

        if suumo_state.next_with_suumo_element(element) == None {
            break;
        }
    }
}

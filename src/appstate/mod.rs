use app_state::{AppState, MutAppState, AppStateTrait, InitAppState, InitMutAppState};

#[derive(Default, InitAppState, InitMutAppState)]
pub struct EmailState {
  counter: Vec<String>,
}

impl EmailState {
    pub fn new(counter: Vec<String>,) -> Self {
        EmailState { counter: counter }
    }

    pub fn get_all_email_list(&self) -> &Vec<String> {
       return &self.counter
    }

    pub fn add_new_email_list(&mut self, email: String) {
        self.counter.push(email);
    }
}
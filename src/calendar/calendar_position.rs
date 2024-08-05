use chrono::{DateTime, Days, Local, Months};

#[derive(Debug)]
pub struct CalendarPosition {
    pub date: DateTime<Local>,
    pub editing: CurrentlyEditing,
}

#[derive(Debug)]
pub enum CurrentlyEditing {
    Year,
    Month,
    Day,
}

impl CalendarPosition {
    pub fn new() -> Self {
        Self {
            date: Local::now(),
            editing: CurrentlyEditing::Month,
        }
    }

    pub fn move_left(&mut self) {
        match self.editing {
            CurrentlyEditing::Year => {
                self.date = self.date.checked_sub_months(Months::new(12)).unwrap()
            }
            CurrentlyEditing::Month => {
                self.date = self.date.checked_sub_months(Months::new(1)).unwrap()
            }
            CurrentlyEditing::Day => self.date = self.date.checked_sub_days(Days::new(1)).unwrap(),
        }
    }

    pub fn move_right(&mut self) {
        match self.editing {
            CurrentlyEditing::Year => {
                self.date = self.date.checked_add_months(Months::new(12)).unwrap()
            }
            CurrentlyEditing::Month => {
                self.date = self.date.checked_add_months(Months::new(1)).unwrap()
            }
            CurrentlyEditing::Day => self.date = self.date.checked_add_days(Days::new(1)).unwrap(),
        }
    }

    pub fn move_up(&mut self) {
        match self.editing {
            CurrentlyEditing::Year => {
                self.date = self.date.checked_sub_months(Months::new(12)).unwrap()
            }
            CurrentlyEditing::Month => {
                self.date = self.date.checked_sub_months(Months::new(4)).unwrap()
            }
            CurrentlyEditing::Day => self.date = self.date.checked_sub_days(Days::new(7)).unwrap(),
        }
    }

    pub fn move_down(&mut self) {
        match self.editing {
            CurrentlyEditing::Year => {
                self.date = self.date.checked_add_months(Months::new(12)).unwrap()
            }
            CurrentlyEditing::Month => {
                self.date = self.date.checked_add_months(Months::new(4)).unwrap()
            }
            CurrentlyEditing::Day => self.date = self.date.checked_add_days(Days::new(7)).unwrap(),
        }
    }

    pub fn choose_selection(&mut self) {
        match self.editing {
            CurrentlyEditing::Year => self.editing = CurrentlyEditing::Month,
            CurrentlyEditing::Month => self.editing = CurrentlyEditing::Day,
            CurrentlyEditing::Day => todo!(),
        }
    }

    pub fn backtrace_selection(&mut self) {
        match self.editing {
            CurrentlyEditing::Year => (),
            CurrentlyEditing::Month => self.editing = CurrentlyEditing::Year,
            CurrentlyEditing::Day => self.editing = CurrentlyEditing::Month,
        }
    }
}

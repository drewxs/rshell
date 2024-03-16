use std::collections::VecDeque;

pub struct History {
    pub entries: VecDeque<String>,
    pub position: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
            position: 0,
        }
    }

    pub fn get_previous(&mut self) -> Option<String> {
        if self.position < self.entries.len() {
            self.position += 1;
            self.entries.get(self.position - 1).cloned()
        } else {
            None
        }
    }

    pub fn get_next(&mut self) -> Option<String> {
        match self.position {
            1 => {
                self.position = 0;
                Some(String::new())
            }
            0 => None,
            _ => {
                self.position -= 1;
                self.entries.get(self.position - 1).cloned()
            }
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        self.entries.push_front(entry);
        self.position = 0;
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub is_done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskList {
    items: Vec<Task>,
    #[serde(skip)]
    selected: Option<usize>,
}

impl Default for TaskList {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
        }
    }

    pub fn from_tasks(tasks: Vec<Task>) -> Self {
        let selected = if tasks.is_empty() { None } else { Some(0) };
        Self {
            items: tasks,
            selected,
        }
    }

    pub fn items(&self) -> &[Task] {
        &self.items
    }

    pub fn into_items(self) -> Vec<Task> {
        self.items
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }

    // Returns the first incomplete task (the "active" focus task).
    pub fn active_task(&self) -> Option<&Task> {
        self.items.iter().find(|t| !t.is_done)
    }

    pub fn add(&mut self, title: String) {
        self.items.push(Task {
            title,
            is_done: false,
        });
        if self.selected.is_none() {
            self.selected = Some(0);
        }
    }

    pub fn edit(&mut self, index: usize, title: String) {
        if let Some(task) = self.items.get_mut(index) {
            task.title = title;
        }
    }

    pub fn delete(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
            if self.items.is_empty() {
                self.selected = None;
            } else if let Some(sel) = self.selected
                && sel >= self.items.len()
            {
                self.selected = Some(self.items.len() - 1);
            }
        }
    }

    pub fn toggle(&mut self, index: usize) {
        if let Some(task) = self.items.get_mut(index) {
            task.is_done = !task.is_done;
        }
    }

    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.selected {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }

    pub fn select_previous(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.selected {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }

    pub fn move_up(&mut self) {
        match self.selected {
            Some(0) | None => (),
            Some(i) => {
                self.items.swap(i, i - 1);
                self.selected = Some(i - 1);
            }
        }
    }

    pub fn move_down(&mut self) {
        match self.selected {
            None => (),
            Some(i) if i >= self.items.len() - 1 => (),
            Some(i) => {
                self.items.swap(i, i + 1);
                self.selected = Some(i + 1);
            }
        }
    }

    pub fn move_task(&mut self, from: usize, to: usize) {
        if from < self.items.len() && to < self.items.len() && from != to {
            let task = self.items.remove(from);
            self.items.insert(to, task);
            self.selected = Some(to);
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
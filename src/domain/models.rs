use std::{fmt::{self, Display}, error};

#[derive(Debug, PartialEq)]
enum TodoStatus {
    Pending,
    Skipped,
    Completed,
}

struct Todo {
    title: String,
    status: TodoStatus,
    is_deleted: bool,
}


impl Todo {
    fn new(title: &str) -> Todo {
        Todo { title: title.to_string(), status: TodoStatus::Pending, is_deleted: false }
    }

    pub fn skip(&mut self) -> Result<(), TodoError> {
        if self.is_deleted {
            return Err(TodoError::new("Cannot skip a deleted todo"));
        }
        match self.status {
            TodoStatus::Completed => {
                Err(TodoError::new("Cannot skip a completed todo"))
            },
            _ => {
                self.status = TodoStatus::Skipped;
                Ok(())
            }
        }
    }

    pub fn complete(&mut self) -> Result<(), TodoError> {
        if self.is_deleted {
            Err(TodoError::new("Cannot complete a deleted todo"))
        } else {
            self.status = TodoStatus::Completed;
            Ok(())
        }
    }

    pub fn delete(&mut self) {
        self.is_deleted = true;
    }
}

#[derive(Clone, Debug)]
struct TodoError {
    message: String,
}

impl TodoError {
    fn new(message: &str) -> TodoError {
        Self {
            message: message.to_string(),
        }
    }
}

impl Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid action for Todo")
    }
}

impl error::Error for TodoError {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_model() {
        let todo = Todo::new("Take logan for a walk");
        assert_eq!(todo.title, "Take logan for a walk");
        assert_eq!(todo.status, TodoStatus::Pending);
    }

    #[test]
    fn test_skip_todo() {
        let mut todo = Todo::new("Take logan for a walk");
        assert_eq!(todo.status, TodoStatus::Pending);
        let res = todo.skip().unwrap();
        assert_eq!(res, ());
        assert_eq!(todo.status, TodoStatus::Skipped);
    }
    
    #[test]
    fn test_complete_todo() {
        let mut todo = Todo::new("Take logan for a walk");
        assert_eq!(todo.status, TodoStatus::Pending);
        todo.complete();
        assert_eq!(todo.status, TodoStatus::Completed);
    }

    #[test]
    fn test_cannot_skip_todo_completed() {
        let mut todo = Todo::new("Take logan for a walk");
        todo.complete().unwrap();
        let _error = todo.skip().unwrap_err();
        assert_eq!(_error.message, "Cannot skip a completed todo");
    }

    #[test]
    fn test_delete_todo() {
        let mut todo = Todo::new("Delete todo");
        assert!(!todo.is_deleted);
        todo.delete();
        assert!(todo.is_deleted);
    }

    #[test]
    fn test_cannot_complete_a_deleted_todo() {
        let mut todo = Todo::new("Delete todo");
        todo.delete();
        let error = todo.complete().unwrap_err();
        assert_eq!(error.message, "Cannot complete a deleted todo");
    }

    #[test]
    fn test_cannot_skip_a_deleted_todo() {
        let mut todo = Todo::new("Delete todo");
        todo.delete();
        let error = todo.skip().unwrap_err();
        assert_eq!(error.message, "Cannot skip a deleted todo");
    }
}
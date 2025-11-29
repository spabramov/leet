use std::collections::{BTreeMap, BTreeSet, HashMap};
type Priority = i32;
type UserId = i32;
type TaskId = i32;

#[derive(Debug)]
struct TaskManager {
    tasks: HashMap<TaskId, (UserId, Priority)>,
    queue: BTreeMap<Priority, BTreeSet<TaskId>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut manager = TaskManager {
            tasks: HashMap::new(),
            queue: BTreeMap::new(),
        };
        for entry in tasks.into_iter() {
            if let &[user_id, task_id, priority] = entry.as_slice() {
                manager.add(user_id, task_id, priority)
            } else {
                panic!("not a valid format")
            }
        }
        manager
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.tasks.insert(task_id, (user_id, priority));
        self.queue.entry(priority).or_default().insert(task_id);
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        let (user_id, _) = *self.tasks.get(&task_id).expect("Task doesn't exist");
        self.rmv(task_id);
        self.add(user_id, task_id, new_priority);
    }

    fn rmv(&mut self, task_id: i32) {
        let (_, priority) = self.tasks.remove(&task_id).expect("Task doesn't exist");
        let set = self.queue.get_mut(&priority).expect("Queue doesn't exist");
        set.remove(&task_id);
        if set.is_empty() {
            self.queue.remove(&priority);
        };
    }

    fn exec_top(&mut self) -> i32 {
        dbg!(&self.queue);
        let task_id = if let Some(entry) = self.queue.last_entry() {
            entry.get().last().copied().unwrap_or(-1)
        } else {
            -1
        };
        let (user_id, _) = self.tasks.get(&task_id).copied().unwrap_or((-1, -1));
        if task_id != -1 {
            self.rmv(task_id);
        }
        user_id
    }
}

/**
 * Your TaskManager object will be instantiated and called as such:
 * let obj = TaskManager::new(tasks);
 * obj.add(userId, taskId, priority);
 * obj.edit(taskId, newPriority);
 * obj.rmv(taskId);
 * let ret_4: i32 = obj.exec_top();
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case1() {
        let mut obj = TaskManager::new(vec![vec![1, 101, 10], vec![2, 102, 20], vec![3, 103, 15]]);
        obj.add(4, 104, 5);
        obj.edit(102, 8);
        assert_eq!(obj.exec_top(), 3);
        obj.rmv(101);
        obj.add(5, 105, 15);
        assert_eq!(obj.exec_top(), 5);
    }

    #[test]
    fn test_case2() {
        let mut obj = TaskManager::new(vec![vec![1, 101, 8], vec![2, 102, 20], vec![3, 103, 5]]);
        obj.add(4, 104, 5);
        obj.edit(102, 9);
        dbg!(&obj);
        assert_eq!(obj.exec_top(), 2);
        obj.rmv(101);
        obj.add(0, 101, 8);
        assert_eq!(obj.exec_top(), 0);
    }

    #[test]
    fn test_case3() {
        let mut obj = TaskManager::new(vec![vec![10, 26, 25]]);
        obj.rmv(26);

        assert_eq!(obj.exec_top(), -1);
    }
}

use super::{ActionUndo, BaseTaskAction, TaskAction};
use crate::Printer;

use crate::task::TaskData;

pub struct InfoTaskAction {
    pub base: BaseTaskAction,
}

impl TaskAction for InfoTaskAction {
    impl_taskaction_from_base!();
    fn do_action(&mut self, printer: &dyn Printer) -> Result<(), String> {
        for task in self.base.get_tasks().to_vec() {
            printer.print_task_info(task)?;
        }
        Ok(())
    }
}

impl InfoTaskAction {
    pub fn get_command_description() -> String {
        "Show a list of tasks".to_string()
    }
}

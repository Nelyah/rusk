use super::{ActionUndo, BaseTaskAction, TaskAction};
use crate::Printer;

use crate::task::TaskData;

pub struct ExportTaskAction {
    pub base: BaseTaskAction,
}

impl TaskAction for ExportTaskAction {
    impl_taskaction_from_base!();
    fn do_action(&mut self, printer: &dyn Printer) -> Result<(), String> {
        printer.show_information_message(
            &serde_json::to_string_pretty(self.base.get_tasks()).unwrap(),
        );
        Ok(())
    }
}

impl ExportTaskAction {
    pub fn get_command_description() -> String {
        r#"Print the tasks as JSON format.
This is useful for scripting access to Rusk.
Both <filters> and <arguments> are treated as filter
"#
        .to_string()
    }
}

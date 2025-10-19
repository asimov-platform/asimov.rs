// This is free and unencumbered software released into the public domain.

use std::process::Command;

pub enum PipelineStep {
    Reader {
        program: String,
        args: Vec<String>,
    },
    Writer {
        program: String,
        args: Vec<String>,
    },
    Custom {
        name: String,
        program: String,
        args: Vec<String>,
    },
}

impl PipelineStep {
    fn get_executable_name(&self) -> &str {
        // let (name, program) = self.get_name_and_program();
        // format!("asimov-{}-{}", name, program)
        self.get_name_and_program().1
    }

    fn get_name_and_program(&self) -> (&str, &str) {
        match self {
            PipelineStep::Reader { program, .. } => ("reader", program),
            PipelineStep::Writer { program, .. } => ("writer", program),
            PipelineStep::Custom { name, program, .. } => (name, program),
        }
    }

    fn get_args(&self) -> Vec<String> {
        match self {
            PipelineStep::Reader { args, .. } => args.clone(),
            PipelineStep::Writer { args, .. } => args.clone(),
            PipelineStep::Custom { args, .. } => args.clone(),
        }
    }

    pub(crate) fn to_command(self) -> Command {
        let mut cmd = Command::new(self.get_executable_name());
        cmd.args(self.get_args());
        cmd
    }
}

// This is free and unencumbered software released into the public domain.

use std::process::Command;

pub enum PipelineStep {
    Reader { program: String, args: Vec<String> },
    Writer { program: String, args: Vec<String> },
    Emitter { program: String, args: Vec<String> },
    Custom { program: String, args: Vec<String> },
}

impl PipelineStep {
    fn get_name_and_args(&self) -> (String, Vec<String>) {
        match self {
            PipelineStep::Reader { program, args } => {
                (format!("asimov-{}-reader", program), args.clone())
            },
            PipelineStep::Writer { program, args } => {
                (format!("asimov-{}-writer", program), args.clone())
            },
            PipelineStep::Emitter { program, args } => {
                (format!("asimov-{}-emitter", program), args.clone())
            },
            PipelineStep::Custom { program, args } => (program.clone(), args.clone()),
        }
    }

    pub(crate) fn to_command(self) -> Command {
        let (program, args) = self.get_name_and_args();
        let mut cmd = Command::new(program);
        cmd.args(args);
        cmd
    }
}

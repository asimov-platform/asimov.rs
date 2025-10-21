// This is free and unencumbered software released into the public domain.

use asimov_runner::{Pipeline, PipelineStep};

#[test]
fn test_pipeline() {
    let pipeline = Pipeline {
        steps: vec![
            PipelineStep::Reader {
                program: "echo".to_string(),
                args: vec!["hello world".to_string()],
            },
            PipelineStep::Writer {
                program: "cat".to_string(),
                args: vec![],
            },
            PipelineStep::Writer {
                program: "cat".to_string(),
                args: vec![],
            },
        ],
    };

    let result = pipeline.execute();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap().into_inner(), b"hello world\n");
}

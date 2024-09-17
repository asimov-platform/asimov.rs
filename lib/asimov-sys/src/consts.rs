// This is free and unencumbered software released into the public domain.

pub const ASI_NULL_HANDLE: AsiInstance = 0;

const _: () = assert!(
    size_of::<AsiFlowExecutionState>() == 4,
    "sizeof(AsiFlowExecutionState) == 4"
);
const _: () = assert!(size_of::<AsiPortState>() == 4, "sizeof(AsiPortState) == 4");
const _: () = assert!(size_of::<AsiPortType>() == 4, "sizeof(AsiPortType) == 4");
const _: () = assert!(size_of::<AsiResult>() == 4, "sizeof(AsiResult) == 4");
const _: () = assert!(
    size_of::<AsiStructureType>() == 4,
    "sizeof(AsiStructureType) == 4"
);

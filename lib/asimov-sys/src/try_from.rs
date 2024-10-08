// This is free and unencumbered software released into the public domain.

impl TryFrom<c_int> for AsiFlowExecutionState {
    type Error = ();

    fn try_from(code: c_int) -> Result<Self, Self::Error> {
        FromPrimitive::from_i64(code as _).ok_or(())
    }
}

impl TryFrom<c_int> for AsiPortDirection {
    type Error = ();

    fn try_from(code: c_int) -> Result<Self, Self::Error> {
        FromPrimitive::from_i64(code as _).ok_or(())
    }
}

impl TryFrom<c_int> for AsiPortState {
    type Error = ();

    fn try_from(code: c_int) -> Result<Self, Self::Error> {
        FromPrimitive::from_i64(code as _).ok_or(())
    }
}

impl TryFrom<c_int> for AsiResult {
    type Error = ();

    fn try_from(code: c_int) -> Result<Self, Self::Error> {
        FromPrimitive::from_i64(code as _).ok_or(())
    }
}

impl TryFrom<c_int> for AsiStructureType {
    type Error = ();

    fn try_from(code: c_int) -> Result<Self, Self::Error> {
        FromPrimitive::from_i64(code as _).ok_or(())
    }
}

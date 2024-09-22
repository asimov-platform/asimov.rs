// This is free and unencumbered software released into the public domain.

impl Display for AsiBlockDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiBlockDefinition")
            .field("name", &self.name_lossy())
            .field("input_port_count", &self.input_port_count)
            .field("output_port_count", &self.output_port_count)
            .field("parameter_count", &self.parameter_count)
            .finish()
    }
}

impl Display for AsiBlockExecuteInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiBlockExecuteInfo")
            .field("name", &self.name_lossy())
            .field("params", &self.params_lossy())
            .finish()
    }
}

impl Display for AsiBlockExecution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiBlockExecution")
            .field("name", &self.name_lossy())
            .field("pid", &self.pid)
            .field("state", &self.state)
            .finish()
    }
}

impl Display for AsiBlockParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiBlockParameter")
            .field("name", &self.name_lossy())
            .field("default_value", &self.default_value_lossy())
            .finish()
    }
}

impl Display for AsiBlockPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiBlockPort")
            .field("name", &self.name_lossy())
            .field("type", &self.type_)
            .finish()
    }
}

impl Display for AsiBlockUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiBlockUsage")
            .field("name", &self.name_lossy())
            .field("type", &self.type_lossy())
            .finish()
    }
}

impl Display for AsiFlowConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiFlowConnection")
            .field("source_block", &self.source_block_lossy())
            .field("source_port", &self.source_port_lossy())
            .field("target_block", &self.target_block_lossy())
            .field("target_port", &self.target_port_lossy())
            .finish()
    }
}

impl Display for AsiFlowCreateInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiFlowCreateInfo")
            .field("name", &self.name_lossy())
            .finish()
    }
}

impl Display for AsiFlowDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiFlowDefinition")
            .field("name", &self.name_lossy())
            .field("block_count", &self.block_count)
            .finish()
    }
}

impl Display for AsiFlowExecuteInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiFlowExecuteInfo")
            .field("name", &self.name_lossy())
            .field("params", &self.params_lossy())
            .finish()
    }
}

impl Display for AsiFlowExecution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiFlowExecution")
            .field("name", &self.name_lossy())
            .field("pid", &self.pid)
            .field("state", &self.state)
            .finish()
    }
}

impl Display for AsiFlowExecutionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for AsiFlowUpdateInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiFlowUpdateInfo")
            .field("name", &self.name_lossy())
            .field("new_name", &self.new_name_lossy())
            .finish()
    }
}

//impl Display for AsiInstance {}

impl Display for AsiModelDownloadInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiModelDownloadInfo")
            .field("name", &self.name_lossy())
            .finish()
    }
}

impl Display for AsiModelManifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiModelManifest")
            .field("name", &self.name_lossy())
            .field("size", &self.size)
            .finish()
    }
}

impl Display for AsiModelTensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiModelTensor")
            .field("name", &self.name_lossy())
            .finish()
    }
}

impl Display for AsiModuleEnableInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiModuleEnableInfo")
            .field("name", &self.name_lossy())
            .field("enabled", &self.enabled)
            .finish()
    }
}

impl Display for AsiModuleRegistration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiModuleRegistration")
            .field("name", &self.name_lossy())
            .field("block_count", &self.block_count)
            .finish()
    }
}

impl Display for AsiPortState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for AsiPortType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for AsiResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for AsiStructureHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiStructureHeader")
            .field("type", &self.type_)
            .field("next", &self.next)
            .finish()
    }
}

impl Display for AsiStructureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

//impl Display for AsiVersion {}

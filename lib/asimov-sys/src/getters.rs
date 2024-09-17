// This is free and unencumbered software released into the public domain.

impl AsiBlockDefinition {
    pub fn new(
        name: &str,
        input_port_count: u32,
        output_port_count: u32,
        parameter_count: u32,
    ) -> Self {
        Self {
            name: string_to_static_array(name),
            input_port_count,
            output_port_count,
            parameter_count,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }

    pub fn port_count(&self) -> usize {
        self.input_port_count() + self.output_port_count()
    }

    pub fn input_port_count(&self) -> usize {
        self.input_port_count as _
    }

    pub fn output_port_count(&self) -> usize {
        self.output_port_count as _
    }

    pub fn parameter_count(&self) -> usize {
        self.parameter_count as _
    }
}

impl AsiBlockExecuteInfo {
    pub fn new(name: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiBlockParameter {
    pub fn new(name: &str, default_value: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            default_value: string_to_static_array(default_value),
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }

    pub fn default_value(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.default_value.as_ptr()) }.to_str()
    }

    pub fn default_value_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.default_value.as_ptr()) }.to_string_lossy()
    }
}

impl AsiBlockPort {
    pub fn new(name: &str, r#type: AsiPortType) -> Self {
        Self {
            name: string_to_static_array(name),
            type_: r#type,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiBlockUsage {
    pub fn new(name: &str, r#type: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            type_: string_to_static_array(r#type),
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }

    pub fn r#type(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.type_.as_ptr()) }.to_str()
    }

    pub fn type_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.type_.as_ptr()) }.to_string_lossy()
    }
}

impl AsiFlowConnection {
    pub fn new(
        source_block: &str,
        source_port: &str,
        target_block: &str,
        target_port: &str,
    ) -> Self {
        Self {
            source_block: string_to_static_array(source_block),
            source_port: string_to_static_array(source_port),
            target_block: string_to_static_array(target_block),
            target_port: string_to_static_array(target_port),
            ..Default::default()
        }
    }

    pub fn source_block(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.source_block.as_ptr()) }.to_str()
    }

    pub fn source_block_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.source_block.as_ptr()) }.to_string_lossy()
    }

    pub fn source_port(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.source_port.as_ptr()) }.to_str()
    }

    pub fn source_port_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.source_port.as_ptr()) }.to_string_lossy()
    }

    pub fn target_block(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.target_block.as_ptr()) }.to_str()
    }

    pub fn target_block_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.target_block.as_ptr()) }.to_string_lossy()
    }

    pub fn target_port(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.target_port.as_ptr()) }.to_str()
    }

    pub fn target_port_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.target_port.as_ptr()) }.to_string_lossy()
    }
}

impl AsiFlowCreateInfo {
    pub fn new(name: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiFlowDefinition {
    pub fn new(name: &str, block_count: u32) -> Self {
        Self {
            name: string_to_static_array(name), // truncated as needed
            block_count,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiFlowExecuteInfo {
    pub fn new(name: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiFlowExecution {
    pub fn named(name: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            ..Default::default()
        }
    }

    pub fn new(name: &str, pid: u64, state: AsiFlowExecutionState) -> Self {
        Self {
            name: string_to_static_array(name),
            pid,
            state,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiFlowExecutionState {}

impl AsiFlowUpdateInfo {
    pub fn new(name: &str, new_name: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            new_name: string_to_static_array(new_name),
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }

    pub fn new_name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.new_name.as_ptr()) }.to_str()
    }

    pub fn new_name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.new_name.as_ptr()) }.to_string_lossy()
    }
}

//impl AsiInstance {}

impl AsiModelDownloadInfo {
    pub fn new(name: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiModelManifest {
    pub fn new(name: &str, size: u64) -> Self {
        Self {
            name: string_to_static_array(name),
            size,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiModelTensor {
    pub fn new(name: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiModuleEnableInfo {
    pub fn new(name: &str, enabled: bool) -> Self {
        Self {
            name: string_to_static_array(name),
            enabled,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiModuleRegistration {
    pub fn new(name: &str, block_count: u32) -> Self {
        Self {
            name: string_to_static_array(name),
            block_count,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiPortState {}

impl AsiPortType {}

impl AsiResult {}

impl AsiStructureHeader {}

impl AsiStructureType {}

//impl AsiVersion {}

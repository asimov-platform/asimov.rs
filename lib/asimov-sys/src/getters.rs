// This is free and unencumbered software released into the public domain.

use alloc::{format, string::String, vec::Vec};

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
    pub fn with_params(name: &str, params: &Vec<(String, String)>) -> Self {
        let params: String =
            params
                .iter()
                .enumerate()
                .fold(String::new(), |mut result, (i, (k, v))| {
                    if i > 0 {
                        result.push(' ');
                    }
                    result.push_str(&format!("{}={}", k, v));
                    result
                });
        Self {
            name: string_to_static_array(name),
            params: string_to_static_array(&params),
            ..Default::default()
        }
    }

    pub fn new(name: &str, params: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            params: string_to_static_array(params),
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }

    pub fn params(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.params.as_ptr()) }.to_str()
    }

    pub fn params_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.params.as_ptr()) }.to_string_lossy()
    }
}

impl AsiBlockExecution {
    pub fn named(name: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            ..Default::default()
        }
    }

    pub fn new(name: &str, timestamp: u64, pid: u64, state: AsiFlowExecutionState) -> Self {
        Self {
            timestamp,
            pid,
            state,
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
    pub fn new(name: &str, r#type: &str, default_value: Option<&str>) -> Self {
        Self {
            name: string_to_static_array(name),
            type_: string_to_static_array(r#type),
            default_value: string_to_static_array(default_value.unwrap_or_default()),
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }

    pub fn r#type(&self) -> Result<Option<&str>, Utf8Error> {
        unsafe { CStr::from_ptr(self.type_.as_ptr()) }
            .to_str()
            .map(|str| if str.is_empty() { None } else { Some(str) })
    }

    pub fn type_lossy(&self) -> Option<Cow<'_, str>> {
        let str = unsafe { CStr::from_ptr(self.type_.as_ptr()) }.to_string_lossy();
        if str.is_empty() {
            None
        } else {
            Some(str)
        }
    }

    pub fn default_value(&self) -> Result<Option<&str>, Utf8Error> {
        unsafe { CStr::from_ptr(self.default_value.as_ptr()) }
            .to_str()
            .map(|str| if str.is_empty() { None } else { Some(str) })
    }

    pub fn default_value_lossy(&self) -> Option<Cow<'_, str>> {
        let str = unsafe { CStr::from_ptr(self.default_value.as_ptr()) }.to_string_lossy();
        if str.is_empty() {
            None
        } else {
            Some(str)
        }
    }
}

impl AsiBlockPort {
    pub fn new(direction: AsiPortDirection, name: &str, r#type: &str) -> Self {
        Self {
            direction,
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

    pub fn r#type(&self) -> Result<Option<&str>, Utf8Error> {
        unsafe { CStr::from_ptr(self.type_.as_ptr()) }
            .to_str()
            .map(|str| if str.is_empty() { None } else { Some(str) })
    }

    pub fn type_lossy(&self) -> Option<Cow<'_, str>> {
        let str = unsafe { CStr::from_ptr(self.type_.as_ptr()) }.to_string_lossy();
        if str.is_empty() {
            None
        } else {
            Some(str)
        }
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
    pub fn with_params(name: &str, params: &Vec<(String, String)>) -> Self {
        let params: String =
            params
                .iter()
                .enumerate()
                .fold(String::new(), |mut result, (i, (k, v))| {
                    if i > 0 {
                        result.push(' ');
                    }
                    result.push_str(&format!("{}={}", k, v));
                    result
                });
        Self {
            name: string_to_static_array(name),
            params: string_to_static_array(&params),
            ..Default::default()
        }
    }

    pub fn new(name: &str, params: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            params: string_to_static_array(params),
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }

    pub fn params(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.params.as_ptr()) }.to_str()
    }

    pub fn params_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.params.as_ptr()) }.to_string_lossy()
    }
}

impl AsiFlowExecution {
    pub fn named(name: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            ..Default::default()
        }
    }

    pub fn new(name: &str, timestamp: u64, pid: u64, state: AsiFlowExecutionState) -> Self {
        Self {
            timestamp,
            pid,
            state,
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

impl AsiPortDirection {}

impl AsiPortState {}

impl AsiResult {}

impl AsiStructureHeader {}

impl AsiStructureType {}

//impl AsiVersion {}

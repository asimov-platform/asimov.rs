// This is free and unencumbered software released into the public domain.

use crate::{
    flow::{FlowDefinition, FlowDefinitionIter, FlowExecution, LocalFlowDefinition},
    prelude::{format, null, Box},
    BlockDefinition, BlockDefinitionIter, BlockExecution, Error, ModelManifest, ModelManifestIter,
    ModuleRegistration, ModuleRegistrationIter, Result,
};
use asimov_sys::{
    asiCloneFlow, asiCreateFlow, asiCreateInstance, asiDestroyInstance, asiDownloadModel,
    asiEnableModule, asiExecuteBlock, asiExecuteFlow, asiPollFlowExecution, asiRemoveFlow,
    asiStartFlowExecution, asiStopFlowExecution, asiUpdateFlow, AsiBlockExecuteInfo,
    AsiBlockExecution, AsiFlowCreateInfo, AsiFlowDefinition, AsiFlowExecuteInfo, AsiFlowExecution,
    AsiFlowExecutionState, AsiFlowUpdateInfo, AsiInstance, AsiModelDownloadInfo,
    AsiModuleEnableInfo, AsiResult, AsiStructureHeader, ASI_NULL_HANDLE,
};

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Instance {
    handle: AsiInstance,
}

impl Instance {
    pub fn new() -> Result<Self> {
        let mut instance = Self {
            handle: ASI_NULL_HANDLE,
        };
        match unsafe { asiCreateInstance(null(), &mut instance.handle) } {
            AsiResult::ASI_SUCCESS => Ok(instance),
            error => Err(error.try_into().unwrap()),
        }
    }

    #[stability::unstable]
    pub fn has_feature(&self, _name: &str) -> Result<bool> {
        Ok(false) // TODO
    }

    #[stability::unstable]
    pub fn has_module(&self, _name: &str) -> Result<bool> {
        Ok(false) // TODO
    }

    #[stability::unstable]
    pub fn blocks(&self) -> Result<impl Iterator<Item = Box<dyn BlockDefinition>>> {
        BlockDefinitionIter::try_from(self.handle)
    }

    #[stability::unstable]
    pub fn constructs(&self) -> Result<()> {
        todo!("Instance#constructs") // TODO
    }

    #[stability::unstable]
    pub fn flows(&self) -> Result<impl Iterator<Item = Box<dyn FlowDefinition>>> {
        FlowDefinitionIter::try_from(self.handle)
    }

    #[stability::unstable]
    pub fn models(&self) -> Result<impl Iterator<Item = Box<dyn ModelManifest>>> {
        ModelManifestIter::try_from(self.handle)
    }

    #[stability::unstable]
    pub fn modules(&self) -> Result<impl Iterator<Item = Box<dyn ModuleRegistration>>> {
        ModuleRegistrationIter::try_from(self.handle)
    }

    #[stability::unstable]
    pub fn schemas(&self) -> Result<()> {
        todo!("Instance#schemas") // TODO
    }

    #[stability::unstable]
    pub fn vaults(&self) -> Result<()> {
        todo!("Instance#vaults") // TODO
    }

    #[stability::unstable]
    pub fn lookup_module(&self, name: &str) -> Option<Box<dyn ModuleRegistration>> {
        self.modules().ok()?.find(|module| module.name() == name)
    }

    #[stability::unstable]
    pub fn enable_module(&self, name: &str) -> Result<()> {
        self.toggle_module(name, true)
    }

    #[stability::unstable]
    pub fn disable_module(&self, name: &str) -> Result<()> {
        self.toggle_module(name, false)
    }

    #[stability::unstable]
    pub fn toggle_module(&self, name: &str, enabled: bool) -> Result<()> {
        let request = AsiModuleEnableInfo::new(name, enabled);
        match unsafe { asiEnableModule(self.handle, &request) } {
            AsiResult::ASI_SUCCESS => Ok(()),
            error => Err(error.try_into().unwrap()),
        }
    }

    #[stability::unstable]
    pub fn download_model(&self, name: &str) -> Result<()> {
        let request = AsiModelDownloadInfo::new(name);
        match unsafe { asiDownloadModel(self.handle, &request) } {
            AsiResult::ASI_SUCCESS => Ok(()),
            error => Err(error.try_into().unwrap()),
        }
    }

    #[stability::unstable]
    pub fn execute_block(&self, name: &str) -> Result<BlockExecution> {
        let request = AsiBlockExecuteInfo::new(name);
        let mut response = AsiBlockExecution::default();
        match unsafe { asiExecuteBlock(self.handle, &request, &mut response) } {
            AsiResult::ASI_SUCCESS => Ok(BlockExecution::from(response)),
            error => Err(error.try_into().unwrap()),
        }
    }

    #[stability::unstable]
    pub fn open_flow(&self, name: &str) -> Result<Box<dyn FlowDefinition>> {
        Ok(Box::new(LocalFlowDefinition::named(self.handle, name)))
    }

    #[stability::unstable]
    pub fn create_flow(&self, name: &str) -> Result<Box<dyn FlowDefinition>> {
        let request = AsiFlowCreateInfo::new(name);
        let mut response = LocalFlowDefinition::default();
        match unsafe { asiCreateFlow(self.handle, &request, &mut response.inner) } {
            AsiResult::ASI_SUCCESS => Ok(Box::new(LocalFlowDefinition::from(response))),
            error => Err(error.try_into().unwrap()),
        }
    }

    #[stability::unstable]
    pub fn remove_flow(&self, name: &str) -> Result<()> {
        let request = AsiFlowDefinition::new(name, 0);
        match unsafe { asiRemoveFlow(self.handle, &request) } {
            AsiResult::ASI_SUCCESS => Ok(()),
            error => Err(error.try_into().unwrap()),
        }
    }

    #[stability::unstable]
    pub fn rename_flow(&self, old_name: &str, new_name: &str) -> Result<()> {
        let request = AsiFlowUpdateInfo::new(old_name, new_name);
        match unsafe { asiUpdateFlow(self.handle, &request) } {
            AsiResult::ASI_SUCCESS => Ok(()),
            error => Err(error.try_into().unwrap()),
        }
    }

    #[stability::unstable]
    pub fn clone_flow(&self, old_name: &str, new_name: &str) -> Result<()> {
        let request = AsiFlowUpdateInfo::new(old_name, new_name);
        match unsafe { asiCloneFlow(self.handle, &request) } {
            AsiResult::ASI_SUCCESS => Ok(()),
            error => Err(error.try_into().unwrap()),
        }
    }

    #[stability::unstable]
    pub fn execute_flow(&self, name: &str) -> Result<FlowExecution> {
        let request = AsiFlowExecuteInfo::new(name);
        let mut response = AsiFlowExecution::default();
        match unsafe { asiExecuteFlow(self.handle, &request, &mut response) } {
            AsiResult::ASI_SUCCESS => Ok(FlowExecution::from(response)),
            error => Err(error.try_into().unwrap()),
        }
    }

    #[stability::unstable]
    pub fn start_flow_execution(&self, name: &str) -> Result<FlowExecution> {
        let request = AsiFlowExecuteInfo::new(name);
        let mut response = AsiFlowExecution::default();
        match unsafe { asiStartFlowExecution(self.handle, &request, &mut response) } {
            AsiResult::ASI_SUCCESS => Ok(FlowExecution::from(response)),
            error => Err(error.try_into().unwrap()),
        }
    }

    #[stability::unstable]
    pub fn poll_flow_execution(&self, name: &str) -> Result<AsiFlowExecutionState> {
        let request = AsiFlowExecution::named(name);
        let mut response = AsiFlowExecutionState::default();
        match unsafe { asiPollFlowExecution(self.handle, &request, &mut response) } {
            AsiResult::ASI_SUCCESS => Ok(response),
            error => Err(error.try_into().unwrap()),
        }
    }

    #[stability::unstable]
    pub fn stop_flow_execution(&self, name: &str) -> Result<()> {
        let request = AsiFlowExecution::named(name);
        match unsafe { asiStopFlowExecution(self.handle, &request) } {
            AsiResult::ASI_SUCCESS => Ok(()),
            error => Err(error.try_into().unwrap()),
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        if self.handle == ASI_NULL_HANDLE {
            return;
        }
        match unsafe { asiDestroyInstance(self.handle) } {
            AsiResult::ASI_SUCCESS => self.handle = ASI_NULL_HANDLE,
            _ => unreachable!("instance should be destroyed"),
        }
    }
}

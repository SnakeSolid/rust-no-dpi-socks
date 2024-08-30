use std::sync::Arc;

use tokio::runtime::Runtime;

use crate::arguments::Arguments;

#[derive(Debug)]
struct ContextImpl {
    bind_address: String,
    bind_port: u16,
    runtime: Runtime,
}

impl ContextImpl {
    pub fn create(arguments: Arguments, runtime: Runtime) -> Self {
        ContextImpl {
            bind_address: arguments.bind_address().into(),
            bind_port: arguments.bind_port(),
            runtime: runtime,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    inner: Arc<ContextImpl>,
}

impl Context {
    pub fn create(arguments: Arguments, runtime: Runtime) -> Self {
        Context {
            inner: Arc::new(ContextImpl::create(arguments, runtime)),
        }
    }

    pub fn bind_address(&self) -> &str {
        &self.inner.bind_address
    }

    pub fn bind_port(&self) -> u16 {
        self.inner.bind_port
    }

    pub fn runtime(&self) -> &Runtime {
        &self.inner.runtime
    }
}

// Copyright 2021 Pants project contributors (see CONTRIBUTORS.md).
// Licensed under the Apache License, Version 2.0 (see LICENSE).

use std::collections::HashMap;

use crate::externs::interop::EngineException::{
    BrokenPipeError, KeyboardInterruptError, SciondClientException,
};
use crate::externs::interop::EngineResult;
use crate::externs::scheduler::PyExecutor;
use task_executor::Executor;

struct NailgunClient {
    port: u16,
    executor: Executor,
}

impl NailgunClient {
    fn new(port: u16, py_executor: &PyExecutor) -> Self {
        Self {
            port,
            executor: py_executor.0.clone(),
        }
    }

    fn execute(
        &self,
        command: String,
        args: Vec<String>,
        env: &HashMap<String, String>,
    ) -> EngineResult<i32> {
        use nailgun::NailgunClientError;

        // NB: We assume that env var names and values are Python strs strictly convertible to UTF-8
        // (that is, with no lone surrogates representing invalid UTF-8 passed from the OS).
        // The Python-side caller must ensure this.
        let env_list: Vec<(String, String)> = env
            .to_owned()
            .into_iter()
            .collect::<Vec<(String, String)>>();

        self.executor
            .block_on(nailgun::client_execute(self.port, command, args, env_list))
            .map_err(|e| match e {
                NailgunClientError::PreConnect(err_str) => SciondClientException(err_str),
                NailgunClientError::PostConnect(err_str) => SciondClientException(err_str),
                NailgunClientError::BrokenPipe => BrokenPipeError,
                NailgunClientError::KeyboardInterrupt => KeyboardInterruptError,
            })
    }
}

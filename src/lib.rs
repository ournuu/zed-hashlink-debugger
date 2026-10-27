use zed::serde_json::{Value, json};
use zed_extension_api::{self as zed, StartDebuggingRequestArgumentsRequest as RequestKind};

struct HashLinkExtension;

impl zed::Extension for HashLinkExtension {
    fn new() -> Self {
        Self
    }

    fn get_dap_binary(
        &mut self,
        _adapter_name: String,
        config: zed::DebugTaskDefinition,
        user_provided_debug_adapter_path: Option<String>,
        worktree: &zed::Worktree,
    ) -> Result<zed::DebugAdapterBinary, String> {
        let mut config: Value = zed::serde_json::from_str(&config.config)
            .map_err(|error| format!("Invalid HashLink configuration: {error}"))?;
        let request = self.dap_request_kind("HashLink".into(), config.clone())?;
        if !config
            .get("program")
            .and_then(Value::as_str)
            .is_some_and(|program| program.ends_with(".hl"))
        {
            return Err("Set program to the .hl bytecode file built with haxe -debug (also required for attach)".into());
        }
        let adapter_path = user_provided_debug_adapter_path
            .or_else(|| {
                config
                    .get("adapterPath")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
            })
            .ok_or("Set adapterPath to the absolute path of the built adapter/adapter.js")?;
        let node = worktree
            .which("node")
            .map(Ok)
            .unwrap_or_else(zed::node_binary_path)?;
        let cwd = config
            .get("cwd")
            .and_then(Value::as_str)
            .unwrap_or(&worktree.root_path())
            .to_owned();
        config["cwd"] = json!(cwd);
        if config.get("classPaths").is_none() {
            config["classPaths"] = json!([cwd]);
        }
        if config.get("port").is_none() {
            config["port"] = json!(6112);
        }
        if let Some(object) = config.as_object_mut() {
            object.remove("adapterPath");
        }
        Ok(zed::DebugAdapterBinary {
            command: Some(node),
            arguments: vec![adapter_path],
            envs: vec![],
            cwd: Some(cwd),
            connection: None,
            request_args: zed::StartDebuggingRequestArguments {
                configuration: config.to_string(),
                request,
            },
        })
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        config: Value,
    ) -> Result<RequestKind, String> {
        match config.get("request").and_then(Value::as_str) {
            Some("launch") => Ok(RequestKind::Launch),
            Some("attach") => Ok(RequestKind::Attach),
            _ => Err("HashLink request must be 'launch' or 'attach'".into()),
        }
    }

    fn dap_config_to_scenario(
        &mut self,
        config: zed::DebugConfig,
    ) -> Result<zed::DebugScenario, String> {
        if config.stop_on_entry == Some(true) {
            return Err(
                "HashLink stop on entry is not supported; set a source breakpoint instead".into(),
            );
        }
        let zed::DebugRequest::Launch(launch) = config.request else {
            return Err("For HashLink attach, set program and port in .zed/debug.json".into());
        };
        Ok(zed::DebugScenario {
            label: config.label,
            adapter: config.adapter,
            build: None,
            config: json!({
                "request": "launch",
                "program": launch.program,
                "args": launch.args,
                "cwd": launch.cwd,
                "env": launch.envs.into_iter().map(|(key, value)| (key, json!(value)))
                    .collect::<zed::serde_json::Map<String, Value>>(),
            })
            .to_string(),
            tcp_connection: None,
        })
    }
}

zed::register_extension!(HashLinkExtension);

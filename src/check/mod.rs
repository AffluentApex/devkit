use std::process::Command;

pub struct EnvironmentChecker;

impl EnvironmentChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn check_tools(&self, tools: &str) -> Vec<ToolStatus> {
        tools
            .split(',')
            .map(|tool| {
                let status = match tool.trim() {
                    "git" => self.check_git(),
                    "rust" => self.check_rust(),
                    "node" => self.check_node(),
                    "python" => self.check_python(),
                    _ => ToolStatus {
                        name: tool.to_string(),
                        installed: false,
                        version: None,
                    },
                };
                status
            })
            .collect()
    }

    fn check_git(&self) -> ToolStatus {
        self.check_command("git", &["--version"])
    }

    fn check_rust(&self) -> ToolStatus {
        self.check_command("rustc", &["--version"])
    }

    fn check_node(&self) -> ToolStatus {
        self.check_command("node", &["--version"])
    }

    fn check_python(&self) -> ToolStatus {
        self.check_command("python", &["--version"])
    }

    fn check_command(&self, cmd: &str, args: &[&str]) -> ToolStatus {
        let output = Command::new(cmd).args(args).output();
        
        match output {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout).to_string();
                ToolStatus {
                    name: cmd.to_string(),
                    installed: true,
                    version: Some(version),
                }
            }
            _ => ToolStatus {
                name: cmd.to_string(),
                installed: false,
                version: None,
            },
        }
    }
}

pub struct ToolStatus {
    pub name: String,
    pub installed: bool,
    pub version: Option<String>,
}

//! Core contracts for BoardClaw.
//!
//! This crate intentionally starts small. It gives the project a tested Rust
//! core, CI target, and stable vocabulary for board profiles, model classes,
//! provider kinds, and hardware tool risk.

#![forbid(unsafe_code)]

/// Board families that need different provider and hardware profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardFamily {
    RaspberryPi,
    Rk3588,
    Jetson,
    X86Gateway,
    BeagleBone,
    LowCostArm,
    IndustrialArm,
}

/// Local or remote model backend families BoardClaw can route to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    Ollama,
    LlamaCpp,
    HailoOllama,
    TensorRt,
    RkLlm,
    OpenVino,
    RemoteOpenAiCompatible,
}

/// Approximate local model class. The concrete model name is intentionally
/// profile/runtime data, not a compile-time identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModelClass {
    Tiny,
    Small,
    Medium,
    Large,
}

/// Risk level for hardware and automation tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ToolRisk {
    ReadOnly,
    Low,
    High,
    Critical,
}

impl ToolRisk {
    /// Returns true when a tool should require explicit approval before
    /// execution on a real board.
    #[must_use]
    pub const fn requires_approval(self) -> bool {
        matches!(self, Self::High | Self::Critical)
    }
}

/// Declarative board capability profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardProfile {
    pub id: &'static str,
    pub family: BoardFamily,
    pub arch: &'static str,
    pub first_class_target: bool,
    pub providers: &'static [ProviderKind],
    pub model_classes: &'static [ModelClass],
    pub gpio: bool,
    pub i2c: bool,
    pub spi: bool,
    pub uart: bool,
    pub camera: bool,
    pub accelerator_required: bool,
}

impl BoardProfile {
    /// The smallest model class that should be considered normal for this
    /// profile. Returns `None` for profiles that should use a LAN/cloud model
    /// until a tiny local model is explicitly configured.
    #[must_use]
    pub fn default_local_model_class(&self) -> Option<ModelClass> {
        self.model_classes.first().copied()
    }

    /// True when the profile has a local provider that does not require a
    /// vendor accelerator stack.
    #[must_use]
    pub fn has_cpu_baseline_provider(&self) -> bool {
        self.providers
            .iter()
            .any(|provider| matches!(provider, ProviderKind::Ollama | ProviderKind::LlamaCpp))
    }
}

/// Tool declaration used by policy and benchmark planning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolSpec {
    pub name: &'static str,
    pub risk: ToolRisk,
    pub write_operation: bool,
}

impl ToolSpec {
    /// Returns true if the tool should be hidden or approval-gated in default
    /// read-only mode.
    #[must_use]
    pub const fn gated_by_default(&self) -> bool {
        self.write_operation || self.risk.requires_approval()
    }
}

pub const RASPBERRY_PI_5: BoardProfile = BoardProfile {
    id: "raspberry_pi_5",
    family: BoardFamily::RaspberryPi,
    arch: "aarch64",
    first_class_target: true,
    providers: &[
        ProviderKind::Ollama,
        ProviderKind::LlamaCpp,
        ProviderKind::HailoOllama,
    ],
    model_classes: &[ModelClass::Small, ModelClass::Medium],
    gpio: true,
    i2c: true,
    spi: true,
    uart: true,
    camera: true,
    accelerator_required: false,
};

pub const JETSON_ORIN_NANO: BoardProfile = BoardProfile {
    id: "jetson_orin_nano",
    family: BoardFamily::Jetson,
    arch: "aarch64",
    first_class_target: false,
    providers: &[
        ProviderKind::TensorRt,
        ProviderKind::Ollama,
        ProviderKind::LlamaCpp,
    ],
    model_classes: &[ModelClass::Small, ModelClass::Medium, ModelClass::Large],
    gpio: true,
    i2c: true,
    spi: true,
    uart: true,
    camera: true,
    accelerator_required: false,
};

pub const LE_POTATO: BoardProfile = BoardProfile {
    id: "le_potato",
    family: BoardFamily::LowCostArm,
    arch: "aarch64",
    first_class_target: false,
    providers: &[ProviderKind::RemoteOpenAiCompatible],
    model_classes: &[ModelClass::Tiny],
    gpio: true,
    i2c: true,
    spi: true,
    uart: true,
    camera: false,
    accelerator_required: false,
};

pub const GPIO_READ: ToolSpec = ToolSpec {
    name: "gpio.read",
    risk: ToolRisk::ReadOnly,
    write_operation: false,
};

pub const GPIO_WRITE: ToolSpec = ToolSpec {
    name: "gpio.write",
    risk: ToolRisk::High,
    write_operation: true,
};

pub const SHELL_SAFE_EXEC: ToolSpec = ToolSpec {
    name: "shell.safe_exec",
    risk: ToolRisk::Critical,
    write_operation: true,
};

/// BoardClaw's recommended trusted-core implementation language.
#[must_use]
pub const fn recommended_core_language() -> &'static str {
    "Rust"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_is_the_core_language() {
        let language = std::hint::black_box(recommended_core_language());
        assert_eq!(language, "Rust");
    }

    #[test]
    fn raspberry_pi_is_first_class_and_local_first() {
        let profile = std::hint::black_box(&RASPBERRY_PI_5);
        assert!(profile.first_class_target);
        assert!(profile.has_cpu_baseline_provider());
        assert_eq!(profile.default_local_model_class(), Some(ModelClass::Small));
    }

    #[test]
    fn jetson_profile_allows_medium_or_larger_models() {
        let profile = std::hint::black_box(&JETSON_ORIN_NANO);
        assert!(profile.model_classes.contains(&ModelClass::Medium));
        assert!(profile.providers.contains(&ProviderKind::TensorRt));
    }

    #[test]
    fn low_cost_boards_are_not_forced_to_host_large_models() {
        let profile = std::hint::black_box(&LE_POTATO);
        assert_eq!(profile.default_local_model_class(), Some(ModelClass::Tiny));
        assert!(!profile.model_classes.contains(&ModelClass::Large));
    }

    #[test]
    fn dangerous_tools_are_gated_by_default() {
        let gpio_read = std::hint::black_box(&GPIO_READ);
        let gpio_write = std::hint::black_box(&GPIO_WRITE);
        let shell_safe_exec = std::hint::black_box(&SHELL_SAFE_EXEC);
        assert!(!gpio_read.gated_by_default());
        assert!(gpio_write.gated_by_default());
        assert!(shell_safe_exec.gated_by_default());
    }
}

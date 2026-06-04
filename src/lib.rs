#![forbid(unsafe_code)]

//! Ternary Shipyard — Construction and assembly of complex agent systems.
//!
//! Provides blueprint-based agent specification, assembly lines, quality assurance,
//! component recycling, and a ship class template library.

// ── Component ──────────────────────────────────────────────────────────

/// A named component with a ternary role tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component {
    pub name: String,
    pub role: TernaryRole,
}

/// Ternary role: how a component contributes to the agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TernaryRole {
    /// Core functionality — agent doesn't work without it.
    Core,
    /// Optional enhancement — improves performance.
    Enhancement,
    /// Adversarial hardening — defends against attacks.
    Hardening,
}

// ── Blueprint ──────────────────────────────────────────────────────────

/// Declarative agent specification: what components to assemble.
#[derive(Debug, Clone)]
pub struct Blueprint {
    pub name: String,
    pub class: String,
    pub components: Vec<Component>,
    pub required_roles: Vec<TernaryRole>,
}

impl Blueprint {
    pub fn new(name: &str, class: &str) -> Self {
        Self {
            name: name.to_string(),
            class: class.to_string(),
            components: Vec::new(),
            required_roles: Vec::new(),
        }
    }

    pub fn with_component(mut self, name: &str, role: TernaryRole) -> Self {
        self.components.push(Component { name: name.to_string(), role });
        self
    }

    pub fn require_role(mut self, role: TernaryRole) -> Self {
        self.required_roles.push(role);
        self
    }

    pub fn has_role(&self, role: TernaryRole) -> bool {
        self.components.iter().any(|c| c.role == role)
    }

    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    pub fn satisfies_requirements(&self) -> bool {
        for req in &self.required_roles {
            if !self.has_role(*req) {
                return false;
            }
        }
        true
    }
}

// ── Agent (assembled product) ──────────────────────────────────────────

/// An assembled agent built from a blueprint.
#[derive(Debug, Clone)]
pub struct Agent {
    pub name: String,
    pub class: String,
    pub components: Vec<Component>,
    pub assembled: bool,
    pub passed_qa: bool,
}

impl Agent {
    pub fn new(name: &str, class: &str, components: Vec<Component>) -> Self {
        Self {
            name: name.to_string(),
            class: class.to_string(),
            components,
            assembled: true,
            passed_qa: false,
        }
    }

    pub fn has_component(&self, name: &str) -> bool {
        self.components.iter().any(|c| c.name == name)
    }

    pub fn component_names(&self) -> Vec<&str> {
        self.components.iter().map(|c| c.name.as_str()).collect()
    }

    pub fn disassemble(self) -> Vec<Component> {
        self.components
    }
}

// ── AssemblyLine ───────────────────────────────────────────────────────

/// Builds agents from blueprints.
pub struct AssemblyLine {
    built_count: usize,
}

impl AssemblyLine {
    pub fn new() -> Self {
        Self { built_count: 0 }
    }

    /// Attempt to build an agent from a blueprint. Fails if requirements not met.
    pub fn build(&mut self, blueprint: &Blueprint) -> Result<Agent, String> {
        if !blueprint.satisfies_requirements() {
            return Err(format!("Blueprint '{}' missing required roles", blueprint.name));
        }
        if blueprint.components.is_empty() {
            return Err(format!("Blueprint '{}' has no components", blueprint.name));
        }
        self.built_count += 1;
        Ok(Agent::new(&blueprint.name, &blueprint.class, blueprint.components.clone()))
    }

    pub fn built_count(&self) -> usize {
        self.built_count
    }
}

// ── QualityAssurance ───────────────────────────────────────────────────

/// Tests assembled agents against requirements.
pub struct QualityAssurance {
    checked: usize,
    passed: usize,
    failed: usize,
}

impl QualityAssurance {
    pub fn new() -> Self {
        Self { checked: 0, passed: 0, failed: 0 }
    }

    /// Inspect an agent: pass if it has at least one Core component.
    pub fn inspect(&mut self, agent: &mut Agent) -> bool {
        self.checked += 1;
        let pass = agent.components.iter().any(|c| c.role == TernaryRole::Core) && agent.assembled;
        agent.passed_qa = pass;
        if pass { self.passed += 1; } else { self.failed += 1; }
        pass
    }

    /// Inspect with custom predicate.
    pub fn inspect_with<F>(&mut self, agent: &mut Agent, predicate: F) -> bool
    where
        F: Fn(&Agent) -> bool,
    {
        self.checked += 1;
        let pass = predicate(agent) && agent.assembled;
        agent.passed_qa = pass;
        if pass { self.passed += 1; } else { self.failed += 1; }
        pass
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        (self.checked, self.passed, self.failed)
    }
}

// ── ScrapYard ──────────────────────────────────────────────────────────

/// Recycles dead agent components back into reusable parts.
#[derive(Debug, Clone)]
pub struct ScrapYard {
    salvaged: Vec<Component>,
    recycled_count: usize,
}

impl ScrapYard {
    pub fn new() -> Self {
        Self { salvaged: Vec::new(), recycled_count: 0 }
    }

    /// Recycle an agent's components into the yard.
    pub fn recycle(&mut self, agent: Agent) -> usize {
        let count = agent.components.len();
        self.recycled_count += count;
        self.salvaged.extend(agent.disassemble());
        count
    }

    /// Salvage a specific component by name.
    pub fn salvage(&mut self, name: &str) -> Option<Component> {
        if let Some(pos) = self.salvaged.iter().position(|c| c.name == name) {
            Some(self.salvaged.remove(pos))
        } else {
            None
        }
    }

    pub fn available(&self) -> &[Component] {
        &self.salvaged
    }

    pub fn recycled_count(&self) -> usize {
        self.recycled_count
    }

    pub fn salvage_count(&self) -> usize {
        self.salvaged.len()
    }
}

// ── ShipClass ──────────────────────────────────────────────────────────

/// Template library for common agent configurations.
pub struct ShipClass {
    templates: std::collections::HashMap<String, Blueprint>,
}

impl ShipClass {
    pub fn new() -> Self {
        let mut sc = Self { templates: std::collections::HashMap::new() };
        sc.register_defaults();
        sc
    }

    pub fn register(&mut self, blueprint: Blueprint) {
        self.templates.insert(blueprint.class.clone(), blueprint);
    }

    pub fn get(&self, class: &str) -> Option<&Blueprint> {
        self.templates.get(class)
    }

    pub fn instantiate(&self, class: &str, name: &str) -> Option<Blueprint> {
        self.templates.get(class).map(|bp| {
            let mut instance = bp.clone();
            instance.name = name.to_string();
            instance
        })
    }

    pub fn class_names(&self) -> Vec<&str> {
        self.templates.keys().map(|s| s.as_str()).collect()
    }

    fn register_defaults(&mut self) {
        self.register(
            Blueprint::new("default-scout", "scout")
                .with_component("sensor", TernaryRole::Core)
                .with_component("comm", TernaryRole::Enhancement)
                .require_role(TernaryRole::Core)
        );
        self.register(
            Blueprint::new("default-worker", "worker")
                .with_component("processor", TernaryRole::Core)
                .with_component("memory", TernaryRole::Core)
                .with_component("comm", TernaryRole::Enhancement)
                .require_role(TernaryRole::Core)
        );
        self.register(
            Blueprint::new("default-guard", "guard")
                .with_component("sensor", TernaryRole::Core)
                .with_component("shield", TernaryRole::Hardening)
                .with_component("comm", TernaryRole::Enhancement)
                .require_role(TernaryRole::Core)
                .require_role(TernaryRole::Hardening)
        );
    }
}

// ── Shipyard ───────────────────────────────────────────────────────────

/// Top-level shipyard that coordinates assembly, QA, and recycling.
pub struct Shipyard {
    pub assembly: AssemblyLine,
    pub qa: QualityAssurance,
    pub scrap: ScrapYard,
    pub classes: ShipClass,
}

impl Shipyard {
    pub fn new() -> Self {
        Self {
            assembly: AssemblyLine::new(),
            qa: QualityAssurance::new(),
            scrap: ScrapYard::new(),
            classes: ShipClass::new(),
        }
    }

    /// Build from a registered class template, run through QA.
    pub fn build_from_class(&mut self, class: &str, name: &str) -> Result<Agent, String> {
        let bp = self.classes.instantiate(class, name)
            .ok_or_else(|| format!("Unknown class: {}", class))?;
        let mut agent = self.assembly.build(&bp)?;
        self.qa.inspect(&mut agent);
        Ok(agent)
    }

    /// Decommission an agent, recycling its components.
    pub fn decommission(&mut self, agent: Agent) -> usize {
        self.scrap.recycle(agent)
    }
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blueprint_builder() {
        let bp = Blueprint::new("test", "scout")
            .with_component("sensor", TernaryRole::Core)
            .with_component("comm", TernaryRole::Enhancement);
        assert_eq!(bp.component_count(), 2);
        assert_eq!(bp.name, "test");
    }

    #[test]
    fn blueprint_has_role() {
        let bp = Blueprint::new("test", "scout")
            .with_component("sensor", TernaryRole::Core);
        assert!(bp.has_role(TernaryRole::Core));
        assert!(!bp.has_role(TernaryRole::Hardening));
    }

    #[test]
    fn blueprint_satisfies_requirements() {
        let bp = Blueprint::new("test", "scout")
            .with_component("sensor", TernaryRole::Core)
            .require_role(TernaryRole::Core);
        assert!(bp.satisfies_requirements());
    }

    #[test]
    fn blueprint_fails_requirements() {
        let bp = Blueprint::new("test", "scout")
            .with_component("sensor", TernaryRole::Core)
            .require_role(TernaryRole::Hardening);
        assert!(!bp.satisfies_requirements());
    }

    #[test]
    fn assembly_build_success() {
        let bp = Blueprint::new("agent-1", "scout")
            .with_component("sensor", TernaryRole::Core)
            .require_role(TernaryRole::Core);
        let mut line = AssemblyLine::new();
        let agent = line.build(&bp).unwrap();
        assert_eq!(agent.name, "agent-1");
        assert!(agent.assembled);
        assert_eq!(line.built_count(), 1);
    }

    #[test]
    fn assembly_build_fails_no_components() {
        let bp = Blueprint::new("empty", "scout");
        let mut line = AssemblyLine::new();
        assert!(line.build(&bp).is_err());
    }

    #[test]
    fn assembly_build_fails_missing_role() {
        let bp = Blueprint::new("bad", "guard")
            .with_component("sensor", TernaryRole::Core)
            .require_role(TernaryRole::Hardening);
        let mut line = AssemblyLine::new();
        assert!(line.build(&bp).is_err());
    }

    #[test]
    fn agent_has_component() {
        let agent = Agent::new("a", "scout", vec![
            Component { name: "sensor".into(), role: TernaryRole::Core },
        ]);
        assert!(agent.has_component("sensor"));
        assert!(!agent.has_component("laser"));
    }

    #[test]
    fn agent_component_names() {
        let agent = Agent::new("a", "scout", vec![
            Component { name: "sensor".into(), role: TernaryRole::Core },
            Component { name: "comm".into(), role: TernaryRole::Enhancement },
        ]);
        assert_eq!(agent.component_names(), vec!["sensor", "comm"]);
    }

    #[test]
    fn agent_disassemble() {
        let agent = Agent::new("a", "scout", vec![
            Component { name: "sensor".into(), role: TernaryRole::Core },
        ]);
        let parts = agent.disassemble();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].name, "sensor");
    }

    #[test]
    fn qa_inspect_pass() {
        let mut qa = QualityAssurance::new();
        let mut agent = Agent::new("a", "scout", vec![
            Component { name: "sensor".into(), role: TernaryRole::Core },
        ]);
        assert!(qa.inspect(&mut agent));
        assert!(agent.passed_qa);
        let (checked, passed, failed) = qa.stats();
        assert_eq!(checked, 1);
        assert_eq!(passed, 1);
        assert_eq!(failed, 0);
    }

    #[test]
    fn qa_inspect_fail_no_core() {
        let mut qa = QualityAssurance::new();
        let mut agent = Agent::new("a", "scout", vec![
            Component { name: "comm".into(), role: TernaryRole::Enhancement },
        ]);
        assert!(!qa.inspect(&mut agent));
        assert!(!agent.passed_qa);
    }

    #[test]
    fn qa_inspect_with_custom() {
        let mut qa = QualityAssurance::new();
        let mut agent = Agent::new("a", "scout", vec![
            Component { name: "sensor".into(), role: TernaryRole::Core },
        ]);
        let pass = qa.inspect_with(&mut agent, |a| a.components.len() >= 1);
        assert!(pass);
    }

    #[test]
    fn scrap_yard_recycle() {
        let mut yard = ScrapYard::new();
        let agent = Agent::new("a", "scout", vec![
            Component { name: "sensor".into(), role: TernaryRole::Core },
            Component { name: "comm".into(), role: TernaryRole::Enhancement },
        ]);
        assert_eq!(yard.recycle(agent), 2);
        assert_eq!(yard.recycled_count(), 2);
        assert_eq!(yard.salvage_count(), 2);
    }

    #[test]
    fn scrap_yard_salvage() {
        let mut yard = ScrapYard::new();
        let agent = Agent::new("a", "scout", vec![
            Component { name: "sensor".into(), role: TernaryRole::Core },
        ]);
        yard.recycle(agent);
        let part = yard.salvage("sensor");
        assert!(part.is_some());
        assert_eq!(part.unwrap().name, "sensor");
        assert_eq!(yard.salvage_count(), 0);
    }

    #[test]
    fn scrap_yard_salvage_missing() {
        let mut yard = ScrapYard::new();
        assert!(yard.salvage("nonexistent").is_none());
    }

    #[test]
    fn ship_class_defaults() {
        let sc = ShipClass::new();
        assert!(sc.get("scout").is_some());
        assert!(sc.get("worker").is_some());
        assert!(sc.get("guard").is_some());
    }

    #[test]
    fn ship_class_instantiate() {
        let sc = ShipClass::new();
        let bp = sc.instantiate("scout", "my-scout").unwrap();
        assert_eq!(bp.name, "my-scout");
        assert_eq!(bp.class, "scout");
    }

    #[test]
    fn ship_class_names() {
        let sc = ShipClass::new();
        let names = sc.class_names();
        assert!(names.contains(&"scout"));
    }

    #[test]
    fn shipyard_full_build() {
        let mut yard = Shipyard::new();
        let agent = yard.build_from_class("scout", "fast-scout").unwrap();
        assert_eq!(agent.name, "fast-scout");
        assert!(agent.passed_qa);
    }

    #[test]
    fn shipyard_unknown_class() {
        let mut yard = Shipyard::new();
        assert!(yard.build_from_class("nonexistent", "x").is_err());
    }

    #[test]
    fn shipyard_decommission() {
        let mut yard = Shipyard::new();
        let agent = yard.build_from_class("scout", "tmp").unwrap();
        let count = yard.decommission(agent);
        assert_eq!(count, 2);
        assert_eq!(yard.scrap.salvage_count(), 2);
    }
}

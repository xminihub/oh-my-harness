/// Where an install lands: the user's home-level `.agents` root, or a
/// project's own `.agents` root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope<'a> {
    /// Install into `<project>/.agents/…`.
    Project(&'a std::path::Path),
    /// Install into `~/.agents/…`.
    Global,
}

/// The global skills root is ~/.agents/skills.
pub fn resolve_skills_dir(scope: Scope<'_>, home: &std::path::Path) -> std::path::PathBuf {
    match scope {
        Scope::Global => home.join(".agents").join("skills"),
        Scope::Project(dir) => dir.join(".agents").join("skills"),
    }
}

/// The global agents root is ~/.agents/agents.
pub fn resolve_agents_dir(scope: Scope<'_>, home: &std::path::Path) -> std::path::PathBuf {
    match scope {
        Scope::Global => home.join(".agents").join("agents"),
        Scope::Project(dir) => dir.join(".agents").join("agents"),
    }
}

/// The global commands root is ~/.agents/commands.
pub fn resolve_commands_dir(scope: Scope<'_>, home: &std::path::Path) -> std::path::PathBuf {
    match scope {
        Scope::Global => home.join(".agents").join("commands"),
        Scope::Project(dir) => dir.join(".agents").join("commands"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn commands_global_installs_to_home_agents_commands() {
        let dir = resolve_commands_dir(Scope::Global, Path::new("/home/u"));
        assert_eq!(dir, Path::new("/home/u/.agents/commands"));
    }

    #[test]
    fn commands_project_installs_to_dot_agents_commands() {
        let dir = resolve_commands_dir(Scope::Project(Path::new("/proj")), Path::new("/home/u"));
        assert_eq!(dir, Path::new("/proj/.agents/commands"));
    }

    #[test]
    fn skills_global_installs_to_home_agents_skills() {
        let dir = resolve_skills_dir(Scope::Global, Path::new("/home/u"));
        assert_eq!(dir, Path::new("/home/u/.agents/skills"));
    }

    #[test]
    fn skills_project_installs_to_dot_agents_skills() {
        let dir = resolve_skills_dir(Scope::Project(Path::new("/proj")), Path::new("/home/u"));
        assert_eq!(dir, Path::new("/proj/.agents/skills"));
    }

    #[test]
    fn agents_global_installs_to_home_agents_agents() {
        let dir = resolve_agents_dir(Scope::Global, Path::new("/home/u"));
        assert_eq!(dir, Path::new("/home/u/.agents/agents"));
    }

    #[test]
    fn agents_project_installs_to_dot_agents_agents() {
        let dir = resolve_agents_dir(Scope::Project(Path::new("/proj")), Path::new("/home/u"));
        assert_eq!(dir, Path::new("/proj/.agents/agents"));
    }
}

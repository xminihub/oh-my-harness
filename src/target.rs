use std::path::{Path, PathBuf};

/// Which payload kind of a repo is being placed: each kind lives in its own
/// subdirectory under the scope's `.agents` root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Skills,
    Agents,
    Commands,
}

impl Kind {
    fn dir(&self) -> &'static str {
        match self {
            Kind::Skills => "skills",
            Kind::Agents => "agents",
            Kind::Commands => "commands",
        }
    }
}

/// Where an install lands: the user's home-level `.agents` root, or a
/// project's own `.agents` root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope<'a> {
    /// Install into `<project>/.agents/…`.
    Project(&'a Path),
    /// Install into `~/.agents/…`.
    Global,
}

impl Scope<'_> {
    /// The `.agents` root this scope installs under; global needs the home dir.
    pub fn agents_root(&self, home: &Path) -> PathBuf {
        match self {
            Scope::Global => home.join(".agents"),
            Scope::Project(dir) => dir.join(".agents"),
        }
    }

    /// The directory a payload kind lands in for this scope.
    pub fn dest(&self, home: &Path, kind: Kind) -> PathBuf {
        self.agents_root(home).join(kind.dir())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn commands_global_installs_to_home_agents_commands() {
        let dir = Scope::Global.dest(Path::new("/home/u"), Kind::Commands);
        assert_eq!(dir, Path::new("/home/u/.agents/commands"));
    }

    #[test]
    fn commands_project_installs_to_dot_agents_commands() {
        let dir = Scope::Project(Path::new("/proj")).dest(Path::new("/home/u"), Kind::Commands);
        assert_eq!(dir, Path::new("/proj/.agents/commands"));
    }

    #[test]
    fn skills_global_installs_to_home_agents_skills() {
        let dir = Scope::Global.dest(Path::new("/home/u"), Kind::Skills);
        assert_eq!(dir, Path::new("/home/u/.agents/skills"));
    }

    #[test]
    fn skills_project_installs_to_dot_agents_skills() {
        let dir = Scope::Project(Path::new("/proj")).dest(Path::new("/home/u"), Kind::Skills);
        assert_eq!(dir, Path::new("/proj/.agents/skills"));
    }

    #[test]
    fn agents_global_installs_to_home_agents_agents() {
        let dir = Scope::Global.dest(Path::new("/home/u"), Kind::Agents);
        assert_eq!(dir, Path::new("/home/u/.agents/agents"));
    }

    #[test]
    fn agents_project_installs_to_dot_agents_agents() {
        let dir = Scope::Project(Path::new("/proj")).dest(Path::new("/home/u"), Kind::Agents);
        assert_eq!(dir, Path::new("/proj/.agents/agents"));
    }
}

use crate::AgentKind;
use std::fmt;
pub struct TickTrace {
    pub shark_birth: i64,
    pub shark_death: i64,
    pub fish_birth: i64,
    pub fish_death: i64,
}

impl fmt::Display for TickTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}, {}", self.shark_birth, self.shark_death, self.fish_birth, self.fish_death)
    }
}
impl TickTrace {
    pub fn new() -> TickTrace {
        TickTrace {
            shark_birth: 0,
            shark_death: 0,
            fish_birth: 0,
            fish_death: 0,
        }
    }

    pub fn birth(&mut self, agent_kind: AgentKind) {
        match agent_kind {
            AgentKind::Fish => self.fish_birth += 1,
            AgentKind::Shark => self.shark_birth += 1,
        }
    }

    pub fn death(&mut self, agent_kind: AgentKind) {
        match agent_kind {
            AgentKind::Fish => self.fish_death += 1,
            AgentKind::Shark => self.shark_death += 1,
        }
    }
}

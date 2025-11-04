use std::sync::{Arc, Mutex};

use crate::{agent::{Agent, Agents}, user::{User, Users}};

enum State {
    Busy = 0,
    Ready,
    Offline,
    Online,
}

pub type GlobalState = Arc<Mutex<GlobalStateStruct>>;

pub struct GlobalStateStruct {
    users: Users,
    agents: Agents,
    state: State,
}

impl GlobalStateStruct {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            agents: Vec::new(),
            state: State::Busy,
        }
    }

    pub fn push_user(mut self, user: User) {
        self.users.push(user);
    }

    pub fn remove_user(mut self, user: User) {
        self.users.retain(|x| *x == user);
    }

    pub fn push_agent(mut self, agent: Agent) {
        self.agents.push(agent);
    }
}


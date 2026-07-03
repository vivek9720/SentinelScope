#[derive(Debug, Clone, PartialEq, Eq)]

pub enum PolicyAction { Accept, Drop, Reject, Log, Return, Unknown(String) }



impl PolicyAction {

    pub fn from_word(word: &str) -> Self {

        match word.to_ascii_uppercase().as_str() { "ACCEPT" | "ALLOW" => PolicyAction::Accept, "DROP" | "DENY" => PolicyAction::Drop, "REJECT" => PolicyAction::Reject, "LOG" => PolicyAction::Log, "RETURN" => PolicyAction::Return, other => PolicyAction::Unknown(other.to_string()) }

    }

    pub fn blocks(&self) -> bool { matches!(self, PolicyAction::Drop | PolicyAction::Reject) }

}



#[derive(Debug, Clone, PartialEq, Eq, Default)]

pub struct FirewallRule { pub family: Option<String>, pub table: Option<String>, pub chain: String, pub protocol: Option<String>, pub source: Option<String>, pub destination: Option<String>, pub source_port: Option<String>, pub destination_port: Option<String>, pub in_interface: Option<String>, pub out_interface: Option<String>, pub state: Vec<String>, pub action: Option<PolicyAction>, pub raw: String }



impl FirewallRule { pub fn key(&self) -> String { format!("{:?}|{:?}|{}|{:?}|{:?}|{:?}|{:?}|{:?}", self.family, self.table, self.chain, self.protocol, self.source, self.destination, self.destination_port, self.action) } }



#[derive(Debug, Clone, PartialEq, Eq, Default)]

pub struct SecurityPolicy { pub rules: Vec<FirewallRule>, pub settings: Vec<(String, String)> }

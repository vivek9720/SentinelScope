#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdsRule { pub action: String, pub protocol: String, pub source: String, pub source_port: String, pub direction: String, pub destination: String, pub destination_port: String, pub options: Vec<RuleOption> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleOption { pub key: String, pub value: Option<String> }

impl IdsRule {
    pub fn sid(&self) -> Option<u32> { self.option_value("sid").and_then(|v| v.parse().ok()) }
    pub fn rev(&self) -> Option<u32> { self.option_value("rev").and_then(|v| v.parse().ok()) }
    pub fn msg(&self) -> Option<&str> { self.option_value("msg") }
    pub fn classtype(&self) -> Option<&str> { self.option_value("classtype") }
    pub fn contents(&self) -> Vec<&str> { self.options.iter().filter(|o| o.key == "content").filter_map(|o| o.value.as_deref()).collect() }
    pub fn option_value(&self, key: &str) -> Option<&str> { self.options.iter().find(|o| o.key.eq_ignore_ascii_case(key)).and_then(|o| o.value.as_deref()) }
}

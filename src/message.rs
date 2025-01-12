use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MessageTemplate {
    pub service: String,  // Service type (e.g., "TradingView", "Telegram", etc.)
    pub template: String, // Message template
}

pub struct TemplateRegistry {
    templates: HashMap<String, MessageTemplate>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        let mut templates = HashMap::new();

        // Default TradingView template
        templates.insert(
            "TradingView".to_string(),
            MessageTemplate {
                service: "TradingView".to_string(),
                template: r#"{
  "target": "{{target}}",
  "ticker": "{{ticker}}",
  "action": "{{strategy.order.action}}",
  "order_size": "100%",
  "position_size": "{{strategy.position_size}}",
  "schema": "2",
  "timestamp": "{{time}}"
}"#
                .to_string(),
            },
        );

        Self { templates }
    }

    pub fn get_template(&self, service: &str) -> Option<&MessageTemplate> {
        self.templates.get(service)
    }
}

/// Replace only the `{{target}}` placeholder in the template
pub fn render_template(template: &str, target: &str) -> String {
    template.replace("{{target}}", target)
}

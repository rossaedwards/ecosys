pub trait RuntimeHook: Send + Sync {
    fn on_event(&self, event: &str);
    fn on_instruction(&self, instr: &str);
}

pub struct RuntimeContext {
    hooks: Vec<Box<dyn RuntimeHook>>,
}

impl RuntimeContext {
    pub fn register_hook(&mut self, hook: Box<dyn RuntimeHook>) {
        self.hooks.push(hook);
    }

    pub fn execute_instruction(&self, instr: &str) {
        for hook in &self.hooks {
            hook.on_instruction(instr);
        }
        // Execute actual instruction here
    }
}

pub struct LoggingHook;

impl RuntimeHook for LoggingHook {
    fn on_event(&self, event: &str) {
        println!("[Event] {}", event);
    }

    fn on_instruction(&self, instr: &str) {
        println!("[Exec] {}", instr);
    }
}
pub fn create_runtime_context() -> RuntimeContext {
    let mut ctx = RuntimeContext { hooks: vec![] };
    ctx.register_hook(Box::new(LoggingHook));
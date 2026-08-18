use std::fs;
use std::env;
use tokio::{signal, sync::oneshot};
use tracing::{info, error};
use tracing_subscriber;

use crate::parser::parse_ritual;
use crate::uir::UirBuilder;
use crate::generator::Generator;
use crate::executor::{get_vm, schedule_async_execution, async_executor_worker};
use crate::runtime_hooks::{create_runtime_context, RuntimeContext};

mod parser;
mod executor;
mod generator;
mod uir;
mod optimizer;
mod runtime_hooks;
mod ast;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let filename = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: fuxyez <source_file.ritual>");
        std::process::exit(1);
    });

    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    tokio::spawn(async move {
        async_executor_worker_with_shutdown(shutdown_rx).await;
    });

    let src = match fs::read_to_string(&filename) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to read '{}': {}", filename, e);
            return;
        }
    };

    let ast = match parse_ritual(&src) {
        Ok(ast) => ast,
        Err(e) => {
            error!("Parsing failed: {}", e);
            return;
        }
    };

    let mut builder = UirBuilder::new();
    let uir = match builder.from_ast(&ast[0]) {
        Ok(uir) => uir,
        Err(e) => {
            error!("UIR build error: {}", e);
            return;
        }
    };

    let mut gen = Generator::new();
    if let Err(e) = gen.generate(&uir).await {
        error!("Code generation failed: {}", e);
        return;
    }
    let bytecode = gen.finalize();

    {
        let mut vm = get_vm();
        vm.execute_sync(&bytecode);
    }

    schedule_async_execution(bytecode).await;

    info!("Compilation complete. Press Ctrl-C to exit.");

    if let Err(e) = signal::ctrl_c().await {
        error!("Failed to listen for ctrl-c: {}", e);
    }

    let _ = shutdown_tx.send(());

    info!("Exiting Fuxyez Compiler; goodbye mighty coder!");
}

async fn async_executor_worker_with_shutdown(mut shutdown_rx: oneshot::Receiver<()>) {
    tokio::select! {
        _ = async_executor_worker() => {},
        _ = &mut shutdown_rx => {
            info!("Executor worker received shutdown signal.");
        }
    }
}
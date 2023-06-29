use tracing::Level;
pub use tracing::{trace,error,info,debug,warn};
use tracing_subscriber;


pub fn init(is_debug:bool){
    let filter=if is_debug {Level::TRACE}else{Level::INFO};

    tracing_subscriber::fmt()
    .with_max_level(filter)
    .without_time()
    .init();
}
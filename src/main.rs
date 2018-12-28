#![feature(uniform_paths)]
mod app;
use android_base::*;

fn main() {
    enable_backtrace();

    AppContainer::init(app::App::new(), AppConfig::new()).run();
}

#![no_main]
use log;
use simple_logger::SimpleLogger;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = SimpleLogger::new().init();
    match std::str::from_utf8(data) {
        Ok(s) => {
            if data.len() > 0 {
                for _ in 0..data[0] {
                    log::warn!("{}", s);
                }
            } else {
                log::warn!("{}", s);
            }
            log::info!("{}", s);
            log::debug!("{}", s);
            log::error!("{}", s);
        },
        _ => {},
    }
    
});

#![forbid(unsafe_code)]
use std::ops;
use sabaton_mw::{AsyncReader, SubscribeOptions};
use std::{time::Duration, sync::Arc};
// Main Library file
use sabaton_mw::{NodeBuilder, error::MiddlewareError, SyncReader, Samples};
use tracing::{debug, info, span, Level};
use vehicle_signals::{v3::{vehicle::{Speed, IsMoving}, self}, units::KilometrePerHour };

pub fn example_node_main() -> Result<(),MiddlewareError> {

    let node =   NodeBuilder::default()
        //.multi_threaded()  Enable this if you want a multi-threaded runtime
        //.with_num_workers(4)    // Number of work threads. Fixed to 1 for single threaded runtime.
        .build("example-node".to_owned()).expect("Node creation error");
        let mut subscribe_options = SubscribeOptions::default();
        let mut speed_reader= node.subscribe_async::<v3::vehicle::Speed>(&subscribe_options).expect("Unable to advertise");
    
   
    
    
    let res = node.spin(move || {
        
        span!(target: "MAIN", Level::TRACE, "Application Main Loop");
        info!("Application Main Loop Started with tick interval 100mS");

        let mut ticker = tokio::time::interval(Duration::from_millis(100));

        let _task = tokio::spawn( async move {
            //let mut vehicle_safe:bool;

            loop {
                let _ = ticker.tick().await;
                debug!("Tick");
                let mut speed = Samples::<Speed>::new(1);
                if let Ok(_) =  speed_reader.take(& mut speed).await{
                    if let Some(speed) = speed.iter().next() { println!("Speed {:?}",speed.value.0)};
                 }
                }
            });
               
            
         });
         
    res

}
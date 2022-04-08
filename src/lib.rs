#![forbid(unsafe_code)]
use std::ops;
use sabaton_mw::AsyncReader;
use std::{time::Duration, sync::Arc};
// Main Library file
use sabaton_mw::{NodeBuilder, error::MiddlewareError, SyncReader, Samples};
use tracing::{debug, info, span, Level};
use vehicle_signals::{v2::{vehicle::{Speed, IsMoving, IgnitionOn}, self}, units::KilometrePerHour };

pub fn example_node_main() -> Result<(),MiddlewareError> {

    let node =   NodeBuilder::default()
        //.multi_threaded()  Enable this if you want a multi-threaded runtime
        //.with_num_workers(4)    // Number of work threads. Fixed to 1 for single threaded runtime.
        .build("example-node".to_owned()).expect("Node creation error");


    let mut Speed_reader= node.subscribe_async::<v2::vehicle::Speed>().expect("Unable to advertise");
    let mut Moving_reader= node.subscribe_async::<v2::vehicle::IsMoving>().expect("Unable to advertise");
    let mut Ignition_reader= node.subscribe_async::<v2::vehicle::IgnitionOn>().expect("Unable to advertise");
    
    
    let res = node.spin(move || {
        
        span!(target: "MAIN", Level::TRACE, "Application Main Loop");
        info!("Application Main Loop Started with tick interval 100mS");

        let mut ticker = tokio::time::interval(Duration::from_millis(100));

        let _task = tokio::spawn( async move {
            //let mut vehicle_safe:bool;

            loop {
                let _ = ticker.tick().await;
                debug!("Tick");
                let mut speed = Samples::new(1);
                
                let currentspeed:f32=if let Ok(res) = Speed_reader.take(&mut speed).await
                {
                    
                    speed.get(0).unwrap().value().0              
                }
                else{
                    0.0
                };
            
                let mut moving = Samples::<IsMoving>::new(1);
                let currentmoving:bool=if let Ok(res) = Moving_reader.take(&mut moving).await
                {   
                    *(moving.get(0).unwrap().value())
                }
                else{
                   true
                };
                
                //let mut CurrentIgnition:bool;
                let mut ignition = Samples::<IgnitionOn>::new(1);
                let currentignition:bool =if let Ok(res) = Ignition_reader.take(&mut ignition).await
                {
                   *(ignition.get(0).unwrap().value())
                }
                else
                {
                    true
                };
                println!("Current speed {}",currentspeed);
                println!("Current moving {}",currentmoving);
                println!("Current ignition {}",currentignition);
                if  currentspeed==0.0 && currentmoving==false &&currentignition==false
                {
                    println!("Vehicle is in a Safe state");
                }
                else {
                   println!("Vehicle is NOT in a  Safe state");
                }

                

                
               // println!("Speed is {}",res1);
            }

         });
         
    });
    res

}
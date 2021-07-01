use legion::Schedule;
use crate::systems::grass_grow::*;



pub fn setup_systems() -> Schedule {
    Schedule::builder()
        .add_system(grass_grow_system())        
        .build()
}

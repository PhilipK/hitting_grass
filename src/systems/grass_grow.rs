use crate::components::*;
use legion::world::SubWorld;
use legion::*;

#[system]
#[write_component(Blade)]
pub fn grass_grow(world: &mut SubWorld) {
    <&mut Blade>::query().for_each_mut(world , |straw|{
        straw.height += 1.;
    });
}
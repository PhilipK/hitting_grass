use crate::components::*;
use legion::world::SubWorld;
use legion::*;

#[system]
#[write_component(Straw)]
pub fn grass_grow(world: &mut SubWorld) {
    <&mut Straw>::query().for_each_mut(world , |straw|{
        straw.height += 1.;
    });
}
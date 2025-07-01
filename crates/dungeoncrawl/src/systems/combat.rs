use legion::{Entity, EntityStore, Query, system, systems::CommandBuffer, world::SubWorld};

use crate::components::{Health, WantsToAttack};

#[system]
#[write_component(Health)]
pub fn combat(
    ecs: &mut SubWorld,
    cmd: &mut CommandBuffer,
    messages: &mut Query<(Entity, &WantsToAttack)>,
) {
    let victims: Vec<_> = messages
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();

    for (message, victim) in victims {
        if let Ok(health) = ecs.entry_mut(victim).unwrap().get_component_mut::<Health>() {
            health.current -= 1;
            if health.current <= 0 {
                cmd.remove(victim);
            }
        }
        cmd.remove(message);
    }
}

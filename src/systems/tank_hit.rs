use bevy::{
    ecs::{Commands, Entity, Query},
    math::Vec3,
    prelude::{DespawnRecursiveExt, GlobalTransform},
};

use crate::components::{HealthEffect, Radius, Tank, Team};

pub fn tank_hit(
    commands: &mut Commands,
    health_eff_query: Query<(Entity, &HealthEffect, &GlobalTransform, &Radius, &Team)>,
    mut tank_query: Query<(Entity, &mut Tank, &GlobalTransform, &Radius, &Team)>,
) {
    for (health_entity, health_eff, eff_trans, eff_radius, eff_team) in health_eff_query.iter() {
        for (tank_entity, mut tank, tank_trans, tank_radius, tank_team) in tank_query.iter_mut() {
            if eff_team.team == tank_team.team {
                continue;
            }
            let diff: Vec3 = eff_trans.translation - tank_trans.translation;
            if diff.length() < eff_radius.radius + tank_radius.radius {
                tank.health += health_eff.amount;
                commands.despawn(health_entity);
                if tank.health <= 0 {
                    commands.despawn_recursive(tank_entity);
                }
            }
        }
    }
}

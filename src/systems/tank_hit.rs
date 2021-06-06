use bevy::{
    math::Vec3,
    prelude::{GlobalTransform, Query, Without},
};

use crate::components::{Health, HealthEffect, Radius, Tank, Team};

pub fn tank_hit(
    mut health_eff_query: Query<
        (&HealthEffect, &mut Health, &GlobalTransform, &Radius, &Team),
        Without<Tank>,
    >,
    mut tank_query: Query<(&Tank, &mut Health, &GlobalTransform, &Radius, &Team)>,
) {
    for (health_eff, mut eff_health, eff_trans, eff_radius, eff_team) in health_eff_query.iter_mut()
    {
        for (_tank, mut tank_health, tank_trans, tank_radius, tank_team) in tank_query.iter_mut() {
            if eff_team.team == tank_team.team {
                continue;
            }
            let diff: Vec3 = eff_trans.translation - tank_trans.translation;
            if diff.length() < eff_radius.radius + tank_radius.radius {
                tank_health.0 += health_eff.amount;
                eff_health.0 -= 1;
            }
        }
    }
}

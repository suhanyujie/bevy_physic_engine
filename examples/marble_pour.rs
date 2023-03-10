use bevy::{
    core::FixedTimestep, prelude::*, reflect::erased_serde::__private::serde::__private::de,
};
use bevy_xpbd::*;
use rand::random;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.9)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(XPBPlugin::default())
        .add_startup_system(startup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1. / 20.))
                .with_system(spawn_marbles),
        )
        .add_system(despawn_marbles)
        .run();
}

struct Materials {
    blue: Handle<StandardMaterial>,
}

struct Meshes {
    sphere: Handle<Mesh>,
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere = meshes.add(Mesh::from(shape::Icosphere {
        radius: 1.,
        subdivisions: 4,
    }));
    let blue = materials.add(StandardMaterial {
        base_color: Color::rgb(0.4, 0.4, 0.6),
        unlit: true,
        ..Default::default()
    });

    let radius = 15.;
    let size = Vec2::new(10., 2.);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::ONE))),
            material: blue.clone(),
            transform: Transform::from_scale(size.extend(1.)),
            ..Default::default()
        })
        .insert_bundle(StaticBoxBundle {
            pos: Pos(Vec2::new(0., -3.)),
            collider: BoxCollider { size },
            ..Default::default()
        });

    commands.insert_resource(Meshes { sphere });

    commands.insert_resource(Materials { blue });

    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
        orthographic_projection: bevy::render::camera::OrthographicProjection {
            scale: 0.01,
            ..Default::default()
        },
        ..OrthographicCameraBundle::new_3d()
    });
}

fn spawn_marbles(mut commands: Commands, materials: Res<Materials>, meshes: Res<Meshes>) {
    let radius = 0.1;
    let pos = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5) * 0.5 + Vec2::Y * 3.;
    let vel = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.sphere.clone(),
            material: materials.blue.clone(),
            transform: Transform {
                scale: Vec3::splat(radius),
                translation: pos.extend(0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ParticleBundle {
            collider: CircleCollider { radius },
            ..ParticleBundle::new_with_pos_and_vel(pos, vel)
        });
}

fn despawn_marbles(mut commands: Commands, query: Query<(Entity, &Pos)>) {
    for (entity, pos) in query.iter() {
        if pos.0.y < -20. {
            commands.entity(entity).despawn();
        }
    }
}

use crate::GameState;
use crate::collision::CircleCollider;
use crate::player::Player;
use crate::game_object::GameObject;
use bevy::prelude::*;

pub struct FinishAreaPlugin;

#[derive(Component, Debug, PartialEq, Eq)]
pub enum FinishArea {
    Win,
    Lose,
}

/// This plugin handles finish_areas - when the player touches a finish area the game is over.
impl Plugin for FinishAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_finish_areas)
            .add_systems(Update, end_game_on_player_touch.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_finish_areas(mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    const RADIUS: f32 = 50.;
    let circle = meshes.add(Circle::new(RADIUS));
    commands.spawn((
        Mesh2d(circle.clone()),
        MeshMaterial2d(materials.add( Color::hsl(120., 0.95, 0.7) )),
        Transform::from_translation(Vec3::new(500., 0., 1.)),
        FinishArea::Win,
        CircleCollider{radius: RADIUS},
        GameObject,
    ));
    commands.spawn((
        Mesh2d(circle.clone()),
        MeshMaterial2d(materials.add( Color::hsl(0., 0.95, 0.7) )),
        Transform::from_translation(Vec3::new(-500., 0., 1.)),
        FinishArea::Lose,
        CircleCollider{radius: RADIUS},
        GameObject,
    ));
}

fn end_game_on_player_touch(
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Query<(&Transform, &CircleCollider), With<Player>>,
    finish_query: Query<(&Transform, &CircleCollider, &FinishArea)>,
) {
    // Normally there is a single player, but the loop works for any count.
    for (player_tf, player_circ) in player_query.iter() {
        let player_pos = player_tf.translation.truncate(); // Vec2 (x, y)

        for (finish_tf, finish_circ, finish_area) in finish_query.iter() {
            let finish_pos = finish_tf.translation.truncate();

            // Vector between the two centres.
            let delta = finish_pos - player_pos;
            // Squared distance (cheaper than sqrt).
            let dist_sq = delta.length_squared();
            // Sum of radii.
            let radius_sum = player_circ.radius + finish_circ.radius;

            // Collision test.
            if dist_sq <= radius_sum * radius_sum {
                match finish_area {
                    FinishArea::Win => {
                        println!("Player won that game!");
                        next_state.set(GameState::Menu);
                    }
                    FinishArea::Lose => {
                        println!("Player lost that game!");
                        next_state.set(GameState::Menu);
                    }
                }
            }
        }
    }
}

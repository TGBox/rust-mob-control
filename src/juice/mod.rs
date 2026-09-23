use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CameraShake {
    pub intensity: f32,
    pub decay: f32,
}

impl CameraShake {
    pub fn trigger(&mut self, strength: f32) {
        self.intensity = (self.intensity + strength).min(1.2);
        self.decay = 4.0;
    }
}

pub struct JuicePlugin;

impl Plugin for JuicePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraShake>()
            .add_systems(Update, update_camera_shake);
    }
}

fn update_camera_shake(
    time: Res<Time>,
    mut shake: ResMut<CameraShake>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
) {
    if shake.intensity > 0.001 {
        let dt = time.delta_secs();
        shake.intensity = (shake.intensity - shake.decay * dt).max(0.0);

        if let Ok(mut transform) = camera_query.get_single_mut() {
            let offset_x = (time.elapsed_secs() * 35.0).sin() * shake.intensity * 0.4;
            let offset_y = (time.elapsed_secs() * 42.0).cos() * shake.intensity * 0.3;
            transform.translation.x = offset_x;
            transform.translation.y = 16.5 + offset_y;
        }
    }
}

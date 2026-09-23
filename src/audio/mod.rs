pub mod sfx;

use bevy::prelude::*;
use std::sync::Arc;
pub use sfx::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySfx>()
            .add_systems(Startup, setup_audio)
            .add_systems(Update, handle_play_sfx);
    }
}

fn setup_audio(mut commands: Commands, mut audio_sources: ResMut<Assets<AudioSource>>) {
    // Generate WAV in memory and register as AudioSource assets
    let shoot = audio_sources.add(AudioSource {
        bytes: Arc::from(ProceduralSfx::shoot().into_boxed_slice()),
    });
    let gate_chime = audio_sources.add(AudioSource {
        bytes: Arc::from(ProceduralSfx::gate_chime().into_boxed_slice()),
    });
    let pop = audio_sources.add(AudioSource {
        bytes: Arc::from(ProceduralSfx::pop().into_boxed_slice()),
    });
    let champion_stomp = audio_sources.add(AudioSource {
        bytes: Arc::from(ProceduralSfx::champion_stomp().into_boxed_slice()),
    });
    let victory = audio_sources.add(AudioSource {
        bytes: Arc::from(ProceduralSfx::victory().into_boxed_slice()),
    });
    let defeat = audio_sources.add(AudioSource {
        bytes: Arc::from(ProceduralSfx::defeat().into_boxed_slice()),
    });

    commands.insert_resource(SoundEffects {
        shoot,
        gate_chime,
        pop,
        champion_stomp,
        victory,
        defeat,
    });
}

fn handle_play_sfx(
    mut events: EventReader<PlaySfx>,
    sfx: Option<Res<SoundEffects>>,
    mut commands: Commands,
) {
    let Some(sfx) = sfx else { return };
    for event in events.read() {
        let handle = match event {
            PlaySfx::Shoot => &sfx.shoot,
            PlaySfx::GateChime => &sfx.gate_chime,
            PlaySfx::Pop => &sfx.pop,
            PlaySfx::ChampionStomp => &sfx.champion_stomp,
            PlaySfx::Victory => &sfx.victory,
            PlaySfx::Defeat => &sfx.defeat,
        };
        commands.spawn((
            AudioPlayer::new(handle.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}

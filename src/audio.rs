use std::{iter::Cycle, vec::IntoIter};

use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{AudioSink, OggFormat, output::Output, Source, SourceHandle},
    ecs::{World, WorldExt},
};

const BOUNCE_SOUND_PATH: &str = "audio/bounce.ogg";
const SCORE_SOUND_PATH: &str = "audio/score.ogg";
const MUSIC_TRACKS: &[&str] = &[
    &"audio/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
    &"audio/Computer_Music_All-Stars_-_Albatross_v2.ogg",
];

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle,
}

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>
}

pub fn initialize_audio(world: &mut World) {
    let (music, sound) = {
        let loader = world.read_resource();
        let mut audio_sink = world.write_resource::<AudioSink>();
        audio_sink.set_volume(0.25);

        (
            Music {
                music: MUSIC_TRACKS
                    .iter()
                    .map(|path| load_audio_track(&loader, world, path))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .cycle()
            },
            Sounds {
                score_sfx: load_audio_track(&loader, &world, SCORE_SOUND_PATH),
                bounce_sfx: load_audio_track(&loader, &world, BOUNCE_SOUND_PATH),
            }
        )
    };

    world.insert(music);
    world.insert(sound);
}

pub fn play_bounce_sound(sounds: &Sounds, sound_storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(output) = output {
        if let Some(sound) = sound_storage.get(&sounds.bounce_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_score_sound(sounds: &Sounds, sound_storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(output) = output {
        if let Some(sound) = sound_storage.get(&sounds.score_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

fn load_audio_track(loader: &Loader, world: &World, path: &str) -> SourceHandle {
    loader.load(path, OggFormat, (), &world.read_resource())
}

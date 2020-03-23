use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{OggFormat, output::Output, Source, SourceHandle},
    ecs::{World, WorldExt},
};

const BOUNCE_SOUND_PATH: &str = "audio/bounce.ogg";
const SCORE_SOUND_PATH: &str = "audio/score.ogg";

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle,
}

pub fn initialize_audio(world: &mut World) {
    world.insert({
        let loader = world.read_resource();
        Sounds {
            score_sfx: load_audio_track(&loader, &world, SCORE_SOUND_PATH),
            bounce_sfx: load_audio_track(&loader, &world, BOUNCE_SOUND_PATH),
        }
    });
}

pub fn play_bounce_sound(sounds: &Sounds, sound_storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(output) = output {
        if let Some(sound) = sound_storage.get(&sounds.bounce_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

fn load_audio_track(loader: &Loader, world: &World, path: &str) -> SourceHandle {
    loader.load(path, OggFormat, (), &world.read_resource())
}

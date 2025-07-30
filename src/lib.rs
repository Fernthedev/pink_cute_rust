#![feature(box_patterns, extend_one)]
#![feature(generic_arg_infer)]

use std::ffi::{CString, c_char};

use bs_cordl::GlobalNamespace::{BeatmapData, IReadonlyBeatmapData, NoteData};
use bs_cordl::TMPro::TextMeshPro;
use bs_cordl::UnityEngine::{self};
use quest_hook::hook;
use quest_hook::libil2cpp::{Gc, Il2CppString};
use tracing::info;

use scotland2_rs::{ModInfoBuf, scotland2_raw::CModInfo};

use std::sync::LazyLock;

unsafe extern "C" {
    unsafe fn doSomething(ptr: Gc<TextMeshPro>);
}

#[hook("TMPro", "TextMeshPro", "Awake")]
fn TextMeshPro_Awake(this: &mut TextMeshPro) {
    TextMeshPro_Awake.original(this);

    let text = "Pink cute";
    this.set_text(Il2CppString::new(text)).unwrap();

    unsafe {
        doSomething(this.into());
    }
}
#[hook("TMPro", "TextMeshPro", "set_text")]
fn TextMeshPro_set_text(this: &mut TextMeshPro, mut text: Gc<Il2CppString>) {
    // if text.
    text = Il2CppString::new("Pink cute");
    TextMeshPro_set_text.original(this, text);
}

static MOD_INFO: LazyLock<ModInfoBuf> = LazyLock::new(|| ModInfoBuf {
    id: "pink_cute".to_string(),
    version: "0.1.0".to_string(),
    version_long: 1,
});

#[unsafe(no_mangle)]
extern "C" fn setup(mod_info: &mut CModInfo) {
    *mod_info = MOD_INFO.clone().into();

    println!("Setup: {}", *mod_info);

    // setup quest-hook
    // which will setup tracing and panic logging
    // TODO: Use paper?
    quest_hook::setup("PinkCute");
}

#[unsafe(no_mangle)]
extern "C" fn late_load() {
    TextMeshPro_Awake.install().unwrap();
    TextMeshPro_set_text.install().unwrap();

    let color = UnityEngine::Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    let mut go = bs_cordl::UnityEngine::GameObject::New_1().unwrap();

    go.set_active(true).unwrap();

    let beatmap = BeatmapData::New(4).unwrap();

    let mut interface_beatmap: Gc<IReadonlyBeatmapData> = beatmap.cast();

    info!(
        "Beatmap notes: {:?}",
        interface_beatmap
            .GetBeatmapDataItems::<Gc<NoteData>>(0)
            .unwrap()
    );
    info!(
        "Beatmap notes count: {:?}",
        interface_beatmap.get_cuttableNotesCount()
    );
}

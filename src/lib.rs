#![feature(box_patterns, extend_one)]
#![feature(generic_arg_infer)]

use std::ffi::{CString, c_char};

use bs_cordl::GlobalNamespace::{BeatmapData, IReadonlyBeatmapData, NoteData};
use bs_cordl::TMPro::TextMeshPro;
use bs_cordl::UnityEngine::{self};
use quest_hook::hook;
use quest_hook::libil2cpp::{Gc, Il2CppString};

extern "C" {
    #[unsafe(no_mangle)]
    unsafe fn doSomething(ptr: Gc<TextMeshPro>);
}

#[hook("TMPro", "TextMeshPro", "Awake")]
fn TextMeshPro_Awake(this: &mut TextMeshPro) {
    TextMeshPro_Awake.original(this);

    let text = "Pink cute";
    this.set_text(Il2CppString::new(text)).unwrap();

    unsafe {
        doSomething(this.cast());
    }
}
#[hook("TMPro", "TextMeshPro", "set_text")]
fn TextMeshPro_set_text(this: &mut TextMeshPro, mut text: Gc<Il2CppString>) {
    // if text.
    text = Il2CppString::new("Pink cute");
    TextMeshPro_set_text.original(this, text);
}

#[repr(C)]
pub struct ModInfo {
    id: *const c_char,
    version: *const c_char,
    version_long: u64,
}

#[no_mangle]
extern "C" fn setup(modinfo: *mut ModInfo) {
    unsafe {
        *modinfo = ModInfo {
            // we have to let the string leak, because the CString is dropped at the end of the function
            id: CString::new("pink_cute").unwrap().into_raw(),
            version: CString::new("1.0.0").unwrap().into_raw(),
            version_long: 0,
        }
    }

    // setup quest-hook
    // which will setup tracing and panic logging
    // TODO: Use paper?
    quest_hook::setup("PinkCute");
}

#[no_mangle]
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

    println!(
        "Beatmap notes: {:?}",
        interface_beatmap
            .GetBeatmapDataItems::<Gc<NoteData>>(0)
            .unwrap()
    );
    println!(
        "Beatmap notes count: {:?}",
        interface_beatmap.get_cuttableNotesCount()
    );
}

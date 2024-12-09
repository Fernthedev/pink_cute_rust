#![feature(box_patterns, extend_one)]
#![feature(generic_arg_infer)]


use bs_cordl::TMPro::TextMeshPro;
use bs_cordl::UnityEngine::{self};
use quest_hook::hook;
use quest_hook::libil2cpp::{
    Il2CppString,
    ObjectExt, WrapRaw,
};

#[hook("TMPro", "TextMeshPro", "Awake")]
fn TextMeshPro_Awake(this: &mut TextMeshPro) {
    TextMeshPro_Awake.original(this);

    let text = "Pink cute";
    this.set_text(Il2CppString::new(text)).unwrap();
}
#[hook("TMPro", "TextMeshPro", "set_text")]
fn TextMeshPro_set_text(this: &mut TextMeshPro, mut text: &mut Il2CppString) {
    // if text.
    text = Il2CppString::new("Pink cute");
    TextMeshPro_set_text.original(this, text);
}

#[no_mangle]
pub extern "C" fn setup() {
    quest_hook::setup("PinkCute");
}

#[no_mangle]
pub extern "C" fn late_load() {
    TextMeshPro_Awake.install().unwrap();
    TextMeshPro_set_text.install().unwrap();

    let color = UnityEngine::Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    let go = unsafe {
        bs_cordl::UnityEngine::GameObject::New_1()
            .unwrap()
            .as_mut()
            .unwrap()
    };
    go.SetActive(true).unwrap();
}

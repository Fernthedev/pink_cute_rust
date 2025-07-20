& cargo ndk --bindgen -t arm64-v8a -o ./build/debug build # --release
& adb push ./build/debug/arm64-v8a/libparty_panel.so /sdcard/ModData/com.beatgames.beatsaber/Modloader/mods/libparty_panel.so

& adb shell am force-stop com.beatgames.beatsaber
& adb shell am start com.beatgames.beatsaber/com.unity3d.player.UnityPlayerActivity
Start-Sleep -Seconds 1.0
& adb shell am start com.beatgames.beatsaber/com.unity3d.player.UnityPlayerActivity

adb logcat -c
adb logcat > log.txt
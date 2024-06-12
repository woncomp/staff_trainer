use bevy::prelude::*;

mod trainer;

#[cfg(target_os = "android")]
use jni::objects::{JObject, JValue};

/// Needs to be called from the main thread
#[cfg(target_os = "android")]
pub fn enable_immersive_mode() -> anyhow::Result<()> {
    let android_app = bevy::winit::ANDROID_APP.get().unwrap();
    let vm = unsafe { jni::JavaVM::from_raw(android_app.vm_as_ptr() as *mut *const _)? };
    let activity = unsafe { JObject::from_raw(android_app.activity_as_ptr() as *mut _) };
    let mut env = vm.attach_current_thread()?;

    // getWindow()
    let window = env
        .call_method(activity, "getWindow", "()Landroid/view/Window;", &[])?
        .l()?;

    // getWindow().getInsetsController()
    let insets_controller = env
        .call_method(
            &window,
            "getInsetsController",
            "()Landroid/view/WindowInsetsController;",
            &[],
        )?
        .l()?;

    let systemBars = 3; // WindowInsetsCompat.Type.systemBars()
    env.call_method(&insets_controller, "hide", "(I)V", &[JValue::from(systemBars)])?;

    Ok(())
}

pub fn run_game() {
    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default().with_max_level(log::LevelFilter::Info),
        );
        let r = enable_immersive_mode();
        println!("enable_immersive_mode {:?}", r);
    }

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Staff Trainer".into(),
            resolution: bevy::window::WindowResolution::new(1200., 540.).with_scale_factor_override(2.),
            mode: bevy::window::WindowMode::Fullscreen,
            ..default()
        }),
        ..default()
    };

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.95, 0.95, 0.95)))
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(trainer::TrainerPlugin)
        .run();
}

#[bevy_main]
fn main() {
    run_game();
}
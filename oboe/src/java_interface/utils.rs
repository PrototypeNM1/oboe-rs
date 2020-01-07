use std::sync::Arc;

use android_ndk::{
    android_app::AndroidApp,
};

pub use jni::{
    Executor,
};

pub use android_ndk::{
    native_activity::NativeActivity,
};

pub use jni::{
    JavaVM, JNIEnv,
    objects::{JObject, JList, JValue},
    strings::JavaStr,
    errors::{Result as JResult},
};

pub fn get_activity() -> NativeActivity {
    let app = unsafe {
        AndroidApp::from_ptr(android_glue::get_android_app())
    };
    app.activity()
}

pub fn with_attached<F, R>(activity: &NativeActivity, closure: F) -> JResult<R>
where
    F: FnOnce(&JNIEnv, JObject) -> JResult<R>,
{
    let vm = Arc::new(unsafe { JavaVM::from_raw(activity.vm())? });
    let activity = activity.activity();
    Executor::new(vm).with_attached(|env| closure(env, activity.into()))
}

pub fn call_method_no_args_ret_int_array<'a>(env: &JNIEnv<'a>, subject: JObject, method: &str) -> JResult<Vec<i32>> {
    let array = env.auto_local(env.call_method(
        subject,
        method,
        "()[I",
        &[],
    )?.l()?);

    let raw_array = array.as_obj().into_inner();

    let length = env.get_array_length(raw_array)?;
    let mut values = Vec::with_capacity(length as usize);

    env.get_int_array_region(raw_array, 0, values.as_mut())?;

    Ok(values)
}

pub fn call_method_no_args_ret_int<'a>(env: &JNIEnv<'a>, subject: JObject, method: &str) -> JResult<i32> {
    env.call_method(
        subject,
        method,
        "()I",
        &[],
    )?.i()
}

pub fn call_method_no_args_ret_bool<'a>(env: &JNIEnv<'a>, subject: JObject, method: &str) -> JResult<bool> {
    env.call_method(
        subject,
        method,
        "()Z",
        &[],
    )?.z()
}

pub fn call_method_no_args_ret_string<'a>(env: &JNIEnv<'a>, subject: JObject, method: &str) -> JResult<String> {
    env.get_string(
        env.call_method(
            subject,
            method,
            "()Ljava/lang/String;",
            &[],
        )?.l()?.into()
    ).map(String::from)
}

pub fn call_method_no_args_ret_char_sequence<'a>(env: &JNIEnv<'a>, subject: JObject, method: &str) -> JResult<String> {
    env.get_string(
        env.call_method(
            env.call_method(
                subject,
                method,
                "()Ljava/lang/CharSequence;",
                &[],
            )?.l()?,
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?.l()?.into()
    ).map(String::from)
}

pub fn call_method_string_arg_ret_bool<'a, S: AsRef<str>>(env: &JNIEnv<'a>, subject: JObject, name: &str, arg: S) -> JResult<bool> {
    env.call_method(
        subject,
        name,
        "(Ljava/lang/String;)Z",
        &[JObject::from(env.new_string(arg)?).into()],
    )?.z()
}

pub fn call_method_string_arg_ret_string<'a: 'b, 'b, S: AsRef<str>>(env: &'b JNIEnv<'a>, subject: JObject, name: &str, arg: S) -> JResult<JavaStr<'a, 'b>> {
    env.get_string(
        env.call_method(
            subject,
            name,
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[JObject::from(env.new_string(arg)?).into()],
        )?.l()?.into()
    )
}

pub fn call_method_string_arg_ret_object<'a>(env: &JNIEnv<'a>, subject: JObject, method: &str, arg: &str) -> JResult<JObject<'a>> {
    env.call_method(
        subject,
        method,
        "(Ljava/lang/String;)Ljava/lang/Object;",
        &[JObject::from(env.new_string(arg)?).into()],
    )?.l()
}

pub fn get_package_manager<'a>(env: &JNIEnv<'a>, subject: JObject) -> JResult<JObject<'a>> {
    env.call_method(
        subject,
        "getPackageManager",
        "()Landroid/content/pm/PackageManager;",
        &[],
    )?.l()
}

pub fn has_system_feature<'a>(env: &JNIEnv<'a>, subject: JObject, name: &str) -> JResult<bool> {
    call_method_string_arg_ret_bool(env, subject, "hasSystemFeature", name)
}

pub fn get_system_service<'a>(env: &JNIEnv<'a>, subject: JObject, name: &str) -> JResult<JObject<'a>> {
    call_method_string_arg_ret_object(env, subject, "getSystemService", name)
}

pub fn get_property<'a: 'b, 'b>(env: &'b JNIEnv<'a>, subject: JObject, name: &str) -> JResult<JavaStr<'a, 'b>> {
    call_method_string_arg_ret_string(env, subject, "getProperty", name)
}

pub fn get_devices<'a: 'b, 'b>(env: &'b JNIEnv<'a>, subject: JObject, flags: i32) -> JResult<JObject<'a>> {
    env.call_method(
        subject,
        "getDevices",
        "(I)[Landroid/media/AudioDeviceInfo;",
        &[flags.into()],
    )?.l()
}

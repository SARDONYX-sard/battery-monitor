use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt as _;

/// # Info
/// Write to the log when an error occurs.
///
/// # Ref
/// - https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/notification
/// - https://zenn.dev/8beeeaaat/scraps/211b820f5c14d7
pub fn notify(app: &AppHandle, message: &str) -> Result<(), tauri_plugin_notification::Error> {
    app.notification()
        .builder()
        .title("[bluetooth battery monitor]")
        .body(message)
        .show()
}

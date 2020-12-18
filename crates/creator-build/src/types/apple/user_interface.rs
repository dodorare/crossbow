use serde::{Deserialize, Serialize};

/// Main User Interface.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct MainUserInterface {
    /// The information about the app's scene-based life-cycle support.
    ///
    /// The presence of this key indicates that the app supports scenes and does not
    /// use an app delegate object to manage transitions to and from the foreground or background.
    #[serde(
        flatten,
        rename = "UIApplicationSceneManifest",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_scene_manifest: Option<ApplicationSceneManifest>,
    /// The name of an app's storyboard resource file.
    #[serde(
        rename = "NSMainStoryboardFile",
        skip_serializing_if = "Option::is_none"
    )]
    pub main_storyboard_resource_file_base_name: Option<String>,
    /// The name of the app’s main storyboard file.
    #[serde(
        rename = "UIMainStoryboardFile",
        skip_serializing_if = "Option::is_none"
    )]
    pub main_storyboard_file_base_name: Option<String>,
    /// The name of an app’s main user interface file.
    #[serde(rename = "NSMainNibFile", skip_serializing_if = "Option::is_none")]
    pub main_nib_file_base_name: Option<String>,
    /// A Boolean value indicating whether the app is an agent app that runs in the background and doesn't appear in the Dock.
    #[serde(rename = "LSUIElement", skip_serializing_if = "Option::is_none")]
    pub application_is_agent: Option<bool>,
}

/// Application Scene Manifest.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ApplicationSceneManifest {
    /// A Boolean value indicating whether the app supports two or more scenes simultaneously.
    ///
    /// If your app supports multiple scenes, set the value of this key to true.
    /// If you set the value to false, UIKit never creates more than one scene for your app.
    ///
    /// Setting this key to true has implications for your code. An app that supports multiple scenes
    /// must coordinate operations to prevent scenes from interfering with each other. For example,
    /// if two scenes access the same shared resource, you must synchronize access to that resource
    /// using a serial dispatch queue or some other mechanism. Failure to do so may lead
    /// to corrupted data or unexpected behavior from your app.
    #[serde(
        rename = "UIApplicationSupportsMultipleScenes",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_multiple_windows: Option<bool>,
    /// The default configuration details for UIKit to use when creating new scenes.
    #[serde(
        flatten,
        rename = "UISceneConfigurations",
        skip_serializing_if = "Option::is_none"
    )]
    pub scene_configurations: Option<SceneConfigurations>,
}

/// Scene Configurations.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SceneConfigurations {
    /// Scenes that you use to display content on the device's main screen and respond to user interactions.
    ///
    /// Use this key to specify the scene configurations for your app.
    /// Each scene corresponds to one you use for content you display on the device's main screen.
    /// Make your app's default scene the first entry in the array.
    #[serde(
        flatten,
        rename = "UIWindowSceneSessionRoleApplication",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_session_role: Option<WindowSceneSessionRole>,
    /// Scenes that you use to display content on an externally connected display.
    ///
    /// Use this key to specify the scene configurations you use when displaying content on an
    /// external display. Make the default scene the first entry in the array.
    #[serde(
        flatten,
        rename = "UIWindowSceneSessionRoleExternalDisplay",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_display_session_role: Option<WindowSceneSessionRole>,
}

/// Window Scene Session Role.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct WindowSceneSessionRole {
    /// The app-specific name you use to identify the scene.
    #[serde(
        rename = "UISceneConfigurationName",
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_name: Option<String>,
    /// The name of the scene class you want UIKit to instantiate.
    ///
    /// Specify UIWindowScene for scenes meant for your app or an external display. Do not specify UIScene.
    #[serde(rename = "UISceneClassName", skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    /// The name of the app-specific class that you want UIKit to instantiate and use as the scene delegate object.
    ///
    /// The class you specify for this key must adopt the UISceneDelegate protocol.
    /// If the class you specify for the UISceneClassName key is UIWindowScene,
    /// your class must adopt the UIWindowSceneDelegate protocol.
    #[serde(
        rename = "UISceneDelegateClassName",
        skip_serializing_if = "Option::is_none"
    )]
    pub delegate_class_name: Option<String>,
    /// The name of the storyboard file containing the scene's initial user interface.
    ///
    /// Specify the name of the storyboard file without the filename extension. For example,
    /// if the filename of your storyboard is Main.storyboard, specify Main as the value for this key.
    #[serde(
        rename = "UISceneStoryboardFile",
        skip_serializing_if = "Option::is_none"
    )]
    pub storyboard_name: Option<String>,
}

/// App Execution.
///
/// Control app launch, execution, and termination.
///
/// Your app interacts with the system during normal execution by calling system APIs.
/// However, you need to communicate information about how to execute your app before you have access to these API calls.
/// For example, you may need to specify under what conditions your app can launch, the environment that it should launch into,
/// and what should happen when it terminates. You add keys to your app’s Information Property List file to manage its execution.
///
use super::{serialize_enum_option, serialize_vec_enum_option};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Launch.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Launch {
    /// The name of the bundle’s main executable class.
    ///
    /// The system uses the class identified by this key to set the principalClass property of a bundle when it’s loaded.
    ///
    /// Xcode sets the default value of this key to NSApplication for macOS apps, and to UIApplication for iOS and tvOS apps.
    /// For other types of bundles, you must set this key in The Info.plist File.
    #[serde(
        rename(serialize = "NSPrincipalClass"),
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_class: Option<String>,
    /// The name of the class that implements the complication data source protocol.
    ///
    /// Xcode automatically includes this key in the information property list when you modify the WatchKit extension’s
    /// data source (General > Complication Configuration > Data Source class).
    #[serde(
        rename(serialize = "CLKComplicationPrincipalClass"),
        skip_serializing_if = "Option::is_none"
    )]
    pub complication_principal_class: Option<Vec<String>>,
    /// The name of the bundle’s executable file.
    ///
    /// For an app, this key is the executable. For a loadable bundle, it's the binary that's loaded dynamically
    /// by the bundle. For a framework, it's the shared library framework and must have the same name as the
    /// framework but without the .framework extension.
    ///
    /// macOS uses this key to locate the bundle’s executable or shared library in cases where the user renames the app or bundle directory.
    #[serde(
        rename(serialize = "CFBundleExecutable"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_executable: Option<String>,
    /// Environment variables to set before launching the app.
    #[serde(
        rename(serialize = "LSEnvironment"),
        skip_serializing_if = "Option::is_none"
    )]
    pub environment: Option<Vec<String>>,
    /// Application shortcut items.
    #[serde(
        rename(serialize = "UIApplicationShortcutItems"),
        skip_serializing_if = "Option::is_none"
    )]
    pub application_shortcut_items: Option<Vec<ApplicationShortcutItem>>,
}

/// Application Shortcut Item.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ApplicationShortcutItem {
    #[serde(
        rename(serialize = "UIApplicationShortcutItemIconFile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_file: Option<String>,
    #[serde(
        rename(serialize = "UIApplicationShortcutItemIconSymbolName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub symbol_name: Option<String>,
    #[serde(
        rename(serialize = "UIApplicationShortcutItemIconType"),
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_type: Option<String>,
    #[serde(
        rename(serialize = "UIApplicationShortcutItemSubtitle"),
        skip_serializing_if = "Option::is_none"
    )]
    pub subtitle: Option<String>,
    #[serde(rename(serialize = "UIApplicationShortcutItemTitle"))]
    pub title: String,
    #[serde(rename(serialize = "UIApplicationShortcutItemType"))]
    pub item_type: String,
    #[serde(
        rename(serialize = "UIApplicationShortcutItemUserInfo"),
        skip_serializing_if = "Option::is_none"
    )]
    pub user_info: Option<BTreeMap<String, String>>,
}

/// Launch Conditions.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct LaunchConditions {
    /// The device-related features that your app requires to run.
    ///
    /// The App Store prevents customers from installing an app on a device that doesn’t support the required capabilities
    /// for that app. Use this key to declare the capabilities your app requires. For a list of the features that different
    /// devices support, see Required Device Capabilities.
    ///
    /// You typically use an array for the key’s associated value. The presence in that array of any of the above possible values
    /// indicates that the app requires the corresponding feature. Omit a value to indicate that the app doesn’t require
    /// the feature, but it can be present.
    ///
    /// Alternatively, you can use a dictionary as the associated value for the UIRequiredDeviceCapabilities key. In that case,
    /// use the values above as the dictionary’s keys, each with an associated Boolean value. Set the value to true to require
    /// the corresponding feature. Set the value to false to indicate that the feature must not be present on the device. Omit
    /// the feature from the dictionary to indicate that your app neither requires nor disallows it.
    ///
    /// Specify only the features that your app absolutely requires. If your app can accommodate missing features by avoiding
    /// the code paths that use those features, don’t include the corresponding key.
    #[serde(
        rename(serialize = "UIRequiredDeviceCapabilities"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_vec_enum_option"
    )]
    pub required_device_capabilities: Option<Vec<DeviceCapabilities>>,
    /// An array of the architectures that the app supports, arranged according to their preferred usage.
    ///
    /// Use this key to prioritize the execution of a specific architecture in a universal binary. This key contains an array
    /// of strings, with each string specifying the name of a supported architecture. The order of the strings in the array
    /// represents your preference for executing the app. For example, if you specify the x86_64 architecture first for a
    /// universal app, the system runs that app under Rosetta translation on Apple silicon. For more information about
    /// Rosetta translation, see About the Rosetta Translation Environment.
    #[serde(
        rename(serialize = "LSArchitecturePriority"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub architecture_priority: Option<ArchitecturePriority>,
    /// A Boolean value that indicates whether to require the execution of the app’s native architecture when multiple
    /// architectures are available.
    ///
    /// When an app supports multiple architectures, the presence of this key causes the system to choose the native architecture
    /// over ones that require translation. For example, this key prevents the system from using the Rosetta translation process
    /// to execute the Intel portion of a universal app on Apple silicon.
    #[serde(
        rename(serialize = "LSRequiresNativeExecution"),
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_native_execution: Option<bool>,
    /// A Boolean value indicating whether the user can install and run the watchOS app independently of its iOS companion app.
    ///
    /// Xcode automatically includes this key in the WatchKit extension’s information property list and sets its value to true
    /// when you create a project using the iOS App with Watch App template. When you set the value of this key to true, the app
    /// doesn’t need its iOS companion app to operate properly. Users can choose to install the iOS app, the watchOS app, or both.
    #[serde(
        rename(serialize = "WKRunsIndependentlyOfCompanionApp"),
        skip_serializing_if = "Option::is_none"
    )]
    pub runs_independently_of_companion_app: Option<bool>,
    /// A Boolean value indicating whether the app is a watch-only app.
    ///
    /// Xcode automatically includes this key in the WatchKit extension’s information property list and sets its value to true
    /// when you create a project using the Watch App template. When you set the value of this key to true, the app is only available
    /// on Apple Watch, with no related iOS app.
    #[serde(
        rename(serialize = "WKWatchOnly"),
        skip_serializing_if = "Option::is_none"
    )]
    pub watch_only: Option<bool>,
    /// A Boolean value that indicates whether a watchOS app should opt out of automatically launching when its companion iOS
    /// app starts playing audio content.
    ///
    /// If your watchOS app does not act as a remote control for the iOS app, set this key to true in your WatchKit extension’s
    /// information property list.
    #[serde(
        rename(serialize = "PUICAutoLaunchAudioOptOut"),
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_launch_audio_opt_out: Option<bool>,
    /// The complication families that the app can provide data for.
    ///
    /// To add this key to the information property list, enable the desired families in the WatchKit extension’s Complication
    /// Configuration settings.
    #[serde(
        rename(serialize = "CLKComplicationSupportedFamilies"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_vec_enum_option"
    )]
    pub complication_supported_families: Option<Vec<ComplicationSupportedFamilies>>,
}

/// Extensions and Services.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ExtensionsAndServices {
    /// The properties of an app extension.
    #[serde(
        rename(serialize = "NSExtension"),
        skip_serializing_if = "Option::is_none"
    )]
    pub extension: Option<Extension>,
    /// The services provided by an app.
    #[serde(
        rename(serialize = "NSServices"),
        skip_serializing_if = "Option::is_none"
    )]
    pub services: Option<Vec<Service>>,
    /// The name of your watchOS app’s extension delegate.
    ///
    /// This key provides the name of a class that adopts the WKExtensionDelegate protocol. Xcode automatically includes
    /// this key in the WatchKit extension’s information property list when you create a watchOS project from a template.
    /// You only modify this value when you rename or replace the extension delegate.
    #[serde(
        rename(serialize = "WKExtensionDelegateClassName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_delegate_class_name: Option<String>,
    /// The bundle ID of the widget that's available as a Home screen quick action in apps that have more than one widget.
    #[serde(
        rename(serialize = "UIApplicationShortcutWidget"),
        skip_serializing_if = "Option::is_none"
    )]
    pub application_shortcut_widget: Option<String>,
}

/// App Clips.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct AppClips {
    /// A collection of keys that an App Clip uses to get additional capabilities.
    #[serde(
        rename(serialize = "NSAppClip"),
        skip_serializing_if = "Option::is_none"
    )]
    pub app_clip: Option<AppClip>,
}

/// Background Execution.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct BackgroundExecution {
    /// Services provided by an app that require it to run in the background.
    #[serde(
        rename(serialize = "UIBackgroundModes"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_vec_enum_option"
    )]
    pub ui_background_modes: Option<Vec<UiBackgroundMode>>,
    /// The services a watchOS app provides that require it to continue running in the background.
    ///
    /// You can only enable one of the extended runtime session modes (self-care, mindfulness, physical-therapy, or alarm).
    /// However, you can enable both an extended runtime session mode and the workout-processing mode. If you set the background
    /// modes using Xcode’s Signing & Capabilities tab, Xcode insures that these values are set properly.
    #[serde(
        rename(serialize = "WKBackgroundModes"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_vec_enum_option"
    )]
    pub wk_background_modes: Option<Vec<WkBackgroundMode>>,
    /// An array of strings containing developer-specified task identifiers in reverse URL notation.
    #[serde(
        rename(serialize = "BGTaskSchedulerPermittedIdentifiers"),
        skip_serializing_if = "Option::is_none"
    )]
    pub task_scheduler_permitted_identifiers: Option<Vec<String>>,
    /// A Boolean value indicating whether the app runs only in the background.
    #[serde(
        rename(serialize = "LSBackgroundOnly"),
        skip_serializing_if = "Option::is_none"
    )]
    pub background_only: Option<bool>,
}

/// Endpoint Security.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct EndpointSecurity {
    #[serde(
        rename(serialize = "NSEndpointSecurityEarlyBoot"),
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_security_early_boot: Option<bool>,
    #[serde(
        rename(serialize = "NSEndpointSecurityRebootRequired"),
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_security_reboot_required: Option<bool>,
}

/// Plugin Support.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct PluginSupport {
    /// The name of the app's plugin bundle.
    #[serde(
        rename(serialize = "NSDockTilePlugIn"),
        skip_serializing_if = "Option::is_none"
    )]
    pub dock_tile_plugin: Option<String>,
}

/// Plugin Configuration.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct PluginConfiguration {
    /// The function to use when dynamically registering a plugin.
    #[serde(
        rename(serialize = "CFPlugInDynamicRegisterFunction"),
        skip_serializing_if = "Option::is_none"
    )]
    pub plugin_dynamic_register_function: Option<String>,
    /// A Boolean value indicating whether the host loads this plugin.
    #[serde(
        rename(serialize = "CFPlugInDynamicRegistration"),
        skip_serializing_if = "Option::is_none"
    )]
    pub plugin_dynamic_registration: Option<bool>,
    /// The interfaces supported by the plugin for static registration.
    #[serde(
        rename(serialize = "CFPlugInFactories"),
        skip_serializing_if = "Option::is_none"
    )]
    pub plugin_factories: Option<BTreeMap<String, String>>,
    /// One or more groups of interfaces supported by the plugin for static registration.
    #[serde(
        rename(serialize = "CFPlugInTypes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub plugin_types: Option<BTreeMap<String, String>>,
    /// The name of the function to call to unload the plugin code from memory.
    #[serde(
        rename(serialize = "CFPlugInUnloadFunction"),
        skip_serializing_if = "Option::is_none"
    )]
    pub plugin_unload_function: Option<String>,
}

/// Termination.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Termination {
    /// A Boolean value indicating whether the app is notified when a child process dies.
    #[serde(
        rename(serialize = "LSGetAppDiedEvents"),
        skip_serializing_if = "Option::is_none"
    )]
    pub get_app_died_events: Option<bool>,
    /// A Boolean value indicating whether the system may terminate the app to log out or shut down more quickly.
    #[serde(
        rename(serialize = "NSSupportsSuddenTermination"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_sudden_termination: Option<bool>,
    /// Deprecated: A Boolean value indicating whether the app terminates, rather than moves to the background, when the app quits.
    #[serde(
        rename(serialize = "UIApplicationExitsOnSuspend"),
        skip_serializing_if = "Option::is_none"
    )]
    pub application_exits_on_suspend: Option<bool>,
}

/// WK Background Mode.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum WkBackgroundMode {
    /// Allows an active workout session to run in the background.
    #[serde(rename(serialize = "workout-processing"))]
    WorkoutProcessing,
    /// Enables extended runtime sessions for brief activities focusing on health or emotional well-being.
    #[serde(rename(serialize = "self-care"))]
    SelfCare,
    /// Enables extended runtime sessions for silent meditation.
    #[serde(rename(serialize = "mindfulness"))]
    Mindfulness,
    /// Enables extended runtime sessions for stretching, strengthening, or range-of-motion exercises.
    #[serde(rename(serialize = "physical-therapy"))]
    PhysicalTherapy,
    /// Enables extended runtime sessions for smart alarms.
    #[serde(rename(serialize = "alarm"))]
    Alarm,
}

/// UI Background Mode.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum UiBackgroundMode {
    #[serde(rename(serialize = "audio"))]
    Audio,
    #[serde(rename(serialize = "location"))]
    Location,
    #[serde(rename(serialize = "voip"))]
    Voip,
    #[serde(rename(serialize = "external-accessory"))]
    ExternalAccessory,
    #[serde(rename(serialize = "bluetooth-central"))]
    BluetoothCentral,
    #[serde(rename(serialize = "bluetooth-peripheral"))]
    BluetoothPeripheral,
    #[serde(rename(serialize = "fetch"))]
    Fetch,
    #[serde(rename(serialize = "remote-notification"))]
    RemoteNotification,
    #[serde(rename(serialize = "processing"))]
    Processing,
}

/// App Clip.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct AppClip {
    /// A Boolean value that indicates whether an App Clip can schedule or receive notifications for a limited amount of time.
    ///
    /// Set the corresponding value to true to enable your App Clip to schedule or receive notifications for up to 8 hours
    /// after each launch. For more information, see Enabling Notifications in App Clips.
    #[serde(
        rename(serialize = "NSAppClipRequestEphemeralUserNotification"),
        skip_serializing_if = "Option::is_none"
    )]
    pub request_ephemeral_user_notification: Option<bool>,
    /// A Boolean value that indicates whether an App Clip can confirm the user’s location.
    ///
    /// Set the value to true to allow your App Clip to confirm the user’s location. For more information, see Responding to Invocations.
    #[serde(
        rename(serialize = "NSAppClipRequestLocationConfirmation"),
        skip_serializing_if = "Option::is_none"
    )]
    pub request_location_confirmation: Option<bool>,
}

/// Extension.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Extension {
    /// The names of the intents that an extension supports.
    #[serde(
        rename(serialize = "IntentsSupported"),
        skip_serializing_if = "Option::is_none"
    )]
    pub intents_supported: Option<Vec<String>>,
    /// A dictionary that specifies the minimum size of the floating window in which Final Cut Pro hosts the extension view.
    #[serde(
        rename(serialize = "ProExtensionAttributes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub pro_extension_attributes: Option<BTreeMap<String, String>>,
    /// The name of the class with the principal implementation of your extension.
    ///
    /// The Compressor app instantiates the class specified in the ProExtensionPrincipalClass key to convert source files
    /// to the output format your extension supports.
    #[serde(
        rename(serialize = "ProExtensionPrincipalClass"),
        skip_serializing_if = "Option::is_none"
    )]
    pub pro_extension_principal_class: Option<String>,
    /// The name of the principal view controller class of your extension.
    ///
    /// This key provides the name of the primary view controller class of your extension that adopts the NSViewController
    /// protocol. When you create an extension, the Xcode template automatically includes this key in the workflow extension
    /// information property list. You only modify the value of this key when you rename the primary view controller class in your extension.
    #[serde(
        rename(serialize = "ProExtensionPrincipalViewControllerClass"),
        skip_serializing_if = "Option::is_none"
    )]
    pub pro_extension_principal_view_controller_class: Option<String>,
    /// A UUID string that uniquely identifies your extension to the Compressor app.
    ///
    /// The value for this key is a placeholder UUID the Xcode template generates. Each extension must have a unique UUID.
    /// When you build an extension for the first time, the build script in the Xcode template replaces the placeholder UUID
    /// with a new UUID. The new UUID fulfills the uniqueness and persistence requirement for ProExtensionUUID. For subsequent
    /// rebuilds, the UUID stays the same because the Compressor app uses this UUID to differentiate between previously
    /// saved and newly discovered extensions.
    #[serde(
        rename(serialize = "ProExtensionUUID"),
        skip_serializing_if = "Option::is_none"
    )]
    pub pro_extension_uuid: Option<String>,
    /// Account Authentication Modification. The rules the system satisfies when generating a strong password for your
    /// extension during an automatic upgrade.
    #[serde(
        rename(serialize = "ASAccountAuthenticationModificationPasswordGenerationRequirements"),
        skip_serializing_if = "Option::is_none"
    )]
    pub password_generation_requirements: Option<String>,
    /// Account Authentication Modification. A Boolean value that indicates whether the extension supports upgrading a user’s
    /// password to a strong password.
    #[serde(
        rename(serialize = "ASAccountAuthenticationModificationSupportsStrongPasswordUpgrade"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_strong_password_upgrade: Option<bool>,
    /// Account Authentication Modification. A Boolean value that indicates whether the extension supports upgrading from using
    /// password authentication to using Sign in with Apple.
    #[serde(
        rename(serialize = "ASAccountAuthenticationModificationSupportsUpgradeToSignInWithApple"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_upgrade_to_sign_in_with_apple: Option<bool>,
    /// A Boolean value indicating whether the Action extension is presented in full screen.
    #[serde(
        rename(serialize = "NSExtensionActionWantsFullScreenPresentation"),
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_action_wants_full_screen_presentation: Option<bool>,
    /// Properties of an app extension.
    #[serde(
        rename(serialize = "NSExtensionAttributes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_attributes: Option<ExtensionAttributes>,
    /// The name of the app extension’s main storyboard file.
    ///
    /// This key is mutually exclusive with NSExtensionPrincipalClass. Typically, Xcode sets the value of this key when creating
    /// an App Extension target in your project. If you change the name of your storyboard file, remember to update the value of this key.
    #[serde(
        rename(serialize = "NSExtensionMainStoryboard"),
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_main_storyboard: Option<String>,
    /// A Boolean value indicating whether the app extension ignores appearance changes made by the host app.
    #[serde(
        rename(serialize = "NSExtensionOverridesHostUIAppearance"),
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_overrides_host_ui_appearance: Option<bool>,
    /// The extension point that supports an app extension.
    #[serde(
        rename(serialize = "NSExtensionPointIdentifier"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub extension_point_identifier: Option<ExtensionPointIdentifier>,
    /// The custom class that implements an app extension’s primary view or functionality.
    ///
    /// This key is mutually exclusive with NSExtensionMainStoryboard. Typically, Xcode sets the value of this key when creating an App
    /// Extension target in your project. If you change the name of the specified class, remember to update the value of this key.
    #[serde(
        rename(serialize = "NSExtensionPrincipalClass"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub extension_principal_class: Option<String>,
    /// The content scripts for a Safari extension.
    #[serde(
        rename(serialize = "SFSafariContentScript"),
        skip_serializing_if = "Option::is_none"
    )]
    pub safari_content_script: Option<Vec<SafariContentScript>>,
    /// The context menu items for a Safari extension.
    #[serde(
        rename(serialize = "SFSafariContextMenu"),
        skip_serializing_if = "Option::is_none"
    )]
    pub safari_context_menu: Option<Vec<SafariContextMenu>>,
    /// The style sheet for a Safari extension.
    #[serde(
        rename(serialize = "SFSafariStyleSheet"),
        skip_serializing_if = "Option::is_none"
    )]
    pub safari_style_sheet: Option<Vec<SafariStyleSheet>>,
    /// The items to add to the toolbar for a Safari extension.
    #[serde(
        rename(serialize = "SFSafariToolbarItem"),
        skip_serializing_if = "Option::is_none"
    )]
    pub safari_toolbar_item: Option<SafariToolbarItem>,
    /// The webpages a Safari extension can access.
    #[serde(
        rename(serialize = "SFSafariWebsiteAccess"),
        skip_serializing_if = "Option::is_none"
    )]
    pub safari_website_access: Option<SafariWebsiteAccess>,
}

/// Safari Website Access.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct SafariWebsiteAccess {
    /// The domains that a Safari extension is allowed access to.
    #[serde(
        rename(serialize = "Allowed Domains"),
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_domains: Option<Vec<String>>,
    /// The level of a Safari extension’s website access.
    #[serde(rename(serialize = "Level"), skip_serializing_if = "Option::is_none")]
    pub level: Option<SafariWebsiteAccessLevel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum SafariWebsiteAccessLevel {
    #[serde(rename(serialize = "None"))]
    None,
    #[serde(rename(serialize = "All"))]
    All,
    #[serde(rename(serialize = "Some"))]
    Some,
}

/// Safari Toolbar Item.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct SafariToolbarItem {
    /// The properties of an app extension's toolbar item that's been added to the Safari window.
    #[serde(rename(serialize = "Action"), skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// The identifier for a Safari extension's toolbar item.
    #[serde(
        rename(serialize = "Identifier"),
        skip_serializing_if = "Option::is_none"
    )]
    pub identifier: Option<String>,
    /// An image that represents a Safari extension's toolbar item.
    #[serde(rename(serialize = "Image"), skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// The label for the Safari extension's toolbar item.
    #[serde(rename(serialize = "Label"), skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Safari Style Sheet.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct SafariStyleSheet {
    /// The webpages that the script can be injected into.
    #[serde(
        rename(serialize = "Allowed URL Patterns"),
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_url_patterns: Option<Vec<String>>,
    /// The webpages that the script can't be injected into.
    #[serde(
        rename(serialize = "Excluded URL Patterns"),
        skip_serializing_if = "Option::is_none"
    )]
    pub excluded_url_patterns: Option<Vec<String>>,
    /// The path to the style sheet, relative to the Resources folder in the app extension's bundle.
    #[serde(
        rename(serialize = "Style Sheet"),
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet: Option<String>,
}

/// The context menu items for a Safari extension.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct SafariContextMenu {
    /// The command to send to the app extension when the user selects the context menu item.
    #[serde(rename(serialize = "Command"), skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// The text to display for the context menu item.
    #[serde(rename(serialize = "Text"), skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Safari Content Script.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct SafariContentScript {
    /// The webpages that the script can be injected into.
    #[serde(
        rename(serialize = "Allowed URL Patterns"),
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_url_patterns: Option<Vec<String>>,
    /// The webpages that the script can't be injected into.
    #[serde(
        rename(serialize = "Excluded URL Patterns"),
        skip_serializing_if = "Option::is_none"
    )]
    pub excluded_url_patterns: Option<Vec<String>>,
    /// The path to the content script, relative to the Resources folder in the app extension's bundle.
    #[serde(rename(serialize = "Script"), skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
}

/// Extension Point Identifier.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum ExtensionPointIdentifier {
    #[serde(rename(serialize = "com.apple.ui-services"))]
    UiServices,
    #[serde(rename(serialize = "com.apple.services"))]
    Services,
    #[serde(rename(serialize = "com.apple.keyboard-service"))]
    KeyboardService,
    #[serde(rename(serialize = "com.apple.fileprovider-nonui"))]
    FileproviderNonui,
    #[serde(rename(serialize = "com.apple.fileprovider-actionsui"))]
    FileproviderActionsui,
    #[serde(rename(serialize = "com.apple.FinderSync"))]
    FinderSync,
    #[serde(rename(serialize = "com.apple.identitylookup.message-filter"))]
    IdentityLookupMessageFilter,
    #[serde(rename(serialize = "com.apple.photo-editing"))]
    PhotoEditing,
    #[serde(rename(serialize = "com.apple.share-services"))]
    ShareServices,
    #[serde(rename(serialize = "com.apple.callkit.call-directory"))]
    CallkitCallDirectory,
    #[serde(rename(
        serialize = "com.apple.authentication-services-account-authentication-modification-ui"
    ))]
    AuthenticationServicesAccountAuthenticationModificationUi,
    #[serde(rename(serialize = "com.apple.AudioUnit-UI"))]
    AudioUnitUi,
    #[serde(rename(serialize = "com.apple.AppSSO.idp-extension"))]
    AppSsoIdpExtension,
    #[serde(rename(serialize = "com.apple.authentication-services-credential-provider-ui"))]
    AuthenticationServicesCredentialProviderUi,
    #[serde(rename(serialize = "com.apple.broadcast-services-setupui"))]
    BroadcastServicesSetupui,
    #[serde(rename(serialize = "com.apple.broadcast-services-upload"))]
    BroadcastServicesUpload,
    #[serde(rename(serialize = "com.apple.classkit.context-provider"))]
    ClasskitContextProvider,
    #[serde(rename(serialize = "com.apple.Safari.content-blocker"))]
    SafariContentBlocker,
    #[serde(rename(serialize = "com.apple.message-payload-provider"))]
    MessagePayloadProvider,
    #[serde(rename(serialize = "com.apple.intents-service"))]
    IntentsService,
    #[serde(rename(serialize = "com.apple.intents-ui-service"))]
    IntentsUiService,
    #[serde(rename(serialize = "com.apple.networkextension.app-proxy"))]
    NetworkExtensionAppProxy,
    #[serde(rename(serialize = "com.apple.usernotifications.content-extension"))]
    UsernotificationsContentExtension,
    #[serde(rename(serialize = "com.apple.usernotifications.service"))]
    UsernotificationsService,
    #[serde(rename(serialize = "com.apple.ctk-tokens"))]
    CtkTokens,
    #[serde(rename(serialize = "com.apple.photo-project"))]
    PhotoProject,
    #[serde(rename(serialize = "com.apple.quicklook.preview"))]
    QuicklookPreview,
    #[serde(rename(serialize = "com.apple.Safari.extension"))]
    SafariExtension,
    #[serde(rename(serialize = "com.apple.spotlight.index"))]
    SpotlightIndex,
    #[serde(rename(serialize = "com.apple.quicklook.thumbnail"))]
    QuicklookThumbnail,
    #[serde(rename(serialize = "com.apple.tv-top-shelf"))]
    TvTopShelf,
    #[serde(rename(serialize = "com.apple.identitylookup.classification-ui"))]
    ClassificationUi,
    #[serde(rename(serialize = "com.apple.widgetkit-extension"))]
    WidgetkitExtension,
    #[serde(rename(serialize = "com.apple.dt.Xcode.extension.source-editor"))]
    ExtensionSourceEditor,
}

/// Extension Attributes.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ExtensionAttributes {
    /// A Boolean value indicating whether the extension appears in the Finder Preview pane and Quick Actions menu.
    #[serde(
        rename(serialize = "NSExtensionServiceAllowsFinderPreviewItem"),
        skip_serializing_if = "Option::is_none"
    )]
    pub allows_finder_preview_item: Option<bool>,
    /// A Boolean value indicating whether an Action extension displays an item in a window’s toolbar.
    #[serde(
        rename(serialize = "NSExtensionServiceAllowsToolbarItem"),
        skip_serializing_if = "Option::is_none"
    )]
    pub allows_toolbar_item: Option<bool>,
    /// A Boolean value indicating whether the extension appears as a Quick Action in the Touch Bar.
    #[serde(
        rename(serialize = "NSExtensionServiceAllowsTouchBarItem"),
        skip_serializing_if = "Option::is_none"
    )]
    pub allows_touch_bar_item: Option<bool>,
    /// The name of an icon for display when the extension appears in the Finder Preview pane and Quick Actions menu.
    ///
    /// This key is used in conjunction with the NSExtensionServiceAllowsFinderPreviewItem key.
    ///
    /// Set the NSExtensionServiceFinderPreviewIconName key's value to a system icon name or the name of an icon in the
    /// extension bundle. This icon should be a template image: a monochromatic image with transparency, anti-aliasing,
    /// and no drop shadow that uses a mask to define its shape. For design guidance, see Human Interface Guidelines > macOS > Custom Icons.
    /// If no icon is specified, a default icon is used.
    #[serde(
        rename(serialize = "NSExtensionServiceFinderPreviewIconName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub finder_preview_icon_name: Option<String>,
    /// A name for display when the extension appears in the Finder Preview pane and Quick Actions menu.
    ///
    /// This key is used in conjunction with the NSExtensionServiceAllowsFinderPreviewItem key.
    ///
    /// If the NSExtensionServiceFinderPreviewLabel key isn't provided, the extension's display name is used.
    #[serde(
        rename(serialize = "NSExtensionServiceFinderPreviewLabel"),
        skip_serializing_if = "Option::is_none"
    )]
    pub finder_preview_label: Option<String>,
    /// The type of task an Action extension performs.
    #[serde(
        rename(serialize = "NSExtensionServiceRoleType"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub role_type: Option<ExtensionServiceRoleType>,
    /// The image for an Action extension’s toolbar item.
    #[serde(
        rename(serialize = "NSExtensionServiceToolbarIconFile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub toolbar_icon_file: Option<String>,
    /// The label for an Action extension's toolbar item.
    #[serde(
        rename(serialize = "NSExtensionServiceToolbarPaletteLabel"),
        skip_serializing_if = "Option::is_none"
    )]
    pub toolbar_palette_label: Option<String>,
    /// The color to use for the bezel around the extension when it appears as a Quick Action in the Touch Bar.
    ///
    /// This key is used in conjunction with the NSExtensionServiceAllowsTouchBarItem key.
    ///
    /// Set the NSExtensionServiceTouchBarBezelColorName key's value to the name of a color that exists in your extension's asset
    /// catalog—a color that matches a system color is recommended. If no color is specified, a default color is used.
    #[serde(
        rename(serialize = "NSExtensionServiceTouchBarBezelColorName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub touch_bar_bezel_color_name: Option<String>,
    /// The name of an icon for display when the extension appears as a Quick Action in the Touch Bar.
    ///
    /// This key is used in conjunction with the NSExtensionServiceAllowsTouchBarItem key.
    ///
    /// Set the NSExtensionServiceTouchBarIconName key's value to a system icon name or the name of an icon within the extension bundle.
    /// This icon should be a template image: a monochromatic image with transparency, anti-aliasing, and no drop shadow that uses
    /// a mask to define its shape. For design guidance, see Human Interface Guidelines > macOS > Custom Icons. If no icon is specified,
    /// a default icon is used.
    #[serde(
        rename(serialize = "NSExtensionServiceTouchBarIconName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub touch_bar_icon_name: Option<String>,
    /// A name for display when the extension appears as a Quick Action in the Touch Bar.
    ///
    /// This key is used in conjunction with the NSExtensionServiceAllowsTouchBarItem key.
    ///
    /// If the NSExtensionServiceTouchBarLabel key isn't provided, the extension's display name is used.
    #[serde(
        rename(serialize = "NSExtensionServiceTouchBarLabel"),
        skip_serializing_if = "Option::is_none"
    )]
    pub touch_bar_label: Option<String>,
    /// A Boolean value indicating whether the Action extension is presented in full screen.
    #[serde(
        rename(serialize = "NSExtensionActionWantsFullScreenPresentation"),
        skip_serializing_if = "Option::is_none"
    )]
    pub action_wants_full_screen_presentation: Option<bool>,
    /// This key is mutually exclusive with NSExtensionPrincipalClass. If the app extension’s Info.plist file contains both keys,
    /// the system won’t load the extension.
    #[serde(
        rename(serialize = "NSExtensionMainStoryboard"),
        skip_serializing_if = "Option::is_none"
    )]
    pub main_storyboard: Option<bool>,
    /// A Boolean value indicating whether the app extension ignores appearance changes made by the host app.
    #[serde(
        rename(serialize = "NSExtensionOverridesHostUIAppearance"),
        skip_serializing_if = "Option::is_none"
    )]
    pub overrides_host_ui_appearance: Option<bool>,
    /// The extension point that supports an app extension.
    #[serde(
        rename(serialize = "NSExtensionPointIdentifier"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub point_identifier: Option<ExtensionPointIdentifier>,
    /// This key is mutually exclusive with NSExtensionMainStoryboard. If the app extension’s Info.plist file contains both keys,
    /// the system won’t load the extension.
    #[serde(
        rename(serialize = "NSExtensionPrincipalClass"),
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_class: Option<String>,
    /// The semantic data types that a Share or Action extension supports.
    #[serde(
        rename(serialize = "NSExtensionActivationRule"),
        skip_serializing_if = "Option::is_none"
    )]
    pub activation_rule: Option<ActivationRule>,
    /// The name of a JavaScript file supplied by a Share or Action extension.
    #[serde(
        rename(serialize = "NSExtensionJavaScriptPreprocessingFile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub java_script_preprocessing_file: Option<String>,
    /// The names of the intents that an extension supports.
    #[serde(
        rename(serialize = "IntentsSupported"),
        skip_serializing_if = "Option::is_none"
    )]
    pub intents_supported: Option<Vec<String>>,
    /// Types of media supported by an app extension’s media-playing intents.
    ///
    /// Specify one or more media categories to allow Siri to invoke your app’s intent handling when a user asks to play media.
    /// Use INMediaCategoryGeneral for media that doesn’t fit into any of the other categories, like white noise or sound effects.
    ///
    /// To specify this information in Xcode, add INPlayMediaIntent to your extension’s list of Supported Intents. Then select the
    /// relevant media types in the list that appears.
    #[serde(
        rename(serialize = "SupportedMediaCategories"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_media_categories: Option<Vec<MediaCategories>>,
    #[serde(
        rename(serialize = "PHProjectExtensionDefinesProjectTypes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub project_extension_defines_project_types: Option<bool>,
    /// The types of assets a Photo Editing extension can edit.
    #[serde(
        rename(serialize = "PHSupportedMediaTypes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_media_types: Option<Vec<MediaTypes>>,
    /// The server that a Message Filter app extension may defer a query to.
    #[serde(
        rename(serialize = "IDMessageFilterExtensionNetworkURL"),
        skip_serializing_if = "Option::is_none"
    )]
    pub id_message_filter_extension_network_url: Option<String>,
    /// The phone number that receives SMS messages when the user reports an SMS message or a call.
    #[serde(
        rename(serialize = "ILClassificationExtensionSMSReportDestination"),
        skip_serializing_if = "Option::is_none"
    )]
    pub classification_extension_sms_report_destination: Option<String>,
    /// A Boolean value indicating whether a custom keyboard displays standard ASCII characters.
    #[serde(
        rename(serialize = "IsASCIICapable"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_ascii_capable: Option<String>,
    /// The contexts that an iMessage app or sticker pack supports.
    #[serde(
        rename(serialize = "MSMessagesAppPresentationContextMessages"),
        skip_serializing_if = "Option::is_none"
    )]
    pub messages_app_presentation_context_messages: Option<Vec<ContextMessages>>,
    /// The custom actions for a File Provider extension.
    #[serde(
        rename(serialize = "NSExtensionFileProviderActions"),
        skip_serializing_if = "Option::is_none"
    )]
    pub file_provider_actions: Option<Vec<FileProviderAction>>,
    /// The identifier of a shared container that can be accessed by a Document Picker extension and its associated File Provider extension.
    #[serde(
        rename(serialize = "NSExtensionFileProviderDocumentGroup"),
        skip_serializing_if = "Option::is_none"
    )]
    pub file_provider_document_group: Option<String>,
    /// A Boolean value indicating whether a File Provider extension enumerates its content.
    #[serde(
        rename(serialize = "NSExtensionFileProviderSupportsEnumeration"),
        skip_serializing_if = "Option::is_none"
    )]
    pub file_provider_supports_enumeration: Option<bool>,
    /// A Boolean value indicating whether a keyboard extension supports right-to-left languages.
    #[serde(
        rename(serialize = "PrefersRightToLeft"),
        skip_serializing_if = "Option::is_none"
    )]
    pub prefers_right_to_left: Option<bool>,
    /// The primary language for a keyboard extension.
    #[serde(
        rename(serialize = "PrimaryLanguage"),
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_language: Option<String>,
    /// A Boolean value indicating whether a custom keyboard uses a shared container and accesses the network.
    #[serde(
        rename(serialize = "RequestsOpenAccess"),
        skip_serializing_if = "Option::is_none"
    )]
    pub requests_open_access: Option<bool>,
    /// The modes that a Document Picker extension supports.
    #[serde(
        rename(serialize = "UIDocumentPickerModes"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_vec_enum_option"
    )]
    pub document_picker_modes: Option<Vec<DocumentPickerModes>>,
    /// The Uniform Type Identifiers that a document picker extension supports.
    #[serde(
        rename(serialize = "UIDocumentPickerSupportedFileTypes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub document_picker_supported_file_types: Option<Vec<String>>,
    /// The identifier of a category declared by the app extension.
    #[serde(
        rename(serialize = "UNNotificationExtensionCategory"),
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_extension_category: Option<String>,
    /// A Boolean value indicating whether only the app extension's custom view controller is displayed in the notification interface.
    #[serde(
        rename(serialize = "UNNotificationExtensionDefaultContentHidden"),
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_extension_default_content_hidden: Option<bool>,
    /// The initial size of the view controller's view for an app extension, expressed as a ratio of its height to its width.
    #[serde(
        rename(serialize = "UNNotificationExtensionInitialContentSizeRatio"),
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_extension_initial_content_size_ratio: Option<f32>,
    /// A Boolean value indicating whether the title of the app extension's view controller is used as the title of the notification.
    #[serde(
        rename(serialize = "UNNotificationExtensionOverridesDefaultTitle"),
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_extension_overrides_default_title: Option<bool>,
    /// A Boolean value indicating whether user interactions in a custom notification are enabled.
    #[serde(
        rename(serialize = "UNNotificationExtensionUserInteractionEnabled"),
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_extension_user_interaction_enabled: Option<bool>,
}

/// Document Picker Modes.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum DocumentPickerModes {
    #[serde(rename(serialize = "UIDocumentPickerModeImport"))]
    Import,
    #[serde(rename(serialize = "UIDocumentPickerModeOpen"))]
    Open,
    #[serde(rename(serialize = "UIDocumentPickerModeExportToService"))]
    ExportToService,
    #[serde(rename(serialize = "UIDocumentPickerModeMoveToService"))]
    MoveToService,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct FileProviderAction {
    /// A predicate that determines whether a File Provider extension action appears in the context menu.
    #[serde(
        rename(serialize = "NSExtensionFileProviderActionActivationRule"),
        skip_serializing_if = "Option::is_none"
    )]
    pub activation_rule: Option<String>,
    /// A unique identifier for a File Provider extension action.
    #[serde(
        rename(serialize = "NSExtensionFileProviderActionIdentifier"),
        skip_serializing_if = "Option::is_none"
    )]
    pub identifier: Option<String>,
    /// The localized name for a File Provider extension action that appears in the context menu.
    #[serde(
        rename(serialize = "NSExtensionFileProviderActionName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
}

/// Context Messages.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum ContextMessages {
    #[serde(rename(serialize = "MSMessagesAppPresentationContextMessages"))]
    Messages,
    #[serde(rename(serialize = "MSMessagesAppPresentationContextMedia"))]
    Media,
}

/// Media Types.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum MediaTypes {
    #[serde(rename(serialize = "Image"))]
    Image,
    #[serde(rename(serialize = "Video"))]
    Video,
}

/// Media Categories.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum MediaCategories {
    #[serde(rename(serialize = "INMediaCategoryAudiobooks"))]
    Audiobooks,
    #[serde(rename(serialize = "INMediaCategoryMusic"))]
    Music,
    #[serde(rename(serialize = "INMediaCategoryGeneral"))]
    General,
    #[serde(rename(serialize = "INMediaCategoryPodcasts"))]
    Podcasts,
    #[serde(rename(serialize = "INMediaCategoryRadio"))]
    Radio,
}

/// Activation Rule.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ActivationRule {
    /// The version of the parent extension-activation rule dictionary.
    #[serde(
        rename(serialize = "NSExtensionActivationDictionaryVersion"),
        skip_serializing_if = "Option::is_none"
    )]
    pub dictionary_version: Option<i32>,
    /// The maximum number of attachments that the app extension supports.
    #[serde(
        rename(serialize = "NSExtensionActivationSupportsAttachmentsWithMaxCount"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_attachments_with_max_count: Option<i32>,
    /// The minimum number of attachments that the app extension supports.
    #[serde(
        rename(serialize = "NSExtensionActivationSupportsAttachmentsWithMinCount"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_attachments_with_min_count: Option<i32>,
    /// The maximum number of all types of files that the app extension supports.
    #[serde(
        rename(serialize = "NSExtensionActivationSupportsFileWithMaxCount"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_file_with_max_count: Option<i32>,
    /// The maximum number of image files that the app extension supports.
    #[serde(
        rename(serialize = "NSExtensionActivationSupportsImageWithMaxCount"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_image_with_max_count: Option<i32>,
    /// The maximum number of movie files that the app extension supports.
    #[serde(
        rename(serialize = "NSExtensionActivationSupportsMovieWithMaxCount"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_movie_with_max_count: Option<i32>,
    /// A Boolean value indicating whether the app extension supports text.
    #[serde(
        rename(serialize = "NSExtensionActivationSupportsText"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_text: Option<i32>,
    /// The maximum number of webpages that the app extension supports.
    #[serde(
        rename(serialize = "NSExtensionActivationSupportsWebPageWithMaxCount"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_web_page_with_max_count: Option<i32>,
    /// The maximum number of HTTP URLs that the app extension supports.
    #[serde(
        rename(serialize = "NSExtensionActivationSupportsWebURLWithMaxCount"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_web_url_with_max_count: Option<i32>,
    /// A Boolean value indicating whether strict or fuzzy matching is used when determining the asset types an app extension handles.
    #[serde(
        rename(serialize = "NSExtensionActivationUsesStrictMatching"),
        skip_serializing_if = "Option::is_none"
    )]
    pub uses_strict_matching: Option<bool>,
}

/// Extension Service Role Type.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum ExtensionServiceRoleType {
    #[serde(rename(serialize = "Editor"))]
    Editor,
    #[serde(rename(serialize = "Viewer"))]
    Viewer,
}

/// Service.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Service {
    /// A keyboard shortcut that invokes the service menu command.
    #[serde(
        rename(serialize = "NSKeyEquivalent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub key_equivalent: Option<DefaultDictionary>,
    /// Text for a Services menu item.
    #[serde(rename(serialize = "NSMenuItem"))]
    pub menu_item: DefaultDictionary,
    /// An instance method that invokes the service.
    #[serde(rename(serialize = "NSMessage"))]
    pub message: String,
    /// The port that the service monitors for incoming requests.
    #[serde(
        rename(serialize = "NSPortName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub port_name: Option<String>,
    /// The data types that the service returns.
    #[serde(
        rename(serialize = "NSReturnTypes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub return_types: Option<Vec<String>>,
    /// The data types that the service can read.
    #[serde(
        rename(serialize = "NSSendTypes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub send_types: Option<Vec<String>>,
    /// The amount of time, in milliseconds, that the system waits for a response from the service.
    #[serde(
        rename(serialize = "NSTimeout"),
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout: Option<String>,
    /// A service-specific string value.
    #[serde(
        rename(serialize = "NSUserData"),
        skip_serializing_if = "Option::is_none"
    )]
    pub user_data: Option<BTreeMap<String, String>>,
}

/// Default Dictionary.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct DefaultDictionary {
    pub default: String,
}

/// Complication Supported Families.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum ComplicationSupportedFamilies {
    #[serde(rename(serialize = "CLKComplicationFamilyModularSmall"))]
    ModularSmall,
    #[serde(rename(serialize = "CLKComplicationFamilyModularLarge"))]
    ModularLarge,
    #[serde(rename(serialize = "CLKComplicationFamilyUtilitarianSmall"))]
    UtilitarianSmall,
    #[serde(rename(serialize = "CLKComplicationFamilyUtilitarianSmallFlat"))]
    UtilitarianSmallFlat,
    #[serde(rename(serialize = "CLKComplicationFamilyUtilitarianLarge"))]
    UtilitarianLarge,
    #[serde(rename(serialize = "CLKComplicationFamilyCircularSmall"))]
    CircularSmall,
    #[serde(rename(serialize = "CLKComplicationFamilyExtraLarge"))]
    ExtraLarge,
    #[serde(rename(serialize = "CLKComplicationFamilyGraphicCorner"))]
    GraphicCorner,
    #[serde(rename(serialize = "CLKComplicationFamilyGraphicBezel"))]
    GraphicBezel,
    #[serde(rename(serialize = "CLKComplicationFamilyGraphicCircular"))]
    GraphicCircular,
    #[serde(rename(serialize = "CLKComplicationFamilyGraphicRectangular"))]
    GraphicRectangular,
}

/// Architecture Priority.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum ArchitecturePriority {
    /// The 32-bit Intel architecture.
    #[serde(rename(serialize = "i386"))]
    I386,
    /// The 64-bit Intel architecture.
    #[serde(rename(serialize = "x86_64"))]
    X86_64,
    /// The 64-bit ARM architecture.
    #[serde(rename(serialize = "arm64"))]
    Arm64,
    /// The 64-bit ARM architecture with pointer authentication code support.
    #[serde(rename(serialize = "arm64e"))]
    Arm64e,
}

/// Device Capabilities.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum DeviceCapabilities {
    /// The presence of accelerometers. Use the Core Motion framework to receive accelerometer events. You don’t need to
    /// include this value if your app detects only device orientation changes. Available in iOS 3.0 and later.
    #[serde(rename(serialize = "accelerometer"))]
    Accelerometer,
    /// Support for ARKit. Available in iOS 11.0 and later.
    #[serde(rename(serialize = "arkit"))]
    Arkit,
    /// Compilation for the armv7 instruction set, or as a 32/64-bit universal app. Available in iOS 3.1 and later.
    #[serde(rename(serialize = "armv7"))]
    Armv7,
    /// Compilation for the arm64 instruction set. Include this key for all 64-bit apps and embedded bundles, like
    /// extensions and frameworks. Available in iOS 8.0 and later.
    #[serde(rename(serialize = "arm64"))]
    Arm64,
    /// Autofocus capabilities in the device’s still camera. You might need to include this value if your app supports
    /// macro photography or requires sharper images to perform certain image-processing tasks. Available in iOS 3.0 and later.
    #[serde(rename(serialize = "auto-focus-camera"))]
    AutoFocusCamera,
    /// Bluetooth low-energy hardware. Available in iOS 5.0 and later.
    #[serde(rename(serialize = "bluetooth-le"))]
    BluetoothLe,
    /// A camera flash. Use the cameraFlashMode property of a UIImagePickerController instance to control the camera’s
    /// flash. Available in iOS 3.0 and later.
    #[serde(rename(serialize = "camera-flash"))]
    CameraFlash,
    /// A forward-facing camera. Use the cameraDevice property of a UIImagePickerController instance to select the
    /// device’s camera. Available in iOS 3.0 and later.
    #[serde(rename(serialize = "front-facing-camera"))]
    FrontFacingCamera,
    /// Access to the Game Center service. Enable the Game Center capability in Xcode to add this value to your app.
    /// Available in iOS 4.1 and later.
    #[serde(rename(serialize = "gamekit"))]
    Gamekit,
    /// GPS (or AGPS) hardware for tracking locations. If you include this value, you should also include the
    /// location-services value. Require GPS only if your app needs location data more accurate than the cellular or Wi-Fi
    /// radios provide. Available in iOS 3.0 and later.
    #[serde(rename(serialize = "gps"))]
    Gps,
    /// A gyroscope. Use the Core Motion framework to retrieve information from gyroscope hardware. Available in iOS 3.0 and later.
    #[serde(rename(serialize = "gyroscope"))]
    Gyroscope,
    /// Support for HealthKit. Available in iOS 8.0 and later.
    #[serde(rename(serialize = "healthkit"))]
    Healthkit,
    /// Performance and capabilities of the A12 Bionic and later chips. Available in iOS 12.0 and later.
    #[serde(rename(serialize = "iphone-ipad-minimum-performance-a12"))]
    IphoneIpadMinimumPerformanceA12,
    /// Access to the device’s current location using the Core Location framework. This value refers to the general location
    /// services feature. If you specifically need GPS-level accuracy, also include the gps feature. Available in iOS 3.0 and later.
    #[serde(rename(serialize = "location-services"))]
    LocationServices,
    /// Magnetometer hardware. Apps use this hardware to receive heading-related events through the Core Location framework.
    /// Available in iOS 3.0 and later.
    #[serde(rename(serialize = "magnetometer"))]
    Magnetometer,
    // Support for graphics processing with Metal. Available in iOS 8.0 and later.
    #[serde(rename(serialize = "metal"))]
    Metal,
    /// The built-in microphone or accessories that provide a microphone. Available in iOS 3.0 and later.
    #[serde(rename(serialize = "microphone"))]
    Microphone,
    /// Near Field Communication (NFC) tag detection and access to messages that contain NFC Data Exchange Format data.
    /// Use the Core NFC framework to detect and read NFC tags. Available in iOS 11.0 and later.
    #[serde(rename(serialize = "nfc"))]
    Nfc,
    /// The OpenGL ES 1.1 interface. Available in iOS 3.0 and later.
    #[serde(rename(serialize = "opengles-1"))]
    Opengles1,
    /// The OpenGL ES 2.0 interface. Available in iOS 3.0 and later.
    #[serde(rename(serialize = "opengles-2"))]
    Opengles2,
    /// The OpenGL ES 3.0 interface. Available in iOS 7.0 and later.
    #[serde(rename(serialize = "opengles-2"))]
    Opengles3,
    /// Peer-to-peer connectivity over a Bluetooth network. Available in iOS 3.1 and later.
    #[serde(rename(serialize = "peer-peer"))]
    PeerPeer,
    /// The Messages app. You might require this feature if your app opens URLs with the sms scheme. Available in iOS 3.0 and later.
    #[serde(rename(serialize = "sms"))]
    Sms,
    /// A camera on the device. Use the UIImagePickerController interface to capture images from the device’s still camera.
    /// Available in iOS 3.0 and later.
    #[serde(rename(serialize = "still-camera"))]
    StillCamera,
    /// The Phone app. You might require this feature if your app opens URLs with the tel scheme. Available in iOS 3.0 and later.
    #[serde(rename(serialize = "telephony"))]
    Telephony,
    /// A camera with video capabilities on the device. Use the UIImagePickerController interface to capture video from the
    /// device’s camera. Available in iOS 3.0 and later.
    #[serde(rename(serialize = "video-camera"))]
    VideoCamera,
    /// Networking features related to Wi-Fi access. Available in iOS 3.0 and later.
    #[serde(rename(serialize = "wifi"))]
    Wifi,
}

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
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Launch {
    /// The name of the bundle’s main executable class.
    ///
    /// The system uses the class identified by this key to set the principalClass property of a bundle when it’s loaded.
    ///
    /// Xcode sets the default value of this key to NSApplication for macOS apps, and to UIApplication for iOS and tvOS apps.
    /// For other types of bundles, you must set this key in The Info.plist File.
    #[serde(rename = "NSPrincipalClass", skip_serializing_if = "Option::is_none")]
    pub principal_class: Option<String>,
    /// The name of the class that implements the complication data source protocol.
    ///
    /// Xcode automatically includes this key in the information property list when you modify the WatchKit extension’s
    /// data source (General > Complication Configuration > Data Source class).
    #[serde(
        rename = "CLKComplicationPrincipalClass",
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
    #[serde(rename = "CFBundleExecutable", skip_serializing_if = "Option::is_none")]
    pub bundle_executable: Option<String>,
    /// Environment variables to set before launching the app.
    #[serde(rename = "LSEnvironment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<String>>,
    /// Application shortcut items.
    #[serde(
        rename = "UIApplicationShortcutItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_shortcut_items: Option<Vec<ApplicationShortcutItem>>,
}

/// Application Shortcut Item.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ApplicationShortcutItem {
    #[serde(
        rename = "UIApplicationShortcutItemIconFile",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_file: Option<String>,
    #[serde(
        rename = "UIApplicationShortcutItemIconSymbolName",
        skip_serializing_if = "Option::is_none"
    )]
    pub symbol_name: Option<String>,
    #[serde(
        rename = "UIApplicationShortcutItemIconType",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_type: Option<String>,
    #[serde(
        rename = "UIApplicationShortcutItemSubtitle",
        skip_serializing_if = "Option::is_none"
    )]
    pub subtitle: Option<String>,
    #[serde(
        rename = "UIApplicationShortcutItemTitle",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: String,
    #[serde(
        rename = "UIApplicationShortcutItemType",
        skip_serializing_if = "Option::is_none"
    )]
    pub item_type: String,
    #[serde(
        rename = "UIApplicationShortcutItemUserInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_info: Option<BTreeMap<String, String>>,
}

/// Launch Conditions.
#[derive(Deserialize, Serialize, Debug, Default)]
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
        rename = "UIRequiredDeviceCapabilities",
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
        rename = "LSArchitecturePriority",
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
        rename = "LSRequiresNativeExecution",
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_native_execution: Option<bool>,
    /// A Boolean value indicating whether the user can install and run the watchOS app independently of its iOS companion app.
    ///
    /// Xcode automatically includes this key in the WatchKit extension’s information property list and sets its value to true
    /// when you create a project using the iOS App with Watch App template. When you set the value of this key to true, the app
    /// doesn’t need its iOS companion app to operate properly. Users can choose to install the iOS app, the watchOS app, or both.
    #[serde(
        rename = "WKRunsIndependentlyOfCompanionApp",
        skip_serializing_if = "Option::is_none"
    )]
    pub runs_independently_of_companion_app: Option<bool>,
    /// A Boolean value indicating whether the app is a watch-only app.
    ///
    /// Xcode automatically includes this key in the WatchKit extension’s information property list and sets its value to true
    /// when you create a project using the Watch App template. When you set the value of this key to true, the app is only available
    /// on Apple Watch, with no related iOS app.
    #[serde(rename = "WKWatchOnly", skip_serializing_if = "Option::is_none")]
    pub watch_only: Option<bool>,
    /// A Boolean value that indicates whether a watchOS app should opt out of automatically launching when its companion iOS
    /// app starts playing audio content.
    ///
    /// If your watchOS app does not act as a remote control for the iOS app, set this key to true in your WatchKit extension’s
    /// information property list.
    #[serde(
        rename = "PUICAutoLaunchAudioOptOut",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_launch_audio_opt_out: Option<bool>,
    /// The complication families that the app can provide data for.
    ///
    /// To add this key to the information property list, enable the desired families in the WatchKit extension’s Complication
    /// Configuration settings.
    #[serde(
        rename = "CLKComplicationSupportedFamilies",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_vec_enum_option"
    )]
    pub complication_supported_families: Option<Vec<ComplicationSupportedFamilies>>,
}

/// Extensions and Services.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ExtensionsAndServices {
    /// The properties of an app extension.
    #[serde(rename = "NSExtension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<Extension>,
    /// The services provided by an app.
    #[serde(rename = "NSServices", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
    /// The name of your watchOS app’s extension delegate.
    ///
    /// This key provides the name of a class that adopts the WKExtensionDelegate protocol. Xcode automatically includes
    /// this key in the WatchKit extension’s information property list when you create a watchOS project from a template.
    /// You only modify this value when you rename or replace the extension delegate.
    #[serde(
        rename = "WKExtensionDelegateClassName",
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_delegate_class_name: Option<String>,
    /// The bundle ID of the widget that's available as a Home screen quick action in apps that have more than one widget.
    #[serde(
        rename = "UIApplicationShortcutWidget",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_shortcut_widget: Option<String>,
}

/// App Clips.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AppClips {
    /// A collection of keys that an App Clip uses to get additional capabilities.
    #[serde(
        rename = "NSAppClip",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_clip: Option<AppClip>,
}

/// App Clip.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AppClip {
    /// A Boolean value that indicates whether an App Clip can schedule or receive notifications for a limited amount of time.
    ///
    /// Set the corresponding value to true to enable your App Clip to schedule or receive notifications for up to 8 hours
    /// after each launch. For more information, see Enabling Notifications in App Clips.
    #[serde(
        rename = "NSAppClipRequestEphemeralUserNotification",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_ephemeral_user_notification: Option<bool>,
    /// A Boolean value that indicates whether an App Clip can confirm the user’s location.
    ///
    /// Set the value to true to allow your App Clip to confirm the user’s location. For more information, see Responding to Invocations.
    #[serde(
        rename = "NSAppClipRequestLocationConfirmation",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_location_confirmation: Option<bool>,
}

/// Extension.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Extension {
    /// The names of the intents that an extension supports.
    #[serde(rename = "IntentsSupported", skip_serializing_if = "Option::is_none")]
    pub intents_supported: Option<Vec<String>>,
    /// A dictionary that specifies the minimum size of the floating window in which Final Cut Pro hosts the extension view.
    #[serde(
        rename = "ProExtensionAttributes",
        skip_serializing_if = "Option::is_none"
    )]
    pub pro_extension_attributes: Option<BTreeMap<String, String>>,
    /// The name of the class with the principal implementation of your extension.
    ///
    /// The Compressor app instantiates the class specified in the ProExtensionPrincipalClass key to convert source files
    /// to the output format your extension supports.
    #[serde(
        rename = "ProExtensionPrincipalClass",
        skip_serializing_if = "Option::is_none"
    )]
    pub pro_extension_principal_class: Option<String>,
    /// The name of the principal view controller class of your extension.
    ///
    /// This key provides the name of the primary view controller class of your extension that adopts the NSViewController
    /// protocol. When you create an extension, the Xcode template automatically includes this key in the workflow extension
    /// information property list. You only modify the value of this key when you rename the primary view controller class in your extension.
    #[serde(
        rename = "ProExtensionPrincipalViewControllerClass",
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
    #[serde(rename = "ProExtensionUUID", skip_serializing_if = "Option::is_none")]
    pub pro_extension_uuid: Option<String>,
    /// Account Authentication Modification. The rules the system satisfies when generating a strong password for your
    /// extension during an automatic upgrade.
    #[serde(
        rename = "ASAccountAuthenticationModificationPasswordGenerationRequirements",
        skip_serializing_if = "Option::is_none"
    )]
    pub password_generation_requirements: Option<String>,
    /// Account Authentication Modification. A Boolean value that indicates whether the extension supports upgrading a user’s
    /// password to a strong password.
    #[serde(
        rename = "ASAccountAuthenticationModificationSupportsStrongPasswordUpgrade",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_strong_password_upgrade: Option<bool>,
    /// Account Authentication Modification. A Boolean value that indicates whether the extension supports upgrading from using
    /// password authentication to using Sign in with Apple.
    #[serde(
        rename = "ASAccountAuthenticationModificationSupportsUpgradeToSignInWithApple",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_upgrade_to_sign_in_with_apple: Option<bool>,
    /// A Boolean value indicating whether the Action extension is presented in full screen.
    #[serde(
        rename = "NSExtensionActionWantsFullScreenPresentation",
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_action_wants_full_screen_presentation: Option<bool>,
    /// Properties of an app extension.
    /// TODO: https://developer.apple.com/documentation/bundleresources/information_property_list/nsextension/nsextensionattributes
    #[serde(
        rename = "NSExtensionAttributes",
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_attributes: Option<BTreeMap<String, String>>,
    /// The name of the app extension’s main storyboard file.
    ///
    /// This key is mutually exclusive with NSExtensionPrincipalClass. Typically, Xcode sets the value of this key when creating
    /// an App Extension target in your project. If you change the name of your storyboard file, remember to update the value of this key.
    #[serde(
        rename = "NSExtensionMainStoryboard",
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_main_storyboard: Option<String>,
    /// A Boolean value indicating whether the app extension ignores appearance changes made by the host app.
    #[serde(
        rename = "NSExtensionOverridesHostUIAppearance",
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_overrides_host_ui_appearance: Option<bool>,
    /// The extension point that supports an app extension.
    #[serde(
        rename = "NSExtensionPointIdentifier",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub extension_point_identifier: Option<ExtensionPointIdentifier>,
    /// The custom class that implements an app extension’s primary view or functionality.
    ///
    /// This key is mutually exclusive with NSExtensionMainStoryboard. Typically, Xcode sets the value of this key when creating an App
    /// Extension target in your project. If you change the name of the specified class, remember to update the value of this key.
    #[serde(
        rename = "NSExtensionPrincipalClass",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub extension_principal_class: Option<String>,
    /// The content scripts for a Safari extension.
    #[serde(
        rename = "SFSafariContentScript",
        skip_serializing_if = "Option::is_none"
    )]
    pub safari_content_script: Option<Vec<SafariContentScript>>,
    /// The context menu items for a Safari extension.
    #[serde(
        rename = "SFSafariContextMenu",
        skip_serializing_if = "Option::is_none"
    )]
    pub safari_context_menu: Option<Vec<SafariContextMenu>>,
    /// The style sheet for a Safari extension.
    #[serde(rename = "SFSafariStyleSheet", skip_serializing_if = "Option::is_none")]
    pub safari_style_sheet: Option<Vec<SafariStyleSheet>>,
    /// The items to add to the toolbar for a Safari extension.
    #[serde(
        rename = "SFSafariToolbarItem",
        skip_serializing_if = "Option::is_none"
    )]
    pub safari_toolbar_item: Option<SafariToolbarItem>,
    /// The webpages a Safari extension can access.
    #[serde(
        rename = "SFSafariWebsiteAccess",
        skip_serializing_if = "Option::is_none"
    )]
    pub safari_website_access: Option<SafariWebsiteAccess>,
}

/// Safari Website Access.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SafariWebsiteAccess {
    /// The domains that a Safari extension is allowed access to.
    #[serde(rename = "Allowed Domains", skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,
    /// The level of a Safari extension’s website access.
    #[serde(rename = "Level", skip_serializing_if = "Option::is_none")]
    pub level: Option<SafariWebsiteAccessLevel>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SafariWebsiteAccessLevel {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "All")]
    All,
    #[serde(rename = "Some")]
    Some,
}

/// Safari Toolbar Item.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SafariToolbarItem {
    /// The properties of an app extension's toolbar item that's been added to the Safari window.
    #[serde(rename = "Action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// The identifier for a Safari extension's toolbar item.
    #[serde(rename = "Identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// An image that represents a Safari extension's toolbar item.
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// The label for the Safari extension's toolbar item.
    #[serde(rename = "Label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Safari Style Sheet.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SafariStyleSheet {
    /// The webpages that the script can be injected into.
    #[serde(
        rename = "Allowed URL Patterns",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_url_patterns: Option<Vec<String>>,
    /// The webpages that the script can't be injected into.
    #[serde(
        rename = "Excluded URL Patterns",
        skip_serializing_if = "Option::is_none"
    )]
    pub excluded_url_patterns: Option<Vec<String>>,
    /// The path to the style sheet, relative to the Resources folder in the app extension's bundle.
    #[serde(rename = "Style Sheet", skip_serializing_if = "Option::is_none")]
    pub style_sheet: Option<String>,
}

/// The context menu items for a Safari extension.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SafariContextMenu {
    /// The command to send to the app extension when the user selects the context menu item.
    #[serde(rename = "Command", skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// The text to display for the context menu item.
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Safari Content Script.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SafariContentScript {
    /// The webpages that the script can be injected into.
    #[serde(
        rename = "Allowed URL Patterns",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_url_patterns: Option<Vec<String>>,
    /// The webpages that the script can't be injected into.
    #[serde(
        rename = "Excluded URL Patterns",
        skip_serializing_if = "Option::is_none"
    )]
    pub excluded_url_patterns: Option<Vec<String>>,
    /// The path to the content script, relative to the Resources folder in the app extension's bundle.
    #[serde(rename = "Script", skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
}

/// Extension Point Identifier.
#[derive(Deserialize, Serialize, Debug)]
pub enum ExtensionPointIdentifier {
    #[serde(rename = "com.apple.ui-services")]
    UiServices,
    #[serde(rename = "com.apple.services")]
    Services,
    #[serde(rename = "com.apple.keyboard-service")]
    KeyboardService,
    #[serde(rename = "com.apple.fileprovider-nonui")]
    FileproviderNonui,
    #[serde(rename = "com.apple.fileprovider-actionsui")]
    FileproviderActionsui,
    #[serde(rename = "com.apple.FinderSync")]
    FinderSync,
    #[serde(rename = "com.apple.identitylookup.message-filter")]
    IdentityLookupMessageFilter,
    #[serde(rename = "com.apple.photo-editing")]
    PhotoEditing,
    #[serde(rename = "com.apple.share-services")]
    ShareServices,
    #[serde(rename = "com.apple.callkit.call-directory")]
    CallkitCallDirectory,
    #[serde(rename = "com.apple.authentication-services-account-authentication-modification-ui")]
    AuthenticationServicesAccountAuthenticationModificationUi,
    #[serde(rename = "com.apple.AudioUnit-UI")]
    AudioUnitUi,
    #[serde(rename = "com.apple.AppSSO.idp-extension")]
    AppSSOIdpExtension,
    #[serde(rename = "com.apple.authentication-services-credential-provider-ui")]
    AuthenticationServicesCredentialProviderUi,
    #[serde(rename = "com.apple.broadcast-services-setupui")]
    BroadcastServicesSetupui,
    #[serde(rename = "com.apple.broadcast-services-upload")]
    BroadcastServicesUpload,
    #[serde(rename = "com.apple.classkit.context-provider")]
    ClasskitContextProvider,
    #[serde(rename = "com.apple.Safari.content-blocker")]
    SafariContentBlocker,
    #[serde(rename = "com.apple.message-payload-provider")]
    MessagePayloadProvider,
    #[serde(rename = "com.apple.intents-service")]
    IntentsService,
    #[serde(rename = "com.apple.intents-ui-service")]
    IntentsUiService,
    #[serde(rename = "com.apple.networkextension.app-proxy")]
    NetworkExtensionAppProxy,
    #[serde(rename = "com.apple.usernotifications.content-extension")]
    UsernotificationsContentExtension,
    #[serde(rename = "com.apple.usernotifications.service")]
    UsernotificationsService,
    #[serde(rename = "com.apple.ctk-tokens")]
    CtkTokens,
    #[serde(rename = "com.apple.photo-project")]
    PhotoProject,
    #[serde(rename = "com.apple.quicklook.preview")]
    QuicklookPreview,
    #[serde(rename = "com.apple.Safari.extension")]
    SafariExtension,
    #[serde(rename = "com.apple.spotlight.index")]
    SpotlightIndex,
    #[serde(rename = "com.apple.quicklook.thumbnail")]
    QuicklookThumbnail,
    #[serde(rename = "com.apple.tv-top-shelf")]
    TvTopShelf,
    #[serde(rename = "com.apple.identitylookup.classification-ui")]
    ClassificationUi,
    #[serde(rename = "com.apple.widgetkit-extension")]
    WidgetkitExtension,
    #[serde(rename = "com.apple.dt.Xcode.extension.source-editor")]
    ExtensionSourceEditor,
}

/// Service.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Service {
    /// A keyboard shortcut that invokes the service menu command.
    #[serde(rename = "NSKeyEquivalent", skip_serializing_if = "Option::is_none")]
    pub key_equivalent: Option<DefaultDictionary>,
    /// Text for a Services menu item.
    #[serde(rename = "NSMenuItem", skip_serializing_if = "Option::is_none")]
    pub menu_item: DefaultDictionary,
    /// An instance method that invokes the service.
    #[serde(rename = "NSMessage", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// The port that the service monitors for incoming requests.
    #[serde(rename = "NSPortName", skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
    /// The data types that the service returns.
    #[serde(rename = "NSReturnTypes", skip_serializing_if = "Option::is_none")]
    pub return_types: Option<Vec<String>>,
    /// The data types that the service can read.
    #[serde(rename = "NSSendTypes", skip_serializing_if = "Option::is_none")]
    pub send_types: Option<Vec<String>>,
    /// The amount of time, in milliseconds, that the system waits for a response from the service.
    #[serde(rename = "NSTimeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    /// A service-specific string value.
    #[serde(rename = "NSUserData", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<BTreeMap<String, String>>,
}

/// Default Dictionary.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DefaultDictionary {
    pub default: String,
}

/// Complication Supported Families.
#[derive(Deserialize, Serialize, Debug, Default)]
pub enum ComplicationSupportedFamilies {
    #[serde(rename = "CLKComplicationFamilyModularSmall")]
    ModularSmall,
    #[serde(rename = "CLKComplicationFamilyModularLarge")]
    ModularLarge,
    #[serde(rename = "CLKComplicationFamilyUtilitarianSmall")]
    UtilitarianSmall,
    #[serde(rename = "CLKComplicationFamilyUtilitarianSmallFlat")]
    UtilitarianSmallFlat,
    #[serde(rename = "CLKComplicationFamilyUtilitarianLarge")]
    UtilitarianLarge,
    #[serde(rename = "CLKComplicationFamilyCircularSmall")]
    CircularSmall,
    #[serde(rename = "CLKComplicationFamilyExtraLarge")]
    ExtraLarge,
    #[serde(rename = "CLKComplicationFamilyGraphicCorner")]
    GraphicCorner,
    #[serde(rename = "CLKComplicationFamilyGraphicBezel")]
    GraphicBezel,
    #[serde(rename = "CLKComplicationFamilyGraphicCircular")]
    GraphicCircular,
    #[serde(rename = "CLKComplicationFamilyGraphicRectangular")]
    GraphicRectangular,
}

/// Architecture Priority.
#[derive(Deserialize, Serialize, Debug, Default)]
pub enum ArchitecturePriority {
    /// The 32-bit Intel architecture.
    #[serde(rename = "i386")]
    I386,
    /// The 64-bit Intel architecture.
    #[serde(rename = "x86_64")]
    X86_64,
    /// The 64-bit ARM architecture.
    #[serde(rename = "arm64")]
    Arm64,
    /// The 64-bit ARM architecture with pointer authentication code support.
    #[serde(rename = "arm64e")]
    Arm64e,
}

/// Device Capabilities.
#[derive(Deserialize, Serialize, Debug, Default)]
pub enum DeviceCapabilities {
    /// The presence of accelerometers. Use the Core Motion framework to receive accelerometer events. You don’t need to
    /// include this value if your app detects only device orientation changes. Available in iOS 3.0 and later.
    #[serde(rename = "accelerometer")]
    Accelerometer,
    /// Support for ARKit. Available in iOS 11.0 and later.
    #[serde(rename = "arkit")]
    Arkit,
    /// Compilation for the armv7 instruction set, or as a 32/64-bit universal app. Available in iOS 3.1 and later.
    #[serde(rename = "armv7")]
    Armv7,
    /// Compilation for the arm64 instruction set. Include this key for all 64-bit apps and embedded bundles, like
    /// extensions and frameworks. Available in iOS 8.0 and later.
    #[serde(rename = "arm64")]
    Arm64,
    /// Autofocus capabilities in the device’s still camera. You might need to include this value if your app supports
    /// macro photography or requires sharper images to perform certain image-processing tasks. Available in iOS 3.0 and later.
    #[serde(rename = "auto-focus-camera")]
    AutoFocusCamera,
    /// Bluetooth low-energy hardware. Available in iOS 5.0 and later.
    #[serde(rename = "bluetooth-le")]
    BluetoothLe,
    /// A camera flash. Use the cameraFlashMode property of a UIImagePickerController instance to control the camera’s
    /// flash. Available in iOS 3.0 and later.
    #[serde(rename = "camera-flash")]
    CameraFlash,
    /// A forward-facing camera. Use the cameraDevice property of a UIImagePickerController instance to select the
    /// device’s camera. Available in iOS 3.0 and later.
    #[serde(rename = "front-facing-camera")]
    FrontFacingCamera,
    /// Access to the Game Center service. Enable the Game Center capability in Xcode to add this value to your app.
    /// Available in iOS 4.1 and later.
    #[serde(rename = "gamekit")]
    Gamekit,
    /// GPS (or AGPS) hardware for tracking locations. If you include this value, you should also include the
    /// location-services value. Require GPS only if your app needs location data more accurate than the cellular or Wi-Fi
    /// radios provide. Available in iOS 3.0 and later.
    #[serde(rename = "gps")]
    Gps,
    /// A gyroscope. Use the Core Motion framework to retrieve information from gyroscope hardware. Available in iOS 3.0 and later.
    #[serde(rename = "gyroscope")]
    Gyroscope,
    /// Support for HealthKit. Available in iOS 8.0 and later.
    #[serde(rename = "healthkit")]
    Healthkit,
    /// Performance and capabilities of the A12 Bionic and later chips. Available in iOS 12.0 and later.
    #[serde(rename = "iphone-ipad-minimum-performance-a12")]
    IphoneIpadMinimumPerformanceA12,
    /// Access to the device’s current location using the Core Location framework. This value refers to the general location
    /// services feature. If you specifically need GPS-level accuracy, also include the gps feature. Available in iOS 3.0 and later.
    #[serde(rename = "location-services")]
    LocationServices,
    /// Magnetometer hardware. Apps use this hardware to receive heading-related events through the Core Location framework.
    /// Available in iOS 3.0 and later.
    #[serde(rename = "magnetometer")]
    Magnetometer,
    // Support for graphics processing with Metal. Available in iOS 8.0 and later.
    #[serde(rename = "metal")]
    Metal,
    /// The built-in microphone or accessories that provide a microphone. Available in iOS 3.0 and later.
    #[serde(rename = "microphone")]
    Microphone,
    /// Near Field Communication (NFC) tag detection and access to messages that contain NFC Data Exchange Format data.
    /// Use the Core NFC framework to detect and read NFC tags. Available in iOS 11.0 and later.
    #[serde(rename = "nfc")]
    Nfc,
    /// The OpenGL ES 1.1 interface. Available in iOS 3.0 and later.
    #[serde(rename = "opengles-1")]
    Opengles1,
    /// The OpenGL ES 2.0 interface. Available in iOS 3.0 and later.
    #[serde(rename = "opengles-2")]
    Opengles2,
    /// The OpenGL ES 3.0 interface. Available in iOS 7.0 and later.
    #[serde(rename = "opengles-2")]
    Opengles3,
    /// Peer-to-peer connectivity over a Bluetooth network. Available in iOS 3.1 and later.
    #[serde(rename = "peer-peer")]
    PeerPeer,
    /// The Messages app. You might require this feature if your app opens URLs with the sms scheme. Available in iOS 3.0 and later.
    #[serde(rename = "sms")]
    Sms,
    /// A camera on the device. Use the UIImagePickerController interface to capture images from the device’s still camera.
    /// Available in iOS 3.0 and later.
    #[serde(rename = "still-camera")]
    StillCamera,
    /// The Phone app. You might require this feature if your app opens URLs with the tel scheme. Available in iOS 3.0 and later.
    #[serde(rename = "telephony")]
    Telephony,
    /// A camera with video capabilities on the device. Use the UIImagePickerController interface to capture video from the
    /// device’s camera. Available in iOS 3.0 and later.
    #[serde(rename = "video-camera")]
    VideoCamera,
    /// Networking features related to Wi-Fi access. Available in iOS 3.0 and later.
    #[serde(rename = "wifi")]
    Wifi,
}

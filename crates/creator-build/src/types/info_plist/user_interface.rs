/// User Interface.
///
/// Configure an app's scenes, storyboards, icons, fonts, and other user interface elements.
///
/// You define the user interface that your app presents during normal operation with a combination of code and storyboards.
/// However, the system needs to know a few things about your app’s user interface before execution begins. For example,
/// on some platforms, you have to specify what device orientations your app supports and what the system should display
/// while your app launches. You add keys to your app’s Information Property List file to control certain aspects of its user interface.
///
use super::{serialize_enum_option, serialize_vec_enum_option};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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

/// Launch Interface.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct LaunchInterface {
    /// The user interface to show while an app launches.
    ///
    /// You use this key to define the launch screen that the system displays while your app launches.
    /// If you need to provide different launch screens in response to being launched by different
    /// URL schemes, use UILaunchScreens instead.
    #[serde(rename = "UILaunchScreen", skip_serializing_if = "Option::is_none")]
    pub launch_screen: Option<LaunchScreen>,
    /// The user interfaces to show while an app launches in response to different URL schemes.
    ///
    /// You use this key if your app supports launching in response to one or more URL schemes, and if
    /// you want to provide different launch screens for different launch triggers.
    /// If you need only one launch screen, use UILaunchScreen instead.
    ///
    /// To define launch screens, create an array of dictionaries, each similar to the one you might
    /// provide for UILaunchScreen, but with an added UILaunchScreenIdentifier key that uniquely
    /// identifies the screen. Store the array as the value for the UILaunchScreenDefinitions key.
    ///
    /// To map from URL schemes to a launch screens, create a dictionary of schemes and identifiers,
    /// and store it as the value for the UIURLToLaunchScreenAssociations key. Additionally,
    /// indicate a default launch screen by setting a value for the UIDefaultLaunchScreen key.
    #[serde(rename = "UILaunchScreens", skip_serializing_if = "Option::is_none")]
    pub launch_screens: Option<LaunchScreens>,
    /// The filename of the storyboard from which to generate the app’s launch image.
    ///
    /// Specify the name of the storyboard file without the filename extension. For example, if the filename
    /// of your storyboard is LaunchScreen.storyboard, specify "LaunchScreen" as the value for this key.
    ///
    /// If you prefer to configure your app’s launch screen without storyboards, use UILaunchScreen instead.
    #[serde(
        rename = "UILaunchStoryboardName",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_storyboard_name: Option<String>,
    /// The launch storyboards.
    #[serde(
        rename = "UILaunchStoryboards",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_storyboards: Option<LaunchStoryboards>,
    /// The initial user-interface mode for the app.
    ///
    /// Possible Values: 0, 1, 2, 3, 4
    #[serde(
        rename = "LSUIPresentationMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub presentation_mode: Option<u8>,
}

/// Icons.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Icons {
    /// Information about all of the icons used by the app.
    #[serde(rename = "CFBundleIcons", skip_serializing_if = "Option::is_none")]
    pub bundle_icons: Option<BundleIcons>,
    /// The names of the bundle’s icon image files.
    #[serde(rename = "CFBundleIconFiles", skip_serializing_if = "Option::is_none")]
    pub bundle_icon_files: Option<Vec<String>>,
    /// The file containing the bundle's icon.
    #[serde(rename = "CFBundleIconFile", skip_serializing_if = "Option::is_none")]
    pub bundle_icon_file: Option<String>,
    /// The name of the asset that represents the app icon.
    #[serde(rename = "CFBundleIconName", skip_serializing_if = "Option::is_none")]
    pub bundle_icon_name: Option<String>,
    /// A Boolean value indicating whether the app’s icon already contains a shine effect.
    #[serde(rename = "UIPrerenderedIcon", skip_serializing_if = "Option::is_none")]
    pub prerendered_icon: Option<bool>,
}

/// Orientation.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Orientation {
    /// The initial orientation of the app’s user interface.
    #[serde(
        rename = "UIInterfaceOrientation",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub interface_orientation: Option<InterfaceOrientation>,
    /// The initial orientation of the app’s user interface.
    #[serde(
        rename = "UISupportedInterfaceOrientations",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_vec_enum_option"
    )]
    pub supported_interface_orientations: Option<Vec<InterfaceOrientation>>,
}

/// Styling.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Styling {
    /// The user interface style for the app.
    #[serde(
        rename = "UIUserInterfaceStyle",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub user_interface_style: Option<UserInterfaceStyle>,
    /// A Boolean value indicating whether Core Animation layers use antialiasing when
    /// drawing a layer that's not aligned to pixel boundaries.
    #[serde(
        rename = "UIViewEdgeAntialiasing",
        skip_serializing_if = "Option::is_none"
    )]
    pub view_edge_antialiasing: Option<bool>,
    /// The app’s white point adaptivity style, enabled on devices with True Tone displays.
    #[serde(
        rename = "UIWhitePointAdaptivityStyle",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub white_point_adaptivity_style: Option<WhitePointAdaptivityStyle>,
    /// A Boolean value indicating whether Core Animation sublayers inherit the opacity of their superlayer.
    #[serde(rename = "UIViewGroupOpacity", skip_serializing_if = "Option::is_none")]
    pub view_group_opacity: Option<bool>,
    /// A Boolean value indicating whether the app requires fullscreen or not.
    #[serde(
        rename = "UIRequiresFullScreen",
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_full_screen: Option<bool>,
    /// The name of a color in an asset catalog to use for a target’s global accent color.
    ///
    /// This Info.plist value controls the global tint color (iOS and watchOS) or accent color (macOS) for the target.
    /// When set in a widget extension, the widget configuration user interface uses this color as the tint color while editing a widget.
    ///
    /// While you can set this directly in your Info.plist, the recommended approach is to use the Global Accent Color
    /// Name build setting (in the Asset Catalog Compiler - Options section) of the target. Set the value of the build
    /// setting to the name of the Color Set in the asset catalog. Xcode automatically sets NSAccentColorName to the appropriate
    /// value in the Info.plist file when building your project.
    #[serde(rename = "NSAccentColorName", skip_serializing_if = "Option::is_none")]
    pub accent_color_name: Option<String>,
    /// The name of a color in an asset catalog to use for a widget’s configuration interface.
    ///
    /// This Info.plist value controls the background color shown in the widget configuration interface while editing a widget.
    ///
    /// While you can set this directly in your Info.plist, the recommended approach is to use the Widget Background Color
    /// Name build setting (in the Asset Catalog Compiler - Options section) of the widget extension target. Set the value
    /// of the build setting to the name of the Color Set in the asset catalog. Xcode automatically sets NSWidgetBackgroundColorName
    /// to the appropriate value in the Info.plist file when building your project.
    #[serde(
        rename = "NSWidgetBackgroundColorName",
        skip_serializing_if = "Option::is_none"
    )]
    pub widget_background_color_name: Option<String>,
}

/// Fonts.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Fonts {
    /// The location of a font file or directory of fonts in the bundle’s Resources folder.
    #[serde(
        rename = "ATSApplicationFontsPath",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_fonts_path: Option<String>,
    /// App-specific font files located in the bundle and that the system loads at runtime.
    #[serde(rename = "UIAppFonts", skip_serializing_if = "Option::is_none")]
    pub app_fonts: Option<Vec<String>>,
}

/// StatusBar.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct StatusBar {
    /// A Boolean value indicating whether the status bar is initially hidden when the app launches.
    #[serde(rename = "UIStatusBarHidden", skip_serializing_if = "Option::is_none")]
    pub status_bar_hidden: Option<bool>,
    /// The style of the status bar as the app launches.
    #[serde(
        rename = "UIStatusBarStyle",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub status_bar_style: Option<StatusBarStyle>,
    /// The status bar tint.
    #[serde(
        rename = "UIStatusBarTintParameters",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_bar_tint_parameters: Option<StatusBarTintParameters>,
    /// A Boolean value indicating whether the status bar appearance is based on the style
    /// preferred for the current view controller.
    #[serde(
        rename = "UIViewControllerBasedStatusBarAppearance",
        skip_serializing_if = "Option::is_none"
    )]
    pub view_controller_based_status_bar_appearance: Option<bool>,
}

/// Preferences.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Preferences {
    /// The name of an image file used to represent a preference pane in the System Preferences app.
    #[serde(rename = "NSPrefPaneIconFile", skip_serializing_if = "Option::is_none")]
    pub pref_pane_icon_file: Option<String>,
    /// The name of a preference pane displayed beneath the preference pane icon in the System Preferences app.
    #[serde(
        rename = "NSPrefPaneIconLabel",
        skip_serializing_if = "Option::is_none"
    )]
    pub pref_pane_icon_label: Option<String>,
}

/// Graphics.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Graphics {
    /// A Boolean value indicating whether the app supports HDR mode on Apple TV 4K.
    #[serde(rename = "UIAppSupportsHDR", skip_serializing_if = "Option::is_none")]
    pub app_supports_hdr: Option<bool>,
    /// A Boolean value indicating whether the Cocoa app supports high-resolution displays.
    #[serde(
        rename = "NSHighResolutionCapable",
        skip_serializing_if = "Option::is_none"
    )]
    pub high_resolution_capable: Option<bool>,
    /// A Boolean value indicating whether an OpenGL app may utilize the integrated GPU.
    #[serde(
        rename = "NSSupportsAutomaticGraphicsSwitching",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_automatic_graphics_switching: Option<bool>,
    /// The preferred system action when an external GPU is connected from the system.
    #[serde(
        rename = "GPUEjectPolicy",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub gpu_eject_policy: Option<GPUEjectPolicy>,
    /// The app's preference for whether it wants to use external graphics processors.
    #[serde(
        rename = "GPUSelectionPolicy",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub gpu_selection_policy: Option<GPUSelectionPolicy>,
}

/// QuickLook.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct QuickLook {
    /// A Boolean value indicating whether a Quick Look app's generator can be run in threads other than the main thread.
    #[serde(
        rename = "QLNeedsToBeRunInMainThread",
        skip_serializing_if = "Option::is_none"
    )]
    pub needs_to_be_run_in_main_thread: Option<bool>,
    /// A hint at the height, in points, of a Quick Look app's previews.
    #[serde(rename = "QLPreviewHeight", skip_serializing_if = "Option::is_none")]
    pub preview_height: Option<f32>,
    /// A hint at the width, in points, of a Quick Look app's previews.
    #[serde(rename = "QLPreviewWidth", skip_serializing_if = "Option::is_none")]
    pub preview_width: Option<f32>,
    /// A Boolean value indicating whether a Quick Look app's generator can handle concurrent thumbnail and preview requests.
    #[serde(
        rename = "QLSupportsConcurrentRequests",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_concurrent_requests: Option<bool>,
    /// The minimum size, in points, along one dimension of thumbnails for a Quick Look app's generator.
    #[serde(
        rename = "QLThumbnailMinimumSize",
        skip_serializing_if = "Option::is_none"
    )]
    pub thumbnail_minimum_size: Option<f32>,
}

/// GPU Eject Policy.
#[derive(Deserialize, Serialize, Debug)]
pub enum GPUEjectPolicy {
    /// Set this value to allow macOS to quit and relaunch your app with another GPU.
    /// Your app can implement the application(_:willEncodeRestorableState:) method to save any
    /// state before it quits, and it can implement the application(_:didDecodeRestorableState:)
    /// method to restore any saved state after it relaunches.
    #[serde(rename = "relaunch")]
    Relaunch,
    /// Set this value to manually respond to the safe disconnect request. Your app must register
    /// and respond to the removalRequested notification posted by Metal. macOS waits for your app
    /// to remove all references to the external GPU before notifying the user that it's safe to disconnect the GPU.
    #[serde(rename = "wait")]
    Wait,
    /// Set this value to allow macOS to force your app to quit.
    #[serde(rename = "kill")]
    Kill,
    /// Tells the system to ignore the disconnect message. Don’t use this key in new macOS apps.
    #[serde(rename = "ignore")]
    Ignore,
}

/// GPU Selection Policy.
#[derive(Deserialize, Serialize, Debug)]
pub enum GPUSelectionPolicy {
    /// Metal tries to avoid creating contexts on external GPUs. For legacy OpenGL apps, OpenGL also avoids creating
    /// contexts using external GPUs. Set this option only if your app doesn't support external GPU event handling.
    #[serde(rename = "avoidRemovable")]
    AvoidRemovable,
    /// If external GPUs are visible to the system, Metal prefers them over other GPUs. Similarly, for legacy OpenGL apps,
    /// OpenGL also prefers to create contexts on the external GPU.
    #[serde(rename = "preferRemovable")]
    PreferRemovable,
}

/// NavigationBar.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct StatusBarTintParameters {
    /// The initial navigation bar’s style and translucency.
    #[serde(rename = "UINavigationBar", skip_serializing_if = "Option::is_none")]
    pub navigation_bar: Option<NavigationBar>,
}

/// NavigationBar.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct NavigationBar {
    #[serde(rename = "BackgroundImage")]
    pub background_image: String,
    #[serde(rename = "Style")]
    pub style: BarStyle,
    #[serde(rename = "Translucent")]
    pub translucent: bool,
    /// The tint color to apply to the background of the navigation bar.
    #[serde(rename = "TintColor", skip_serializing_if = "Option::is_none")]
    pub tint_color: Option<TintColor>,
}

/// Bar Style.
#[derive(Deserialize, Serialize, Debug)]
pub enum BarStyle {
    #[serde(rename = "UIBarStyleDefault")]
    Default,
    #[serde(rename = "UIBarStyleBlack")]
    Black,
}

impl Default for BarStyle {
    fn default() -> Self {
        Self::Default
    }
}

/// TintColor.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct TintColor {
    #[serde(rename = "Blue")]
    pub blue: f32,
    #[serde(rename = "Green")]
    pub green: f32,
    #[serde(rename = "Red")]
    pub red: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum StatusBarStyle {
    #[serde(rename = "UIStatusBarStyleDefault")]
    Default,
    #[serde(rename = "UIStatusBarStyleBlackTranslucent")]
    BlackTranslucent,
    #[serde(rename = "UIStatusBarStyleBlackOpaque")]
    BlackOpaque,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum WhitePointAdaptivityStyle {
    #[serde(rename = "UIWhitePointAdaptivityStyleStandard")]
    Standard,
    #[serde(rename = "UIWhitePointAdaptivityStyleReading")]
    Reading,
    #[serde(rename = "UIWhitePointAdaptivityStylePhoto")]
    Photo,
    #[serde(rename = "UIWhitePointAdaptivityStyleVideo")]
    Video,
    #[serde(rename = "UIWhitePointAdaptivityStyleGame")]
    Game,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum UserInterfaceStyle {
    /// Set this value to adopt the systemwide user interface style, and observe any changes to that style.
    /// This is the default value, and provides the same functionality as if the key weren’t explicitly set.
    Automatic,
    /// Set this value to force the light user interface style, even when the systemwide style is set to dark.
    /// Your app will ignore any changes to the systemwide style.
    Light,
    /// Set this value to force the dark user interface style, even when the systemwide style is set to light.
    /// Your app will ignore any changes to the systemwide style.
    Dark,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum InterfaceOrientation {
    /// The app supports the display in portrait mode, with the device upright and the front camera at the top.
    #[serde(rename = "UIInterfaceOrientationPortrait")]
    Portrait,
    /// The app supports the display in portrait mode but is upside down, with the device upright and the front
    /// camera at the bottom. UIViewController ignores this option on devices without a Home button.
    #[serde(rename = "UIInterfaceOrientationPortraitUpsideDown")]
    PortraitUpsideDown,
    /// The app supports the display in landscape mode, with the device upright and the front camera on the left.
    #[serde(rename = "UIInterfaceOrientationLandscapeLeft")]
    LandscapeLeft,
    /// The app supports the display in landscape mode, with the device upright and the front camera on the right.
    #[serde(rename = "UIInterfaceOrientationLandscapeRight")]
    LandscapeRight,
}

/// Bundle Icons.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BundleIcons {
    #[serde(
        rename = "CFBundleAlternateIcons",
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_alternate_icons: Option<BTreeMap<String, AppIconReferenceName>>,
    /// The primary icon for the Home screen and Settings app, among others.
    #[serde(rename = "CFBundlePrimaryIcon")]
    pub bundle_primary_icon: BundlePrimaryIcon,
}

/// App Icon Reference Name.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AppIconReferenceName {
    #[serde(rename = "CFBundleIconFiles", skip_serializing_if = "Option::is_none")]
    pub bundle_icon_files: Option<Vec<String>>,
    #[serde(rename = "UIPrerenderedIcon", skip_serializing_if = "Option::is_none")]
    pub prerendered_icon: Option<bool>,
}

/// Bundle Primary Icon.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BundlePrimaryIcon {
    /// The names of a bundle’s icon files.
    #[serde(rename = "CFBundleIconFiles")]
    pub bundle_icon_files: Vec<String>,
    /// The name of a symbol from SF Symbols.
    ///
    /// Action extensions use template images for their icons. To use a symbol from SF Symbols
    /// as the icon, set the value of CFBundleSymbolName to the symbol’s name.
    #[serde(rename = "CFBundleSymbolName", skip_serializing_if = "Option::is_none")]
    pub bundle_symbol_name: Option<String>,
    /// A Boolean value indicating whether the icon files already incorporate a shine effect.
    #[serde(rename = "UIPrerenderedIcon")]
    pub prerendered_icon: bool,
}

/// Launch Screen.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct LaunchScreen {
    // Main Interface.
    /// The name of a color to use as the background color on the launch screen.
    ///
    /// Provide a value for this key that’s the name of a color in your asset catalog.
    /// You use the same string for the value that you might use when calling the init(named:) initializer of UIColor.
    ///
    /// If you don’t set a color, the system uses a default of systemBackground, which varies according to whether
    /// the user has selected the light appearance or Dark Mode for the device.
    #[serde(rename = "UIColorName", skip_serializing_if = "Option::is_none")]
    pub color_name: Option<String>,
    /// The name of an image to display during app launch.
    ///
    /// Provide a value for this key that’s the name of an image in your asset catalog. You use the same string for
    /// the value that you might use when calling the init(named:) initializer of UIImage. Because the image comes
    /// from your asset catalog, you can use slicing to provide a small image that works on many different platforms.
    ///
    /// If you don’t specify an image, the display shows the background color, as given by the UIColorName key.
    /// The background color may also show through any transparency in your image.
    #[serde(rename = "UIImageName", skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// A Boolean that specifies whether the launch image should respect the safe area insets.
    #[serde(
        rename = "UIImageRespectsSafeAreaInsets",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_respects_safe_area_insets: Option<bool>,
    // Border Elements.
    /// Navigation bar visibility and configuration during launch.
    ///
    /// When you provide a dictionary for this key, the system displays a navigation bar during launch.
    /// You can optionally set the dictionary’s UIImageName key to define a custom image for the navigation bar.
    ///
    /// Omit this key if you don’t want to display a navigation bar during launch.
    #[serde(rename = "UINavigationBar", skip_serializing_if = "Option::is_none")]
    pub navigation_bar: Option<Bar>,
    /// Tab bar visibility and configuration during launch.
    ///
    /// When you provide a dictionary for this key, the system displays a tab bar during launch.
    /// You can optionally set the dictionary’s UIImageName key to define a custom image for the tab bar.
    ///
    /// Omit this key if you don’t want to display a tab bar during launch.
    #[serde(rename = "UITabBar", skip_serializing_if = "Option::is_none")]
    pub tab_bar: Option<Bar>,
    /// When you provide a dictionary for this key, the system displays a toolbar during launch.
    /// You can optionally set the dictionary’s UIImageName key to define a custom image for the toolbar.
    ///
    /// Omit this key if you don’t want to display a toolbar during launch.
    #[serde(rename = "UIToolbar", skip_serializing_if = "Option::is_none")]
    pub toolbar: Option<Bar>,
}

/// Application Scene Manifest.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Bar {
    /// A custom image that replaces the navigation/tab/tool bar during launch.
    ///
    /// Provide a value for this key that’s the name of an image in your asset catalog. You use the same string for
    /// the value that you might use when calling the init(named:) initializer of UIImage.
    #[serde(rename = "UIImageName", skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
}

/// Launch Screens.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct LaunchScreens {
    // Launch Screen Definitions.
    /// A collection of launch screen configuration dictionaries.
    ///
    /// Each dictionary in the array resembles the one you might define for the UILaunchScreen key,
    /// with the addition of a UILaunchScreenIdentifier key that provides a unique identifier for the dictionary.
    /// You use that identifier when associating to the dictionary with a URL scheme in the UIURLToLaunchScreenAssociations
    /// array, or to indicate it as the default launch screen with the UIDefaultLaunchScreen key.
    #[serde(
        rename = "UILaunchScreenDefinitions",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_screen_definitions: Option<LaunchScreenDefinitions>,
    // Associations.
    /// The mapping of URL schemes to launch screen configurations.
    ///
    /// Set the keys of this dictionary to the URL schemes that your app supports.
    /// Provide a value for each key that is the identifier, stored in the UILaunchScreenIdentifier key,
    /// of one of the launch screen definitions in your UILaunchScreenDefinitions array.
    ///
    /// Any Key - A URL scheme. Set one of the configuration identifiers as the value.
    #[serde(
        rename = "UIURLToLaunchScreenAssociations",
        skip_serializing_if = "Option::is_none"
    )]
    pub url_to_launch_screen_associations: Option<BTreeMap<String, String>>,
    /// The default launch screen configuration.
    ///
    /// Provide the identifier, stored in the UILaunchScreenIdentifier key, of one of the launch screen
    /// definitions in your UILaunchScreenDefinitions array. The system displays the named launch screen
    /// when launching your app in response to a URL scheme that you don’t enumerate in the
    /// UIURLToLaunchStoryboardAssociations dictionary, or when the user launches your app directly.
    #[serde(
        rename = "UIDefaultLaunchScreen",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_launch_screen: Option<String>,
}

/// Launch Screen Definitions.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct LaunchScreenDefinitions {
    /// A unique name for the launch screen configuration.
    ///
    /// You can choose any name you want for the identifier, as long as it’s unique among all your app’s configuration
    /// identifiers. Use this value to refer to the configuration when storing a URL to configuration mapping as the
    /// value for the UIURLToLaunchScreenAssociations key, or when specifying a default configuration with the UIDefaultLaunchScreen key.
    #[serde(rename = "UIColorName", skip_serializing_if = "Option::is_none")]
    pub color_name: Option<String>,
    /// Launch Storyboards.
    #[serde(flatten)]
    pub launch_screen: LaunchScreen,
}

/// Launch Storyboards.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct LaunchStoryboards {
    #[serde(
        rename = "UIDefaultLaunchStoryboard",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_launch_storyboard: Option<String>,
    #[serde(
        rename = "UILaunchStoryboardDefinitions",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_storyboard_definitions: Option<Vec<LaunchStoryboardDefinition>>,
    #[serde(
        rename = "UIURLToLaunchStoryboardAssociations",
        skip_serializing_if = "Option::is_none"
    )]
    pub url_to_launch_storyboard_associations: Option<BTreeMap<String, String>>,
}

/// Launch Storyboard Definition.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct LaunchStoryboardDefinition {
    #[serde(
        rename = "UILaunchStoryboardFile",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_storyboard_file: Option<String>,
    #[serde(
        rename = "UILaunchStoryboardIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_storyboard_identifier: Option<String>,
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

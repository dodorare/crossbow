/// Bundle Configuration.
///
/// Define basic characteristics of a bundle, like its name, type, and version.
///
/// The Information Property List file associated with a bundle tells you how to interpret the bundle’s contents.
/// The file describes fundamental features, like whether the bundle contains an app, a framework, or something else.
/// It also includes identifying characteristics of the bundle, like an identifier, a human-readable name, and a version.
///
use super::serialize_enum_option;
use serde::{Deserialize, Serialize};

/// Categorization.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Categorization {
    /// The type of bundle.
    ///
    /// This key consists of a four-letter code for the bundle type.
    /// For apps, the code is APPL, for frameworks, it's FMWK, and for bundles,
    /// it's BNDL. The default value is derived from the bundle extension or,
    /// if it can't be derived, the default value is BNDL.
    #[serde(
        rename(serialize = "CFBundlePackageType"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_package_type: Option<String>,
    /// The category that best describes your app for the App Store.
    #[serde(
        rename(serialize = "LSApplicationCategoryType"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub application_category_type: Option<AppCategoryType>,
}

/// Identification.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Identification {
    /// A unique identifier for a bundle.
    ///
    /// A bundle ID uniquely identifies a single app throughout the system.
    /// The bundle ID string must contain only alphanumeric characters (A-Z, a-z, and 0-9),
    /// hyphens (-), and periods (.). The string should be in reverse-DNS format.
    /// Bundle IDs are case sensitive.
    #[serde(rename(serialize = "CFBundleIdentifier"))]
    pub bundle_identifier: String,
    /// The bundle ID of the watchOS app.
    ///
    /// This key is automatically included in your WatchKit extension’s
    /// information property list when you create a watchOS project from a template.
    #[serde(
        rename(serialize = "WKAppBundleIdentifier"),
        skip_serializing_if = "Option::is_none"
    )]
    pub app_bundle_identifier: Option<String>,
    /// The bundle ID of the watchOS app’s companion iOS app.
    ///
    /// Xcode automatically includes this key in the WatchKit app’s information
    /// property list when you create a watchOS project from a template.
    /// The value should be the same as the iOS app’s CFBundleIdentifier.
    #[serde(
        rename(serialize = "WKCompanionAppBundleIdentifier"),
        skip_serializing_if = "Option::is_none"
    )]
    pub companion_app_bundle_identifier: Option<String>,
}

/// Naming.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Naming {
    /// A user-visible short name for the bundle.
    ///
    /// This name can contain up to 15 characters. The system may display
    /// it to users if CFBundleDisplayName isn't set.
    #[serde(rename(serialize = "CFBundleName"))]
    pub bundle_name: Option<String>,
    /// The user-visible name for the bundle, used by Siri and visible on the iOS Home screen.
    ///
    /// Use this key if you want a product name that's longer than CFBundleName.
    #[serde(
        rename(serialize = "CFBundleDisplayName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_display_name: Option<String>,
    /// A replacement for the app name in text-to-speech operations.
    #[serde(
        rename(serialize = "CFBundleSpokenName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_spoken_name: Option<String>,
}

/// Bundle Version.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct BundleVersion {
    /// The version of the build that identifies an iteration of the bundle.
    ///
    /// This key is a machine-readable string composed of one to three period-separated integers,
    /// such as 10.14.1. The string can only contain numeric characters (0-9) and periods.
    ///
    /// Each integer provides information about the build version in the format \[Major\].\[Minor\].\[Patch\]:
    /// - Major: A major revision number.
    /// - Minor: A minor revision number.
    /// - Patch: A maintenance release number.
    ///
    /// You can include more integers but the system ignores them.
    /// You can also abbreviate the build version by using only one or two integers,
    /// where missing integers in the format are interpreted as zeros.
    /// For example, 0 specifies 0.0.0, 10 specifies 10.0.0, and 10.5 specifies 10.5.0.
    /// This key is required by the App Store and is used throughout the system to identify the version of the build.
    /// For macOS apps, increment the build version before you distribute a build.
    #[serde(
        rename(serialize = "CFBundleVersion"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_version: Option<String>,
    /// The release or version number of the bundle.
    ///
    /// This key is a user-visible string for the version of the bundle. The required format is three period-separated integers,
    /// such as 10.14.1. The string can only contain numeric characters (0-9) and periods.
    ///
    /// Each integer provides information about the release in the format \[Major\].\[Minor\].\[Patch\]:
    /// - Major: A major revision number.
    /// - Minor: A minor revision number.
    /// - Patch: A maintenance release number.
    ///
    /// This key is used throughout the system to identify the version of the bundle.
    #[serde(
        rename(serialize = "CFBundleShortVersionString"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_short_version_string: Option<String>,
    /// The current version of the Information Property List structure.
    ///
    /// Xcode adds this key automatically. Don’t change the value.
    #[serde(
        rename(serialize = "CFBundleInfoDictionaryVersion"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_info_dictionary_version: Option<String>,
    /// A human-readable copyright notice for the bundle.
    #[serde(
        rename(serialize = "NSHumanReadableCopyright"),
        skip_serializing_if = "Option::is_none"
    )]
    pub human_readable_copyright: Option<String>,
}

/// Operating System Version.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct OperatingSystemVersion {
    /// The minimum operating system version required for the app to run.
    ///
    /// The Mac App Store uses this key to indicate the OS releases on
    /// which your app can run and show compatibility with the user’s Mac.
    #[serde(
        rename(serialize = "LSMinimumSystemVersion"),
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_system_version: Option<String>,
    /// The minimum version of macOS required for the app to run on a set of architectures.
    #[serde(
        rename(serialize = "LSMinimumSystemVersionByArchitecture"),
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_system_version_by_architecture: Option<MinimumSystemVersionByArchitecture>,
    /// The minimum operating system version required for the app to run on iOS, tvOS, and watchOS.
    ///
    /// The App Store uses this key to indicate the OS releases on which your app can run.
    #[serde(
        rename(serialize = "MinimumOSVersion"),
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_os_version: Option<String>,
    /// A Boolean value indicating whether the app must run in iOS.
    #[serde(
        rename(serialize = "LSRequiresIPhoneOS"),
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_iphone_os: Option<bool>,
    /// A Boolean value that indicates whether the bundle is a watchOS app.
    ///
    /// Xcode automatically includes this key in the WatchKit app’s information
    /// property list when you create a watchOS project from a template.
    #[serde(
        rename(serialize = "WKWatchKitApp"),
        skip_serializing_if = "Option::is_none"
    )]
    pub watch_kit_app: Option<bool>,
}

/// Localization.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Localization {
    /// The default language and region for the bundle, as a language ID.
    ///
    /// The system uses this key as the language if it can't locate a resource for the user’s preferred language.
    /// The value should be a language ID that identifies a language, dialect, or script.
    ///
    /// To distinguish between different languages and regional dialects, use a language designator with a region
    /// designator and a script designator separated by hyphens. To specify the English language as it's used in
    /// the United Kingdom, use en-GB, where GB is the region designator. To represent Mandarin Chinese,
    /// spoken in Taiwan, and written in Traditional Chinese script, use zh-Hant-TW.
    ///
    /// To specify a script, combine a language designator with a script designator separated by a hyphen,
    /// as in az-Arab for Azerbaijani in the Arabic script.
    #[serde(
        rename(serialize = "CFBundleDevelopmentRegion"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_development_region: Option<String>,
    /// The localizations handled manually by your app.
    #[serde(
        rename(serialize = "CFBundleLocalizations"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_localizations: Option<Vec<String>>,
    /// A Boolean value that indicates whether the bundle supports the retrieval of localized strings from frameworks.
    #[serde(
        rename(serialize = "CFBundleAllowMixedLocalizations"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_allow_mixed_localizations: Option<bool>,
    /// A Boolean value that enables the Caps Lock key to switch between Latin and non-Latin input sources.
    ///
    /// Latin input sources, such as ABC, U.S., and Vietnamese, output characters in Latin script.
    /// Non-Latin input sources, such as Bulgarian (Cyrillic script), Hindi (Devanagari script), and Urdu (Arabic script),
    /// output characters in scripts other than Latin.
    ///
    /// After implementing the key, users can enable or disable this functionality by modifying
    /// the “Use Caps Lock to switch to and from” preference, which can be found in System Preferences > Keyboard > Input Sources.
    #[serde(
        rename(serialize = "TICapsLockLanguageSwitchCapable"),
        skip_serializing_if = "Option::is_none"
    )]
    pub caps_lock_language_switch_capable: Option<bool>,
}

/// Help.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Help {
    /// The name of the bundle’s HTML help file.
    #[serde(
        rename(serialize = "CFAppleHelpAnchor"),
        skip_serializing_if = "Option::is_none"
    )]
    pub apple_help_anchor: Option<String>,
    /// The name of the help file that will be opened in Help Viewer.
    #[serde(
        rename(serialize = "CFBundleHelpBookName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_help_book_name: Option<String>,
    /// The name of the folder containing the bundle’s help files.
    #[serde(
        rename(serialize = "CFBundleHelpBookFolder"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_help_book_folder: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum AppCategoryType {
    #[serde(rename(serialize = "public.app-category.business"))]
    Business,
    #[serde(rename(serialize = "public.app-category.developer-tools"))]
    DeveloperTools,
    #[serde(rename(serialize = "public.app-category.education"))]
    Education,
    #[serde(rename(serialize = "public.app-category.entertainment"))]
    Entertainment,
    #[serde(rename(serialize = "public.app-category.finance"))]
    Finance,
    #[serde(rename(serialize = "public.app-category.games"))]
    Games,
    #[serde(rename(serialize = "public.app-category.action-games"))]
    ActionGames,
    #[serde(rename(serialize = "public.app-category.adventure-games"))]
    AdventureGames,
    #[serde(rename(serialize = "public.app-category.arcade-games"))]
    ArcadeGames,
    #[serde(rename(serialize = "public.app-category.board-games"))]
    BoardGames,
    #[serde(rename(serialize = "public.app-category.card-games"))]
    CardGames,
    #[serde(rename(serialize = "public.app-category.casino-games"))]
    CasinoGames,
    #[serde(rename(serialize = "public.app-category.dice-games"))]
    DiceGames,
    #[serde(rename(serialize = "public.app-category.educational-games"))]
    EducationalGames,
    #[serde(rename(serialize = "public.app-category.family-games"))]
    FamilyGames,
    #[serde(rename(serialize = "public.app-category.kids-games"))]
    KidsGames,
    #[serde(rename(serialize = "public.app-category.music-games"))]
    MusicGames,
    #[serde(rename(serialize = "public.app-category.puzzle-games"))]
    PuzzleGames,
    #[serde(rename(serialize = "public.app-category.racing-games"))]
    RacingGames,
    #[serde(rename(serialize = "public.app-category.role-playing-games"))]
    RolePlayingGames,
    #[serde(rename(serialize = "public.app-category.simulation-games"))]
    SimulationGames,
    #[serde(rename(serialize = "public.app-category.sports-games"))]
    SportsGames,
    #[serde(rename(serialize = "public.app-category.strategy-games"))]
    StrategyGames,
    #[serde(rename(serialize = "public.app-category.trivia-games"))]
    TriviaGames,
    #[serde(rename(serialize = "public.app-category.word-games"))]
    WordGames,
    #[serde(rename(serialize = "public.app-category.graphics-design"))]
    GraphicsDesign,
    #[serde(rename(serialize = "public.app-category.healthcare-fitness"))]
    HealthcareFitness,
    #[serde(rename(serialize = "public.app-category.lifestyle"))]
    Lifestyle,
    #[serde(rename(serialize = "public.app-category.medical"))]
    Medical,
    #[serde(rename(serialize = "public.app-category.music"))]
    Music,
    #[serde(rename(serialize = "public.app-category.news"))]
    News,
    #[serde(rename(serialize = "public.app-category.photography"))]
    Photography,
    #[serde(rename(serialize = "public.app-category.productivity"))]
    Productivity,
    #[serde(rename(serialize = "public.app-category.reference"))]
    Reference,
    #[serde(rename(serialize = "public.app-category.social-networking"))]
    SocialNetworking,
    #[serde(rename(serialize = "public.app-category.sports"))]
    Sports,
    #[serde(rename(serialize = "public.app-category.travel"))]
    Travel,
    #[serde(rename(serialize = "public.app-category.utilities"))]
    Utilities,
    #[serde(rename(serialize = "public.app-category.video"))]
    Video,
    #[serde(rename(serialize = "public.app-category.weather"))]
    Weather,
}

/// Operating System Version.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MinimumSystemVersionByArchitecture {
    pub i386: String,
    pub ppc: String,
    pub ppc64: String,
    pub x86_64: String,
}

impl Default for MinimumSystemVersionByArchitecture {
    fn default() -> Self {
        Self {
            i386: "10.0.0".to_owned(),
            ppc: "10.0.0".to_owned(),
            ppc64: "10.0.0".to_owned(),
            x86_64: "10.0.0".to_owned(),
        }
    }
}

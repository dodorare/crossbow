use serde::{Deserialize, Serialize};

/// Information property list.
/// https://developer.apple.com/documentation/bundleresources/information_property_list/bundle_configuration
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct InfoPlist {
    #[serde(flatten)]
    pub categorization: Categorization,
}

/// Categorization
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Categorization {
    /// The type of bundle.
    ///
    /// This key consists of a four-letter code for the bundle type.
    /// For apps, the code is APPL, for frameworks, it's FMWK, and for bundles,
    /// it's BNDL. The default value is derived from the bundle extension or,
    /// if it can't be derived, the default value is BNDL.
    #[serde(rename = "CFBundlePackageType", skip_serializing_if = "Option::is_none")]
    pub bundle_package_type: Option<String>,
    /// The category that best describes your app for the App Store.
    #[serde(flatten, default, rename = "LSApplicationCategoryType", skip_serializing_if = "Option::is_none")]
    pub application_category_type: Option<AppCategoryType>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum AppCategoryType {
    Business,
    DeveloperTools,
    Education,
    Entertainment,
    Finance,
    Games,
    ActionGames,
    AdventureGames,
    ArcadeGames,
    BoardGames,
    CardGames,
    CasinoGames,
    DiceGames,
    EducationalGames,
    FamilyGames,
    KidsGames,
    MusicGames,
    PuzzleGames,
    RacingGames,
    RolePlayingGames,
    SimulationGames,
    SportsGames,
    StrategyGames,
    TriviaGames,
    WordGames,
    GraphicsDesign,
    HealthcareFitness,
    Lifestyle,
    Medical,
    Music,
    News,
    Photography,
    Productivity,
    Reference,
    SocialNetworking,
    Sports,
    Travel,
    Utilities,
    Video,
    Weather,
}

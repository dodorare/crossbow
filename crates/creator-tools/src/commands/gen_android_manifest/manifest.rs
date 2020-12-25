use std::fmt;

#[derive(Debug)]
pub struct AndroidManifest {
    pub package_name: String,
    pub package_label: String,
    pub version_name: String,
    pub version_code: u32,
    pub target_name: String,
    pub target_sdk_version: u32,
    pub min_sdk_version: u32,
    pub opengles_version: (u8, u8),
    pub features: Vec<Feature>,
    pub permissions: Vec<Permission>,
    pub intent_filters: Vec<IntentFilter>,
    pub icon: Option<String>,
    pub fullscreen: bool,
    pub orientation: Option<String>,
    pub debuggable: bool,
    pub split: Option<String>,
    pub application_metadatas: Vec<ApplicationMetadata>,
    pub activity_metadatas: Vec<ActivityMetadata>,
}

impl fmt::Display for AndroidManifest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let split = if let Some(split) = self.split.as_ref() {
            format!(r#"split="{}" android:isFeatureSplit="true""#, split)
        } else {
            "".to_string()
        };
        let (major, minor) = self.opengles_version;
        let opengles_version = format!("0x{:04}{:04}", major, minor);

        let icon = self
            .icon
            .as_ref()
            .map(|icon| format!(r#"android:icon="{}""#, icon))
            .unwrap_or_default();

        let fullscreen = if self.fullscreen {
            r#"android:theme="@android:style/Theme.DeviceDefault.NoActionBar.Fullscreen""#
        } else {
            ""
        };

        let orientation = self.orientation.as_deref().unwrap_or("unspecified");

        let features: Vec<String> = self.features.iter().map(|f| f.to_string()).collect();
        let permissions: Vec<String> = self.permissions.iter().map(|p| p.to_string()).collect();
        let intent_filters: Vec<String> =
            self.intent_filters.iter().map(|i| i.to_string()).collect();
        let application_metadatas: Vec<String> = self
            .application_metadatas
            .iter()
            .map(|f| f.to_string())
            .collect();
        let activity_metadatas: Vec<String> = self
            .activity_metadatas
            .iter()
            .map(|f| f.to_string())
            .collect();

        fmt::write(
            f,
            format_args!(
                r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
        package="{package_name}"
        android:versionCode="{version_code}"
        android:versionName="{version_name}"
        {split}>
    <uses-sdk
        android:targetSdkVersion="{target_sdk_version}"
        android:minSdkVersion="{min_sdk_version}" />
    <uses-feature android:glEsVersion="{opengles_version}" android:required="true"></uses-feature>
    {features}
    {permissions}
    <application
            android:hasCode="false"
            android:label="{package_label}"
            android:debuggable="{debuggable}"
            {icon}
            {fullscreen}>
            {application_metadatas}
        <activity
                android:name="android.app.NativeActivity"
                android:label="{package_label}"
                android:screenOrientation="{orientation}"
                android:configChanges="orientation|keyboardHidden|screenSize">
            <meta-data android:name="android.app.lib_name" android:value="{target_name}" />
            {activity_metadatas}
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
            {intent_filters}
        </activity>
    </application>
</manifest>"#,
                package_name = &self.package_name,
                package_label = &self.package_label,
                version_name = &self.version_name,
                version_code = self.version_code,
                split = split,
                target_sdk_version = self.target_sdk_version,
                min_sdk_version = self.min_sdk_version,
                opengles_version = opengles_version,
                target_name = &self.target_name,
                icon = icon,
                fullscreen = fullscreen,
                orientation = orientation,
                application_metadatas = application_metadatas.join("\n"),
                activity_metadatas = activity_metadatas.join("\n"),
                debuggable = self.debuggable,
                features = features.join("\n"),
                permissions = permissions.join("\n"),
                intent_filters = intent_filters.join("\n"),
            ),
        )
    }
}

#[derive(Debug)]
pub struct Feature {
    pub name: String,
    pub required: bool,
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::write(
            f,
            format_args!(
                r#"<uses-feature android:name="{}" android:required="{}"/>"#,
                &self.name, self.required,
            ),
        )
    }
}

#[derive(Debug)]
pub struct ApplicationMetadata {
    pub name: String,
    pub value: String,
}

impl fmt::Display for ApplicationMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::write(
            f,
            format_args!(
                r#"<meta-data android:name="{}" android:value="{}"/>"#,
                self.name, self.value
            ),
        )
    }
}

#[derive(Debug)]
pub struct ActivityMetadata {
    pub name: String,
    pub value: String,
}

impl fmt::Display for ActivityMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::write(
            f,
            format_args!(
                r#"<meta-data android:name="{}" android:value="{}"/>"#,
                self.name, self.value
            ),
        )
    }
}

#[derive(Debug)]
pub struct Permission {
    pub name: String,
    pub max_sdk_version: Option<u32>,
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_sdk_version = self
            .max_sdk_version
            .as_ref()
            .map(|max_sdk_version| format!(r#"android:maxSdkVersion="{}""#, max_sdk_version))
            .unwrap_or_default();
        fmt::write(
            f,
            format_args!(
                r#"<uses-permission android:name="{}" {}/>"#,
                &self.name, max_sdk_version,
            ),
        )
    }
}

#[derive(Debug)]
pub struct IntentFilterData {
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub prefix: Option<String>,
}

impl fmt::Display for IntentFilterData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let host = if let Some(host) = self.host.as_ref() {
            format!(" android:host=\"{}\"", host)
        } else {
            "".into()
        };
        let prefix = if let Some(prefix) = self.prefix.as_ref() {
            format!(" android:pathPrefix=\"{}\"", prefix)
        } else {
            "".into()
        };
        let scheme = if let Some(scheme) = self.scheme.as_ref() {
            format!(" android:scheme=\"{}\"", scheme)
        } else {
            "".into()
        };
        fmt::write(f, format_args!("<data {} {} {}/>", scheme, &host, &prefix))
    }
}

#[derive(Debug)]
pub struct IntentFilter {
    pub name: String,
    pub categories: Vec<String>,
    pub data: Vec<IntentFilterData>,
}

impl fmt::Display for IntentFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut categories = "".to_string();
        for category in &self.categories {
            categories = format!("{}<category android:name=\"{}\"/>", categories, category)
        }
        let mut data = "".to_string();
        for d in &self.data {
            data = format!("{}{}", data, d.to_string())
        }
        fmt::write(
            f,
            format_args!(
                "<intent-filter>
                \t{}
                \t{}
                \t<action android:name=\"{}\"/>
                </intent-filter>",
                &categories, &data, &self.name,
            ),
        )
    }
}

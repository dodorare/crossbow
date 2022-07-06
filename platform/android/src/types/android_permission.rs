/// Android Permissions
///
/// See for more details: https://developer.android.com/reference/android/Manifest.permission
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidPermission {
    /// Allows a calling app to continue a call which was started in another app.
    ///
    /// Protection level: dangerous
    AcceptHandover,
    /// Allows an app to access location in the background.
    ///
    /// Protection level: dangerous
    AccessBackgroundLocation,
    /// Allows an application to access data blobs across users.
    AccessBlobsAcrossUsers,
    /// Allows read/write access to the "properties" table in the checkin database,
    /// to change values that get uploaded.
    ///
    /// Not for use by third-party applications.
    AccessCheckinProperties,
    /// Allows an app to access approximate location.
    ///
    /// Protection level: dangerous
    AccessCoarseLocation,
    /// Allows an app to access precise location.
    ///
    /// Protection level: dangerous
    AccessFineLocation,
    /// Allows an application to access extra location provider commands.
    ///
    /// Protection level: normal
    AccessLocationExtraCommands,
    /// Allows an application to access any geographic locations persisted in the
    /// user's shared collection.
    ///
    /// Protection level: dangerous
    AccessMediaLocation,
    /// Allows applications to access information about networks.
    ///
    /// Protection level: normal
    AccessNetworkState,
    /// Marker permission for applications that wish to access notification policy.
    ///
    /// Protection level: normal
    AccessNotificationPolicy,
    /// Allows an application to access SupplementalApis
    ///
    /// Protection level: normal
    AccessSupplementalApis,
    /// Allows applications to access information about Wi-Fi networks.
    ///
    /// Protection level: normal
    AccessWifiState,
    /// Allows applications to call into AccountAuthenticators.
    /// Not for use by third-party applications.
    AccountManager,
    /// Allows an application to recognize physical activity.
    ///
    /// Protection level: dangerous
    ActivityRecognition,
    /// Allows an application to add voicemails into the system.
    ///
    /// Protection level: dangerous
    AddVoicemail,
    /// Allows the app to answer an incoming phone call.
    ///
    /// Protection level: dangerous
    AnswerPhoneCalls,
    /// Allows an application to collect battery statistics.
    ///
    /// Protection level: signature|privileged|development
    BattertStats,
    /// Must be required by an [AccessibilityService], to ensure that only the
    /// system can bind to it.
    ///
    /// Protection level: signature
    ///
    /// [AccessibilityService]: https://developer.android.com/reference/android/accessibilityservice/AccessibilityService
    BindAccessibilityService,
    /// Allows an application to tell the AppWidget service which application can
    /// access AppWidget's data. The normal user flow is that a user picks an
    /// AppWidget to go into a particular host, thereby giving that host
    /// application access to the private data from the AppWidget app. An
    /// application that has this permission should honor that contract.
    ///
    /// Not for use by third-party applications.
    BindAppwidget,
    /// Must be required by a [AutofillService], to ensure that only the system
    /// can bind to it.
    ///
    /// Protection level: signature
    ///
    /// [AutofillService]: https://developer.android.com/reference/android/service/autofill/AutofillService
    BindAutofillService,
    /// Must be required by a [CallRedirectionService], to ensure that only the
    /// system can bind to it.
    ///
    /// Protection level: signature|privileged
    ///
    /// [CallRedirectionService]: https://developer.android.com/reference/android/telecom/CallRedirectionService
    BindCallRedirectionService,
    /// A subclass of [CarrierMessagingClientService] must be protected with
    /// this permission.
    ///
    /// Protection level: signature
    ///
    /// [CarrierMessagingClientService]: https://developer.android.com/reference/android/service/carrier/CarrierMessagingClientService
    BindCarrierMessagingClientService,
    /// This constant was deprecated in API level 23. Use [BIND_CARRIER_SERVICES]
    /// instead.
    ///
    /// [BIND_CARRIER_SERVICES]: https://developer.android.com/reference/android/Manifest.permission#BIND_CARRIER_SERVICES
    BindCarrierMessagingService,
    /// The system process that is allowed to bind to services in carrier apps
    /// will have this permission.
    ///
    /// Protection level: signature|privileged
    BindCarrierServices,
    /// This constant was deprecated in API level 30. For publishing direct share
    /// targets, please follow the instructions in
    /// https://developer.android.com/training/sharing/receive.html#providing-direct-share-targets instead
    ///
    /// Protection level: signature
    BindChooserTargetService,
    /// Must be required by any [CompanionDeviceServices] to ensure that only the
    /// system can bind to it.
    ///
    /// [CompanionDeviceServices]: https://developer.android.com/reference/android/companion/CompanionDeviceService
    BindCompanionDeviceService,
    /// Must be required by a [ConditionProviderService], to ensure that only the
    /// system can bind to it.
    ///
    /// Protection level: signature
    ///
    /// [ConditionProviderService]: https://developer.android.com/reference/android/service/notification/ConditionProviderServic
    BindConditionProviderService,
    /// Allows SystemUI to request third party controls. Should only be requested
    /// by the System and required by [ControlsProviderService] declarations.
    ///
    /// [ControlsProviderService]: https://developer.android.com/reference/android/service/controls/ControlsProviderService
    BindControls,
    /// Must be required by device administration receiver, to ensure that only the
    /// system can interact with it.
    ///
    /// Protection level: signature
    BindDeviceAdmin,
    /// Must be required by an [DreamService], to ensure that only the system can
    /// bind to it.
    ///
    /// Protection level: signature
    ///
    /// [DreamService]: https://developer.android.com/reference/android/service/dreams/DreamService
    BindDreamService,
    /// Must be required by a [InCallService], to ensure that only the system can
    /// bind to it.
    ///
    /// Protection level: signature|privileged
    ///
    /// [InCallService]: https://developer.android.com/reference/android/telecom/InCallService
    BindIncallService,
    /// Must be required by an [InputMethodService], to ensure that only the system
    /// can bind to it.
    ///
    /// Protection level: signature
    ///
    /// [InputMethodService]: https://developer.android.com/reference/android/inputmethodservice/InputMethodService
    BindInputMethod,
    /// Must be required by an [MidiDeviceService], to ensure that only the system
    /// can bind to it.
    ///
    /// Protection level: signature
    ///
    /// [MidiDeviceService]: https://developer.android.com/reference/android/media/midi/MidiDeviceService
    BindMidiDeviceService,
    /// Must be required by a [HostApduService] or [OffHostApduService] to ensure
    /// that only the system can bind to it.
    ///
    /// Protection level: signature
    ///
    /// [HostApduService]: https://developer.android.com/reference/android/nfc/cardemulation/HostApduService
    /// [OffHostApduService]: https://developer.android.com/reference/android/nfc/cardemulation/OffHostApduService
    BindNfcService,
    /// Must be required by an [NotificationListenerService], to ensure that only
    /// the system can bind to it.
    ///
    /// Protection level: signature
    ///
    /// [NotificationListenerService]: https://developer.android.com/reference/android/service/notification/NotificationListenerService
    BindNotificationListenerService,
    /// Must be required by a [PrintService], to ensure that only the system can
    /// bind to it.
    ///
    /// Protection level: signature
    ///
    /// [PrintService]: https://developer.android.com/reference/android/printservice/PrintService
    BindPrintService,
    /// Must be required by a [QuickAccessWalletService] to ensure that only the
    /// system can bind to it.
    ///
    /// Protection level: signature
    ///
    /// [QuickAccessWalletService]: https://developer.android.com/reference/android/service/quickaccesswallet/QuickAccessWalletService
    BindQuickAccessWalletService,
    /// Allows an application to bind to third party quick settings tiles.
    ///
    /// Should only be requested by the System, should be required by TileService
    /// declarations.
    BindQuickSettingsTile,
    /// Must be required by a [RemoteViewsService], to ensure that only the system
    /// can bind to it.
    ///
    /// Protection level: signature|privileged
    ///
    /// [RemoteViewsService]: https://developer.android.com/reference/android/widget/RemoteViewsService
    BindRrmoteviews,
    /// Must be required by a [CallScreeningService], to ensure that only the system
    /// can bind to it.
    ///
    /// Protection level: signature|privileged
    ///
    /// [CallScreeningService]: https://developer.android.com/reference/android/telecom/CallScreeningService
    BindScreeningService,
    /// Must be required by a [ConnectionService], to ensure that only the system can
    /// bind to it.
    ///
    /// Protection level: signature|privileged
    ///
    /// [ConnectionService]: https://developer.android.com/reference/android/telecom/ConnectionService
    BindTelecomConnectionService,
    /// Must be required by a TextService (e.g. SpellCheckerService) to ensure that
    /// only the system can bind to it.
    ///
    /// Protection level: signature
    BindTextService,
    /// Must be required by a [TvInputService] to ensure that only the system can
    /// bind to it.
    ///
    /// Protection level: signature|privileged
    ///
    /// [TvInputService]: https://developer.android.com/reference/android/media/tv/TvInputService
    BindTvInput,
    /// Must be required by a TvInteractiveAppService to ensure that only the system can bind to it.
    ///
    /// Protection level: signature|privileged
    BindTvInteractiveApp,
    /// Must be required by a link [VisualVoicemailService] to ensure that only the
    /// system can bind to it.
    ///
    /// [VisualVoicemailService]: https://developer.android.com/reference/android/telephony/VisualVoicemailService
    BindVisualVoicemailService,
    /// Must be required by a [VoiceInteractionService], to ensure that only the
    /// system can bind to it.
    ///
    /// Protection level: signature|privileged
    ///
    /// [VoiceInteractionService]: https://developer.android.com/reference/android/service/voice/VoiceInteractionService
    BindVoiceInteraction,
    /// Must be required by a [VpnService], to ensure that only the system can bind
    /// to it.
    ///
    /// Protection level: signature
    ///
    /// [VpnService]: https://developer.android.com/reference/android/net/VpnService
    BindVpnService,
    /// Must be required by an [VrListenerService], to ensure that only the system
    /// can bind to it.
    ///
    /// Protection level: signature
    ///
    /// [VrListenerService]: https://developer.android.com/reference/android/service/vr/VrListenerService
    BindVrListenerService,
    /// Must be required by a [WallpaperService], to ensure that only the system can
    /// bind to it.
    ///
    /// Protection level: signature|privileged
    ///
    /// [WallpaperService]: https://developer.android.com/reference/android/service/wallpaper/WallpaperService
    BindWallpaper,
    /// Allows applications to connect to paired bluetooth devices.
    ///
    /// Protection level: normal
    Bluetooth,
    /// Allows applications to discover and pair bluetooth devices.
    ///
    /// Protection level: normal
    BluetoothAdmin,
    /// Required to be able to advertise to nearby Bluetooth devices.
    ///
    /// Protection level: dangerous
    BluetoothAdvertise,
    /// Required to be able to connect to paired Bluetooth devices.
    ///
    /// Protection level: dangerous
    BluetoothConnect,
    /// Allows applications to pair bluetooth devices without user interaction, and to
    /// allow or disallow phonebook access or message access.
    ///
    /// Not for use by third-party applications.
    BluetoothPrivileged,
    /// Required to be able to discover and pair nearby Bluetooth devices.
    ///
    /// Protection level: dangerous
    BluetoothScan,
    /// Allows an application to access data from sensors that the user uses to measure
    /// what is happening inside their body, such as heart rate.
    ///
    /// Protection level: dangerous
    BodySensors,
    /// Allows an application to access data from sensors that the user uses to measure what is happening inside their body, such as heart rate. If you're requesting this permission, you must also request BODY_SENSORS. Requesting this permission by itself doesn't give you Body sensors access.
    ///
    /// Protection level: dangerous
    BodySensorsBackground,
    /// Allows an application to broadcast a notification that an application package
    /// has been removed.
    ///
    /// Not for use by third-party applications.
    BroadcastPackageRemoved,
    /// Allows an application to broadcast an SMS receipt notification.
    ///
    /// Not for use by third-party applications.
    BroadcastSMS,
    /// Allows an application to broadcast sticky intents.
    ///
    /// Protection level: normal
    BroadcastSticky,
    /// Allows an application to broadcast a WAP PUSH receipt notification.
    ///
    /// Not for use by third-party applications.
    BroadcastWapPush,
    /// Allows an app which implements the [InCallService] API to be eligible to be
    /// enabled as a calling companion app.  This means that the Telecom framework will
    /// bind to the app's InCallService implementation when there are calls active. The
    /// app can use the InCallService API to view information about calls on the system
    /// and control these calls.
    ///
    /// Protection level: normal
    ///
    /// [InCallService]: https://developer.android.com/reference/android/telecom/InCallService
    CallCompanionApp,
    /// Allows an application to initiate a phone call without going through the Dialer
    /// user interface for the user to confirm the call.
    ///
    /// Protection level: dangerous
    CallPhone,
    /// Allows an application to call any phone number, including emergency numbers,
    /// without going through the Dialer user interface for the user to confirm the
    /// call being placed.
    ///
    /// Not for use by third-party applications.
    CallPrivileged,
    /// Required to be able to access the camera device.
    ///
    /// Protection level: dangerous
    Camera,
    /// Allows an application to capture audio output. Use the `CAPTURE_MEDIA_OUTPU`
    /// permission if only the `USAGE_UNKNOWN`), `USAGE_MEDIA`) or `USAGE_GAME`) usages
    /// are intended to be captured.
    ///
    /// Not for use by third-party applications.
    CaptureAudioOutput,
    /// Allows an application to change whether an application component
    /// (other than its own) is enabled or not.
    ///
    /// Not for use by third-party applications.
    ChangeComponentEnabledState,
    /// Allows an application to modify the current configuration, such as locale.
    ///
    /// Protection level: signature|privileged|development
    ChangeConfiguration,
    /// Allows applications to change network connectivity state.
    ///
    /// Protection level: normal
    ChangeNetworkState,
    /// Allows applications to enter Wi-Fi Multicast mode.
    ///
    /// Protection level: normal
    ChangeWifiMulticastState,
    /// Allows applications to change Wi-Fi connectivity state.
    ///
    /// Protection level: normal
    ChangeWifiState,
    /// Allows an application to clear the caches of all installed applications on
    /// the device.
    ///
    /// Protection level: signature|privileged
    ClearAppCache,
    /// Allows enabling/disabling location update notifications from the radio.
    ///
    /// Not for use by third-party applications.
    ControlLocationUpdates,
    /// Old permission for deleting an app's cache files, no longer used, but signals
    /// for us to quietly ignore calls instead of throwing an exception.
    ///
    /// Protection level: signature|privileged
    DeleteCacheFiles,
    /// Allows an application to delete packages.
    ///
    /// Not for use by third-party applications.
    DeletePackages,
    /// Allows an application to deliver companion messages to system.
    DeliverCompanionMessages,
    /// Allows applications to RW to diagnostic resources.
    ///
    /// Not for use by third-party applications.
    Diagnostic,
    /// Allows applications to disable the keyguard if it is not secure.
    ///
    /// Protection level: normal
    DisableKeyguard,
    /// Allows an application to retrieve state dump information from system services.
    ///
    /// Not for use by third-party applications.
    Dump,
    /// Allows an application to expand or collapse the status bar.
    ///
    /// Protection level: normal
    ExpandStatusBar,
    /// Run as a manufacturer test application, running as the root user.
    ///
    /// Not for use by third-party applications.
    FactoryTest,
    /// Allows a regular application to use [Service.startForeground].
    ///
    /// Protection level: normal
    ///
    /// [Service.startForeground]: https://developer.android.com/reference/android/app/Service#startForeground(int,%20android.app.Notification)
    ForegroundService,
    /// Allows access to the list of accounts in the Accounts Service.
    ///
    /// ## Note
    /// Beginning with Android 6.0 (API level 23), if an app shares the signature
    /// of the authenticator that manages an account, it does not need `"GET_ACCOUNTS"`
    /// permission to read information about that account. On Android 5.1 and lower,
    /// all apps need `"GET_ACCOUNTS"` permission to read information about any account.
    ///
    /// Protection level: dangerous
    GetAccounts,
    /// Allows access to the list of accounts in the Accounts Service.
    ///
    /// Protection level: signature|privileged
    GetAccountsPrivileged,
    /// Allows an application to find out the space used by any package.
    ///
    /// Protection level: normal
    GetPackageSize,
    /// This constant was deprecated in API level 21. No longer enforced.
    GetTasks,
    /// This permission can be used on content providers to allow the global search
    /// system to access their data. Typically it used when the provider has some
    /// permissions protecting it (which global search would not be expected to hold),
    /// and added as a read-only permission to the path in the provider where global
    /// search queries are performed. This permission can not be held by regular
    /// applications; it is used by applications to protect themselves from everyone
    /// else besides global search.
    ///
    /// Protection level: signature|privileged
    GlobalSearch,
    /// Allows an app to prevent non-system-overlay windows from being drawn on top
    /// of it
    HighOverlayWindows,
    /// Allows an app to access sensor data with a sampling rate greater than 200 Hz.
    ///
    /// Protection level: normal
    HighSamplingRateSensors,
    /// Allows an application to install a location provider into the Location Manager.
    ///
    /// Not for use by third-party applications.
    InstallLocationProvider,
    /// Allows an application to install packages.
    ///
    /// Not for use by third-party applications.
    InstallPackages,
    /// Allows an application to install a shortcut in Launcher.
    ///
    /// In Android O (API level 26) and higher, the INSTALL_SHORTCUT broadcast no
    /// longer has any effect on your app because it's a private, implicit broadcast.
    /// Instead, you should create an app shortcut by using the requestPinShortcut()
    /// method from the ShortcutManager class.
    ///
    /// Protection level: normal
    InstallShortcut,
    /// Allows an instant app to create foreground services.
    ///
    /// Protection level: signature|development|instant|appop
    InstantAppForegroundService,
    /// Allows interaction across profiles in the same profile group.
    InteractAcrossProfiles,
    /// Allows applications to open network sockets.
    ///
    /// Protection level: normal
    Internet,
    /// Allows an application to call [ActivityManager.killBackgroundProcesses(String)].
    ///
    /// Protection level: normal
    ///
    /// [ActivityManager.killBackgroundProcesses(String)]: https://developer.android.com/reference/android/app/ActivityManager#killBackgroundProcesses(java.lang.String)
    KillBackgroundProcesses,
    /// An application needs this permission for
    /// [Settings.ACTION_SETTINGS_EMBED_DEEP_LINK_ACTIVITY] to show its [Activity]
    /// embedded in Settings app.
    ///
    /// [Settings.ACTION_SETTINGS_EMBED_DEEP_LINK_ACTIVITY]: https://developer.android.com/reference/android/provider/Settings#ACTION_SETTINGS_EMBED_DEEP_LINK_ACTIVITY
    /// [Activity]: https://developer.android.com/reference/android/app/Activity
    LaunchMultiPaneSettingsDeepLink,
    /// Allows a data loader to read a package's access logs. The access logs contain
    /// the set of pages referenced over time.
    ///
    /// Declaring the permission implies intention to use the API and the user of the
    /// device can grant permission through the Settings application.
    ///
    /// Protection level: signature|privileged|appop
    ///
    /// A data loader has to be the one which provides data to install an app.
    ///
    /// A data loader has to have both permission:LOADER_USAGE_STATS AND
    /// appop:LOADER_USAGE_STATS allowed to be able to access the read logs.
    LoaderUsageStats,
    /// Allows an application to use location features in hardware, such as the
    /// geofencing api.
    ///
    /// Not for use by third-party applications.
    LocationHardware,
    /// Allows an application to manage access to documents, usually as part of a
    /// document picker.
    ///
    /// This permission should only be requested by the platform document management
    /// app. This permission cannot be granted to third-party apps.
    ManageDocuments,
    /// Allows an application a broad access to external storage in scoped storage.
    /// Intended to be used by few apps that need to manage files on behalf of the users.
    ///
    /// protection level: signature|appop|preinstalled
    ManageExternalStorage,
    /// Allows an application to modify and delete media files on this device or any
    /// connected storage device without user confirmation.  Applications must already be
    /// granted the READ_EXTERNAL_STORAGE or MANAGE_EXTERNAL_STORAGE} permissions for this
    /// permission to take effect.
    ///
    /// Even if applications are granted this permission, if applications want to modify or
    /// delete media files, they also must get the access by calling
    /// MediaStore.createWriteRequest(ContentResolver, Collection),
    /// MediaStore.createDeleteRequest(ContentResolver, Collection), or
    /// MediaStore.createTrashRequest(ContentResolver, Collection, boolean).
    ///
    /// This permission doesn't give read or write access directly. It only prevents the
    /// user confirmation dialog for these requests.
    ///
    /// If applications are not granted ACCESS_MEDIA_LOCATION, the system also pops up the
    /// user confirmation dialog for the write request.
    ///
    /// Protection level: signature|appop|preinstalled
    ManageMedia,
    /// Allows to query ongoing call details and manage ongoing calls.
    ///
    /// Protection level: signature|appop
    ManageOngoingCalls,
    /// Allows a calling application which manages its own calls through the
    /// self-managed [ConnectionService] APIs.
    ///
    /// [ConnectionService]: https://developer.android.com/reference/android/telecom/ConnectionService
    ManageOwnCalls,
    /// Allows applications to enable/disable wifi auto join. This permission is used to let OEMs grant their trusted app access to a subset of privileged wifi APIs to improve wifi performance.
    /// Not for use by third-party applications.
    ManageWifiAutoJoin,
    /// Allows applications to get notified when a Wi-Fi interface request cannot be satisfied without tearing down one or more other interfaces, and provide a decision whether to approve the request or reject it.
    /// Not for use by third-party applications.
    ManageWifiInterfaces,
    /// Not for use by third-party applications.
    MasterClear,
    /// Allows an application to know what content is playing and control its
    /// playback.
    ///
    /// Not for use by third-party applications due to privacy of media consumption
    MediaContentControl,
    /// Allows an application to modify global audio settings.
    ///
    /// Protection level: normal
    ModifyAudioSettings,
    /// Allows modification of the telephony state - power on, mmi, etc. Does not
    /// include placing calls.
    ///
    /// Not for use by third-party applications.
    ModifyPhoneState,
    /// Allows formatting file systems for removable storage.
    ///
    /// Not for use by third-party applications.
    MountFormatFilesystems,
    /// Allows mounting and unmounting file systems for removable storage.
    ///
    /// Not for use by third-party applications.
    MountUnmountFilesystems,
    /// Required to be able to advertise and connect to nearby devices via Wi-Fi.
    ///
    /// Protection level: dangerous
    NearbyWifiDevices,
    /// Allows applications to perform I/O operations over NFC.
    ///
    /// Protection level: normal
    Nfc,
    /// Allows applications to receive NFC preferred payment service information.
    ///
    /// Protection level: normal
    NfcPreferredPatmentInfo,
    /// Allows applications to receive NFC transaction events.
    ///
    /// Protection level: normal
    NfcTransactionEvent,
    /// Allows an application to modify any wifi configuration, even if created by another application. Once reconfigured the original creator cannot make any further modifications.
    /// Not for use by third-party applications.
    OverrideWifiConfig,
    /// Allows an application to collect component usage statistics.
    ///
    /// Declaring the permission implies intention to use the API and the user of
    /// the device can grant permission through the Settings application.
    ///
    /// Protection level: signature|privileged|development|appop|retailDemo
    PackageUsageStats,
    /// This constant was deprecated in API level 15. This functionality will be
    /// removed in the future; please do not use. Allow an application to make
    /// its activities persistent.
    PersistentActivity,
    /// Allows an app to post notifications.
    ///
    /// Allows an app to post notifications
    PostNotifications,
    /// This constant was deprecated in API level 29. Applications should use
    /// [CallRedirectionService] instead of the [Intent.ACTION_NEW_OUTGOING_CALL]
    /// broadcast.
    ///
    /// Protection level: dangerous
    ///
    /// [CallRedirectionService]: https://developer.android.com/reference/android/telecom/CallRedirectionService
    /// [Intent.ACTION_NEW_OUTGOING_CALL]: https://developer.android.com/reference/android/content/Intent#ACTION_NEW_OUTGOING_CALL
    ProcessOutgoingCalls,
    /// Allows query of any normal app on the device, regardless of manifest
    /// declarations.
    ///
    /// Protection level: normal
    QueryAllPackages,
    /// Allows an application to query over global data in AppSearch that's visible to the ASSISTANT role.
    ReadAssistantAppSearchData,
    /// Allows read only access to phone state with a non dangerous permission, including the information like cellular network type, software version.
    ReadBasicPhoneState,
    /// Allows an application to read the user's calendar data.
    ///
    /// Protection level: dangerous
    ReadCalendar,
    /// Allows an application to read the user's call log.
    ///
    /// ## Note
    /// If your app uses the READ_CONTACTS permission and both your minSdkVersion
    /// and targetSdkVersion values are set to 15 or lower, the system implicitly
    /// grants your app this permission. If you don't need this permission, be
    /// sure your targetSdkVersion is 16 or higher.
    ///
    /// Protection level: dangerous
    ///
    /// This is a hard restricted permission which cannot be held by an app until
    /// the installer on record whitelists the permission. For more details see
    /// PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set).
    ReadCallLog,
    /// Allows an application to read the user's contacts data.
    ///
    /// Protection level: dangerous
    ReadContacts,
    /// Allows an application to read from external storage.
    ///
    /// Any app that declares the WRITE_EXTERNAL_STORAGE permission is implicitly
    /// granted this permission.
    ///
    /// This permission is enforced starting in API level 19. Before API level 19,
    /// this permission is not enforced and all apps still have access to read
    /// from external storage. You can test your app with the permission enforced
    /// by enabling Protect USB storage under Developer options in the Settings
    /// app on a device running Android 4.1 or higher.
    ///
    /// Also starting in API level 19, this permission is not required to
    /// read/write files in your application-specific directories returned by
    /// Context.getExternalFilesDir(String) and Context.getExternalCacheDir().
    ///
    /// This is a soft restricted permission which cannot be held by an app it its
    /// full form until the installer on record whitelists the permission.
    /// Specifically, if the permission is allowlisted the holder app can access
    /// external storage and the visual and aural media collections while if the
    /// permission is not allowlisted the holder app can only access to the visual
    /// and aural medial collections. Also the permission is immutably restricted
    /// meaning that the allowlist state can be specified only at install time and
    /// cannot change until the app is installed. For more details see
    /// PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set).
    ///
    /// Protection level: dangerous
    ///
    /// ## Note
    /// If both your minSdkVersion and targetSdkVersion values are set to 3 or
    /// lower, the system implicitly grants your app this permission. If you don't
    /// need this permission, be sure your targetSdkVersion is 4 or higher.
    ReadExternalStorage,
    /// This constant was deprecated in API level 16. The API that used this
    /// permission has been removed.
    ///
    /// Not for use by third-party applications.
    ReadInputState,
    /// Allows an application to read the low-level system log files.
    ///
    /// Not for use by third-party applications, because Log entries can contain
    /// the user's private information.
    ReadLogs,
    /// Allows an application to read audio files from external storage.
    ///
    /// This permission is enforced starting in API level Build.VERSION_CODES.TIRAMISU. For apps with a targetSdkVersion of Build.VERSION_CODES.S or lower, this permission must not be used and the READ_EXTERNAL_STORAGE permission must be used instead.
    ///
    /// Protection level: dangerous
    ReadMediaAudio,
    /// Allows an application to read image files from external storage.
    ///
    /// This permission is enforced starting in API level Build.VERSION_CODES.TIRAMISU. For apps with a targetSdkVersion of Build.VERSION_CODES.S or lower, this permission must not be used and the READ_EXTERNAL_STORAGE permission must be used instead.
    ///
    /// Protection level: dangerous
    ReadMediaImage,
    /// Allows an application to read audio files from external storage.
    ///
    /// This permission is enforced starting in API level Build.VERSION_CODES.TIRAMISU. For apps with a targetSdkVersion of Build.VERSION_CODES.S or lower, this permission must not be used and the READ_EXTERNAL_STORAGE permission must be used instead.
    ///
    /// Protection level: dangerous
    ReadMediaVideo,
    /// Allows an application to read nearby streaming policy. The policy controls whether to allow the device to stream its notifications and apps to nearby devices. Applications that are not the device owner will need this permission to call DevicePolicyManager.getNearbyNotificationStreamingPolicy() or DevicePolicyManager.getNearbyAppStreamingPolicy().
    ReadNearbyStreamingPolicy,
    /// Allows read access to the device's phone number(s). This is a subset of
    /// the capabilities granted by READ_PHONE_STATE but is exposed to instant
    /// applications.
    ///
    /// Protection level: dangerous
    ReadPhoneNumbers,
    /// Allows read only access to phone state, including the current cellular
    /// network information, the status of any ongoing calls, and a list of
    /// any [PhoneAccounts] registered on the device.
    ///
    /// Protection level: dangerous
    ///
    /// ## Note
    /// If both your minSdkVersion and targetSdkVersion values are set to 3 or
    /// lower, the system implicitly grants your app this permission. If you don't
    /// need this permission, be sure your targetSdkVersion is 4 or higher.
    ///
    /// [PhoneAccounts]: https://developer.android.com/reference/android/telecom/PhoneAccount
    ReadPhoneState,
    /// Allows read only access to precise phone state. Allows reading of detailed
    /// information about phone state for special-use applications such as dialers,
    /// carrier applications, or ims applications.
    ReadPrecisePhoneState,
    /// Allows an application to read SMS messages.
    ///
    /// Protection level: dangerous
    ///
    /// This is a hard restricted permission which cannot be held by an app until
    /// the installer on record whitelists the permission. For more details see
    /// PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set)
    ReadSMS,
    /// Allows applications to read the sync settings.
    ///
    /// Protection level: normal
    ReadSyncSettings,
    /// Allows applications to read the sync stats.
    ///
    /// Protection level: normal
    ReadSyncStats,
    /// Allows an application to read voicemails in the system.
    ///
    /// Protection level: signature|privileged|role
    ReadVoicemail,
    /// Required to be able to reboot the device.
    ///
    /// Not for use by third-party applications.
    Reboot,
    /// Allows an application to receive the [Intent.ACTION_BOOT_COMPLETED]
    /// that is broadcast after the system finishes booting. If you don't
    /// request this permission, you will not receive the broadcast at that time.
    /// Though holding this permission does not have any security implications,
    /// it can have a negative impact on the user experience by increasing the
    /// amount of time it takes the system to start and allowing applications to
    /// have themselves running without the user being aware of them. As such,
    /// you must explicitly declare your use of this facility to make that
    /// visible to the user.
    ///
    /// Protection level: normal
    ///
    /// [Intent.ACTION_BOOT_COMPLETED]: https://developer.android.com/reference/android/content/Intent#ACTION_BOOT_COMPLETED
    ReceiveBootCompleted,
    /// Allows an application to monitor incoming MMS messages.
    ///
    /// Protection level: dangerous
    ///
    /// This is a hard restricted permission which cannot be held by an app
    /// until the installer on record whitelists the permission. For more
    /// details see
    /// PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set).
    ReceiveMMS,
    /// Allows an application to receive SMS messages.
    ///
    /// Protection level: dangerous
    ///
    /// This is a hard restricted permission which cannot be held by an app
    /// until the installer on record whitelists the permission. For more
    /// details see
    /// PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set).
    ReceiveSMS,
    /// Allows an application to receive WAP push messages.
    ///
    /// Protection level: dangerous
    ReceiveWapPush,
    /// Allows an application to record audio.
    ///
    /// Protection level: dangerous
    RecordAudio,
    /// Allows an application to change the Z-order of tasks.
    ///
    /// Protection level: normal
    ReorderTasks,
    /// Allows an application to read nearby streaming policy. The policy controls whether to allow the device to stream its notifications and apps to nearby devices. Applications that are not the device owner will need this permission to call DevicePolicyManager.getNearbyNotificationStreamingPolicy() or DevicePolicyManager.getNearbyAppStreamingPolicy().
    ///
    /// Not for use by third-party applications.
    RequestCompanionProfileAppStreaming,
    /// Allows application to request to be associated with a vehicle head unit capable of automotive projection (AssociationRequest.DEVICE_PROFILE_AUTOMOTIVE_PROJECTION) by CompanionDeviceManager.
    ///
    /// Not for use by third-party applications.
    RequestCompanionProfileAutomotiveProjection,
    /// Allows application to request to be associated with a computer to share functionality and/or data with other devices, such as notifications, photos and media (AssociationRequest.DEVICE_PROFILE_COMPUTER) by CompanionDeviceManager.
    ///
    /// Not for use by third-party applications.
    RequestCompanionProfileComputer,
    /// Allows app to request to be associated with a device via
    /// CompanionDeviceManager as a "watch".
    ///
    /// Protection level: normal
    RequestCompanionProfileWatch,
    /// Allows a companion app to run in the background.
    ///
    /// Protection level: normal
    RequestCompanionRunInBackground,
    /// Allows an application to create a "self-managed" association.
    RequestCompanionSelfManaged,
    /// Allows a companion app to start a foreground service from the background.
    ///
    /// Protection level: normal
    RequestCompanionStartForegroundServicesFromBackground,
    /// Allows a companion app to use data in the background.
    ///
    /// Protection level: normal
    RequestCompanionUseDataInBackground,
    /// Allows an application to request deleting packages.
    ///
    /// Protection level: normal
    RequestDeletePackages,
    /// Permission an application must hold in order to use
    /// [Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS].
    ///
    /// Protection level: normal
    ///
    /// [Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS]: https://developer.android.com/reference/android/provider/Settings#ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS
    RequestIgnoreBatteryOptimizations,
    /// Allows an application to request installing packages.
    ///
    /// Protection level: signature
    RequestInstallPackages,
    /// Allows an application to subscribe to notifications about the presence
    /// status change of their associated companion device.
    RequestObserveCompanionDevicePresence,
    /// Allows an application to request the screen lock complexity and prompt
    /// users to update the screen lock to a certain complexity level.
    ///
    /// Protection level: normal
    RequestPasswordComplexity,
    /// This constant was deprecated in API level 15
    ///
    /// The [ActivityManager.restartPackage(String)] API is no longer supported.
    ///
    /// [ActivityManager.restartPackage(String)]: https://developer.android.com/reference/android/app/ActivityManager#restartPackage(java.lang.String)
    RestartPackages,
    /// Allows applications to use exact alarm APIs.
    ///
    /// Exact alarms should only be used for user-facing features. For more
    /// details, see Exact alarm permission.
    ///
    /// Apps who hold this permission and target API level 31 or above, always
    /// stay in the WORKING_SET or lower standby bucket. Applications targeting
    /// API level 30 or below do not need this permission to use exact alarm APIs.
    ScheduleExactAlarm,
    /// Allows an application (Phone) to send a request to other applications to
    /// handle the respond-via-message action during incoming calls.
    ///
    /// Not for use by third-party applications.
    SendRespondViaMessage,
    /// Allows an application to send SMS messages.
    ///
    /// Protection level: dangerous
    SendSMS,
    /// Allows an application to broadcast an Intent to set an alarm for the user.
    ///
    /// Protection level: normal
    SetAlarm,
    /// Allows an application to control whether activities are immediately finished
    /// when put in the background.
    ///
    /// Not for use by third-party applications.
    SetAlwaysFinish,
    /// Modify the global animation scaling factor.
    ///
    /// Not for use by third-party applications.
    SetAnimationScale,
    /// Configure an application for debugging.
    ///
    /// Not for use by third-party applications.
    SetDebugApp,
    /// This constant was deprecated in API level 15. No longer useful, see
    /// [PackageManager.addPackageToPreferred(String)] for details.
    ///
    /// [PackageManager.addPackageToPreferred(String)]: https://developer.android.com/reference/android/content/pm/PackageManager#addPackageToPreferred(java.lang.String)
    SetPreferredApplications,
    /// Allows an application to set the maximum number of (not needed)
    /// application processes that can be running.
    ///
    /// Not for use by third-party applications.
    SetProcessLimit,
    /// Allows applications to set the system time directly.
    ///
    /// Not for use by third-party applications.
    SetTime,
    /// Allows applications to set the system time zone directly.
    ///
    /// Not for use by third-party applications.
    SetTimeZone,
    /// Allows applications to set the wallpaper.
    ///
    /// Protection level: normal
    SetWallpaper,
    /// Allows applications to set the wallpaper hints.
    ///
    /// Protection level: normal
    SetWallpaperHints,
    /// Allow an application to request that a signal be sent to all persistent processes.
    ///
    /// Not for use by third-party applications.
    SignalPersisteneProcesses,
    /// This constant was deprecated in API level 31. The API that used this permission
    /// is no longer functional.
    ///
    ///  Protection level: signature|appop
    SMSFinancialTransactions,
    /// Allows an application to start foreground services from the background at any time.
    /// This permission is not for use by third-party applications, with the only
    /// exception being if the app is the default SMS app. Otherwise, it's only usable by
    /// privileged apps, app verifier app, and apps with any of the EMERGENCY or SYSTEM
    /// GALLERY roles.
    StartForegroundServicesFromBackground,
    /// Allows the holder to start the screen with a list of app features.
    ///
    /// Protection level: signature|installer
    StartViewAppFeatures,
    /// Allows the holder to start the permission usage screen for an app.
    ///
    /// Protection level: signature|installer
    StartViewPermissionUsage,
    /// Allows an application to open, close, or disable the status bar and its icons.
    ///
    /// Not for use by third-party applications.
    StatusBar,
    /// Allows an app to create windows using the type
    /// [WindowManager.LayoutParams.TYPE_APPLICATION_OVERLAY], shown on top of all
    /// other apps.
    ///
    /// Protection level: signature|setup|appop|installer|pre23|development
    ///
    /// ## Note
    /// If the app targets API level 23 or higher, the app user must explicitly grant
    /// this permission to the app through a permission management screen. The app
    /// requests the user's approval by sending an intent with action
    /// Settings.ACTION_MANAGE_OVERLAY_PERMISSION. The app can check whether it has
    /// this authorization by calling Settings.canDrawOverlays().
    ///
    /// [WindowManager.LayoutParams.TYPE_APPLICATION_OVERLAY]: https://developer.android.com/reference/android/view/WindowManager.LayoutParams#TYPE_APPLICATION_OVERLAY
    SystemAlertWindow,
    /// Allows using the device's IR transmitter, if available.
    ///
    /// Protection level: normal
    TransmitIr,
    /// Don't use this permission in your app.
    UninstallShortcut,
    /// Allows an application to update device statistics.
    ///
    /// Not for use by third-party applications.
    UpdateDeviceStats,
    /// Allows an application to indicate via
    /// [PackageInstaller.SessionParams.setRequireUserAction(int)] that user action
    /// should not be required for an app update.
    ///
    /// Protection level: normal
    ///
    /// [PackageInstaller.SessionParams.setRequireUserAction(int)]: https://developer.android.com/reference/android/content/pm/PackageInstaller.SessionParams#setRequireUserAction(int)
    UpdatePackagesWithoutUserAction,
    /// Allows an app to use device supported biometric modalities.
    ///
    /// Protection level: normal
    UseBiometric,
    /// Allows apps to use exact alarms just like with SCHEDULE_EXACT_ALARM but without needing to request this permission from the user.
    ///
    /// This is only for apps that rely on exact alarms for their core functionality. App stores may enforce policies to audit and review the use of this permission. Any app that requests this but is found to not require exact alarms for its primary function may be removed from the app store.
    UseExactAlarm,
    /// This constant was deprecated in API level 28. Applications should request
    /// [USE_BIOMETRIC] instead.
    ///
    /// Protection level: normal
    ///
    /// [USE_BIOMETRIC]: https://developer.android.com/reference/android/Manifest.permission#USE_BIOMETRIC
    UseFingerprint,
    /// Required for apps targeting [Build.VERSION_CODES.Q] that want to use
    /// [notification full screen intents].
    ///
    /// Protection level: normal
    ///
    /// [Build.VERSION_CODES.Q]: https://developer.android.com/reference/android/os/Build.VERSION_CODES#Q
    /// [notification full screen intents]: https://developer.android.com/reference/android/app/Notification.Builder#setFullScreenIntent(android.app.PendingIntent,%20boolean)
    UseFullScreenIntent,
    /// Allows to read device identifiers and use ICC based authentication like
    /// EAP-AKA.
    ///
    /// Protection level: signature|appop
    UseIccAuthWithDeviceIdentifier,
    /// Allows an application to use SIP service.
    ///
    /// Protection level: dangerous
    UseSip,
    /// Required to be able to range to devices using ultra-wideband.
    ///
    /// Protection level: dangerous
    UwbRanging,
    /// Allows access to the vibrator.
    ///
    /// Protection level: normal
    Vibrate,
    /// Allows using PowerManager WakeLocks to keep processor from sleeping or screen
    /// from dimming.
    ///
    /// Protection level: normal
    WakeLock,
    /// Allows applications to write the apn settings and read sensitive fields of an
    /// existing apn settings like user and password.
    ///
    /// Not for use by third-party applications.
    WriteApnSettings,
    /// Allows an application to write the user's calendar data.
    ///
    /// Protection level: dangerous
    ///
    /// ## Note
    /// If your app uses the WRITE_CONTACTS permission and both your minSdkVersion and
    /// targetSdkVersion values are set to 15 or lower, the system implicitly grants
    /// your app this permission. If you don't need this permission, be sure your
    /// targetSdkVersion is 16 or higher.
    WriteCalendar,
    /// Allows an application to write (but not read) the user's call log data.
    ///
    /// Protection level: dangerous
    WriteCallLog,
    /// Allows an application to write the user's contacts data.
    ///
    /// Protection level: dangerous
    WriteContacts,
    /// Allows an application to write to external storage.
    ///
    /// Protection level: dangerous
    ///
    /// ## Note
    /// If both your minSdkVersion and targetSdkVersion values are set to 3 or lower,
    /// the system implicitly grants your app this permission. If you don't need this
    /// permission, be sure your targetSdkVersion is 4 or higher.
    WriteExternalStorage,
    /// Allows an application to modify the Google service map.
    ///
    /// Not for use by third-party applications.
    WriteGservices,
    /// Allows an application to read or write the secure system settings.
    ///
    /// Not for use by third-party applications.
    WriteSecureSettings,
    /// Allows an application to read or write the system settings.
    ///
    /// Protection level: signature|preinstalled|appop|pre23
    ///
    /// ## Note
    /// If the app targets API level 23 or higher, the app user must explicitly grant
    /// this permission to the app through a permission management screen. The app
    /// requests the user's approval by sending an intent with action
    /// Settings.ACTION_MANAGE_WRITE_SETTINGS. The app can check whether it has this
    /// authorization by calling Settings.System.canWrite().
    WriteSettings,
    /// Allows applications to write the sync settings.
    ///
    /// Protection level: normal
    WriteSyncSettings,
    /// Allows an application to modify and remove existing voicemails in the system.
    ///
    /// Protection level: signature|privileged|role
    WriteVoicemail,
}

impl AndroidPermission {
    pub fn android_permission_name(&self) -> String {
        "android.permission.".to_string() + self.to_string().as_str()
    }
}

impl std::fmt::Display for AndroidPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AcceptHandover => write!(f, "ACCEPT_HANDOVER"),
            Self::AccessBackgroundLocation => write!(f, "ACCESS_BACKGROUND_LOCATION"),
            Self::AccessBlobsAcrossUsers => write!(f, "ACCESS_BLOBS_ACROSS_USERS"),
            Self::AccessCheckinProperties => write!(f, "ACCESS_CHECKIN_PROPERTIES"),
            Self::AccessCoarseLocation => write!(f, "ACCESS_COARSE_LOCATION"),
            Self::AccessFineLocation => write!(f, "ACCESS_FINE_LOCATION"),
            Self::AccessLocationExtraCommands => write!(f, "ACCESS_LOCATION_EXTRA_COMMANDS"),
            Self::AccessMediaLocation => write!(f, "ACCESS_MEDIA_LOCATION"),
            Self::AccessNetworkState => write!(f, "ACCESS_NETWORK_STATE"),
            Self::AccessNotificationPolicy => write!(f, "ACCESS_NOTIFICATION_POLICY"),
            Self::AccessSupplementalApis => write!(f, "ACCESS_SUPPLEMENTAL_APIS"),
            Self::AccessWifiState => write!(f, "ACCESS_WIFI_STATE"),
            Self::AccountManager => write!(f, "ACCOUNT_MANAGER"),
            Self::ActivityRecognition => write!(f, "ACTIVITY_RECOGNITION"),
            Self::AddVoicemail => write!(f, "ADD_VOICEMAIL"),
            Self::AnswerPhoneCalls => write!(f, "ANSWER_PHONE_CALLS"),
            Self::BattertStats => write!(f, "BATTERY_STATS"),
            Self::BindAccessibilityService => write!(f, "BIND_ACCESSIBILITY_SERVICE"),
            Self::BindAppwidget => write!(f, "BIND_APPWIDGET"),
            Self::BindAutofillService => write!(f, "BIND_AUTOFILL_SERVICE"),
            Self::BindCallRedirectionService => write!(f, "BIND_CALL_REDIRECTION_SERVICE"),
            Self::BindCarrierMessagingClientService => {
                write!(f, "BIND_CARRIER_MESSAGING_CLIENT_SERVICE")
            }
            Self::BindCarrierMessagingService => write!(f, "BIND_CARRIER_MESSAGING_SERVICE"),
            Self::BindCarrierServices => write!(f, "BIND_CARRIER_SERVICES"),
            Self::BindChooserTargetService => write!(f, "BIND_CHOOSER_TARGET_SERVICE"),
            Self::BindCompanionDeviceService => write!(f, "BIND_COMPANION_DEVICE_SERVICE"),
            Self::BindConditionProviderService => write!(f, "BIND_CONDITION_PROVIDER_SERVICE"),
            Self::BindControls => write!(f, "BIND_CONTROLS"),
            Self::BindDeviceAdmin => write!(f, "BIND_DEVICE_ADMIN"),
            Self::BindDreamService => write!(f, "BIND_DREAM_SERVICE"),
            Self::BindIncallService => write!(f, "BIND_INCALL_SERVICE"),
            Self::BindInputMethod => write!(f, "BIND_INPUT_METHOD"),
            Self::BindMidiDeviceService => write!(f, "BIND_MIDI_DEVICE_SERVICE"),
            Self::BindNfcService => write!(f, "BIND_NFC_SERVICE"),
            Self::BindNotificationListenerService => {
                write!(f, "BIND_NOTIFICATION_LISTENER_SERVICE")
            }
            Self::BindPrintService => write!(f, "BIND_PRINT_SERVICE"),
            Self::BindQuickAccessWalletService => write!(f, "BIND_QUICK_ACCESS_WALLET_SERVICE"),
            Self::BindQuickSettingsTile => write!(f, "BIND_QUICK_SETTINGS_TILE"),
            Self::BindRrmoteviews => write!(f, "BIND_REMOTEVIEWS"),
            Self::BindScreeningService => write!(f, "BIND_SCREENING_SERVICE"),
            Self::BindTelecomConnectionService => write!(f, "BIND_TELECOM_CONNECTION_SERVICE"),
            Self::BindTextService => write!(f, "BIND_TEXT_SERVICE"),
            Self::BindTvInput => write!(f, "BIND_TV_INPUT"),
            Self::BindTvInteractiveApp => write!(f, "BIND_TV_INTERACTIVE_APP"),
            Self::BindVisualVoicemailService => write!(f, "BIND_VISUAL_VOICEMAIL_SERVICE"),
            Self::BindVoiceInteraction => write!(f, "BIND_VOICE_INTERACTION"),
            Self::BindVpnService => write!(f, "BIND_VPN_SERVICE"),
            Self::BindVrListenerService => write!(f, "BIND_VR_LISTENER_SERVICE"),
            Self::BindWallpaper => write!(f, "BIND_WALLPAPER"),
            Self::Bluetooth => write!(f, "BLUETOOTH"),
            Self::BluetoothAdmin => write!(f, "BLUETOOTH_ADMIN"),
            Self::BluetoothAdvertise => write!(f, "BLUETOOTH_ADVERTISE"),
            Self::BluetoothConnect => write!(f, "BLUETOOTH_CONNECT"),
            Self::BluetoothPrivileged => write!(f, "BLUETOOTH_PRIVILEGED"),
            Self::BluetoothScan => write!(f, "BLUETOOTH_SCAN"),
            Self::BodySensors => write!(f, "BODY_SENSORS"),
            Self::BodySensorsBackground => write!(f, "BODY_SENSORS_BACKGROUND"),
            Self::BroadcastPackageRemoved => write!(f, "BROADCAST_PACKAGE_REMOVED"),
            Self::BroadcastSMS => write!(f, "BROADCAST_SMS"),
            Self::BroadcastSticky => write!(f, "BROADCAST_STICKY"),
            Self::BroadcastWapPush => write!(f, "BROADCAST_WAP_PUSH"),
            Self::CallCompanionApp => write!(f, "CALL_COMPANION_APP"),
            Self::CallPhone => write!(f, "CALL_PHONE"),
            Self::CallPrivileged => write!(f, "CALL_PRIVILEGED"),
            Self::Camera => write!(f, "CAMERA"),
            Self::CaptureAudioOutput => write!(f, "CAPTURE_AUDIO_OUTPUT"),
            Self::ChangeComponentEnabledState => write!(f, "CHANGE_COMPONENT_ENABLED_STATE"),
            Self::ChangeConfiguration => write!(f, "CHANGE_CONFIGURATION"),
            Self::ChangeNetworkState => write!(f, "CHANGE_NETWORK_STATE"),
            Self::ChangeWifiMulticastState => write!(f, "CHANGE_WIFI_MULTICAST_STATE"),
            Self::ChangeWifiState => write!(f, "CHANGE_WIFI_STATE"),
            Self::ClearAppCache => write!(f, "CLEAR_APP_CACHE"),
            Self::ControlLocationUpdates => write!(f, "CONTROL_LOCATION_UPDATES"),
            Self::DeleteCacheFiles => write!(f, "DELETE_CACHE_FILES"),
            Self::DeletePackages => write!(f, "DELETE_PACKAGES"),
            Self::DeliverCompanionMessages => write!(f, "DELIVER_COMPANION_MESSAGES"),
            Self::Diagnostic => write!(f, "DIAGNOSTIC"),
            Self::DisableKeyguard => write!(f, "DISABLE_KEYGUARD"),
            Self::Dump => write!(f, "DUMP"),
            Self::ExpandStatusBar => write!(f, "EXPAND_STATUS_BAR"),
            Self::FactoryTest => write!(f, "FACTORY_TEST"),
            Self::ForegroundService => write!(f, "FOREGROUND_SERVICE"),
            Self::GetAccounts => write!(f, "GET_ACCOUNTS"),
            Self::GetAccountsPrivileged => write!(f, "GET_ACCOUNTS_PRIVILEGED"),
            Self::GetPackageSize => write!(f, "GET_PACKAGE_SIZE"),
            Self::GetTasks => write!(f, "GET_TASKS"),
            Self::GlobalSearch => write!(f, "GLOBAL_SEARCH"),
            Self::HighOverlayWindows => write!(f, "HIDE_OVERLAY_WINDOWS"),
            Self::HighSamplingRateSensors => write!(f, "HIGH_SAMPLING_RATE_SENSORS"),
            Self::InstallLocationProvider => write!(f, "INSTALL_LOCATION_PROVIDER"),
            Self::InstallPackages => write!(f, "INSTALL_PACKAGES"),
            Self::InstallShortcut => write!(f, "INSTALL_SHORTCUT"),
            Self::InstantAppForegroundService => write!(f, "INSTANT_APP_FOREGROUND_SERVICE"),
            Self::InteractAcrossProfiles => write!(f, "INTERACT_ACROSS_PROFILES"),
            Self::Internet => write!(f, "INTERNET"),
            Self::KillBackgroundProcesses => write!(f, "KILL_BACKGROUND_PROCESSES"),
            Self::LaunchMultiPaneSettingsDeepLink => {
                write!(f, "LAUNCH_MULTI_PANE_SETTINGS_DEEP_LINK")
            }
            Self::LoaderUsageStats => write!(f, "LOADER_USAGE_STATS"),
            Self::LocationHardware => write!(f, "LOCATION_HARDWARE"),
            Self::ManageDocuments => write!(f, "MANAGE_DOCUMENTS"),
            Self::ManageExternalStorage => write!(f, "MANAGE_EXTERNAL_STORAGE"),
            Self::ManageMedia => write!(f, "MANAGE_MEDIA"),
            Self::ManageOngoingCalls => write!(f, "MANAGE_ONGOING_CALLS"),
            Self::ManageOwnCalls => write!(f, "MANAGE_OWN_CALLS"),
            Self::ManageWifiAutoJoin => write!(f, "MANAGE_WIFI_AUTO_JOIN"),
            Self::ManageWifiInterfaces => write!(f, "MANAGE_WIFI_INTERFACES"),
            Self::MasterClear => write!(f, "MASTER_CLEAR"),
            Self::MediaContentControl => write!(f, "MEDIA_CONTENT_CONTROL"),
            Self::ModifyAudioSettings => write!(f, "MODIFY_AUDIO_SETTINGS"),
            Self::ModifyPhoneState => write!(f, "MODIFY_PHONE_STATE"),
            Self::MountFormatFilesystems => write!(f, "MOUNT_FORMAT_FILESYSTEMS"),
            Self::MountUnmountFilesystems => write!(f, "MOUNT_UNMOUNT_FILESYSTEMS"),
            Self::NearbyWifiDevices => write!(f, "NEARBY_WIFI_DEVICES"),
            Self::Nfc => write!(f, "NFC"),
            Self::NfcPreferredPatmentInfo => write!(f, "NFC_PREFERRED_PAYMENT_INFO"),
            Self::NfcTransactionEvent => write!(f, "NFC_TRANSACTION_EVENT"),
            Self::OverrideWifiConfig => write!(f, "OVERRIDE_WIFI_CONFIG"),
            Self::PackageUsageStats => write!(f, "PACKAGE_USAGE_STATS"),
            Self::PersistentActivity => write!(f, "PERSISTENT_ACTIVITY"),
            Self::PostNotifications => write!(f, "POST_NOTIFICATIONS"),
            Self::ProcessOutgoingCalls => write!(f, "PROCESS_OUTGOING_CALLS"),
            Self::QueryAllPackages => write!(f, "QUERY_ALL_PACKAGES"),
            Self::ReadAssistantAppSearchData => write!(f, "READ_ASSISTANT_APP_SEARCH_DATA"),
            Self::ReadBasicPhoneState => write!(f, "READ_BASIC_PHONE_STATE"),
            Self::ReadCalendar => write!(f, "READ_CALENDAR"),
            Self::ReadCallLog => write!(f, "READ_CALL_LOG"),
            Self::ReadContacts => write!(f, "READ_CONTACTS"),
            Self::ReadExternalStorage => write!(f, "READ_EXTERNAL_STORAGE"),
            Self::ReadInputState => write!(f, "READ_INPUT_STATE"),
            Self::ReadLogs => write!(f, "READ_LOGS"),
            Self::ReadMediaAudio => write!(f, "READ_MEDIA_AUDIO"),
            Self::ReadMediaImage => write!(f, "READ_MEDIA_IMAGE"),
            Self::ReadMediaVideo => write!(f, "READ_MEDIA_VIDEO"),
            Self::ReadNearbyStreamingPolicy => write!(f, "READ_NEARBY_STREAMING_POLICY"),
            Self::ReadPhoneNumbers => write!(f, "READ_PHONE_NUMBERS"),
            Self::ReadPhoneState => write!(f, "READ_PHONE_STATE"),
            Self::ReadPrecisePhoneState => write!(f, "READ_PRECISE_PHONE_STATE"),
            Self::ReadSMS => write!(f, "READ_SMS"),
            Self::ReadSyncSettings => write!(f, "READ_SYNC_SETTINGS"),
            Self::ReadSyncStats => write!(f, "READ_SYNC_STATS"),
            Self::ReadVoicemail => write!(f, "READ_VOICEMAIL"),
            Self::Reboot => write!(f, "REBOOT"),
            Self::ReceiveBootCompleted => write!(f, "RECEIVE_BOOT_COMPLETED"),
            Self::ReceiveMMS => write!(f, "RECEIVE_MMS"),
            Self::ReceiveSMS => write!(f, "RECEIVE_SMS"),
            Self::ReceiveWapPush => write!(f, "RECEIVE_WAP_PUSH"),
            Self::RecordAudio => write!(f, "RECORD_AUDIO"),
            Self::ReorderTasks => write!(f, "REORDER_TASKS"),
            Self::RequestCompanionProfileAppStreaming => {
                write!(f, "REQUEST_COMPANION_PROFILE_APP_STREAMING")
            }
            Self::RequestCompanionProfileAutomotiveProjection => {
                write!(f, "REQUEST_COMPANION_PROFILE_AUTOMOTIVE_PROJECTION")
            }
            Self::RequestCompanionProfileComputer => {
                write!(f, "REQUEST_COMPANION_PROFILE_COMPUTER")
            }
            Self::RequestCompanionProfileWatch => write!(f, "REQUEST_COMPANION_PROFILE_WATCH"),
            Self::RequestCompanionRunInBackground => {
                write!(f, "REQUEST_COMPANION_RUN_IN_BACKGROUND")
            }
            Self::RequestCompanionSelfManaged => write!(f, "REQUEST_COMPANION_SELF_MANAGED"),
            Self::RequestCompanionStartForegroundServicesFromBackground => write!(
                f,
                "REQUEST_COMPANION_START_FOREGROUND_SERVICES_FROM_BACKGROUND"
            ),
            Self::RequestCompanionUseDataInBackground => {
                write!(f, "REQUEST_COMPANION_USE_DATA_IN_BACKGROUND")
            }
            Self::RequestDeletePackages => write!(f, "REQUEST_DELETE_PACKAGES"),
            Self::RequestIgnoreBatteryOptimizations => {
                write!(f, "REQUEST_IGNORE_BATTERY_OPTIMIZATIONS")
            }
            Self::RequestInstallPackages => write!(f, "REQUEST_INSTALL_PACKAGES"),
            Self::RequestObserveCompanionDevicePresence => {
                write!(f, "REQUEST_OBSERVE_COMPANION_DEVICE_PRESENCE")
            }
            Self::RequestPasswordComplexity => write!(f, "REQUEST_PASSWORD_COMPLEXITY"),
            Self::RestartPackages => write!(f, "RESTART_PACKAGES"),
            Self::ScheduleExactAlarm => write!(f, "SCHEDULE_EXACT_ALARM"),
            Self::SendRespondViaMessage => write!(f, "SEND_RESPOND_VIA_MESSAGE"),
            Self::SendSMS => write!(f, "SEND_SMS"),
            Self::SetAlarm => write!(f, "SET_ALARM"),
            Self::SetAlwaysFinish => write!(f, "SET_ALWAYS_FINISH"),
            Self::SetAnimationScale => write!(f, "SET_ANIMATION_SCALE"),
            Self::SetDebugApp => write!(f, "SET_DEBUG_APP"),
            Self::SetPreferredApplications => write!(f, "SET_PREFERRED_APPLICATIONS"),
            Self::SetProcessLimit => write!(f, "SET_PROCESS_LIMIT"),
            Self::SetTime => write!(f, "SET_TIME"),
            Self::SetTimeZone => write!(f, "SET_TIME_ZONE"),
            Self::SetWallpaper => write!(f, "SET_WALLPAPER"),
            Self::SetWallpaperHints => write!(f, "SET_WALLPAPER_HINTS"),
            Self::SignalPersisteneProcesses => write!(f, "SIGNAL_PERSISTENT_PROCESSES"),
            Self::SMSFinancialTransactions => write!(f, "SMS_FINANCIAL_TRANSACTIONS"),
            Self::StartForegroundServicesFromBackground => {
                write!(f, "START_FOREGROUND_SERVICES_FROM_BACKGROUND")
            }
            Self::StartViewAppFeatures => write!(f, "START_VIEW_APP_FEATURES"),
            Self::StartViewPermissionUsage => write!(f, "START_VIEW_PERMISSION_USAGE"),
            Self::StatusBar => write!(f, "STATUS_BAR"),
            Self::SystemAlertWindow => write!(f, "SYSTEM_ALERT_WINDOW"),
            Self::TransmitIr => write!(f, "TRANSMIT_IR"),
            Self::UninstallShortcut => write!(f, "UNINSTALL_SHORTCUT"),
            Self::UpdateDeviceStats => write!(f, "UPDATE_DEVICE_STATS"),
            Self::UpdatePackagesWithoutUserAction => {
                write!(f, "UPDATE_PACKAGES_WITHOUT_USER_ACTION")
            }
            Self::UseBiometric => write!(f, "USE_BIOMETRIC"),
            Self::UseExactAlarm => write!(f, "USE_EXACT_ALARM"),
            Self::UseFingerprint => write!(f, "USE_FINGERPRINT"),
            Self::UseFullScreenIntent => write!(f, "USE_FULL_SCREEN_INTENT"),
            Self::UseIccAuthWithDeviceIdentifier => {
                write!(f, "USE_ICC_AUTH_WITH_DEVICE_IDENTIFIER")
            }
            Self::UseSip => write!(f, "USE_SIP"),
            Self::UwbRanging => write!(f, "UWB_RANGING"),
            Self::Vibrate => write!(f, "VIBRATE"),
            Self::WakeLock => write!(f, "WAKE_LOCK"),
            Self::WriteApnSettings => write!(f, "WRITE_APN_SETTINGS"),
            Self::WriteCalendar => write!(f, "WRITE_CALENDAR"),
            Self::WriteCallLog => write!(f, "WRITE_CALL_LOG"),
            Self::WriteContacts => write!(f, "WRITE_CONTACTS"),
            Self::WriteExternalStorage => write!(f, "WRITE_EXTERNAL_STORAGE"),
            Self::WriteGservices => write!(f, "WRITE_GSERVICES"),
            Self::WriteSecureSettings => write!(f, "WRITE_SECURE_SETTINGS"),
            Self::WriteSettings => write!(f, "WRITE_SETTINGS"),
            Self::WriteSyncSettings => write!(f, "WRITE_SYNC_SETTINGS"),
            Self::WriteVoicemail => write!(f, "WRITE_VOICEMAIL"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_string() {
        let permission = AndroidPermission::AccessCheckinProperties;
        assert_eq!(
            permission.android_permission_name(),
            "android.permission.ACCESS_CHECKIN_PROPERTIES"
        );
    }
}

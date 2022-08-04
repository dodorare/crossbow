# Development Roadmap

| Number | Title | Specification | Status |
| -----: | ----------- | ------------- | ------------- |
| 1. | Support AAB | Add support of generation AAB file. [Android App Bundle](https://developer.android.com/guide/app-bundle) is a publishing format that includes all your app’s compiled code and resources. | ✅ |
| 2. | Support Macroquad engine | Add support of [Macroquad](https://macroquad.rs/) engine. We will change our crossbundle command-line tool to support Android and iOS building of Macroquad. | ✅ |
| 3. | Support Android Plugins | Add support of Android plugins to help add additional functionality provided by the Android platform and ecosystem (like Ads, Auth, In-app purchases, etc.). Something similar to [Godot Android plugins](https://docs.godotengine.org/en/stable/tutorials/plugins/android/android_plugin.html). | ✅ |
| 4. | Support Cross-platform permissions | Provide a single cross-platform permission API that works with any [iOS](https://developer.apple.com/design/human-interface-guidelines/ios/app-architecture/accessing-user-data/), [Android](https://developer.android.com/games/develop/permissions), etc application that can be accessed from shared code no matter how the user interface is created. | ✅ |
| 5. | Simple installation | Simple installation with environment variables, libs, etc. Make installation of Android SDK, NDK, tools more robust. | ✅ |
| 6. | Support iOS Plugins | Add support of iOS plugins to help add additional functionality provided by the Apple platforms and ecosystem (like Ads, Auth, In-app purchases, etc.). Something similar to [Godot iOS plugins](https://docs.godotengine.org/en/stable/tutorials/platform/ios/ios_plugin.html). | 🛠 |
| 7. | Sign in with Google | Add support of [Google Sign In](https://developers.google.com/games/services/common/concepts/sign-in) inside any application. | 🛠 |
| 8. | Sign in with Apple | Add support of [Apple Sign In](https://github.com/lupidan/apple-signin-unity) inside any application. | 📝 |
| 9. | Better support for Apple xcrun, xcode proj | Add better support and rust wrappers for Apple xcode tools, xcrun. Make cool xcode project generation library. | 📝 |
| 10. | Apple Game Center | Add [Apple Game Center](https://developer.apple.com/documentation/gamekit) support. | 📝 |
| 11. | Android In-App purchases & Google Play Billing | Add support for [Google Play Billing](https://github.com/godotengine/godot-google-play-billing). Make it possible to buy items from your application. | 🛠 |
| 12. | Support Apple In-App purchases | Support Apple [StoreKit](https://developer.apple.com/documentation/storekit/in-app_purchase). Make it possible to buy items from your application. | 📝 |
| 13. | Support Android In-App updates | Add support for [Android In-App updates](https://developer.android.com/guide/playcore/in-app-updates). | 🛠 |

✅ = Works and tested — 🆗 = Works but may contain bugs — 🛠 = Under development — 📝 = Planned

## Useful links

- https://developers.google.com/games/services/common/concepts/sign-in
- https://developers.google.com/games/services/downloads/sdks
- https://developers.google.com/games/services/cpp/GettingStartedNativeClient
- https://developers.google.com/identity/sign-in/web/build-button
- https://github.com/cgisca/PGSGP
- https://docs.godotengine.org/en/stable/tutorials/platform/ios/ios_plugin.html
- https://github.com/polyhorn/simctl
- https://docs.godotengine.org/en/stable/tutorials/plugins/android/android_plugin.html
- https://github.com/godotengine/godot-google-play-billing
- https://github.com/google/play-unity-plugins
- https://android.googlesource.com/platform/frameworks/opt/gamesdk
- https://developer.android.com/games/develop/permissions
- https://docs.microsoft.com/en-gb/xamarin/essentials
- https://docs.microsoft.com/en-us/xamarin/essentials/permissions

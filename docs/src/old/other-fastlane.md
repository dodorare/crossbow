# Fastlane automation for Android

We want to make you work less on the hard stuff behind the automation of publishing Rust mobile apps so we prepared some best practices on how you can automate the boring stuff with our [creator Fastlane plugin](https://github.com/creator-rs/fastlane-plugin). You can read more about `Fastlane` [here](https://fastlane.tools/).

## Setup

First of all, we would suggest you pass through our `crossbow` basic tutorials for [Android](https://github.com/dodorare/crossbow/wiki/Android-setup-on-Windows) or/and [iOS](https://github.com/dodorare/crossbow/wiki/iOS-setup-on-MacOS).

After you got familiar with `crossbow` you can get to the following steps to run `Fastlane` locally:

1. Make sure you have [crossbundle](https://github.com/dodorare/crossbow/wiki/Command-line-tool) installed.
2. Install [Fastlane](https://docs.fastlane.tools/#getting-started).
3. Setup [XCode](https://github.com/dodorare/crossbow/wiki/iOS-setup-on-MacOS) or [Android Studio](https://github.com/dodorare/crossbow/wiki/Android-setup-on-Windows) depending on which platform you want to run your workflow.
4. Create a test project (you can use [our template](https://github.com/dodorare/crossbundle-templates/tree/fastlane-example)) or add Fastlane to your project.
5. Install our `fastlane` plugin with the next command `fastlane add_plugin crossbow`.

## Setup for Play Market

1. Open [Google Play Console](https://play.google.com/console) (you will need a developer account for this).
2. Click on the button "Create app" ![Create app](https://i.imgur.com/2IYrMqV.png) and specify your application data in the form.
3. Generate signing key through Android Studio as shown in [this official tutorial](https://developer.android.com/studio/publish/app-signing#sign-apk).
4. Generate a "Play Store Json Key" for Fastlane as shown [here](https://docs.fastlane.tools/actions/upload_to_play_store/#setup).
5. Configure corresponding variables in `fastlane/Fastfile` file.

<!---
## Setup for Apple TestFlight
1. [Apple TestFlight](https://docs.fastlane.tools/actions/upload_to_testflight/#usage)/[App Store](https://docs.fastlane.tools/actions/upload_to_app_store/) upload settings and configure remaining variables in `Fastfile`.
-->

## Run lane

To run our Fastlane pipeline just call `fastlane android` or `fastlane ios` at the root of the project.

Please note, Fastlane will ask you to log in on the first run to configure the setup.

## See results

After the successful finish of publishing as an internal testing draft, you should see your release on the `Google Play Console`. Find section "Testing" and click on the "Internal testing". You will see something like this:

![Internal testing](https://i.imgur.com/2DTnidE.png)

Feel free to add some other lanes to fill your needs, here's the [full list](http://docs.fastlane.tools/actions/) of them.

If you found a bug or issue - you can find or create an issue in [this](https://github.com/dodorare/crossbow) repository.

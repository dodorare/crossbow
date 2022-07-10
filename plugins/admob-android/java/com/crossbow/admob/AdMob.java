package com.crossbow.admob;

import com.dodorare.crossbow.Crossbow;
import com.dodorare.crossbow.plugin.CrossbowPlugin;
import com.dodorare.crossbow.plugin.SignalInfo;
import com.dodorare.crossbow.plugin.ExposedToCrossbow;

import com.google.android.gms.ads.AdError;
import com.google.android.gms.ads.FullScreenContentCallback;
import com.google.android.gms.ads.MobileAds;
import com.google.android.gms.ads.LoadAdError;
import com.google.android.gms.ads.AdRequest;

import com.google.android.gms.ads.AdView;
import com.google.android.gms.ads.AdSize;
import com.google.android.gms.ads.AdListener;

import com.google.android.gms.ads.interstitial.InterstitialAd;
import com.google.android.gms.ads.interstitial.InterstitialAdLoadCallback;

import com.google.android.gms.ads.RequestConfiguration;
import com.google.android.gms.ads.rewarded.RewardedAd;
import com.google.android.gms.ads.rewarded.RewardedAdLoadCallback;

import com.google.android.gms.ads.rewardedinterstitial.RewardedInterstitialAd;
import com.google.android.gms.ads.rewardedinterstitial.RewardedInterstitialAdLoadCallback;
import com.google.android.ump.ConsentDebugSettings;
import com.google.android.ump.ConsentInformation;
import com.google.android.ump.ConsentRequestParameters;
import com.google.android.ump.UserMessagingPlatform;

import android.app.Activity;
import android.graphics.Rect;
import android.os.Build;
import android.util.DisplayMetrics;
import android.view.Display;
import android.view.DisplayCutout;
import android.view.Gravity;
import android.view.WindowInsets;
import android.widget.FrameLayout; //get Crossbow Layout
import android.view.View;
import android.provider.Settings;
import androidx.annotation.NonNull;
import androidx.collection.ArraySet;

import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Locale;
import java.util.Objects;
import java.util.Set;

public class AdMob extends CrossbowPlugin {
    private boolean aIsInitialized = false;
    private Activity aActivity;

    private ConsentInformation aConsentInformation;
    private boolean aIsTestEuropeUserConsent;

    private boolean aIsForChildDirectedTreatment;

    private boolean aIsBannerLoaded = false;
    private boolean aIsInterstitialLoaded = false;
    private boolean aIsRewardedLoaded = false;
    private boolean aIsRewardedInterstitialLoaded = false;

    private FrameLayout aCrossbowLayout; // store the crossbow layout
    private FrameLayout.LayoutParams aCrossbowLayoutParams; // Store the crossbow layout params

    private AdView aAdView; //view of banner
    private AdSize aAdSize; //adSize of banner
    private InterstitialAd aInterstitialAd;
    private RewardedAd aRewardedAd;
    private RewardedInterstitialAd aRewardedInterstitialAd;

    public AdMob(Crossbow crossbow) {
        super(crossbow);
    }

    @Override
    public View onMainCreate(Activity pActivity) {
        aActivity = pActivity;
        aCrossbowLayout = new FrameLayout(pActivity);
        return aCrossbowLayout;
    }

    @NonNull
    @Override
    public String getPluginName() {
        return getClass().getSimpleName();
    }

    @NonNull
    @Override
    public Set<SignalInfo> getPluginSignals() {
        Set<SignalInfo> signals = new ArraySet<>();

        signals.add(new SignalInfo("initialization_complete", Integer.class, String.class));

        signals.add(new SignalInfo("consent_form_dismissed"));
        signals.add(new SignalInfo("consent_status_changed", String.class));
        signals.add(new SignalInfo("consent_form_load_failure", Integer.class, String.class));
        signals.add(new SignalInfo("consent_info_update_success", String.class));
        signals.add(new SignalInfo("consent_info_update_failure", Integer.class, String.class));

        signals.add(new SignalInfo("banner_loaded"));
        signals.add(new SignalInfo("banner_failed_to_load", Integer.class));
        signals.add(new SignalInfo("banner_opened"));
        signals.add(new SignalInfo("banner_clicked"));
        signals.add(new SignalInfo("banner_closed"));
        signals.add(new SignalInfo("banner_recorded_impression"));
        signals.add(new SignalInfo("banner_destroyed"));

        signals.add(new SignalInfo("interstitial_failed_to_load", Integer.class));
        signals.add(new SignalInfo("interstitial_loaded"));
        signals.add(new SignalInfo("interstitial_failed_to_show", Integer.class));
        signals.add(new SignalInfo("interstitial_opened"));
        signals.add(new SignalInfo("interstitial_closed"));

        signals.add(new SignalInfo("rewarded_ad_failed_to_load", Integer.class));
        signals.add(new SignalInfo("rewarded_ad_loaded"));
        signals.add(new SignalInfo("rewarded_ad_failed_to_show", Integer.class));
        signals.add(new SignalInfo("rewarded_ad_opened"));
        signals.add(new SignalInfo("rewarded_ad_closed"));

        signals.add(new SignalInfo("rewarded_interstitial_ad_failed_to_load", Integer.class));
        signals.add(new SignalInfo("rewarded_interstitial_ad_loaded"));
        signals.add(new SignalInfo("rewarded_interstitial_ad_failed_to_show", Integer.class));
        signals.add(new SignalInfo("rewarded_interstitial_ad_opened"));
        signals.add(new SignalInfo("rewarded_interstitial_ad_closed"));

        signals.add(new SignalInfo("user_earned_rewarded", String.class, Integer.class));

        return signals;
    }

    @ExposedToCrossbow
    public boolean get_is_initialized() {
        return aIsInitialized;
    }
    @ExposedToCrossbow
    public boolean get_is_banner_loaded() {
        return aIsBannerLoaded;
    }
    @ExposedToCrossbow
    public boolean get_is_interstitial_loaded() {
        return aIsInterstitialLoaded;
    }
    @ExposedToCrossbow
    public boolean get_is_rewarded_loaded() {
        return aIsRewardedLoaded;
    }
    @ExposedToCrossbow
    public boolean get_is_rewarded_interstitial_loaded() {
        return aIsRewardedInterstitialLoaded;
    }

    @ExposedToCrossbow
    public void initialize(boolean pIsForChildDirectedTreatment, String pMaxAdContentRating, boolean pIsReal, boolean pIsTestEuropeUserConsent) {
        if (!aIsInitialized){
            aIsForChildDirectedTreatment = pIsForChildDirectedTreatment;
            aConsentInformation = UserMessagingPlatform.getConsentInformation(aActivity);
            aIsTestEuropeUserConsent = pIsTestEuropeUserConsent;

            setMobileAdsRequestConfiguration(aIsForChildDirectedTreatment, pMaxAdContentRating, pIsReal); //First call MobileAds.setRequestConfiguration https://groups.google.com/g/google-admob-ads-sdk/c/17oVu0sABjs
            MobileAds.initialize(aActivity, initializationStatus -> {
                int statusGADMobileAds = Objects.requireNonNull(initializationStatus.getAdapterStatusMap().get("com.google.android.gms.ads.MobileAds")).getInitializationState().ordinal();

                if (statusGADMobileAds == 0) {
                    aIsInitialized = false;
                }
                else if (statusGADMobileAds == 1){
                    aIsInitialized = true;
                }

                emitSignal("initialization_complete", statusGADMobileAds, "GADMobileAds");
            });
        }
    }

    @ExposedToCrossbow
    public void request_user_consent() {
        aConsentInformation = UserMessagingPlatform.getConsentInformation(aActivity);

        ConsentRequestParameters.Builder paramsBuilder = new ConsentRequestParameters.Builder().setTagForUnderAgeOfConsent(aIsForChildDirectedTreatment);

        ConsentRequestParameters params;
        if (aIsTestEuropeUserConsent) //https://developers.google.com/admob/ump/android/quick-start#testing
        {
            ConsentDebugSettings debugSettings = new ConsentDebugSettings.Builder(aActivity)
                    .setDebugGeography(ConsentDebugSettings.DebugGeography.DEBUG_GEOGRAPHY_EEA)
                    .addTestDeviceHashedId(getDeviceId())
                    .build();
            params = paramsBuilder.setConsentDebugSettings(debugSettings).build();
        } else {
            params = paramsBuilder.build();
        }

        aConsentInformation.requestConsentInfoUpdate(aActivity, params,
                () -> {
                    if (aConsentInformation.isConsentFormAvailable()) {
                        emitSignal("consent_info_update_success", "Consent Form Available");
                        loadConsentForm();
                    } else {
                        emitSignal("consent_info_update_success", "Consent Form not Available");
                    }
                },
                formError -> emitSignal("consent_info_update_failure", formError.getErrorCode(), formError.getMessage())
        );
    }

    @ExposedToCrossbow
    public void reset_consent_state() {
        aConsentInformation.reset(); //https://developers.google.com/admob/ump/android/quick-start#reset_consent_state
    }

    // BANNER only one is allowed, please do not try to place more than one, as your ads on the app may have the chance to be banned!
    @ExposedToCrossbow
    public void load_banner(final String pAdUnitId, final int pPosition, final String pSize, final boolean pShowInstantly, final boolean pRespectSafeArea) {
        aActivity.runOnUiThread(() -> {
            if (aIsInitialized) {
                if (aAdView != null) destroy_banner();
                aAdView = new AdView(aActivity);

                aAdView.setAdUnitId(pAdUnitId);
                switch (pSize) {
                    case "BANNER":
                        aAdView.setAdSize(AdSize.BANNER);
                        break;
                    case "LARGE_BANNER":
                        aAdView.setAdSize(AdSize.LARGE_BANNER);
                        break;
                    case "MEDIUM_RECTANGLE":
                        aAdView.setAdSize(AdSize.MEDIUM_RECTANGLE);
                        break;
                    case "FULL_BANNER":
                        aAdView.setAdSize(AdSize.FULL_BANNER);
                        break;
                    case "LEADERBOARD":
                        aAdView.setAdSize(AdSize.LEADERBOARD);
                        break;
                    case "ADAPTIVE":
                        aAdView.setAdSize(getAdSizeAdaptive());
                        break;
                    default:
                        aAdView.setAdSize(AdSize.SMART_BANNER);
                        break;
                }
                aAdSize = aAdView.getAdSize(); //store AdSize of banner due a bug (throws error when do aAdView.getAdSize(); called by Crossbow)
                aAdView.setAdListener(new AdListener() {
                    @Override
                    public void onAdLoaded() {
                        // Code to be executed when an ad finishes loading.
                        emitSignal("banner_loaded");

                        if (pShowInstantly){
                            show_banner();
                        }
                        else{
                            hide_banner();
                        }
                        aIsBannerLoaded = true;
                    }

                    @Override
                    public void onAdFailedToLoad(@NonNull LoadAdError adError) {
                        // Code to be executed when an ad request fails.
                        emitSignal("banner_failed_to_load", adError.getCode());
                    }

                    @Override
                    public void onAdOpened() {
                        // Code to be executed when an ad opens an overlay that
                        // covers the screen.
                        emitSignal("banner_opened");
                    }

                    @Override
                    public void onAdClicked() {
                        // Code to be executed when the native ad is closed.
                        emitSignal("banner_clicked");
                    }

                    @Override
                    public void onAdClosed() {
                        // Code to be executed when the user is about to return
                        // to the app after tapping on an ad.
                        emitSignal("banner_closed");
                    }

                    @Override
                    public void onAdImpression() {
                        // Code to be executed when the user is about to return
                        // to the app after tapping on an ad.
                        emitSignal("banner_recorded_impression");
                    }
                });

                aCrossbowLayoutParams = new FrameLayout.LayoutParams(FrameLayout.LayoutParams.MATCH_PARENT, FrameLayout.LayoutParams.WRAP_CONTENT);
                if (pPosition == 0)//BOTTOM
                {
                    aCrossbowLayoutParams.gravity = Gravity.BOTTOM;
                    if (pRespectSafeArea)
                        aAdView.setY(-getSafeArea().bottom); //Need to validate if this value will be positive or negative
                } else if (pPosition == 1)//TOP
                {
                    aCrossbowLayoutParams.gravity = Gravity.TOP;
                    if (pRespectSafeArea)
                        aAdView.setY(getSafeArea().top);
                }
                aCrossbowLayout.addView(aAdView, aCrossbowLayoutParams);

                aAdView.loadAd(getAdRequest());

            }
        });
    }

    @ExposedToCrossbow
    public void destroy_banner()//IF THIS METHOD IS CALLED ON CROSSBOW, THE BANNER WILL ONLY APPEAR AGAIN IF THE BANNER IS LOADED AGAIN
    {
        aActivity.runOnUiThread(() -> {
            if (aIsInitialized && aAdView != null) {
                aCrossbowLayout.removeView(aAdView);
                aAdView.destroy();
                aAdView = null;

                emitSignal("banner_destroyed");
                aIsBannerLoaded = false;
            }
        });
    }

    @ExposedToCrossbow
    public void show_banner()
    {
        aActivity.runOnUiThread(() -> {
            if (aIsInitialized && aAdView != null) {
                if (aAdView.getVisibility() != View.VISIBLE){
                    aAdView.setVisibility(View.VISIBLE);
                    aAdView.resume();
                }
            }
        });
    }

    @ExposedToCrossbow
    public void hide_banner()
    {
        aActivity.runOnUiThread(() -> {
            if (aIsInitialized && aAdView != null) {
                if (aAdView.getVisibility() != View.GONE){
                    aAdView.setVisibility(View.GONE);
                    aAdView.pause();
                }
            }
        });
    }

    @ExposedToCrossbow
    public int get_banner_width() {
        if (aIsInitialized && aAdSize != null) {
            return aAdSize.getWidth();
        }
        return 0;
    }

    @ExposedToCrossbow
    public int get_banner_height() {
        if (aIsInitialized && aAdSize != null) {
            return aAdSize.getHeight();
        }
        return 0;
    }

    @ExposedToCrossbow
    public int get_banner_width_in_pixels() {
        if (aIsInitialized && aAdSize != null) {
            return aAdSize.getWidthInPixels(aActivity);
        }
        return 0;
    }

    @ExposedToCrossbow
    public int get_banner_height_in_pixels() {
        if (aIsInitialized && aAdSize != null) {
            return aAdSize.getHeightInPixels(aActivity);
        }
        return 0;
    }

    // BANNER
    // INTERSTITIAL
    @ExposedToCrossbow
    public void load_interstitial(final String pAdUnitId)
    {
        aActivity.runOnUiThread(() -> {
            if (aIsInitialized) {
                InterstitialAd.load(aActivity, pAdUnitId, getAdRequest(), new InterstitialAdLoadCallback() {
                    @Override
                    public void onAdLoaded(@NonNull InterstitialAd interstitialAd) {
                        // Code to be executed when an ad finishes loading.
                        aInterstitialAd = interstitialAd;

                        emitSignal("interstitial_loaded");
                        aIsInterstitialLoaded = true;

                        interstitialAd.setFullScreenContentCallback(new FullScreenContentCallback() {
                            @Override
                            public void onAdDismissedFullScreenContent() {
                                // Called when fullscreen content is dismissed.
                                aInterstitialAd = null;
                                emitSignal("interstitial_closed");
                                aIsInterstitialLoaded = false;
                            }

                            @Override
                            public void onAdFailedToShowFullScreenContent(@NonNull AdError adError) {
                                // Called when fullscreen content failed to show.
                                aInterstitialAd = null;
                                emitSignal("interstitial_failed_to_show", adError.getCode());
                                aIsInterstitialLoaded = false;
                            }

                            @Override
                            public void onAdShowedFullScreenContent() {
                                // Called when fullscreen content is shown.
                                emitSignal("interstitial_opened");
                            }
                        });
                    }

                    @Override
                    public void onAdFailedToLoad(@NonNull LoadAdError adError) {
                        // Code to be executed when an ad request fails.
                        aInterstitialAd = null;
                        emitSignal("interstitial_failed_to_load", adError.getCode());
                    }
                });
            }
        });
    }

    @ExposedToCrossbow
    public void show_interstitial()
    {
        aActivity.runOnUiThread(() -> {
            if (aIsInitialized) {
                if (aInterstitialAd != null) {
                    aInterstitialAd.show(aActivity);
                }
            }
        });
    }

    //INTERSTITIAL
    //REWARDED
    @ExposedToCrossbow
    public void load_rewarded(final String pAdUnitId)
    {
        aActivity.runOnUiThread(() -> {
            if (aIsInitialized) {
                RewardedAd.load(aActivity, pAdUnitId, getAdRequest(), new RewardedAdLoadCallback(){
                    @Override
                    public void onAdFailedToLoad(@NonNull LoadAdError loadAdError) {
                        // Handle the error.
                        aRewardedAd = null;
                        emitSignal("rewarded_ad_failed_to_load", loadAdError.getCode());

                    }

                    @Override
                    public void onAdLoaded(@NonNull RewardedAd rewardedAd) {
                        aRewardedAd = rewardedAd;
                        emitSignal("rewarded_ad_loaded");

                        aIsRewardedLoaded = true;
                    }
                });
            }
        });
    }

    @ExposedToCrossbow
    public void show_rewarded()
    {
        aActivity.runOnUiThread(() -> {
            if (aIsInitialized) {
                if (aRewardedAd != null) {
                    aRewardedAd.setFullScreenContentCallback(new FullScreenContentCallback() {
                        @Override
                        public void onAdShowedFullScreenContent() {
                            // Called when ad is shown.

                            emitSignal("rewarded_ad_opened");
                        }

                        @Override
                        public void onAdFailedToShowFullScreenContent(@NonNull AdError adError) {
                            // Called when ad fails to show.
                            aRewardedAd = null;
                            emitSignal("rewarded_ad_failed_to_show", adError.getCode());
                        }

                        @Override
                        public void onAdDismissedFullScreenContent() {
                            // Called when ad is dismissed.
                            aRewardedAd = null;
                            emitSignal("rewarded_ad_closed");
                            aIsRewardedLoaded = false;
                        }
                    });

                    aRewardedAd.show(aActivity, rewardItem -> {
                        // Handle the reward.
                        emitSignal("user_earned_rewarded", rewardItem.getType(), rewardItem.getAmount());
                    });
                }
            }
        });
    }

    @ExposedToCrossbow
    public void load_rewarded_interstitial(final String pAdUnitId)
    {
        aActivity.runOnUiThread(() -> {
            if (aIsInitialized) {
                RewardedInterstitialAd.load(aActivity, pAdUnitId, getAdRequest(), new RewardedInterstitialAdLoadCallback(){
                    @Override
                    public void onAdFailedToLoad(@NonNull LoadAdError loadAdError) {
                        // Handle the error.
                        aRewardedInterstitialAd = null;
                        emitSignal("rewarded_interstitial_ad_failed_to_load", loadAdError.getCode());
                    }

                    @Override
                    public void onAdLoaded(@NonNull RewardedInterstitialAd rewardedInterstitialAd) {
                        aRewardedInterstitialAd = rewardedInterstitialAd;
                        emitSignal("rewarded_interstitial_ad_loaded");
                        aIsRewardedInterstitialLoaded = true;
                    }
                });
            }
        });
    }

    @ExposedToCrossbow
    public void show_rewarded_interstitial()
    {
        aActivity.runOnUiThread(() -> {
            if (aIsInitialized) {
                if (aRewardedInterstitialAd != null) {
                    aRewardedInterstitialAd.setFullScreenContentCallback(new FullScreenContentCallback() {
                        @Override
                        public void onAdShowedFullScreenContent() {
                            // Called when ad is shown.
                            emitSignal("rewarded_interstitial_ad_opened");
                        }

                        @Override
                        public void onAdFailedToShowFullScreenContent(@NonNull AdError adError) {
                            // Called when ad fails to show.
                            aRewardedInterstitialAd = null;
                            emitSignal("rewarded_interstitial_ad_failed_to_show", adError.getCode());
                            aIsRewardedInterstitialLoaded = false;
                        }

                        @Override
                        public void onAdDismissedFullScreenContent() {
                            // Called when ad is dismissed.
                            aRewardedInterstitialAd = null;
                            emitSignal("rewarded_interstitial_ad_closed");
                            aIsRewardedInterstitialLoaded = false;
                        }
                    });

                    aRewardedInterstitialAd.show(aActivity, rewardItem -> {
                        // Handle the reward.
                        emitSignal("user_earned_rewarded", rewardItem.getType(), rewardItem.getAmount());
                    });
                }
            }
        });
    }

    private void loadConsentForm() {
        UserMessagingPlatform.loadConsentForm(
            aActivity,
            consentForm -> {
                String consentStatusMsg = "";
                if (aConsentInformation.getConsentStatus() == ConsentInformation.ConsentStatus.REQUIRED) {
                    consentForm.show(
                            aActivity,
                            formError -> {
                                loadConsentForm();
                                emitSignal("consent_form_dismissed");
                            }
                    );
                    consentStatusMsg = "User consent required but not yet obtained.";
                }
                switch (aConsentInformation.getConsentStatus()) {
                    case ConsentInformation.ConsentStatus.UNKNOWN:
                        consentStatusMsg = "Unknown consent status.";
                        break;
                    case ConsentInformation.ConsentStatus.NOT_REQUIRED:
                        consentStatusMsg = "User consent not required. For example, the user is not in the EEA or the UK.";
                        break;
                    case ConsentInformation.ConsentStatus.OBTAINED:
                        consentStatusMsg = "User consent obtained. Personalization not defined.";
                        break;
                }
                emitSignal("consent_status_changed", consentStatusMsg);
            },
            formError -> emitSignal("consent_form_load_failure", formError.getErrorCode(), formError.getMessage())
        );
    }

    private void setMobileAdsRequestConfiguration(boolean pIsForChildDirectedTreatment, String pMaxAdContentRating, boolean pIsReal) {
        RequestConfiguration requestConfiguration;
        RequestConfiguration.Builder requestConfigurationBuilder = new RequestConfiguration.Builder();

        if (!pIsReal) {
            requestConfigurationBuilder.setTestDeviceIds(Collections.singletonList(getDeviceId()));
        }

        requestConfigurationBuilder.setTagForChildDirectedTreatment(pIsForChildDirectedTreatment ? 1 : 0);

        if (pIsForChildDirectedTreatment) {
            requestConfigurationBuilder.setMaxAdContentRating(RequestConfiguration.MAX_AD_CONTENT_RATING_G);
        } else {
            switch (pMaxAdContentRating) {
                case RequestConfiguration.MAX_AD_CONTENT_RATING_G:
                case RequestConfiguration.MAX_AD_CONTENT_RATING_MA:
                case RequestConfiguration.MAX_AD_CONTENT_RATING_PG:
                case RequestConfiguration.MAX_AD_CONTENT_RATING_T:
                case RequestConfiguration.MAX_AD_CONTENT_RATING_UNSPECIFIED:
                    requestConfigurationBuilder.setMaxAdContentRating(pMaxAdContentRating);
                    break;
            }
        }

        requestConfiguration = requestConfigurationBuilder.build();

        MobileAds.setRequestConfiguration(requestConfiguration);
    }

    private AdRequest getAdRequest() {
        AdRequest.Builder adRequestBuilder = new AdRequest.Builder();

        return adRequestBuilder.build();
    }

    private Rect getSafeArea() {
        final Rect safeInsetRect = new Rect();
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.P) {
            return safeInsetRect;
        }

        final WindowInsets windowInsets = aActivity.getWindow().getDecorView().getRootWindowInsets();
        if (windowInsets == null) {
            return safeInsetRect;
        }

        final DisplayCutout displayCutout = windowInsets.getDisplayCutout();
        if (displayCutout != null) {
            safeInsetRect.set(displayCutout.getSafeInsetLeft(), displayCutout.getSafeInsetTop(), displayCutout.getSafeInsetRight(), displayCutout.getSafeInsetBottom());
        }

        return safeInsetRect;
    }

    private AdSize getAdSizeAdaptive() {
        // Determine the screen width (less decorations) to use for the ad width.
        Display display = aActivity.getWindowManager().getDefaultDisplay();
        DisplayMetrics outMetrics = new DisplayMetrics();
        display.getMetrics(outMetrics);

        float density = outMetrics.density;

        float adWidthPixels = aCrossbowLayout.getWidth();

        // If the ad hasn't been laid out, default to the full screen width.
        if (adWidthPixels == 0) {
            adWidthPixels = outMetrics.widthPixels;
        }

        int adWidth = (int) (adWidthPixels / density);
        return AdSize.getCurrentOrientationAnchoredAdaptiveBannerAdSize(aActivity, adWidth);
    }

    /**
     * Generate MD5 for the deviceID
     * @param  s The string to generate de MD5
     * @return String The MD5 generated
     */
    private String md5(final String s)
    {
        try
        {
            // Create MD5 Hash
            MessageDigest digest = MessageDigest.getInstance("MD5");
            digest.update(s.getBytes());
            byte[] messageDigest = digest.digest();

            // Create Hex String
            StringBuilder hexString = new StringBuilder();
            for (byte b : messageDigest) {
                StringBuilder h = new StringBuilder(Integer.toHexString(0xFF & b));
                while (h.length() < 2) h.insert(0, "0");
                hexString.append(h);
            }
            return hexString.toString();
        }
        catch(NoSuchAlgorithmException e)
        {
            //Logger.logStackTrace(TAG,e);
        }
        return "";
    }

    /**
     * Get the Device ID for AdMob
     * @return String Device ID
     */
    private String getDeviceId()
    {
        String android_id = Settings.Secure.getString(aActivity.getContentResolver(), Settings.Secure.ANDROID_ID);
        return md5(android_id).toUpperCase(Locale.US);
    }
}

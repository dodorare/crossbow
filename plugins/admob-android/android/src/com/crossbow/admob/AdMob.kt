package com.crossbow.admob

import com.crossbow.library.Crossbow
import com.crossbow.library.plugin.CrossbowPlugin
import com.crossbow.library.plugin.SignalInfo
import com.crossbow.library.plugin.ExposedToCrossbow

import com.google.android.gms.ads.AdError
import com.google.android.gms.ads.FullScreenContentCallback
import com.google.android.gms.ads.MobileAds
import com.google.android.gms.ads.LoadAdError
import com.google.android.gms.ads.AdRequest

import com.google.android.gms.ads.AdView
import com.google.android.gms.ads.AdSize
import com.google.android.gms.ads.AdListener

import com.google.android.gms.ads.interstitial.InterstitialAd
import com.google.android.gms.ads.interstitial.InterstitialAdLoadCallback

import com.google.android.gms.ads.RequestConfiguration
import com.google.android.gms.ads.rewarded.RewardedAd
import com.google.android.gms.ads.rewarded.RewardedAdLoadCallback

import com.google.android.gms.ads.rewardedinterstitial.RewardedInterstitialAd
import com.google.android.gms.ads.rewardedinterstitial.RewardedInterstitialAdLoadCallback
import com.google.android.ump.ConsentDebugSettings
import com.google.android.ump.ConsentInformation
import com.google.android.ump.ConsentRequestParameters
import com.google.android.ump.UserMessagingPlatform

import android.app.Activity
import android.graphics.Rect
import android.os.Build
import android.util.DisplayMetrics
import android.view.Display
import android.view.DisplayCutout
import android.view.Gravity
import android.view.WindowInsets
import android.widget.FrameLayout
import android.view.View
import android.provider.Settings
import androidx.annotation.NonNull
import androidx.collection.ArraySet

import java.security.MessageDigest
import java.security.NoSuchAlgorithmException
import java.util.Arrays
import java.util.Collections
import java.util.Locale
import java.util.Objects

class AdMob(crossbow: Crossbow) : CrossbowPlugin(crossbow) {
    private var aIsInitialized = false
    private var aActivity: Activity? = null
    private var aConsentInformation: ConsentInformation? = null
    private var aIsTestEuropeUserConsent = false
    private var aIsForChildDirectedTreatment = false
    private var aIsBannerLoaded = false
    private var aIsInterstitialLoaded = false
    private var aIsRewardedLoaded = false
    private var aIsRewardedInterstitialLoaded = false
    // Stores the crossbow layout
    private var aCrossbowLayout: FrameLayout? = null
    // Stores the crossbow layout params
    private var aCrossbowLayoutParams: FrameLayout.LayoutParams? = null
    // View of banner
    private var aAdView: AdView? = null
    // The adSize of banner
    private var aAdSize: AdSize? = null
    private var aInterstitialAd: InterstitialAd? = null
    private var aRewardedAd: RewardedAd? = null
    private var aRewardedInterstitialAd: RewardedInterstitialAd? = null

    override fun onMainCreate(pActivity: Activity): View? {
        aActivity = pActivity
        aCrossbowLayout = FrameLayout(pActivity)
        return aCrossbowLayout
    }

    override val pluginName: String
        get() = javaClass.simpleName

    override val pluginSignals: Set<SignalInfo>
        get() {
            val s: MutableSet<SignalInfo> = ArraySet<SignalInfo>()
            s.add(SignalInfo("initialization_complete", Int::class.java, String::class.java))
            s.add(SignalInfo("consent_form_dismissed"))
            s.add(SignalInfo("consent_status_changed", String::class.java))
            s.add(SignalInfo("consent_form_load_failure", Int::class.java, String::class.java))
            s.add(SignalInfo("consent_info_update_success", String::class.java))
            s.add(SignalInfo("consent_info_update_failure", Int::class.java, String::class.java))
            s.add(SignalInfo("banner_loaded"))
            s.add(SignalInfo("banner_failed_to_load", Int::class.java))
            s.add(SignalInfo("banner_opened"))
            s.add(SignalInfo("banner_clicked"))
            s.add(SignalInfo("banner_closed"))
            s.add(SignalInfo("banner_recorded_impression"))
            s.add(SignalInfo("banner_destroyed"))
            s.add(SignalInfo("interstitial_failed_to_load", Int::class.java))
            s.add(SignalInfo("interstitial_loaded"))
            s.add(SignalInfo("interstitial_failed_to_show", Int::class.java))
            s.add(SignalInfo("interstitial_opened"))
            s.add(SignalInfo("interstitial_closed"))
            s.add(SignalInfo("rewarded_ad_failed_to_load", Int::class.java))
            s.add(SignalInfo("rewarded_ad_loaded"))
            s.add(SignalInfo("rewarded_ad_failed_to_show", Int::class.java))
            s.add(SignalInfo("rewarded_ad_opened"))
            s.add(SignalInfo("rewarded_ad_closed"))
            s.add(SignalInfo("rewarded_interstitial_ad_failed_to_load", Int::class.java))
            s.add(SignalInfo("rewarded_interstitial_ad_loaded"))
            s.add(SignalInfo("rewarded_interstitial_ad_failed_to_show", Int::class.java))
            s.add(SignalInfo("rewarded_interstitial_ad_opened"))
            s.add(SignalInfo("rewarded_interstitial_ad_closed"))
            s.add(SignalInfo("user_earned_rewarded", String::class.java, Int::class.java))
            return s
        }

    @ExposedToCrossbow
    fun get_is_initialized(): Boolean {
        return aIsInitialized
    }

    @ExposedToCrossbow
    fun get_is_banner_loaded(): Boolean {
        return aIsBannerLoaded
    }

    @ExposedToCrossbow
    fun get_is_interstitial_loaded(): Boolean {
        return aIsInterstitialLoaded
    }

    @ExposedToCrossbow
    fun get_is_rewarded_loaded(): Boolean {
        return aIsRewardedLoaded
    }

    @ExposedToCrossbow
    fun get_is_rewarded_interstitial_loaded(): Boolean {
        return aIsRewardedInterstitialLoaded
    }

    @ExposedToCrossbow
    fun initialize(
        pIsForChildDirectedTreatment: Boolean,
        pMaxAdContentRating: String,
        pIsReal: Boolean,
        pIsTestEuropeUserConsent: Boolean
    ) {
        if (!aIsInitialized) {
            aIsForChildDirectedTreatment = pIsForChildDirectedTreatment
            aConsentInformation = UserMessagingPlatform.getConsentInformation(aActivity!!)
            aIsTestEuropeUserConsent = pIsTestEuropeUserConsent
            // First call MobileAds.setRequestConfiguration https://groups.google.com/g/google-admob-ads-sdk/c/17oVu0sABjs
            setMobileAdsRequestConfiguration(
                aIsForChildDirectedTreatment,
                pMaxAdContentRating,
                pIsReal
            )
            MobileAds.initialize(aActivity!!) { initializationStatus ->
                val statusGADMobileAds: Int = Objects.requireNonNull(
                    initializationStatus.getAdapterStatusMap()
                        .get("com.google.android.gms.ads.MobileAds")!!
                ).getInitializationState().ordinal
                if (statusGADMobileAds == 0) {
                    aIsInitialized = false
                } else if (statusGADMobileAds == 1) {
                    aIsInitialized = true
                }
                emitSignal("initialization_complete", statusGADMobileAds, "GADMobileAds")
            }
        }
    }

    @ExposedToCrossbow
    fun request_user_consent() {
        aConsentInformation = UserMessagingPlatform.getConsentInformation(aActivity!!)
        val paramsBuilder: ConsentRequestParameters.Builder =
            ConsentRequestParameters.Builder().setTagForUnderAgeOfConsent(aIsForChildDirectedTreatment)
        val params: ConsentRequestParameters
        // https://developers.google.com/admob/ump/android/quick-start#testing
        params = if (aIsTestEuropeUserConsent) {
            val debugSettings: ConsentDebugSettings = ConsentDebugSettings.Builder(aActivity!!)
                .setDebugGeography(ConsentDebugSettings.DebugGeography.DEBUG_GEOGRAPHY_EEA)
                .addTestDeviceHashedId(deviceId)
                .build()
            paramsBuilder.setConsentDebugSettings(debugSettings).build()
        } else {
            paramsBuilder.build()
        }
        aConsentInformation!!.requestConsentInfoUpdate(aActivity!!, params,
            {
                if (aConsentInformation!!.isConsentFormAvailable()) {
                    emitSignal("consent_info_update_success", "Consent Form Available")
                    loadConsentForm()
                } else {
                    emitSignal("consent_info_update_success", "Consent Form not Available")
                }
            }
        ) { formError ->
            emitSignal(
                "consent_info_update_failure",
                formError.getErrorCode(),
                formError.getMessage()
            )
        }
    }

    // https://developers.google.com/admob/ump/android/quick-start#reset_consent_state
    @ExposedToCrossbow
    fun reset_consent_state() {
        aConsentInformation!!.reset()
    }

    // BANNER only one is allowed, please do not try to place more than one, as your ads on the app may have the chance to be banned!
    @ExposedToCrossbow
    fun load_banner(
        pAdUnitId: String?,
        pPosition: Int,
        pSize: String?,
        pShowInstantly: Boolean,
        pRespectSafeArea: Boolean
    ) {
        aActivity!!.runOnUiThread(Runnable {
            if (aIsInitialized) {
                if (aAdView != null) destroy_banner()
                aAdView = AdView(aActivity!!)
                aAdView!!.setAdUnitId(pAdUnitId!!)
                when (pSize) {
                    "BANNER" -> aAdView!!.setAdSize(AdSize.BANNER)
                    "LARGE_BANNER" -> aAdView!!.setAdSize(AdSize.LARGE_BANNER)
                    "MEDIUM_RECTANGLE" -> aAdView!!.setAdSize(AdSize.MEDIUM_RECTANGLE)
                    "FULL_BANNER" -> aAdView!!.setAdSize(AdSize.FULL_BANNER)
                    "LEADERBOARD" -> aAdView!!.setAdSize(AdSize.LEADERBOARD)
                    else -> aAdView!!.setAdSize(adSizeAdaptive)
                }
                // Store AdSize of banner due a bug (throws error when do aAdView!!.getAdSize() called by Crossbow)
                aAdSize = aAdView!!.getAdSize()
                aAdView!!.setAdListener(object : AdListener() {
                    override fun onAdLoaded() {
                        // Code to be executed when an ad finishes loading.
                        emitSignal("banner_loaded")
                        if (pShowInstantly) {
                            show_banner()
                        } else {
                            hide_banner()
                        }
                        aIsBannerLoaded = true
                    }

                    override fun onAdFailedToLoad(adError: LoadAdError) {
                        // Code to be executed when an ad request fails.
                        emitSignal("banner_failed_to_load", adError.getCode())
                    }

                    override fun onAdOpened() {
                        // Code to be executed when an ad opens an overlay that
                        // covers the screen.
                        emitSignal("banner_opened")
                    }

                    override fun onAdClicked() {
                        // Code to be executed when the native ad is closed.
                        emitSignal("banner_clicked")
                    }

                    override fun onAdClosed() {
                        // Code to be executed when the user is about to return
                        // to the app after tapping on an ad.
                        emitSignal("banner_closed")
                    }

                    override fun onAdImpression() {
                        // Code to be executed when the user is about to return
                        // to the app after tapping on an ad.
                        emitSignal("banner_recorded_impression")
                    }
                })
                aCrossbowLayoutParams = FrameLayout.LayoutParams(
                    FrameLayout.LayoutParams.MATCH_PARENT,
                    FrameLayout.LayoutParams.WRAP_CONTENT
                )
                if (pPosition == 0) {
                    aCrossbowLayoutParams!!.gravity = Gravity.BOTTOM
                    // Need to validate if this value will be positive or negative
                    if (pRespectSafeArea) aAdView!!.setY(-safeArea.bottom.toFloat())
                } else if (pPosition == 1) {
                    aCrossbowLayoutParams!!.gravity = Gravity.TOP
                    if (pRespectSafeArea) aAdView!!.setY(safeArea.top.toFloat())
                }
                aCrossbowLayout!!.addView(aAdView, aCrossbowLayoutParams)
                aAdView!!.loadAd(adRequest)
            }
        })
    }

    // IF THIS METHOD IS CALLED, THE BANNER WILL ONLY APPEAR AGAIN IF THE BANNER IS LOADED AGAIN
    @ExposedToCrossbow
    fun destroy_banner()
    {
        aActivity!!.runOnUiThread(Runnable {
            if (aIsInitialized && aAdView != null) {
                aCrossbowLayout!!.removeView(aAdView)
                aAdView!!.destroy()
                aAdView = null
                emitSignal("banner_destroyed")
                aIsBannerLoaded = false
            }
        })
    }

    @ExposedToCrossbow
    fun show_banner() {
        aActivity!!.runOnUiThread(Runnable {
            if (aIsInitialized && aAdView != null) {
                if (aAdView!!.getVisibility() == View.VISIBLE) {
                    aAdView!!.setVisibility(View.VISIBLE)
                    aAdView!!.resume()
                }
            }
        })
    }

    @ExposedToCrossbow
    fun hide_banner() {
        aActivity!!.runOnUiThread(Runnable {
            if (aIsInitialized && aAdView != null) {
                if (aAdView!!.getVisibility() == View.GONE) {
                    aAdView!!.setVisibility(View.GONE)
                    aAdView!!.pause()
                }
            }
        })
    }

    @ExposedToCrossbow
    fun get_banner_width(): Int {
        return if (aIsInitialized && aAdSize != null) {
            aAdSize!!.getWidth()
        } else 0
    }

    @ExposedToCrossbow
    fun get_banner_height(): Int {
        return if (aIsInitialized && aAdSize != null) {
            aAdSize!!.getHeight()
        } else 0
    }

    @ExposedToCrossbow
    fun get_banner_width_in_pixels(): Int {
        return if (aIsInitialized && aAdSize != null) {
            aAdSize!!.getWidthInPixels(aActivity!!)
        } else 0
    }

    @ExposedToCrossbow
    fun get_banner_height_in_pixels(): Int {
        return if (aIsInitialized && aAdSize != null) {
            aAdSize!!.getHeightInPixels(aActivity!!)
        } else 0
    }

    // INTERSTITIAL
    @ExposedToCrossbow
    fun load_interstitial(pAdUnitId: String?) {
        aActivity!!.runOnUiThread(Runnable {
            if (aIsInitialized) {
                InterstitialAd.load(
                    aActivity!!,
                    pAdUnitId!!,
                    adRequest,
                    object : InterstitialAdLoadCallback() {
                        override fun onAdLoaded(interstitialAd: InterstitialAd) {
                            // Code to be executed when an ad finishes loading.
                            aInterstitialAd = interstitialAd
                            emitSignal("interstitial_loaded")
                            aIsInterstitialLoaded = true
                            interstitialAd.setFullScreenContentCallback(object :
                                FullScreenContentCallback() {
                                override fun onAdDismissedFullScreenContent() {
                                    // Called when fullscreen content is dismissed.
                                    aInterstitialAd = null
                                    emitSignal("interstitial_closed")
                                    aIsInterstitialLoaded = false
                                }

                                override fun onAdFailedToShowFullScreenContent(adError: AdError) {
                                    // Called when fullscreen content failed to show.
                                    aInterstitialAd = null
                                    emitSignal("interstitial_failed_to_show", adError.getCode())
                                    aIsInterstitialLoaded = false
                                }

                                override fun onAdShowedFullScreenContent() {
                                    // Called when fullscreen content is shown.
                                    emitSignal("interstitial_opened")
                                }
                            })
                        }

                        override fun onAdFailedToLoad(adError: LoadAdError) {
                            // Code to be executed when an ad request fails.
                            aInterstitialAd = null
                            emitSignal("interstitial_failed_to_load", adError.getCode())
                        }
                    })
            }
        })
    }

    @ExposedToCrossbow
    fun show_interstitial() {
        aActivity!!.runOnUiThread(Runnable {
            if (aIsInitialized) {
                if (aInterstitialAd != null) {
                    aInterstitialAd!!.show(aActivity!!)
                }
            }
        })
    }

    // REWARDED INTERSTITIAL
    @ExposedToCrossbow
    fun load_rewarded(pAdUnitId: String?) {
        aActivity!!.runOnUiThread(Runnable {
            if (aIsInitialized) {
                RewardedAd.load(aActivity!!, pAdUnitId!!, adRequest, object : RewardedAdLoadCallback() {
                    override fun onAdFailedToLoad(loadAdError: LoadAdError) {
                        // Handle the error.
                        aRewardedAd = null
                        emitSignal("rewarded_ad_failed_to_load", loadAdError.getCode())
                    }

                    override fun onAdLoaded(rewardedAd: RewardedAd) {
                        aRewardedAd = rewardedAd
                        emitSignal("rewarded_ad_loaded")
                        aIsRewardedLoaded = true
                    }
                })
            }
        })
    }

    @ExposedToCrossbow
    fun show_rewarded() {
        aActivity!!.runOnUiThread(Runnable {
            if (aIsInitialized) {
                if (aRewardedAd != null) {
                    aRewardedAd!!.setFullScreenContentCallback(object : FullScreenContentCallback() {
                        override fun onAdShowedFullScreenContent() {
                            // Called when ad is shown.
                            emitSignal("rewarded_ad_opened")
                        }

                        override fun onAdFailedToShowFullScreenContent(adError: AdError) {
                            // Called when ad fails to show.
                            aRewardedAd = null
                            emitSignal("rewarded_ad_failed_to_show", adError.getCode())
                        }

                        override fun onAdDismissedFullScreenContent() {
                            // Called when ad is dismissed.
                            aRewardedAd = null
                            emitSignal("rewarded_ad_closed")
                            aIsRewardedLoaded = false
                        }
                    })
                    aRewardedAd!!.show(aActivity!!) { rewardItem ->
                        // Handle the reward.
                        emitSignal(
                            "user_earned_rewarded",
                            rewardItem.getType(),
                            rewardItem.getAmount()
                        )
                    }
                }
            }
        })
    }

    @ExposedToCrossbow
    fun load_rewarded_interstitial(pAdUnitId: String?) {
        aActivity!!.runOnUiThread(Runnable {
            if (aIsInitialized) {
                RewardedInterstitialAd.load(
                    aActivity!!,
                    pAdUnitId!!,
                    adRequest,
                    object : RewardedInterstitialAdLoadCallback() {
                        override fun onAdFailedToLoad(loadAdError: LoadAdError) {
                            // Handle the error.
                            aRewardedInterstitialAd = null
                            emitSignal(
                                "rewarded_interstitial_ad_failed_to_load",
                                loadAdError.getCode()
                            )
                        }

                        override fun onAdLoaded(rewardedInterstitialAd: RewardedInterstitialAd) {
                            aRewardedInterstitialAd = rewardedInterstitialAd
                            emitSignal("rewarded_interstitial_ad_loaded")
                            aIsRewardedInterstitialLoaded = true
                        }
                    })
            }
        })
    }

    @ExposedToCrossbow
    fun show_rewarded_interstitial() {
        aActivity!!.runOnUiThread(Runnable {
            if (aIsInitialized) {
                if (aRewardedInterstitialAd != null) {
                    aRewardedInterstitialAd!!.setFullScreenContentCallback(object :
                        FullScreenContentCallback() {
                        override fun onAdShowedFullScreenContent() {
                            // Called when ad is shown.
                            emitSignal("rewarded_interstitial_ad_opened")
                        }

                        override fun onAdFailedToShowFullScreenContent(adError: AdError) {
                            // Called when ad fails to show.
                            aRewardedInterstitialAd = null
                            emitSignal("rewarded_interstitial_ad_failed_to_show", adError.getCode())
                            aIsRewardedInterstitialLoaded = false
                        }

                        override fun onAdDismissedFullScreenContent() {
                            // Called when ad is dismissed.
                            aRewardedInterstitialAd = null
                            emitSignal("rewarded_interstitial_ad_closed")
                            aIsRewardedInterstitialLoaded = false
                        }
                    })
                    aRewardedInterstitialAd!!.show(aActivity!!) { rewardItem ->
                        // Handle the reward.
                        emitSignal(
                            "user_earned_rewarded",
                            rewardItem.getType(),
                            rewardItem.getAmount()
                        )
                    }
                }
            }
        })
    }

    private fun loadConsentForm() {
        UserMessagingPlatform.loadConsentForm(
            aActivity!!,
            { consentForm ->
                var consentStatusMsg = ""
                if (aConsentInformation!!.getConsentStatus() == ConsentInformation.ConsentStatus.REQUIRED) {
                    consentForm.show(
                        aActivity!!
                    ) { _ ->
                        loadConsentForm()
                        emitSignal("consent_form_dismissed")
                    }
                    consentStatusMsg = "User consent required but not yet obtained."
                }
                when (aConsentInformation!!.getConsentStatus()) {
                    ConsentInformation.ConsentStatus.UNKNOWN -> consentStatusMsg =
                        "Unknown consent status."
                    ConsentInformation.ConsentStatus.NOT_REQUIRED -> consentStatusMsg =
                        "User consent not required. For example, the user is not in the EEA or the UK."
                    ConsentInformation.ConsentStatus.OBTAINED -> consentStatusMsg =
                        "User consent obtained. Personalization not defined."
                }
                emitSignal("consent_status_changed", consentStatusMsg)
            }
        ) { formError ->
            emitSignal(
                "consent_form_load_failure",
                formError.getErrorCode(),
                formError.getMessage()
            )
        }
    }

    private fun setMobileAdsRequestConfiguration(
        pIsForChildDirectedTreatment: Boolean,
        pMaxAdContentRating: String,
        pIsReal: Boolean
    ) {
        val requestConfiguration: RequestConfiguration
        val requestConfigurationBuilder = RequestConfiguration.Builder()
        if (!pIsReal) {
            requestConfigurationBuilder.setTestDeviceIds(listOf(deviceId))
        }
        requestConfigurationBuilder.setTagForChildDirectedTreatment(if (pIsForChildDirectedTreatment) 1 else 0)
        if (pIsForChildDirectedTreatment) {
            requestConfigurationBuilder.setMaxAdContentRating(RequestConfiguration.MAX_AD_CONTENT_RATING_G)
        } else {
            when (pMaxAdContentRating) {
                RequestConfiguration.MAX_AD_CONTENT_RATING_G, RequestConfiguration.MAX_AD_CONTENT_RATING_MA, RequestConfiguration.MAX_AD_CONTENT_RATING_PG, RequestConfiguration.MAX_AD_CONTENT_RATING_T, RequestConfiguration.MAX_AD_CONTENT_RATING_UNSPECIFIED -> requestConfigurationBuilder.setMaxAdContentRating(
                    pMaxAdContentRating
                )
            }
        }
        requestConfiguration = requestConfigurationBuilder.build()
        MobileAds.setRequestConfiguration(requestConfiguration)
    }

    private val adRequest: AdRequest
        get() {
            val adRequestBuilder = AdRequest.Builder()
            return adRequestBuilder.build()
        }

    private val safeArea: Rect
        get() {
            val safeInsetRect = Rect()
            if (Build.VERSION.SDK_INT < Build.VERSION_CODES.P) {
                return safeInsetRect
            }
            val windowInsets: WindowInsets =
                aActivity!!.getWindow().getDecorView().getRootWindowInsets()
                    ?: return safeInsetRect
            val displayCutout = windowInsets.getDisplayCutout()
            if (displayCutout != null) {
                safeInsetRect.set(
                    displayCutout.getSafeInsetLeft(),
                    displayCutout.getSafeInsetTop(),
                    displayCutout.getSafeInsetRight(),
                    displayCutout.getSafeInsetBottom()
                )
            }
            return safeInsetRect
        }

    // Determine the screen width (less decorations) to use for the ad width.
    private val adSizeAdaptive: AdSize
        // If the ad hasn't been laid out, default to the full screen width.
        get() {
            // Determine the screen width (less decorations) to use for the ad width.
            val outMetrics = DisplayMetrics()
            if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.R) {
                val display = aActivity!!.display
                @Suppress("DEPRECATION")
                display?.getRealMetrics(outMetrics)
            } else {
                @Suppress("DEPRECATION")
                val display = aActivity!!.windowManager.defaultDisplay
                @Suppress("DEPRECATION")
                display.getMetrics(outMetrics)
            }
            val density: Float = outMetrics.density
            var adWidthPixels: Float = aCrossbowLayout!!.getWidth().toFloat()

            // If the ad hasn't been laid out, default to the full screen width.
            if (adWidthPixels == 0f) {
                adWidthPixels = outMetrics.widthPixels.toFloat()
            }
            val adWidth = (adWidthPixels / density).toInt()
            return AdSize.getCurrentOrientationAnchoredAdaptiveBannerAdSize(aActivity!!, adWidth)
        }

    /**
     * Generate MD5 for the deviceID
     * @param  s The string to generate de MD5
     * @return String The MD5 generated
     */
    private fun md5(s: String): String {
        try {
            // Create MD5 Hash
            val digest: MessageDigest = MessageDigest.getInstance("MD5")
            digest.update(s.toByteArray())
            val messageDigest: ByteArray = digest.digest()

            // Create Hex String
            val hexString = StringBuilder()
            for (b in messageDigest) {
                val h = StringBuilder(Integer.toHexString(0xFF and b.toInt()))
                while (h.length < 2) h.insert(0, "0")
                hexString.append(h)
            }
            return hexString.toString()
        } catch (e: NoSuchAlgorithmException) {
            //Logger.logStackTrace(TAG,e)
        }
        return ""
    }

    /**
     * Get the Device ID for AdMob
     * @return String Device ID
     */
    private val deviceId: String
         get() {
            val android_id = Settings.Secure.getString(
                aActivity!!.getContentResolver(),
                Settings.Secure.ANDROID_ID
            )
            return md5(android_id).uppercase()
        }
}

package com.crossbow.play_billing

import com.crossbow.library.Crossbow
import com.crossbow.library.plugin.SignalInfo
import com.crossbow.library.plugin.CrossbowPlugin
import com.crossbow.library.plugin.ExposedToCrossbow
import com.crossbow.library.Dictionary

import com.android.billingclient.api.PurchasesUpdatedListener
import com.android.billingclient.api.BillingClientStateListener
import com.android.billingclient.api.PriceChangeConfirmationListener
import com.android.billingclient.api.BillingClient
import com.android.billingclient.api.SkuDetails
import com.android.billingclient.api.BillingResult
import com.android.billingclient.api.Purchase
import com.android.billingclient.api.BillingFlowParams
import com.android.billingclient.api.BillingFlowParams.SubscriptionUpdateParams
import com.android.billingclient.api.PurchasesResponseListener
import com.android.billingclient.api.SkuDetailsParams
import com.android.billingclient.api.SkuDetailsResponseListener
import com.android.billingclient.api.AcknowledgePurchaseParams
import com.android.billingclient.api.AcknowledgePurchaseResponseListener
import com.android.billingclient.api.ConsumeParams
import com.android.billingclient.api.ConsumeResponseListener
import com.android.billingclient.api.PriceChangeFlowParams

import androidx.collection.ArraySet
import java.util.HashMap
import java.util.Arrays
import java.util.ArrayList

class CrossbowPlayBilling(crossbow: Crossbow) : CrossbowPlugin(crossbow),
    PurchasesUpdatedListener, BillingClientStateListener, PriceChangeConfirmationListener {
    private val billingClient: BillingClient
    private val skuDetailsCache = HashMap<String, SkuDetails>()
    private var calledStartConnection: Boolean
    private var obfuscatedAccountId: String
    private var obfuscatedProfileId: String

    init {
        billingClient = BillingClient
            .newBuilder(activity!!)
            .enablePendingPurchases()
            .setListener(this)
            .build()
        calledStartConnection = false
        obfuscatedAccountId = ""
        obfuscatedProfileId = ""
    }

    override val pluginName: String
        get() = javaClass.simpleName

    override val pluginSignals: Set<SignalInfo>
        get() {
            val signals: MutableSet<SignalInfo> = ArraySet()
            signals.add(SignalInfo("connected"))
            signals.add(SignalInfo("disconnected"))
            signals.add(SignalInfo("billing_resume"))
            signals.add(SignalInfo("connect_error", Int::class.java, String::class.java))
            signals.add(SignalInfo("purchases_updated", Array<Any>::class.java))
            signals.add(SignalInfo("query_purchases_response", Any::class.java))
            signals.add(SignalInfo("purchase_error", Int::class.java, String::class.java))
            signals.add(SignalInfo("sku_details_query_completed", Array<Any>::class.java))
            signals.add(
                SignalInfo(
                    "sku_details_query_error",
                    Int::class.java,
                    String::class.java,
                    Array<String>::class.java
                )
            )
            signals.add(SignalInfo("price_change_acknowledged", Int::class.java))
            signals.add(SignalInfo("purchase_acknowledged", String::class.java))
            signals.add(
                SignalInfo(
                    "purchase_acknowledgement_error",
                    Int::class.java,
                    String::class.java,
                    String::class.java
                )
            )
            signals.add(SignalInfo("purchase_consumed", String::class.java))
            signals.add(
                SignalInfo(
                    "purchase_consumption_error",
                    Int::class.java,
                    String::class.java,
                    String::class.java
                )
            )
            return signals
        }

    override fun onBillingSetupFinished(billingResult: BillingResult) {
        if (billingResult.responseCode == BillingClient.BillingResponseCode.OK) {
            emitSignal("connected")
        } else {
            emitSignal("connect_error", billingResult.responseCode, billingResult.debugMessage)
        }
    }

    override fun onBillingServiceDisconnected() {
        emitSignal("disconnected")
    }

    override fun onPurchasesUpdated(billingResult: BillingResult, list: List<Purchase>?) {
        if (billingResult.responseCode == BillingClient.BillingResponseCode.OK && list != null) {
            emitSignal(
                "purchases_updated",
                CrossbowPlayBillingUtils.convertPurchaseListToDictionaryObjectArray(list) as Any
            )
        } else {
            emitSignal("purchase_error", billingResult.responseCode, billingResult.debugMessage)
        }
    }

    override fun onPriceChangeConfirmationResult(billingResult: BillingResult) {
        emitSignal("price_change_acknowledged", billingResult.responseCode)
    }

    override fun onMainResume() {
        if (calledStartConnection) {
            emitSignal("billing_resume")
        }
    }

    private fun purchaseInternal(oldToken: String, sku: String, prorationMode: Int): Dictionary {
        if (!skuDetailsCache.containsKey(sku)) {
            val returnValue = Dictionary()
            returnValue["status"] = 1 // FAILED = 1
            returnValue["response_code"] =
                null // Null since there is no ResponseCode to return but to keep the interface (status, response_code, debug_message)
            returnValue["debug_message"] =
                "You must query the sku details and wait for the result before purchasing!"
            return returnValue
        }
        val skuDetails = skuDetailsCache[sku]
        val purchaseParamsBuilder = BillingFlowParams.newBuilder()
        purchaseParamsBuilder.setSkuDetails(skuDetails!!)
        if (!obfuscatedAccountId.isEmpty()) {
            purchaseParamsBuilder.setObfuscatedAccountId(obfuscatedAccountId)
        }
        if (!obfuscatedProfileId.isEmpty()) {
            purchaseParamsBuilder.setObfuscatedProfileId(obfuscatedProfileId)
        }
        if (!oldToken.isEmpty() && prorationMode != BillingFlowParams.ProrationMode.UNKNOWN_SUBSCRIPTION_UPGRADE_DOWNGRADE_POLICY) {
            val updateParams = SubscriptionUpdateParams.newBuilder()
                .setOldSkuPurchaseToken(oldToken)
                .setReplaceSkusProrationMode(prorationMode)
                .build()
            purchaseParamsBuilder.setSubscriptionUpdateParams(updateParams)
        }
        val result = billingClient.launchBillingFlow(activity!!, purchaseParamsBuilder.build())
        val returnValue = Dictionary()
        if (result.responseCode == BillingClient.BillingResponseCode.OK) {
            returnValue["status"] = 0 // OK = 0
        } else {
            returnValue["status"] = 1 // FAILED = 1
            returnValue["response_code"] = result.responseCode
            returnValue["debug_message"] = result.debugMessage
        }
        return returnValue
    }

    @ExposedToCrossbow
    fun startConnection() {
        calledStartConnection = true
        billingClient.startConnection(this)
    }

    @ExposedToCrossbow
    fun endConnection() {
        billingClient.endConnection()
    }

    @get:ExposedToCrossbow
    val isReady: Boolean
        get() = billingClient.isReady

    @get:ExposedToCrossbow
    val connectionState: Int
        get() = billingClient.connectionState

    @ExposedToCrossbow
    fun queryPurchases(type: String?) {
        billingClient.queryPurchasesAsync(type!!) { billingResult, purchaseList ->
            val returnValue = Dictionary()
            if (billingResult.responseCode == BillingClient.BillingResponseCode.OK) {
                returnValue["status"] = 0 // OK = 0
                returnValue["purchases"] =
                    CrossbowPlayBillingUtils.convertPurchaseListToDictionaryObjectArray(purchaseList)
            } else {
                returnValue["status"] = 1 // FAILED = 1
                returnValue["response_code"] = billingResult.responseCode
                returnValue["debug_message"] = billingResult.debugMessage
            }
            emitSignal("query_purchases_response", returnValue as Any)
        }
    }

    @ExposedToCrossbow
    fun querySkuDetails(list: Array<String?>, type: String?) {
        val skuList = Arrays.asList(*list)
        val params = SkuDetailsParams.newBuilder()
            .setSkusList(skuList)
            .setType(type!!)
        billingClient.querySkuDetailsAsync(params.build()) { billingResult, skuDetailsList ->
            if (billingResult.responseCode == BillingClient.BillingResponseCode.OK) {
                for (skuDetails in skuDetailsList!!) {
                    skuDetailsCache[skuDetails.sku] = skuDetails
                }
                emitSignal(
                    "sku_details_query_completed",
                    CrossbowPlayBillingUtils.convertSkuDetailsListToDictionaryObjectArray(
                        skuDetailsList
                    ) as Any
                )
            } else {
                emitSignal(
                    "sku_details_query_error",
                    billingResult.responseCode,
                    billingResult.debugMessage,
                    list
                )
            }
        }
    }

    @ExposedToCrossbow
    fun acknowledgePurchase(purchaseToken: String?) {
        val acknowledgePurchaseParams = AcknowledgePurchaseParams.newBuilder()
            .setPurchaseToken(purchaseToken!!)
            .build()
        billingClient.acknowledgePurchase(acknowledgePurchaseParams) { billingResult ->
            if (billingResult.responseCode == BillingClient.BillingResponseCode.OK) {
                emitSignal("purchase_acknowledged", purchaseToken)
            } else {
                emitSignal(
                    "purchase_acknowledgement_error",
                    billingResult.responseCode,
                    billingResult.debugMessage,
                    purchaseToken
                )
            }
        }
    }

    @ExposedToCrossbow
    fun consumePurchase(purchaseToken: String?) {
        val consumeParams = ConsumeParams.newBuilder()
            .setPurchaseToken(purchaseToken!!)
            .build()
        billingClient.consumeAsync(consumeParams) { billingResult, token ->
            if (billingResult.responseCode == BillingClient.BillingResponseCode.OK) {
                emitSignal("purchase_consumed", token)
            } else {
                emitSignal(
                    "purchase_consumption_error",
                    billingResult.responseCode,
                    billingResult.debugMessage,
                    token
                )
            }
        }
    }

    @ExposedToCrossbow
    fun confirmPriceChange(sku: String): Dictionary {
        if (!skuDetailsCache.containsKey(sku)) {
            val returnValue = Dictionary()
            returnValue["status"] = 1 // FAILED = 1
            returnValue["response_code"] =
                null // Null since there is no ResponseCode to return but to keep the interface (status, response_code, debug_message)
            returnValue["debug_message"] =
                "You must query the sku details and wait for the result before confirming a price change!"
            return returnValue
        }
        val skuDetails = skuDetailsCache[sku]
        val priceChangeFlowParams = PriceChangeFlowParams.newBuilder().setSkuDetails(
            skuDetails!!
        ).build()
        billingClient.launchPriceChangeConfirmationFlow(activity!!, priceChangeFlowParams, this)
        val returnValue = Dictionary()
        returnValue["status"] = 0 // OK = 0
        return returnValue
    }

    @ExposedToCrossbow
    fun purchase(sku: String): Dictionary {
        return purchaseInternal(
            "", sku,
            BillingFlowParams.ProrationMode.UNKNOWN_SUBSCRIPTION_UPGRADE_DOWNGRADE_POLICY
        )
    }

    @ExposedToCrossbow
    fun updateSubscription(oldToken: String, sku: String, prorationMode: Int): Dictionary {
        return purchaseInternal(oldToken, sku, prorationMode)
    }

    @ExposedToCrossbow
    fun setObfuscatedAccountId(accountId: String) {
        obfuscatedAccountId = accountId
    }

    @ExposedToCrossbow
    fun setObfuscatedProfileId(profileId: String) {
        obfuscatedProfileId = profileId
    }
}

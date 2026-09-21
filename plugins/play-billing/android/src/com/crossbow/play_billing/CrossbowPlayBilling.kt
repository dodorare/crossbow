package com.crossbow.play_billing

import androidx.collection.ArraySet
import com.android.billingclient.api.AcknowledgePurchaseParams
import com.android.billingclient.api.BillingClient
import com.android.billingclient.api.BillingClientStateListener
import com.android.billingclient.api.BillingFlowParams
import com.android.billingclient.api.BillingResult
import com.android.billingclient.api.ConsumeParams
import com.android.billingclient.api.PendingPurchasesParams
import com.android.billingclient.api.ProductDetails
import com.android.billingclient.api.Purchase
import com.android.billingclient.api.PurchasesUpdatedListener
import com.android.billingclient.api.QueryProductDetailsParams
import com.android.billingclient.api.QueryPurchasesParams
import com.crossbow.library.Crossbow
import com.crossbow.library.Dictionary
import com.crossbow.library.plugin.CrossbowPlugin
import com.crossbow.library.plugin.ExposedToCrossbow
import com.crossbow.library.plugin.SignalInfo

class CrossbowPlayBilling(crossbow: Crossbow) : CrossbowPlugin(crossbow),
    PurchasesUpdatedListener, BillingClientStateListener {
    private val billingClient: BillingClient = BillingClient.newBuilder(activity)
        .enablePendingPurchases(
            PendingPurchasesParams.newBuilder()
                .enableOneTimeProducts()
                .enablePrepaidPlans()
                .build()
        )
        .enableAutoServiceReconnection()
        .setListener(this)
        .build()
    private val productDetailsCache = HashMap<String, ProductDetails>()
    private var calledStartConnection = false
    private var obfuscatedAccountId = ""
    private var obfuscatedProfileId = ""

    override val pluginName: String
        get() = javaClass.simpleName

    override val pluginSignals: Set<SignalInfo>
        get() = ArraySet<SignalInfo>().apply {
            add(SignalInfo("connected"))
            add(SignalInfo("disconnected"))
            add(SignalInfo("billing_resume"))
            add(SignalInfo("connect_error", Int::class.java, String::class.java))
            add(SignalInfo("purchases_updated", Array<Any>::class.java))
            add(SignalInfo("query_purchases_response", Any::class.java))
            add(SignalInfo("purchase_error", Int::class.java, String::class.java))
            add(SignalInfo("product_details_query_completed", Array<Any>::class.java))
            add(SignalInfo("sku_details_query_completed", Array<Any>::class.java))
            add(
                SignalInfo(
                    "product_details_query_error",
                    Int::class.java,
                    String::class.java,
                    Array<String>::class.java,
                )
            )
            add(
                SignalInfo(
                    "sku_details_query_error",
                    Int::class.java,
                    String::class.java,
                    Array<String>::class.java,
                )
            )
            add(SignalInfo("price_change_acknowledged", Int::class.java))
            add(SignalInfo("purchase_acknowledged", String::class.java))
            add(
                SignalInfo(
                    "purchase_acknowledgement_error",
                    Int::class.java,
                    String::class.java,
                    String::class.java,
                )
            )
            add(SignalInfo("purchase_consumed", String::class.java))
            add(
                SignalInfo(
                    "purchase_consumption_error",
                    Int::class.java,
                    String::class.java,
                    String::class.java,
                )
            )
        }

    override fun onBillingSetupFinished(billingResult: BillingResult) {
        if (billingResult.responseCode == BillingClient.BillingResponseCode.OK) {
            emitSignal("connected")
        } else {
            emitSignal("connect_error", billingResult.responseCode, billingResult.debugMessage)
        }
    }

    override fun onBillingServiceDisconnected() = emitSignal("disconnected")

    override fun onPurchasesUpdated(billingResult: BillingResult, purchases: List<Purchase>?) {
        if (billingResult.responseCode == BillingClient.BillingResponseCode.OK && purchases != null) {
            emitSignal(
                "purchases_updated",
                CrossbowPlayBillingUtils.convertPurchaseListToDictionaryObjectArray(purchases) as Any,
            )
        } else {
            emitSignal("purchase_error", billingResult.responseCode, billingResult.debugMessage)
        }
    }

    override fun onMainResume() {
        if (calledStartConnection) emitSignal("billing_resume")
    }

    private fun failure(message: String, responseCode: Int? = null) = Dictionary().apply {
        this["status"] = 1
        this["response_code"] = responseCode
        this["debug_message"] = message
    }

    private fun purchaseInternal(
        legacyOldToken: String,
        oldProductId: String,
        productId: String,
        offerToken: String,
        replacementMode: Int,
    ): Dictionary {
        val details = productDetailsCache[productId]
            ?: return failure("Query product details and wait for the result before purchasing.")
        val productParams = BillingFlowParams.ProductDetailsParams.newBuilder()
            .setProductDetails(details)
            .apply {
                val selectedOfferToken = offerToken.ifEmpty {
                    details.subscriptionOfferDetails?.firstOrNull()?.offerToken.orEmpty()
                }
                if (
                    selectedOfferToken.isNotEmpty() &&
                    replacementMode != BillingFlowParams.ProductDetailsParams
                        .SubscriptionProductReplacementParams.ReplacementMode.KEEP_EXISTING
                ) {
                    setOfferToken(selectedOfferToken)
                }
                if (oldProductId.isNotEmpty() && replacementMode != 0) {
                    setSubscriptionProductReplacementParams(
                        BillingFlowParams.ProductDetailsParams.SubscriptionProductReplacementParams
                            .newBuilder()
                            .setOldProductId(oldProductId)
                            .setReplacementMode(replacementMode)
                            .build()
                    )
                }
            }
            .build()
        val flowParams = BillingFlowParams.newBuilder()
            .setProductDetailsParamsList(listOf(productParams))
            .apply {
                if (obfuscatedAccountId.isNotEmpty()) setObfuscatedAccountId(obfuscatedAccountId)
                if (obfuscatedProfileId.isNotEmpty()) setObfuscatedProfileId(obfuscatedProfileId)
                if (legacyOldToken.isNotEmpty()) {
                    val updateParams = BillingFlowParams.SubscriptionUpdateParams.newBuilder()
                        .setOldPurchaseToken(legacyOldToken)
                        .apply {
                            if (oldProductId.isEmpty() && replacementMode != 0) {
                                @Suppress("DEPRECATION")
                                setSubscriptionReplacementMode(replacementMode)
                            }
                        }
                        .build()
                    setSubscriptionUpdateParams(updateParams)
                }
            }
            .build()
        val result = billingClient.launchBillingFlow(activity, flowParams)
        return if (result.responseCode == BillingClient.BillingResponseCode.OK) {
            Dictionary().apply { this["status"] = 0 }
        } else {
            failure(result.debugMessage, result.responseCode)
        }
    }

    @ExposedToCrossbow
    fun startConnection() {
        calledStartConnection = true
        billingClient.startConnection(this)
    }

    @ExposedToCrossbow
    fun endConnection() = billingClient.endConnection()

    @get:ExposedToCrossbow
    val isReady: Boolean
        get() = billingClient.isReady

    @get:ExposedToCrossbow
    val connectionState: Int
        get() = billingClient.connectionState

    @ExposedToCrossbow
    fun queryPurchases(type: String) {
        val params = QueryPurchasesParams.newBuilder().setProductType(type).build()
        billingClient.queryPurchasesAsync(params) { billingResult, purchases ->
            val response = if (billingResult.responseCode == BillingClient.BillingResponseCode.OK) {
                Dictionary().apply {
                    this["status"] = 0
                    this["purchases"] =
                        CrossbowPlayBillingUtils.convertPurchaseListToDictionaryObjectArray(purchases)
                }
            } else {
                failure(billingResult.debugMessage, billingResult.responseCode)
            }
            emitSignal("query_purchases_response", response as Any)
        }
    }

    @ExposedToCrossbow
    fun queryProductDetails(productIds: Array<String?>, type: String) {
        val products = productIds.filterNotNull().map { productId ->
            QueryProductDetailsParams.Product.newBuilder()
                .setProductId(productId)
                .setProductType(type)
                .build()
        }
        if (products.isEmpty()) {
            emitProductQueryError(
                BillingClient.BillingResponseCode.DEVELOPER_ERROR,
                "At least one product ID is required.",
                productIds,
            )
            return
        }
        val params = QueryProductDetailsParams.newBuilder().setProductList(products).build()
        billingClient.queryProductDetailsAsync(params) { billingResult, result ->
            if (billingResult.responseCode == BillingClient.BillingResponseCode.OK) {
                result.productDetailsList.forEach { details ->
                    productDetailsCache[details.productId] = details
                }
                val converted = CrossbowPlayBillingUtils
                    .convertProductDetailsListToDictionaryObjectArray(result.productDetailsList)
                emitSignal("product_details_query_completed", converted as Any)
                // Legacy signal retained so existing Rust applications keep working.
                emitSignal("sku_details_query_completed", converted as Any)
                if (result.unfetchedProductList.isNotEmpty()) {
                    val unfetched = result.unfetchedProductList.map { it.productId }.toTypedArray()
                    emitProductQueryError(
                        BillingClient.BillingResponseCode.ITEM_UNAVAILABLE,
                        "Some products could not be fetched.",
                        unfetched,
                    )
                }
            } else {
                emitProductQueryError(
                    billingResult.responseCode,
                    billingResult.debugMessage,
                    productIds,
                )
            }
        }
    }

    @ExposedToCrossbow
    fun querySkuDetails(productIds: Array<String?>, type: String) =
        queryProductDetails(productIds, type)

    private fun emitProductQueryError(code: Int, message: String, productIds: Array<out String?>) {
        val ids = productIds.filterNotNull().toTypedArray()
        emitSignal("product_details_query_error", code, message, ids)
        emitSignal("sku_details_query_error", code, message, ids)
    }

    @ExposedToCrossbow
    fun acknowledgePurchase(purchaseToken: String) {
        val params = AcknowledgePurchaseParams.newBuilder().setPurchaseToken(purchaseToken).build()
        billingClient.acknowledgePurchase(params) { result ->
            if (result.responseCode == BillingClient.BillingResponseCode.OK) {
                emitSignal("purchase_acknowledged", purchaseToken)
            } else {
                emitSignal(
                    "purchase_acknowledgement_error",
                    result.responseCode,
                    result.debugMessage,
                    purchaseToken,
                )
            }
        }
    }

    @ExposedToCrossbow
    fun consumePurchase(purchaseToken: String) {
        val params = ConsumeParams.newBuilder().setPurchaseToken(purchaseToken).build()
        billingClient.consumeAsync(params) { result, token ->
            if (result.responseCode == BillingClient.BillingResponseCode.OK) {
                emitSignal("purchase_consumed", token)
            } else {
                emitSignal(
                    "purchase_consumption_error",
                    result.responseCode,
                    result.debugMessage,
                    token,
                )
            }
        }
    }

    @Deprecated("Price change confirmation was removed from Play Billing Library 9.")
    @ExposedToCrossbow
    fun confirmPriceChange(productId: String): Dictionary = failure(
        "Price change confirmation is no longer available in Play Billing Library 9 ($productId).",
        BillingClient.BillingResponseCode.DEVELOPER_ERROR,
    )

    @ExposedToCrossbow
    fun purchase(productId: String): Dictionary = purchaseInternal("", "", productId, "", 0)

    @ExposedToCrossbow
    fun purchaseWithOffer(productId: String, offerToken: String): Dictionary =
        purchaseInternal("", "", productId, offerToken, 0)

    @Deprecated("Use replaceSubscription with the old purchase token and product ID for Billing 9.")
    @ExposedToCrossbow
    fun updateSubscription(oldToken: String, productId: String, replacementMode: Int): Dictionary =
        purchaseInternal(oldToken, "", productId, "", replacementMode)

    @Deprecated("Use replaceSubscription with the old purchase token and product ID for Billing 9.")
    @ExposedToCrossbow
    fun updateSubscriptionWithOffer(
        oldToken: String,
        productId: String,
        offerToken: String,
        replacementMode: Int,
    ): Dictionary = purchaseInternal(oldToken, "", productId, offerToken, replacementMode)

    @ExposedToCrossbow
    fun replaceSubscription(
        oldPurchaseToken: String,
        oldProductId: String,
        newProductId: String,
        offerToken: String,
        replacementMode: Int,
    ): Dictionary = purchaseInternal(
        oldPurchaseToken,
        oldProductId,
        newProductId,
        offerToken,
        replacementMode,
    )

    @ExposedToCrossbow
    fun setObfuscatedAccountId(accountId: String) {
        obfuscatedAccountId = accountId
    }

    @ExposedToCrossbow
    fun setObfuscatedProfileId(profileId: String) {
        obfuscatedProfileId = profileId
    }
}

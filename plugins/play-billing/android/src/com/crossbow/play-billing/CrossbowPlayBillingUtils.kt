package com.crossbow.play_billing

import com.crossbow.library.Crossbow
import com.crossbow.library.plugin.CrossbowPlugin
import com.crossbow.library.Dictionary
import com.crossbow.library.plugin.ExposedToCrossbow
import com.crossbow.library.plugin.SignalInfo

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

import java.util.HashMap
import java.util.Arrays
import java.util.ArrayList

object CrossbowPlayBillingUtils {
    fun convertPurchaseToDictionary(purchase: Purchase): Dictionary {
        val dictionary = Dictionary()
        dictionary["original_json"] = purchase.originalJson
        dictionary["order_id"] = purchase.orderId
        dictionary["package_name"] = purchase.packageName
        dictionary["purchase_state"] = purchase.purchaseState
        dictionary["purchase_time"] = purchase.purchaseTime
        dictionary["purchase_token"] = purchase.purchaseToken
        dictionary["quantity"] = purchase.quantity
        dictionary["signature"] = purchase.signature
        // PBL V4 replaced getSku with getSkus to support multi-sku purchases,
        // use the first entry for "sku" and generate an array for "skus"
        val skus = purchase.skus
        dictionary["sku"] = skus[0]
        val skusArray = skus.toTypedArray()
        dictionary["skus"] = skusArray
        dictionary["is_acknowledged"] = purchase.isAcknowledged
        dictionary["is_auto_renewing"] = purchase.isAutoRenewing
        return dictionary
    }

    fun convertSkuDetailsToDictionary(details: SkuDetails): Dictionary {
        val dictionary = Dictionary()
        dictionary["sku"] = details.sku
        dictionary["title"] = details.title
        dictionary["description"] = details.description
        dictionary["price"] = details.price
        dictionary["price_currency_code"] = details.priceCurrencyCode
        dictionary["price_amount_micros"] = details.priceAmountMicros
        dictionary["free_trial_period"] = details.freeTrialPeriod
        dictionary["icon_url"] = details.iconUrl
        dictionary["introductory_price"] = details.introductoryPrice
        dictionary["introductory_price_amount_micros"] = details.introductoryPriceAmountMicros
        dictionary["introductory_price_cycles"] = details.introductoryPriceCycles
        dictionary["introductory_price_period"] = details.introductoryPricePeriod
        dictionary["original_price"] = details.originalPrice
        dictionary["original_price_amount_micros"] = details.originalPriceAmountMicros
        dictionary["subscription_period"] = details.subscriptionPeriod
        dictionary["type"] = details.type
        return dictionary
    }

    fun convertPurchaseListToDictionaryObjectArray(purchases: List<Purchase>): Array<Any?> {
        val purchaseDictionaries = arrayOfNulls<Any>(purchases.size)
        for (i in purchases.indices) {
            purchaseDictionaries[i] = convertPurchaseToDictionary(purchases[i])
        }
        return purchaseDictionaries
    }

    fun convertSkuDetailsListToDictionaryObjectArray(skuDetails: List<SkuDetails>?): Array<Any?> {
        val skuDetailsDictionaries = arrayOfNulls<Any>(
            skuDetails!!.size
        )
        for (i in skuDetails.indices) {
            skuDetailsDictionaries[i] = convertSkuDetailsToDictionary(
                skuDetails[i]
            )
        }
        return skuDetailsDictionaries
    }
}
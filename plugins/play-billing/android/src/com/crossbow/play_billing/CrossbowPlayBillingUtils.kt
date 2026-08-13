package com.crossbow.play_billing

import com.android.billingclient.api.ProductDetails
import com.android.billingclient.api.Purchase
import com.crossbow.library.Dictionary

object CrossbowPlayBillingUtils {
    fun convertPurchaseToDictionary(purchase: Purchase): Dictionary {
        val products = purchase.products.toTypedArray()
        return Dictionary().apply {
            this["original_json"] = purchase.originalJson
            this["order_id"] = purchase.orderId
            this["package_name"] = purchase.packageName
            this["purchase_state"] = purchase.purchaseState
            this["purchase_time"] = purchase.purchaseTime
            this["purchase_token"] = purchase.purchaseToken
            this["quantity"] = purchase.quantity
            this["signature"] = purchase.signature
            this["product_id"] = products.firstOrNull()
            this["products"] = products
            // Preserve the Billing 4 field names for existing Rust applications.
            this["sku"] = products.firstOrNull()
            this["skus"] = products
            this["is_acknowledged"] = purchase.isAcknowledged
            this["is_auto_renewing"] = purchase.isAutoRenewing
            this["is_suspended"] = purchase.isSuspended
        }
    }

    fun convertProductDetailsToDictionary(details: ProductDetails): Dictionary {
        val oneTimeOffers = details.oneTimePurchaseOfferDetailsList.orEmpty().map { offer ->
            Dictionary().apply {
                this["formatted_price"] = offer.formattedPrice
                this["price_currency_code"] = offer.priceCurrencyCode
                this["price_amount_micros"] = offer.priceAmountMicros
                this["offer_id"] = offer.offerId
                this["offer_token"] = offer.offerToken
                this["purchase_option_id"] = offer.purchaseOptionId
                this["offer_tags"] = offer.offerTags.orEmpty().toTypedArray()
            }
        }
        val subscriptionOffers = details.subscriptionOfferDetails.orEmpty().map { offer ->
            val pricingPhases = offer.pricingPhases.pricingPhaseList.map { phase ->
                Dictionary().apply {
                    this["billing_cycle_count"] = phase.billingCycleCount
                    this["recurrence_mode"] = phase.recurrenceMode
                    this["price_amount_micros"] = phase.priceAmountMicros
                    this["billing_period"] = phase.billingPeriod
                    this["formatted_price"] = phase.formattedPrice
                    this["price_currency_code"] = phase.priceCurrencyCode
                }
            }
            Dictionary().apply {
                this["base_plan_id"] = offer.basePlanId
                this["offer_id"] = offer.offerId
                this["offer_token"] = offer.offerToken
                this["offer_tags"] = offer.offerTags.toTypedArray()
                this["pricing_phases"] = pricingPhases.toTypedArray()
            }
        }
        val legacyPrice = oneTimeOffers.firstOrNull()
            ?: subscriptionOffers.firstOrNull()?.get("pricing_phases")
                ?.let { it as? Array<*> }
                ?.firstOrNull() as? Dictionary

        return Dictionary().apply {
            this["product_id"] = details.productId
            this["product_type"] = details.productType
            this["name"] = details.name
            this["title"] = details.title
            this["description"] = details.description
            this["one_time_purchase_offer_details"] = oneTimeOffers.toTypedArray()
            this["subscription_offer_details"] = subscriptionOffers.toTypedArray()
            // Preserve the most useful Billing 4 fields during the public API transition.
            this["sku"] = details.productId
            this["type"] = details.productType
            this["price"] = legacyPrice?.get("formatted_price")
            this["price_currency_code"] = legacyPrice?.get("price_currency_code")
            this["price_amount_micros"] = legacyPrice?.get("price_amount_micros")
        }
    }

    fun convertPurchaseListToDictionaryObjectArray(purchases: List<Purchase>): Array<Any?> =
        purchases.map { convertPurchaseToDictionary(it) as Any? }.toTypedArray()

    fun convertProductDetailsListToDictionaryObjectArray(
        productDetails: List<ProductDetails>,
    ): Array<Any?> = productDetails
        .map { convertProductDetailsToDictionary(it) as Any? }
        .toTypedArray()
}

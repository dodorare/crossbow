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
        val legacyOneTimeOffers = details.oneTimePurchaseOfferDetailsList.orEmpty().map { offer ->
            LegacyPricingPhase(
                offer.formattedPrice,
                offer.priceCurrencyCode,
                offer.priceAmountMicros,
            )
        }
        val legacySubscriptionPhases = details.subscriptionOfferDetails
            .orEmpty()
            .firstOrNull()
            ?.pricingPhases
            ?.pricingPhaseList
            .orEmpty()
            .map { phase ->
                LegacyPricingPhase(
                    phase.formattedPrice,
                    phase.priceCurrencyCode,
                    phase.priceAmountMicros,
                    phase.billingPeriod,
                    phase.billingCycleCount,
                )
            }
        val legacyFields = legacyProductDetailsFields(
            legacyOneTimeOffers,
            legacySubscriptionPhases,
        )

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
            putAll(legacyFields)
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

internal data class LegacyPricingPhase(
    val formattedPrice: String,
    val priceCurrencyCode: String,
    val priceAmountMicros: Long,
    val billingPeriod: String = "",
    val billingCycleCount: Int = 0,
)

internal fun legacyProductDetailsFields(
    oneTimeOffers: List<LegacyPricingPhase>,
    subscriptionPhases: List<LegacyPricingPhase>,
): Map<String, Any?> {
    val regularPrice = oneTimeOffers.firstOrNull() ?: subscriptionPhases.lastOrNull()
    val freeTrial = subscriptionPhases.firstOrNull { it.priceAmountMicros == 0L }
    val introductoryPrice = subscriptionPhases
        .dropLast(1)
        .firstOrNull { it.priceAmountMicros > 0L }

    return mapOf(
        "price" to regularPrice?.formattedPrice.orEmpty(),
        "price_currency_code" to regularPrice?.priceCurrencyCode.orEmpty(),
        "price_amount_micros" to (regularPrice?.priceAmountMicros ?: 0L),
        "free_trial_period" to freeTrial?.billingPeriod.orEmpty(),
        "icon_url" to "",
        "introductory_price" to introductoryPrice?.formattedPrice.orEmpty(),
        "introductory_price_amount_micros" to
            (introductoryPrice?.priceAmountMicros ?: 0L),
        "introductory_price_cycles" to (introductoryPrice?.billingCycleCount ?: 0),
        "introductory_price_period" to introductoryPrice?.billingPeriod.orEmpty(),
        "original_price" to regularPrice?.formattedPrice.orEmpty(),
        "original_price_amount_micros" to (regularPrice?.priceAmountMicros ?: 0L),
        "subscription_period" to regularPrice?.billingPeriod.orEmpty(),
    )
}

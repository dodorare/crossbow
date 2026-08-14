package com.crossbow.play_billing

import org.junit.Assert.assertEquals
import org.junit.Test

class LegacyProductDetailsTest {
    @Test
    fun subscription_details_keep_the_billing_4_compatibility_payload() {
        val fields = legacyProductDetailsFields(
            oneTimeOffers = emptyList(),
            subscriptionPhases = listOf(
                LegacyPricingPhase("Free", "USD", 0, "P7D", 1),
                LegacyPricingPhase("\$1.00", "USD", 1_000_000, "P1M", 3),
                LegacyPricingPhase("\$5.00", "USD", 5_000_000, "P1M", 0),
            ),
        )

        assertEquals(
            mapOf(
                "price" to "\$5.00",
                "price_currency_code" to "USD",
                "price_amount_micros" to 5_000_000L,
                "free_trial_period" to "P7D",
                "icon_url" to "",
                "introductory_price" to "\$1.00",
                "introductory_price_amount_micros" to 1_000_000L,
                "introductory_price_cycles" to 3,
                "introductory_price_period" to "P1M",
                "original_price" to "\$5.00",
                "original_price_amount_micros" to 5_000_000L,
                "subscription_period" to "P1M",
            ),
            fields,
        )
    }
}

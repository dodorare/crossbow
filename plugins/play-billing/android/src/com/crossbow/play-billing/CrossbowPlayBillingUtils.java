package com.crossbow.play_billing;

import com.crossbow.library.Dictionary;

import com.android.billingclient.api.Purchase;
import com.android.billingclient.api.SkuDetails;

import java.util.ArrayList;
import java.util.List;

public class CrossbowPlayBillingUtils {
	public static Dictionary convertPurchaseToDictionary(Purchase purchase) {
		Dictionary dictionary = new Dictionary();
		dictionary.put("original_json", purchase.getOriginalJson());
		dictionary.put("order_id", purchase.getOrderId());
		dictionary.put("package_name", purchase.getPackageName());
		dictionary.put("purchase_state", purchase.getPurchaseState());
		dictionary.put("purchase_time", purchase.getPurchaseTime());
		dictionary.put("purchase_token", purchase.getPurchaseToken());
		dictionary.put("quantity", purchase.getQuantity());
		dictionary.put("signature", purchase.getSignature());
		// PBL V4 replaced getSku with getSkus to support multi-sku purchases,
		// use the first entry for "sku" and generate an array for "skus"
		ArrayList<String> skus = purchase.getSkus();
		dictionary.put("sku", skus.get(0));
		String[] skusArray = skus.toArray(new String[0]);
		dictionary.put("skus", skusArray);
		dictionary.put("is_acknowledged", purchase.isAcknowledged());
		dictionary.put("is_auto_renewing", purchase.isAutoRenewing());
		return dictionary;
	}

	public static Dictionary convertSkuDetailsToDictionary(SkuDetails details) {
		Dictionary dictionary = new Dictionary();
		dictionary.put("sku", details.getSku());
		dictionary.put("title", details.getTitle());
		dictionary.put("description", details.getDescription());
		dictionary.put("price", details.getPrice());
		dictionary.put("price_currency_code", details.getPriceCurrencyCode());
		dictionary.put("price_amount_micros", details.getPriceAmountMicros());
		dictionary.put("free_trial_period", details.getFreeTrialPeriod());
		dictionary.put("icon_url", details.getIconUrl());
		dictionary.put("introductory_price", details.getIntroductoryPrice());
		dictionary.put("introductory_price_amount_micros", details.getIntroductoryPriceAmountMicros());
		dictionary.put("introductory_price_cycles", details.getIntroductoryPriceCycles());
		dictionary.put("introductory_price_period", details.getIntroductoryPricePeriod());
		dictionary.put("original_price", details.getOriginalPrice());
		dictionary.put("original_price_amount_micros", details.getOriginalPriceAmountMicros());
		dictionary.put("subscription_period", details.getSubscriptionPeriod());
		dictionary.put("type", details.getType());
		return dictionary;
	}

	public static Object[] convertPurchaseListToDictionaryObjectArray(List<Purchase> purchases) {
		Object[] purchaseDictionaries = new Object[purchases.size()];

		for (int i = 0; i < purchases.size(); i++) {
			purchaseDictionaries[i] = CrossbowPlayBillingUtils.convertPurchaseToDictionary(purchases.get(i));
		}

		return purchaseDictionaries;
	}

	public static Object[] convertSkuDetailsListToDictionaryObjectArray(List<SkuDetails> skuDetails) {
		Object[] skuDetailsDictionaries = new Object[skuDetails.size()];

		for (int i = 0; i < skuDetails.size(); i++) {
			skuDetailsDictionaries[i] = CrossbowPlayBillingUtils.convertSkuDetailsToDictionary(skuDetails.get(i));
		}

		return skuDetailsDictionaries;
	}
}

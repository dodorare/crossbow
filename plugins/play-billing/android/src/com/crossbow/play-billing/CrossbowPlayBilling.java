package com.crossbow.play_billing;

import com.crossbow.library.Dictionary;
import com.crossbow.library.Crossbow;
import com.crossbow.library.plugin.CrossbowPlugin;
import com.crossbow.library.plugin.SignalInfo;
import com.crossbow.library.plugin.ExposedToCrossbow;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.collection.ArraySet;

import com.android.billingclient.api.AcknowledgePurchaseParams;
import com.android.billingclient.api.AcknowledgePurchaseResponseListener;
import com.android.billingclient.api.BillingClient;
import com.android.billingclient.api.BillingClientStateListener;
import com.android.billingclient.api.BillingFlowParams;
import com.android.billingclient.api.BillingResult;
import com.android.billingclient.api.ConsumeParams;
import com.android.billingclient.api.ConsumeResponseListener;
import com.android.billingclient.api.PriceChangeConfirmationListener;
import com.android.billingclient.api.PriceChangeFlowParams;
import com.android.billingclient.api.Purchase;
import com.android.billingclient.api.PurchasesResponseListener;
import com.android.billingclient.api.PurchasesUpdatedListener;
import com.android.billingclient.api.SkuDetails;
import com.android.billingclient.api.SkuDetailsParams;
import com.android.billingclient.api.SkuDetailsResponseListener;

import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Set;

public class CrossbowPlayBilling extends CrossbowPlugin implements PurchasesUpdatedListener, BillingClientStateListener, PriceChangeConfirmationListener {
	private final BillingClient billingClient;
	private final HashMap<String, SkuDetails> skuDetailsCache = new HashMap<>();
	private boolean calledStartConnection;
	private String obfuscatedAccountId;
	private String obfuscatedProfileId;

	public CrossbowPlayBilling(Crossbow crossbow) {
		super(crossbow);

		billingClient = BillingClient
								.newBuilder(getActivity())
								.enablePendingPurchases()
								.setListener(this)
								.build();
		calledStartConnection = false;
		obfuscatedAccountId = "";
		obfuscatedProfileId = "";
	}

	@NonNull
	@Override
	public String getPluginName() {
		return "CrossbowPlayBilling";
	}

	@NonNull
	@Override
	public Set<SignalInfo> getPluginSignals() {
		Set<SignalInfo> signals = new ArraySet<>();

		signals.add(new SignalInfo("connected"));
		signals.add(new SignalInfo("disconnected"));
		signals.add(new SignalInfo("billing_resume"));
		signals.add(new SignalInfo("connect_error", Integer.class, String.class));
		signals.add(new SignalInfo("purchases_updated", Object[].class));
		signals.add(new SignalInfo("query_purchases_response", Object.class));
		signals.add(new SignalInfo("purchase_error", Integer.class, String.class));
		signals.add(new SignalInfo("sku_details_query_completed", Object[].class));
		signals.add(new SignalInfo("sku_details_query_error", Integer.class, String.class, String[].class));
		signals.add(new SignalInfo("price_change_acknowledged", Integer.class));
		signals.add(new SignalInfo("purchase_acknowledged", String.class));
		signals.add(new SignalInfo("purchase_acknowledgement_error", Integer.class, String.class, String.class));
		signals.add(new SignalInfo("purchase_consumed", String.class));
		signals.add(new SignalInfo("purchase_consumption_error", Integer.class, String.class, String.class));

		return signals;
	}

	@Override
	public void onBillingSetupFinished(BillingResult billingResult) {
		if (billingResult.getResponseCode() == BillingClient.BillingResponseCode.OK) {
			emitSignal("connected");
		} else {
			emitSignal("connect_error", billingResult.getResponseCode(), billingResult.getDebugMessage());
		}
	}

	@Override
	public void onBillingServiceDisconnected() {
		emitSignal("disconnected");
	}

	@Override
	public void onPurchasesUpdated(final BillingResult billingResult, @Nullable final List<Purchase> list) {
		if (billingResult.getResponseCode() == BillingClient.BillingResponseCode.OK && list != null) {
			emitSignal("purchases_updated", (Object)CrossbowPlayBillingUtils.convertPurchaseListToDictionaryObjectArray(list));
		} else {
			emitSignal("purchase_error", billingResult.getResponseCode(), billingResult.getDebugMessage());
		}
	}

	@Override
	public void onPriceChangeConfirmationResult(BillingResult billingResult) {
		emitSignal("price_change_acknowledged", billingResult.getResponseCode());
	}

	@Override
	public void onMainResume() {
		if (calledStartConnection) {
			emitSignal("billing_resume");
		}
	}

	@ExposedToCrossbow
	public void startConnection() {
		calledStartConnection = true;
		billingClient.startConnection(this);
	}

	@ExposedToCrossbow
	public void endConnection() {
		billingClient.endConnection();
	}

	@ExposedToCrossbow
	public boolean isReady() {
		return this.billingClient.isReady();
	}

	@ExposedToCrossbow
	public int getConnectionState() {
		return billingClient.getConnectionState();
	}

	@ExposedToCrossbow
	public void queryPurchases(String type) {
		billingClient.queryPurchasesAsync(type, new PurchasesResponseListener() {
			@Override
			public void onQueryPurchasesResponse(BillingResult billingResult,
					List<Purchase> purchaseList) {
				Dictionary returnValue = new Dictionary();
				if (billingResult.getResponseCode() == BillingClient.BillingResponseCode.OK) {
					returnValue.put("status", 0); // OK = 0
					returnValue.put("purchases", CrossbowPlayBillingUtils.convertPurchaseListToDictionaryObjectArray(purchaseList));
				} else {
					returnValue.put("status", 1); // FAILED = 1
					returnValue.put("response_code", billingResult.getResponseCode());
					returnValue.put("debug_message", billingResult.getDebugMessage());
				}
				emitSignal("query_purchases_response", (Object)returnValue);
			}
		});
	}

	@ExposedToCrossbow
	public void querySkuDetails(final String[] list, String type) {
		List<String> skuList = Arrays.asList(list);

		SkuDetailsParams.Builder params = SkuDetailsParams.newBuilder()
												  .setSkusList(skuList)
												  .setType(type);

		billingClient.querySkuDetailsAsync(params.build(), new SkuDetailsResponseListener() {
			@Override
			public void onSkuDetailsResponse(BillingResult billingResult,
					List<SkuDetails> skuDetailsList) {
				if (billingResult.getResponseCode() == BillingClient.BillingResponseCode.OK) {
					for (SkuDetails skuDetails : skuDetailsList) {
						skuDetailsCache.put(skuDetails.getSku(), skuDetails);
					}
					emitSignal("sku_details_query_completed", (Object)CrossbowPlayBillingUtils.convertSkuDetailsListToDictionaryObjectArray(skuDetailsList));
				} else {
					emitSignal("sku_details_query_error", billingResult.getResponseCode(), billingResult.getDebugMessage(), list);
				}
			}
		});
	}

	@ExposedToCrossbow
	public void acknowledgePurchase(final String purchaseToken) {
		AcknowledgePurchaseParams acknowledgePurchaseParams =
				AcknowledgePurchaseParams.newBuilder()
						.setPurchaseToken(purchaseToken)
						.build();
		billingClient.acknowledgePurchase(acknowledgePurchaseParams, new AcknowledgePurchaseResponseListener() {
			@Override
			public void onAcknowledgePurchaseResponse(BillingResult billingResult) {
				if (billingResult.getResponseCode() == BillingClient.BillingResponseCode.OK) {
					emitSignal("purchase_acknowledged", purchaseToken);
				} else {
					emitSignal("purchase_acknowledgement_error", billingResult.getResponseCode(), billingResult.getDebugMessage(), purchaseToken);
				}
			}
		});
	}

	@ExposedToCrossbow
	public void consumePurchase(String purchaseToken) {
		ConsumeParams consumeParams = ConsumeParams.newBuilder()
											  .setPurchaseToken(purchaseToken)
											  .build();

		billingClient.consumeAsync(consumeParams, new ConsumeResponseListener() {
			@Override
			public void onConsumeResponse(BillingResult billingResult, String purchaseToken) {
				if (billingResult.getResponseCode() == BillingClient.BillingResponseCode.OK) {
					emitSignal("purchase_consumed", purchaseToken);
				} else {
					emitSignal("purchase_consumption_error", billingResult.getResponseCode(), billingResult.getDebugMessage(), purchaseToken);
				}
			}
		});
	}

	@ExposedToCrossbow
	public Dictionary confirmPriceChange(String sku) {
		if (!skuDetailsCache.containsKey(sku)) {
			Dictionary returnValue = new Dictionary();
			returnValue.put("status", 1); // FAILED = 1
			returnValue.put("response_code", null); // Null since there is no ResponseCode to return but to keep the interface (status, response_code, debug_message)
			returnValue.put("debug_message", "You must query the sku details and wait for the result before confirming a price change!");
			return returnValue;
		}

		SkuDetails skuDetails = skuDetailsCache.get(sku);

		PriceChangeFlowParams priceChangeFlowParams =
			PriceChangeFlowParams.newBuilder().setSkuDetails(skuDetails).build();

		billingClient.launchPriceChangeConfirmationFlow(getActivity(), priceChangeFlowParams, this);

		Dictionary returnValue = new Dictionary();
		returnValue.put("status", 0); // OK = 0
		return returnValue;
	}

	@ExposedToCrossbow
	public Dictionary purchase(String sku) {
		return purchaseInternal("", sku,
			BillingFlowParams.ProrationMode.UNKNOWN_SUBSCRIPTION_UPGRADE_DOWNGRADE_POLICY);
	}

	@ExposedToCrossbow
	public Dictionary updateSubscription(String oldToken, String sku, int prorationMode) {
		return purchaseInternal(oldToken, sku, prorationMode);
	}

	@ExposedToCrossbow
	private Dictionary purchaseInternal(String oldToken, String sku, int prorationMode) {
		if (!skuDetailsCache.containsKey(sku)) {
			Dictionary returnValue = new Dictionary();
			returnValue.put("status", 1); // FAILED = 1
			returnValue.put("response_code", null); // Null since there is no ResponseCode to return but to keep the interface (status, response_code, debug_message)
			returnValue.put("debug_message", "You must query the sku details and wait for the result before purchasing!");
			return returnValue;
		}

		SkuDetails skuDetails = skuDetailsCache.get(sku);
		BillingFlowParams.Builder purchaseParamsBuilder = BillingFlowParams.newBuilder();
		purchaseParamsBuilder.setSkuDetails(skuDetails);
		if (!obfuscatedAccountId.isEmpty()) {
			purchaseParamsBuilder.setObfuscatedAccountId(obfuscatedAccountId);
		}
		if (!obfuscatedProfileId.isEmpty()) {
			purchaseParamsBuilder.setObfuscatedProfileId(obfuscatedProfileId);
		}
		if (!oldToken.isEmpty() && prorationMode != BillingFlowParams.ProrationMode.UNKNOWN_SUBSCRIPTION_UPGRADE_DOWNGRADE_POLICY) {
			BillingFlowParams.SubscriptionUpdateParams updateParams =
				BillingFlowParams.SubscriptionUpdateParams.newBuilder()
					.setOldSkuPurchaseToken(oldToken)
					.setReplaceSkusProrationMode(prorationMode)
					.build();
			purchaseParamsBuilder.setSubscriptionUpdateParams(updateParams);
		}
		BillingResult result = billingClient.launchBillingFlow(getActivity(), purchaseParamsBuilder.build());

		Dictionary returnValue = new Dictionary();
		if (result.getResponseCode() == BillingClient.BillingResponseCode.OK) {
			returnValue.put("status", 0); // OK = 0
		} else {
			returnValue.put("status", 1); // FAILED = 1
			returnValue.put("response_code", result.getResponseCode());
			returnValue.put("debug_message", result.getDebugMessage());
		}

		return returnValue;
	}

	@ExposedToCrossbow
	public void setObfuscatedAccountId(String accountId) {
		obfuscatedAccountId = accountId;
	}

	@ExposedToCrossbow
	public void setObfuscatedProfileId(String profileId) {
		obfuscatedProfileId = profileId;
	}
}

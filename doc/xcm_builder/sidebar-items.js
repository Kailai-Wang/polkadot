window.SIDEBAR_ITEMS = {"mod":[["test_utils",""]],"struct":[["Account32Hash",""],["AccountId32Aliases","Extracts the `AccountId32` from the passed `location` if the network matches."],["AccountKey20Aliases",""],["AllowKnownQueryResponses","Allows only messages if the generic `ResponseHandler` expects them via `expecting_response`."],["AllowSubscriptionsFrom","Allows execution from `origin` if it is just a straight `SubscribeVerison` or `UnsubscribeVersion` instruction."],["AllowTopLevelPaidExecutionFrom","Allows execution from `origin` if it is contained in `T` (i.e. `T::Contains(origin)`) taking payments into account."],["AllowUnpaidExecutionFrom","Allows execution from any origin that is contained in `T` (i.e. `T::Contains(origin)`) without any payments. Use only for executions from trusted origin groups."],["AsPrefixedGeneralIndex","Converter struct implementing `AssetIdConversion` converting a numeric asset ID (must be `TryFrom/TryInto<u128>`) into a `GeneralIndex` junction, prefixed by some `MultiLocation` value. The `MultiLocation` value will typically be a `PalletInstance` junction."],["BackingToPlurality","`Convert` implementation to convert from some an origin which implements `Backing` into a corresponding `Plurality` `MultiLocation`."],["Case","Accepts an asset if it is contained in the given `T`’s `Get` implementation."],["ChildParachainAsNative",""],["ChildParachainConvertsVia",""],["ChildSystemParachainAsSuperuser",""],["ConvertedAbstractAssetId",""],["ConvertedConcreteAssetId",""],["CurrencyAdapter","Simple adapter to use a currency as asset transactor. This type can be used as `type AssetTransactor` in `xcm::Config`."],["EnsureXcmOrigin","`EnsureOrigin` barrier to convert from dispatch origin to XCM origin, if one exists."],["FixedRateOfConcreteFungible","Simple fee calculator that requires payment in a single concrete fungible at a fixed rate."],["FixedRateOfFungible","Simple fee calculator that requires payment in a single fungible at a fixed rate."],["FixedWeightBounds",""],["FungiblesAdapter",""],["FungiblesMutateAdapter",""],["FungiblesTransferAdapter",""],["IsAbstract","Same as [`IsConcrete`] but for a fungible with abstract location."],["IsChildSystemParachain","Allows a message only if it is from a system-level child parachain."],["IsConcrete","Converts a `MultiAsset` into balance `B` if it is a concrete fungible with an id equal to that given by `T`’s `Get`."],["LocationInverter","Simple location inverter; give it this location’s ancestry and it’ll figure out the inverted location."],["NativeAsset","Accepts an asset iff it is a native asset."],["ParentAsSuperuser",""],["ParentIsPreset","A [`MultiLocation`] consisting of a single `Parent` [`Junction`] will be converted to the parent `AccountId`."],["RelayChainAsNative",""],["SiblingParachainAsNative",""],["SiblingParachainConvertsVia",""],["SiblingSystemParachainAsSuperuser",""],["SignedAccountId32AsNative",""],["SignedAccountKey20AsNative",""],["SignedToAccountId32","`Convert` implementation to convert from some a `Signed` (system) `Origin` into an `AccountId32`."],["SovereignSignedViaLocation","Sovereign accounts use the system’s `Signed` origin with an account ID derived from the `LocationConverter`."],["TakeWeightCredit","Execution barrier that just takes `max_weight` from `weight_credit`."],["UsingComponents","Weight trader which uses the `TransactionPayment` pallet to set the right price for weight and then places any weight bought into the right account."],["WeightInfoBounds",""]],"trait":[["TakeRevenue","Function trait for handling some revenue. Similar to a negative imbalance (credit) handler, but for a `MultiAsset`. Sensible implementations will deposit the asset in some known treasury or block-author account."]]};
/* esm.sh - esbuild bundle(@wallet-standard/wallet@1.0.1) es2022 development */
// ../esmd/npm/@wallet-standard/wallet@1.0.1/node_modules/.pnpm/@wallet-standard+wallet@1.0.1/node_modules/@wallet-standard/wallet/lib/esm/register.js
var __classPrivateFieldSet = function (receiver, state, value, kind, f) {
	if (kind === "m") {
		throw new TypeError("Private method is not writable");
	}
	if (kind === "a" && !f) {
		throw new TypeError("Private accessor was defined without a setter");
	}
	if (
		typeof state === "function"
			? receiver !== state || !f
			: !state.has(receiver)
	) {
		throw new TypeError(
			"Cannot write private member to an object whose class did not declare it",
		);
	}
	return kind === "a"
		? f.call(receiver, value)
		: f
		? f.value = value
		: state.set(receiver, value),
		value;
};
var __classPrivateFieldGet = function (receiver, state, kind, f) {
	if (kind === "a" && !f) {
		throw new TypeError("Private accessor was defined without a getter");
	}
	if (
		typeof state === "function"
			? receiver !== state || !f
			: !state.has(receiver)
	) {
		throw new TypeError(
			"Cannot read private member from an object whose class did not declare it",
		);
	}
	return kind === "m"
		? f
		: kind === "a"
		? f.call(receiver)
		: f
		? f.value
		: state.get(receiver);
};
var _RegisterWalletEvent_detail;
function registerWallet(wallet) {
	const callback = ({ register }) => register(wallet);
	try {
		window.dispatchEvent(new RegisterWalletEvent(callback));
	} catch (error) {
		console.error(
			"wallet-standard:register-wallet event could not be dispatched\n",
			error,
		);
	}
	try {
		window.addEventListener(
			"wallet-standard:app-ready",
			({ detail: api }) => callback(api),
		);
	} catch (error) {
		console.error(
			"wallet-standard:app-ready event listener could not be added\n",
			error,
		);
	}
}
var RegisterWalletEvent = class extends Event {
	constructor(callback) {
		super("wallet-standard:register-wallet", {
			bubbles: false,
			cancelable: false,
			composed: false,
		});
		_RegisterWalletEvent_detail.set(this, void 0);
		__classPrivateFieldSet(this, _RegisterWalletEvent_detail, callback, "f");
	}
	get detail() {
		return __classPrivateFieldGet(this, _RegisterWalletEvent_detail, "f");
	}
	get type() {
		return "wallet-standard:register-wallet";
	}
	/** @deprecated */
	preventDefault() {
		throw new Error("preventDefault cannot be called");
	}
	/** @deprecated */
	stopImmediatePropagation() {
		throw new Error("stopImmediatePropagation cannot be called");
	}
	/** @deprecated */
	stopPropagation() {
		throw new Error("stopPropagation cannot be called");
	}
};
_RegisterWalletEvent_detail = /* @__PURE__ */ new WeakMap();
function DEPRECATED_registerWallet(wallet) {
	var _a;
	registerWallet(wallet);
	try {
		((_a = window.navigator).wallets || (_a.wallets = [])).push((
			{ register },
		) => register(wallet));
	} catch (error) {
		console.error("window.navigator.wallets could not be pushed\n", error);
	}
}

// ../esmd/npm/@wallet-standard/wallet@1.0.1/node_modules/.pnpm/@wallet-standard+wallet@1.0.1/node_modules/@wallet-standard/wallet/lib/esm/util.js
var __classPrivateFieldSet2 = function (receiver, state, value, kind, f) {
	if (kind === "m") {
		throw new TypeError("Private method is not writable");
	}
	if (kind === "a" && !f) {
		throw new TypeError("Private accessor was defined without a setter");
	}
	if (
		typeof state === "function"
			? receiver !== state || !f
			: !state.has(receiver)
	) {
		throw new TypeError(
			"Cannot write private member to an object whose class did not declare it",
		);
	}
	return kind === "a"
		? f.call(receiver, value)
		: f
		? f.value = value
		: state.set(receiver, value),
		value;
};
var __classPrivateFieldGet2 = function (receiver, state, kind, f) {
	if (kind === "a" && !f) {
		throw new TypeError("Private accessor was defined without a getter");
	}
	if (
		typeof state === "function"
			? receiver !== state || !f
			: !state.has(receiver)
	) {
		throw new TypeError(
			"Cannot read private member from an object whose class did not declare it",
		);
	}
	return kind === "m"
		? f
		: kind === "a"
		? f.call(receiver)
		: f
		? f.value
		: state.get(receiver);
};
var _ReadonlyWalletAccount_address;
var _ReadonlyWalletAccount_publicKey;
var _ReadonlyWalletAccount_chains;
var _ReadonlyWalletAccount_features;
var _ReadonlyWalletAccount_label;
var _ReadonlyWalletAccount_icon;
var ReadonlyWalletAccount = class _ReadonlyWalletAccount {
	/**
	 * Create and freeze a read-only account.
	 *
	 * @param account Account to copy properties from.
	 */
	constructor(account) {
		_ReadonlyWalletAccount_address.set(this, void 0);
		_ReadonlyWalletAccount_publicKey.set(this, void 0);
		_ReadonlyWalletAccount_chains.set(this, void 0);
		_ReadonlyWalletAccount_features.set(this, void 0);
		_ReadonlyWalletAccount_label.set(this, void 0);
		_ReadonlyWalletAccount_icon.set(this, void 0);
		if (new.target === _ReadonlyWalletAccount) {
			Object.freeze(this);
		}
		__classPrivateFieldSet2(
			this,
			_ReadonlyWalletAccount_address,
			account.address,
			"f",
		);
		__classPrivateFieldSet2(
			this,
			_ReadonlyWalletAccount_publicKey,
			account.publicKey.slice(),
			"f",
		);
		__classPrivateFieldSet2(
			this,
			_ReadonlyWalletAccount_chains,
			account.chains.slice(),
			"f",
		);
		__classPrivateFieldSet2(
			this,
			_ReadonlyWalletAccount_features,
			account.features.slice(),
			"f",
		);
		__classPrivateFieldSet2(
			this,
			_ReadonlyWalletAccount_label,
			account.label,
			"f",
		);
		__classPrivateFieldSet2(
			this,
			_ReadonlyWalletAccount_icon,
			account.icon,
			"f",
		);
	}
	/** Implementation of {@link "@wallet-standard/base".WalletAccount.address | WalletAccount::address} */
	get address() {
		return __classPrivateFieldGet2(this, _ReadonlyWalletAccount_address, "f");
	}
	/** Implementation of {@link "@wallet-standard/base".WalletAccount.publicKey | WalletAccount::publicKey} */
	get publicKey() {
		return __classPrivateFieldGet2(this, _ReadonlyWalletAccount_publicKey, "f")
			.slice();
	}
	/** Implementation of {@link "@wallet-standard/base".WalletAccount.chains | WalletAccount::chains} */
	get chains() {
		return __classPrivateFieldGet2(this, _ReadonlyWalletAccount_chains, "f")
			.slice();
	}
	/** Implementation of {@link "@wallet-standard/base".WalletAccount.features | WalletAccount::features} */
	get features() {
		return __classPrivateFieldGet2(this, _ReadonlyWalletAccount_features, "f")
			.slice();
	}
	/** Implementation of {@link "@wallet-standard/base".WalletAccount.label | WalletAccount::label} */
	get label() {
		return __classPrivateFieldGet2(this, _ReadonlyWalletAccount_label, "f");
	}
	/** Implementation of {@link "@wallet-standard/base".WalletAccount.icon | WalletAccount::icon} */
	get icon() {
		return __classPrivateFieldGet2(this, _ReadonlyWalletAccount_icon, "f");
	}
};
_ReadonlyWalletAccount_address = /* @__PURE__ */ new WeakMap(),
	_ReadonlyWalletAccount_publicKey = /* @__PURE__ */ new WeakMap(),
	_ReadonlyWalletAccount_chains = /* @__PURE__ */ new WeakMap(),
	_ReadonlyWalletAccount_features = /* @__PURE__ */ new WeakMap(),
	_ReadonlyWalletAccount_label = /* @__PURE__ */ new WeakMap(),
	_ReadonlyWalletAccount_icon = /* @__PURE__ */ new WeakMap();
function arraysEqual(a, b) {
	if (a === b) {
		return true;
	}
	const length = a.length;
	if (length !== b.length) {
		return false;
	}
	for (let i = 0; i < length; i++) {
		if (a[i] !== b[i]) {
			return false;
		}
	}
	return true;
}
function bytesEqual(a, b) {
	return arraysEqual(a, b);
}
function concatBytes(first, ...others) {
	const length = others.reduce(
		(length2, bytes2) => length2 + bytes2.length,
		first.length,
	);
	const bytes = new Uint8Array(length);
	bytes.set(first, 0);
	for (const other of others) {
		bytes.set(other, bytes.length);
	}
	return bytes;
}
function pick(source, ...keys) {
	const picked = {};
	for (const key of keys) {
		picked[key] = source[key];
	}
	return picked;
}
function guard(callback) {
	try {
		callback();
	} catch (error) {
		console.error(error);
	}
}
export {
	arraysEqual,
	bytesEqual,
	concatBytes,
	DEPRECATED_registerWallet,
	guard,
	pick,
	ReadonlyWalletAccount,
	registerWallet,
};
//# sourceMappingURL=wallet.development.mjs.map

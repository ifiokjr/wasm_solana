var __typeError = (msg) => {
	throw new TypeError(msg);
};
var __accessCheck = (obj, member, msg) =>
	member.has(obj) || __typeError(`Cannot ${msg}`);
var __privateGet = (
	obj,
	member,
	getter,
) => (__accessCheck(obj, member, "read from private field"),
	getter ? getter.call(obj) : member.get(obj));
var __privateAdd = (obj, member, value) =>
	member.has(obj)
		? __typeError("Cannot add the same private member more than once")
		: member instanceof WeakSet
		? member.add(obj)
		: member.set(obj, value);
var __privateSet = (
	obj,
	member,
	value,
	setter,
) => (__accessCheck(obj, member, "write to private field"),
	setter ? setter.call(obj, value) : member.set(obj, value),
	value);

// node_modules/.pnpm/@wallet-standard+wallet@1.0.1/node_modules/@wallet-standard/wallet/src/register.ts
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

var _detail;
var RegisterWalletEvent = class extends Event {
	constructor(callback) {
		super("wallet-standard:register-wallet", {
			bubbles: false,
			cancelable: false,
			composed: false,
		});
		__privateAdd(this, _detail);
		__privateSet(this, _detail, callback);
	}
	get detail() {
		return __privateGet(this, _detail);
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
_detail = new WeakMap();

function DEPRECATED_registerWallet(wallet) {
	registerWallet(wallet);
	try {
		(window.navigator.wallets ||= []).push(({ register }) => register(wallet));
	} catch (error) {
		console.error("window.navigator.wallets could not be pushed\n", error);
	}
}

// node_modules/.pnpm/@wallet-standard+wallet@1.0.1/node_modules/@wallet-standard/wallet/src/util.ts
var _address, _publicKey, _chains, _features, _label, _icon;
var _ReadonlyWalletAccount = class _ReadonlyWalletAccount {
	/**
	 * Create and freeze a read-only account.
	 *
	 * @param account Account to copy properties from.
	 */
	constructor(account) {
		__privateAdd(this, _address);
		__privateAdd(this, _publicKey);
		__privateAdd(this, _chains);
		__privateAdd(this, _features);
		__privateAdd(this, _label);
		__privateAdd(this, _icon);

		if (new.target === _ReadonlyWalletAccount) {
			Object.freeze(this);
		}

		__privateSet(this, _address, account.address);
		__privateSet(this, _publicKey, [...account.publicKey]);
		__privateSet(this, _chains, [...account.chains]);
		__privateSet(this, _features, [...account.features]);
		__privateSet(this, _label, account.label);
		__privateSet(this, _icon, account.icon);
	}
	/** Implementation of {@link "@wallet-standard/base".WalletAccount.address | WalletAccount::address} */
	get address() {
		return __privateGet(this, _address);
	}
	/** Implementation of {@link "@wallet-standard/base".WalletAccount.publicKey | WalletAccount::publicKey} */
	get publicKey() {
		return [...__privateGet(this, _publicKey)];
	}
	/** Implementation of {@link "@wallet-standard/base".WalletAccount.chains | WalletAccount::chains} */
	get chains() {
		return [...__privateGet(this, _chains)];
	}
	/** Implementation of {@link "@wallet-standard/base".WalletAccount.features | WalletAccount::features} */
	get features() {
		return [...__privateGet(this, _features)];
	}
	/** Implementation of {@link "@wallet-standard/base".WalletAccount.label | WalletAccount::label} */
	get label() {
		return __privateGet(this, _label);
	}
	/** Implementation of {@link "@wallet-standard/base".WalletAccount.icon | WalletAccount::icon} */
	get icon() {
		return __privateGet(this, _icon);
	}
};
_address = new WeakMap();
_publicKey = new WeakMap();
_chains = new WeakMap();
_features = new WeakMap();
_label = new WeakMap();
_icon = new WeakMap();
var ReadonlyWalletAccount = _ReadonlyWalletAccount;

function arraysEqual(a, b) {
	if (a === b) return true;

	const length = a.length;

	if (length !== b.length) return false;

	for (let i = 0; i < length; i++) {
		if (a[i] !== b[i]) return false;
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

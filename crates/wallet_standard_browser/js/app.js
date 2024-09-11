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

// node_modules/.pnpm/@wallet-standard+app@1.0.1/node_modules/@wallet-standard/app/src/wallets.ts
var wallets = void 0;
var registered = /* @__PURE__ */ new Set();
var listeners = {};

function getWallets() {
	if (wallets) return wallets;

	wallets = Object.freeze({ register, get, on });

	if (typeof window === "undefined") return wallets;

	const api = Object.freeze({ register });
	try {
		window.addEventListener(
			"wallet-standard:register-wallet",
			({ detail: callback }) => callback(api),
		);
	} catch (error) {
		console.error(
			"wallet-standard:register-wallet event listener could not be added\n",
			error,
		);
	}
	try {
		window.dispatchEvent(new AppReadyEvent(api));
	} catch (error) {
		console.error(
			"wallet-standard:app-ready event could not be dispatched\n",
			error,
		);
	}
	return wallets;
}

function register(...wallets2) {
	wallets2 = wallets2.filter((wallet) => !registered.has(wallet));

	if (wallets2.length === 0) {
		return () => {
		};
	}

	for (const wallet of wallets2) registered.add(wallet);

	if (listeners["register"]) {
		for (const listener of listeners["register"]) {
			guard(() => listener(...wallets2));
		}
	}

	return function unregister() {
		for (const wallet of wallets2) registered.delete(wallet);

		if (listeners["unregister"]) {
			for (const listener of listeners["unregister"]) {
				guard(() => listener(...wallets2));
			}
		}
	};
}

function get() {
	return [...registered];
}

function on(event, listener) {
	listeners[event]?.push(listener) || (listeners[event] = [listener]);
	return function off() {
		listeners[event] = listeners[event]?.filter((existingListener) =>
			listener !== existingListener
		);
	};
}

function guard(callback) {
	try {
		callback();
	} catch (error) {
		console.error(error);
	}
}

var _detail;
var AppReadyEvent = class extends Event {
	constructor(api) {
		super("wallet-standard:app-ready", {
			bubbles: false,
			cancelable: false,
			composed: false,
		});
		__privateAdd(this, _detail);
		__privateSet(this, _detail, api);
	}
	get detail() {
		return __privateGet(this, _detail);
	}
	get type() {
		return "wallet-standard:app-ready";
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

function DEPRECATED_getWallets() {
	if (wallets) return wallets;

	wallets = getWallets();

	if (typeof window === "undefined") return wallets;

	const callbacks = window.navigator.wallets || [];

	if (!Array.isArray(callbacks)) {
		console.error("window.navigator.wallets is not an array");
		return wallets;
	}

	const { register: register2 } = wallets;
	const push = (...callbacks2) => {
		for (const callback of callbacks2) {
			guard(() => callback({ register: register2 }));
		}
	};
	try {
		Object.defineProperty(window.navigator, "wallets", {
			value: Object.freeze({ push }),
		});
	} catch {
		console.error("window.navigator.wallets could not be set");
		return wallets;
	}
	push(...callbacks);
	return wallets;
}

export { DEPRECATED_getWallets, getWallets };

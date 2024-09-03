/// Event that will be dispatched by the app on the `window` when the app is
/// ready to register {@link Wallet | Wallets}.
///
/// Wallets must listen for this event, and {@link
/// WindowAppReadyEventAPI.register register} themselves when the event is
/// dispatched.
pub const APP_READY_EVENT: &str = "wallet-standard:app-ready";

/// Event that will be dispatched on the `window` by each {@link Wallet |
/// Wallet} when the Wallet is ready to be registered by the app.
///
/// The app must listen for this event, and register Wallets when the event is
/// dispatched.
pub const REGISTER_WALLET_EVENT: &str = "wallet-standard:register-wallet";

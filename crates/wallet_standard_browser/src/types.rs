use js_sys::Object;
use js_sys::Reflect;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub trait FeatureFromJs: JsCast {
	/// The colon separated name of the feature in the JS object.
	const NAME: &'static str;

	/// Get the wallet feature from the JS Object.
	fn feature_from_js_object(object: &Object) -> Option<Self> {
		let feature = Reflect::get(object, &JsValue::from_str(Self::NAME))
			.ok()?
			.unchecked_into();

		Some(feature)
	}

	fn feature_from_js_value(value: &JsValue) -> Option<Self> {
		let object = value.dyn_ref()?;
		Self::feature_from_js_object(object)
	}
}

macro_rules! impl_feature_from_js {
	($ident:ident, $name:expr) => {
		impl $crate::FeatureFromJs for $ident {
			const NAME: &'static str = $name;
		}
	};
}

pub(crate) use impl_feature_from_js;

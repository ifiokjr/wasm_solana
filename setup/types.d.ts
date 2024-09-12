declare module "eslint-plugin-unicorn" {
	import type { Linter } from "eslint";

	export const configs: Record<
		"flat/recommended",
		{ readonly rules: Readonly<Linter.RulesRecord> }
	>;
	const module: {
		configs: typeof configs;
	};
	export default module;
}

declare module "eslint-plugin-file-progress" {
	const module: any;
	export default module;
}

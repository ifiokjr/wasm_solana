/// <reference types="./setup/types.d.ts" />

import js from "@eslint/js";
import fileProgress from "eslint-plugin-file-progress";
import playwright from "eslint-plugin-playwright";
import unicorn from "eslint-plugin-unicorn";
import globals from "globals";
import tseslint from "typescript-eslint";

export default tseslint.config(
	{
		ignores: [
			"**/.tmp**",
			"**/tmp**",
			"**/.anchor/**",
			"**/.git/**",
			"**/.helix/**",
			"**/.local-cache/**",
			"**/.playwright-state/**",
			"**/.state/",
			".vscode/**",
			"test-results/",
			"**/*.d.ts",
			"**/*.rs",
			"**/.bin/**",
			"**/.devenv/**",
			"**/.direnv/**",
			"**/.local-cache/**",
			"**/public/**",
			"**/target/**",
			"**/node_modules/",
			"**/playwright-report/",
			"**/dist/",
			"**/pkg/",
			"**/extensions/",
		],
	},
	unicorn.configs["flat/recommended"],
	js.configs.recommended,
	tseslint.configs.eslintRecommended,
	...tseslint.configs.strictTypeChecked,
	{
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.node,
			},
			parserOptions: {
				project: true,
				tsconfigDirName: import.meta.dirname,
			},
			ecmaVersion: 2023,
			sourceType: "module",
		},
	},
	{
		...playwright.configs["flat/recommended"],
		files: ["apps/kickjump/e2e/**/*.test.ts"],
	},
	{
		plugins: {
			"file-progress": fileProgress,
		},
		settings: {
			progress: {
				hide: false,
			},
		},
	},
	{
		files: ["**/*.js", "**/*.ts", "**/.*.js"],
		rules: {
			"file-progress/activate": 1,

			"prefer-const": ["error", {
				destructuring: "all",
			}],

			"unicorn/no-keyword-prefix": "off",
			"unicorn/no-unsafe-regex": "off",
			"unicorn/no-unused-properties": "off",
			"unicorn/string-content": "off",
			"unicorn/custom-error-definition": "off",
			"unicorn/empty-brace-spaces": "off",
			"unicorn/prevent-abbreviations": "off",
			"unicorn/no-nested-ternary": "off",
			"unicorn/no-null": "off",
			"no-nested-ternary": "off",
			"unicorn/prefer-module": "off",
			"sort-imports": "off",

			"@typescript-eslint/no-unused-expressions": ["warn", {
				allowTernary: true,
				allowShortCircuit: true,
			}],

			"@typescript-eslint/no-unused-vars": ["off"],

			"@typescript-eslint/naming-convention": ["warn", {
				selector: "typeParameter",
				format: ["StrictPascalCase"],
			}],

			"@typescript-eslint/no-non-null-assertion": "warn",
			"@typescript-eslint/no-inferrable-types": "warn",

			"@typescript-eslint/consistent-type-imports": ["error", {
				disallowTypeAnnotations: false,
			}],

			"@typescript-eslint/explicit-module-boundary-types": "off",
			"@typescript-eslint/no-use-before-define": "off",

			"@typescript-eslint/member-ordering": ["warn", {
				default: [
					"signature",
					"static-field",
					"static-method",
					"field",
					"constructor",
					"method",
				],
			}],

			"@typescript-eslint/method-signature-style": "warn",
			"@typescript-eslint/prefer-function-type": "error",

			"@typescript-eslint/array-type": ["error", {
				default: "array-simple",
				readonly: "array-simple",
			}],

			"@typescript-eslint/prefer-readonly": "warn",
			"@typescript-eslint/consistent-type-exports": ["error"],
			"@typescript-eslint/await-thenable": "warn",
			"@typescript-eslint/no-unnecessary-type-arguments": "warn",
			"@typescript-eslint/restrict-plus-operands": "warn",
			"@typescript-eslint/no-misused-promises": "warn",
			"@typescript-eslint/no-unnecessary-type-assertion": "error",
			"no-constant-condition": "off",
			"no-empty": "warn",
			"no-else-return": "warn",
			"no-useless-escape": "warn",
			"default-case": "off",
			"default-case-last": "error",
			"prefer-template": "warn",
			"guard-for-in": "warn",
			"prefer-object-spread": "warn",
			curly: ["warn", "all"],
			"no-invalid-regexp": "error",
			"no-multi-str": "error",
			"no-extra-boolean-cast": "error",
			radix: "error",
			"no-return-assign": ["error", "except-parens"],

			eqeqeq: ["error", "always", {
				null: "ignore",
			}],

			"prefer-exponentiation-operator": "error",

			"prefer-arrow-callback": ["error", {
				allowNamedFunctions: true,
			}],

			"padding-line-between-statements": ["warn", {
				blankLine: "always",
				prev: "*",
				next: ["if", "switch", "for", "do", "while", "class", "function"],
			}, {
				blankLine: "always",
				prev: ["if", "switch", "for", "do", "while", "class", "function"],
				next: "*",
			}],

			"no-restricted-syntax": ["error", {
				selector:
					"ImportDeclaration[source.value='react'] :matches(ImportDefaultSpecifier, ImportNamespaceSpecifier)",
				message:
					"Default React import not allowed since we use the TypeScript jsx-transform.",
			}],

			"@typescript-eslint/no-var-requires": "off",
		},
	},
	{
		files: ["**/*.js"],
		...tseslint.configs.disableTypeChecked,
	},
);

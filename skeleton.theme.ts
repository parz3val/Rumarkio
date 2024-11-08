import type { CustomThemeConfig } from '@skeletonlabs/tw-plugin';

export const myCustomTheme: CustomThemeConfig = {
    name: 'my-custom-theme',
    properties: {
		// =~= Theme Properties =~=
		"--theme-font-family-base": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
		"--theme-font-family-heading": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
		"--theme-font-color-base": "0 0 0",
		"--theme-font-color-dark": "255 255 255",
		"--theme-rounded-base": "9999px",
		"--theme-rounded-container": "8px",
		"--theme-border-base": "0px",
		// =~= Theme On-X Colors =~=
		"--on-primary": "0 0 0",
		"--on-secondary": "255 255 255",
		"--on-tertiary": "0 0 0",
		"--on-success": "0 0 0",
		"--on-warning": "255 255 255",
		"--on-error": "255 255 255",
		"--on-surface": "255 255 255",
		// =~= Theme Colors  =~=
		// primary | #3584e4 
		"--color-primary-50": "225 237 251", // #e1edfb
		"--color-primary-100": "215 230 250", // #d7e6fa
		"--color-primary-200": "205 224 248", // #cde0f8
		"--color-primary-300": "174 206 244", // #aecef4
		"--color-primary-400": "114 169 236", // #72a9ec
		"--color-primary-500": "53 132 228", // #3584e4
		"--color-primary-600": "48 119 205", // #3077cd
		"--color-primary-700": "40 99 171", // #2863ab
		"--color-primary-800": "32 79 137", // #204f89
		"--color-primary-900": "26 65 112", // #1a4170
		// secondary | #7054aa 
		"--color-secondary-50": "234 229 242", // #eae5f2
		"--color-secondary-100": "226 221 238", // #e2ddee
		"--color-secondary-200": "219 212 234", // #dbd4ea
		"--color-secondary-300": "198 187 221", // #c6bbdd
		"--color-secondary-400": "155 135 196", // #9b87c4
		"--color-secondary-500": "112 84 170", // #7054aa
		"--color-secondary-600": "101 76 153", // #654c99
		"--color-secondary-700": "84 63 128", // #543f80
		"--color-secondary-800": "67 50 102", // #433266
		"--color-secondary-900": "55 41 83", // #372953
		// tertiary | #ec8271 
		"--color-tertiary-50": "252 236 234", // #fcecea
		"--color-tertiary-100": "251 230 227", // #fbe6e3
		"--color-tertiary-200": "250 224 220", // #fae0dc
		"--color-tertiary-300": "247 205 198", // #f7cdc6
		"--color-tertiary-400": "242 168 156", // #f2a89c
		"--color-tertiary-500": "236 130 113", // #ec8271
		"--color-tertiary-600": "212 117 102", // #d47566
		"--color-tertiary-700": "177 98 85", // #b16255
		"--color-tertiary-800": "142 78 68", // #8e4e44
		"--color-tertiary-900": "116 64 55", // #744037
		// success | #d431e8 
		"--color-success-50": "249 224 252", // #f9e0fc
		"--color-success-100": "246 214 250", // #f6d6fa
		"--color-success-200": "244 204 249", // #f4ccf9
		"--color-success-300": "238 173 246", // #eeadf6
		"--color-success-400": "225 111 239", // #e16fef
		"--color-success-500": "212 49 232", // #d431e8
		"--color-success-600": "191 44 209", // #bf2cd1
		"--color-success-700": "159 37 174", // #9f25ae
		"--color-success-800": "127 29 139", // #7f1d8b
		"--color-success-900": "104 24 114", // #681872
		// warning | #55664d 
		"--color-warning-50": "230 232 228", // #e6e8e4
		"--color-warning-100": "221 224 219", // #dde0db
		"--color-warning-200": "213 217 211", // #d5d9d3
		"--color-warning-300": "187 194 184", // #bbc2b8
		"--color-warning-400": "136 148 130", // #889482
		"--color-warning-500": "85 102 77", // #55664d
		"--color-warning-600": "77 92 69", // #4d5c45
		"--color-warning-700": "64 77 58", // #404d3a
		"--color-warning-800": "51 61 46", // #333d2e
		"--color-warning-900": "42 50 38", // #2a3226
		// error | #355b05 
		"--color-error-50": "225 230 218", // #e1e6da
		"--color-error-100": "215 222 205", // #d7decd
		"--color-error-200": "205 214 193", // #cdd6c1
		"--color-error-300": "174 189 155", // #aebd9b
		"--color-error-400": "114 140 80", // #728c50
		"--color-error-500": "53 91 5", // #355b05
		"--color-error-600": "48 82 5", // #305205
		"--color-error-700": "40 68 4", // #284404
		"--color-error-800": "32 55 3", // #203703
		"--color-error-900": "26 45 2", // #1a2d02
		// surface | #2c26b2 
		"--color-surface-50": "223 222 243", // #dfdef3
		"--color-surface-100": "213 212 240", // #d5d4f0
		"--color-surface-200": "202 201 236", // #cac9ec
		"--color-surface-300": "171 168 224", // #aba8e0
		"--color-surface-400": "107 103 201", // #6b67c9
		"--color-surface-500": "44 38 178", // #2c26b2
		"--color-surface-600": "40 34 160", // #2822a0
		"--color-surface-700": "33 29 134", // #211d86
		"--color-surface-800": "26 23 107", // #1a176b
		"--color-surface-900": "22 19 87", // #161357
		
	}
}

#!/usr/bin/env node
'use strict'

/**
Property access without traversing the prototype chain.
@template T
@param {{[k: PropertyKey]: T}} o
@param {PropertyKey} k
@return {T | undefined}
*/
const getOwn = (o, k) => Object.hasOwn(o, k) ? o[k] : undefined

/**
Returns the indices of all elements in the array where predicate is `true`.

@template T
@param {T[]} a list to search
@param {(value: T, index: number, array: T[]) => boolean} predicate
called once for each element of the array, in ascending order.
@param {unknown} [thisArg]
If provided, it will be used as the this value for each invocation of predicate.
If it is not provided, `undefined` is used instead.
*/
const findIndices = (a, predicate, thisArg) => {
	/**@type {number[]}*/
	const out = []
	a.forEach((value, index, array) => {
		if (predicate.call(thisArg, value, index, array))
			out.push(index)
	})
	return out
}

/**
Returns the entries of all elements in the array where predicate is `true`.

@template T
@param {T[]} a list to search
@param {(value: T, index: number, array: T[]) => boolean} predicate
called once for each element of the array, in ascending order.
@param {unknown} [thisArg]
If provided, it will be used as the this value for each invocation of predicate.
If it is not provided, `undefined` is used instead.
@return {[number, T][]}
*/
const findEntries = (a, predicate, thisArg) => findIndices(a, predicate, thisArg).map(i => [i, a[i]])

/**
generate a SVG gradient using passed CSS colors
@param {'l'|'r'} type l: linear (vertical), r: radial
@param {...string} colors
*/
const generator = (type = 'l', ...colors) => {
	{
		const invalid_cols = findEntries(colors, x => x.includes('"'))
		if (invalid_cols.length)
			throw new SyntaxError('CSS colors cannot contain quotes:\n' + JSON.stringify(invalid_cols))
	}

	const t = /**@type {const}*/({ l: 'linear', r: 'radial' })[type]

	return '<?xml version="1.0" encoding="utf-8"?>' +
		//should this have a viewBox?
		'<svg xmlns="http://www.w3.org/2000/svg">' +
		'<defs>' +
		`<${t}Gradient id="g"${type === 'l' ? ' gradientTransform="rotate(90)"' : ''}>` +
		colors.map((c, i) =>
			`<stop offset="${i / (colors.length - +!!i) * 100}%" stop-color="${c}"/>`
		).join('') +
		`</${t}Gradient>` +
		'</defs>' +
		`<rect width="100%" height="100%" fill="url('#g')"/>` +
		'</svg>'
}

const main = (/**@type {string[]}*/ ...args) => {
	const { log, error: err } = console

	const NAME = 'sv3g'

	if (args.length < 2) {
		const NO_ARG_TXT = /**@type {const}*/(`No arguments provided. Run "${NAME} help" for more info`)
		err(NO_ARG_TXT)
		return NO_ARG_TXT
	}

	const sub_cmd = args[1].toLowerCase()
	switch (sub_cmd) {
		case 'help': case 'man':
		case '/?': case '❔': case '❓':
		case 'ℹ️': case 'ℹ': {
			const HELP_TXT =
				`usage: ${args[0]} <subcommand> [colors...]\n` +
				'help | man | /? | ❔ | ❓ | ℹ️ | ℹ : print this text\n' +
				'wb : grayscale\n' +
				'rainbow | 🌈: RYGCBM\n' +
				'rgb : Red, Green, Blue\n' +
				'sky : like a skybox\n' +
				'mint : Linux Mint\n' +
				'fire | 🔥 : is it a candle?\n' +
				'custom : to specify arbitrary colors'
			log(HELP_TXT)
			return HELP_TXT
		}

		case 'custom': {
			const svg = generator('l', ...args.slice(2))
			log(svg)
			return svg
		}

		default: {
			/** RYGCBM as CSS hex colors */
			const RAINBOW = Object.freeze(/**@type {const}*/([
				'#f00', '#ff0', '#0f0', '#0ff', '#00f', '#f0f'
			]))
			/**🔥*/
			const FIRE = Object.freeze(/**@type {const}*/([
				'#000', '#700', '#f70', '#ff0', '#fff'
			]))

			/** gradient presets */
			const PRESET = Object.freeze(/**@type {const}*/({
				wb: ['#fff', '#000'],
				rainbow: RAINBOW,
				'🌈': RAINBOW,
				rgb: ['#f00', '#0f0', '#00f'],
				sky: ['#00e', '#07e', '#0ff'],
				mint: ['#fff', '#0e1'],
				fire: FIRE,
				'🔥': FIRE
			}))

			const pre = getOwn(PRESET, sub_cmd)
			if (pre === undefined) {
				const subcmd_err = `Unrecognized sub-command:\n${sub_cmd}\nRun "${NAME} help" to get list of valid ones`
				err(subcmd_err)
				process.exitCode = 1
				return subcmd_err
			}
			const svg = generator('l', ...pre)
			log(svg)
			return svg
		}
	}
}

if (require?.main === module)
	main(...process.argv.slice(1))

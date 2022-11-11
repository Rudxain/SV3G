#!/usr/bin/env node
'use strict'

/**
Property access without traversing the prototype chain.
@template {PropertyKey} K
@template T
@param {{[k: K]: T}} o
@param {K} k
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
Check if it _could_ be a valid CSS color `string`.
It only checks syntax, because it's intended to be future-proof and permissive.
@template T
@param {T} x
@return {T extends string ? boolean : false}
*/
const is_CSS_color = x => {
	if (typeof x != 'string') return false

	x = x.trim().toLowerCase() //WARNING. this removes \n ! fix later

	//named | hex | fn
	return /^(?:(?:#(?:[\da-f]{3,4}|[\da-f]{6}|[\da-f]{8}))|(?:[a-z]+(?:\([\da-z., /%]+\))?))$/g
		.test(x)
}

/**
generate a SVG gradient using passed CSS colors
@param {'l'|'r'} type l: linear (vertical), r: radial
@param {...string} colors
*/
const svg_gradient = (type = 'l', ...colors) => {
	{
		const invalid_cols = findEntries(colors, x => !is_CSS_color(x))
		if (invalid_cols.length)
			throw new SyntaxError('invalid CSS colors:\n' + JSON.stringify(invalid_cols))
	}

	/**@type {'linear'|'radial'}*/
	const t = getOwn(/**@type {const}*/{l: 'linear', r: 'radial'}, type) ?? 'linear'

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
		//eslint-disable-next-line quotes
		`<rect width="100%" height="100%" fill="url('#g')"/>` +
		'</svg>'
}

const main = (/**@type {string[]}*/ ...args) => {
	const {log, error: err} = console

	if (args.length < 2) {
		const NO_ARG_TXT = `No arguments provided. Run "${args[0]} help" for more info`
		err(NO_ARG_TXT)
		return NO_ARG_TXT
	}

	const sub_cmd = args[1].toLowerCase()
	switch (sub_cmd) {
		case 'help': case 'man': case '/?': {
			const HELP_TXT =
				`usage: ${args[0]} [subcommand] colors...\n` +
				'help | man | /?: print this text\n' +
				'wb : grayscale\n' +
				'rainbow | 🌈: RYGCBM\n' +
				'rgb : Red, Green, Blue\n' +
				'sky : like a skybox\n' +
				'mint : Linux Mint'
			log(HELP_TXT)
			return HELP_TXT
		}

		default: {
			/** RYGCBM as CSS hex colors */
			const RAINBOW = Object.freeze(/**@type {const}*/(['#f00', '#ff0', '#0f0', '#0ff', '#00f', '#f0f']))

			/** gradient presets */
			const PRESET = Object.freeze(/**@type {const}*/({
				wb: ['#fff', '#000'],
				rainbow: RAINBOW,
				'🌈': RAINBOW,
				rgb: ['#f00', '#0f0', '#00f'],
				sky: ['indigo', 'cyan', 'yellow', 'orange'],
				mint: ['white', 'mint']
			}))

			const svg = svg_gradient('l', ...(getOwn(PRESET, sub_cmd) ?? args.slice(1)))
			log(svg)
			return svg
		}

		case 'test': {
			log('testing CSS color validator...')

			/**@type {[string, boolean][]}*/
			const TESTS = [
				['', false],
				['amogus', true],
				['#', false],
				['#ff7', true],
				['#ff70', true],
				['#ff', false],
				['#ff700', false],
				['#000000', true],
				['#00000000', true],
				['#0000000', false],
				['#yyy', false],
				['bruh(hey)', true],
				['bruh (hey)', false],
				['bruh(0', false],
				['bruh0)', false],
				['bruh 0)', false],
				['bruh(0)', true],
				['bruh(0,0,0)', true],
				['bruh(0%, 0%, 0%)', true],
				['bruh(0deg 0rad 0grad)', true]
			]

			const fails = TESTS.filter(x => is_CSS_color(x[0]) != x[1])

			if (fails.length > 0) {
				const FAIL_TXT = 'tests failed...\n'
				err(FAIL_TXT, fails)
				return FAIL_TXT
			}
			else {
				const PASSED_TXT = 'tests succesfully passed!'
				log(PASSED_TXT)
				return PASSED_TXT
			}
		}
	}
}

if (require?.main === module)
	main(...process.argv.slice(1))

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
Regular Expression for a _potentially valid_ CSS color.
It only checks syntax, because it's intended to be future-proof and permissive.

Format: hex | named | fn.
The sub-regex for fn notation has bugs.
*/
const CSS_COLOR =
	/^ *(?:(?:#(?:[\da-f]{3,4}|[\da-f]{6}|[\da-f]{8}))|(?:[a-z]+(?:\([\da-z., /%]+\))?)) *$/gi

/**
tests against {@link CSS_COLOR}
@param {string} x
*/
const is_CSS_color = x => CSS_COLOR.test(x)

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
	const t = getOwn(/**@type {const}*/{ l: 'linear', r: 'radial' }, type) ?? 'linear'

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
	const { log, error: err } = console

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

			const svg = svg_gradient('l', ...(getOwn(PRESET, sub_cmd) ?? args.slice(1)))
			log(svg)
			return svg
		}

		case 'test': {
			log('testing CSS color validator...')

			/**@type {[string, boolean][]}*/
			const TESTS = [
				['', false],
				['   ', false],
				[' amogus ', true],
				['amogus ', true],
				[' amongus', true],
				[' Amongus', true],
				[' Amogus ', true],
				['Amogus ', true],
				['mogus ', true],
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

			const fails = TESTS.filter(([c, b]) => is_CSS_color(c) != b)

			if (fails.length > 0) {
				const FAIL_TXT = 'tests failed...\n'
				err(FAIL_TXT, fails)
				process.exitCode = 1
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

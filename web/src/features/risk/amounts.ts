/** Textual zero test for decimal strings: the browser does no arithmetic on money (X-3). */
export const isZeroAmount = (value: string): boolean => /^-?0+(\.0+)?$/.test(value);

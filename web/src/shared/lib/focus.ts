/**
 * A ref callback that moves focus to an element when it appears (an error summary, a result notice).
 * Its identity is stable on purpose: an inline `ref={(node) => node?.focus()}` is a new function on every render,
 * so React calls it again and pulls focus back from whatever the user is typing in.
 */
export const focusOnMount = (node: HTMLElement | null): void => node?.focus();

/**
 * The e-mail typed at login, kept in memory only (never stored) so a login that went through the second factor
 * and expired shows it again (S3-02). It is not a secret, and a page reload drops it.
 */
let pending: string | null = null;

export const rememberPendingEmail = (email: string) => {
  pending = email;
};
export const takePendingEmail = (): string => {
  const email = pending ?? "";
  pending = null;
  return email;
};
export const forgetPendingEmail = () => {
  pending = null;
};

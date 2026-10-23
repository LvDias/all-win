import type { ChallengeIssued } from "@/shared/api/types";

/**
 * The login in progress between the e-mail page and the code page, kept in memory only: never in the URL,
 * storage or a persisted cache. A reload drops it and the user starts again (S3 spec §1, challenge model).
 */
export type PendingLogin = { email: string; remember: boolean; challenge: ChallengeIssued };

let pending: PendingLogin | null = null;

export const startPendingLogin = (login: PendingLogin) => {
  pending = login;
};
export const readPendingLogin = (): PendingLogin | null => pending;
export const updatePendingChallenge = (challenge: ChallengeIssued) => {
  if (pending) pending = { ...pending, challenge };
};
export const clearPendingLogin = () => {
  pending = null;
};

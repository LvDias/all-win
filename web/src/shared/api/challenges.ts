import { apiFetch } from "./client";
import type { ChallengeDelivery, ChallengeIssued } from "./types";

/** Replaces the code of the same challenge; the previous code stops working. */
export const resendChallenge = (challengeId: string) => apiFetch<ChallengeIssued>("/api/v1/auth/challenges/resend", { method: "POST", body: { challenge_id: challengeId } });

/** `pending | sent | failed`; sign-in challenges always answer `not_reported`. */
export const challengeDelivery = (challengeId: string) =>
  apiFetch<{ delivery: ChallengeDelivery }>("/api/v1/auth/challenges/delivery", { method: "POST", body: { challenge_id: challengeId } }).then((answer) => answer.delivery);

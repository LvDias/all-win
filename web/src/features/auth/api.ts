"use client";

import { useMutation } from "@tanstack/react-query";
import { apiFetch } from "@/shared/api/client";
import type { ApiLocale, ChallengeIssued, InvitationLookup, LoginResult } from "@/shared/api/types";

/** Every code travels by e-mail until the Core reports another channel as available (S3b). */
const channel = "email" as const;

export const requestLoginCode = (email: string) => apiFetch<ChallengeIssued>("/api/v1/auth/login/challenges", { method: "POST", body: { email, channel } });
export const verifyLoginCode = (body: { challenge_id: string; code: string; remember_me: boolean }) => apiFetch<LoginResult>("/api/v1/auth/login/verify", { method: "POST", body });
export const useLookupInvitation = () => useMutation({ mutationFn: (token: string) => apiFetch<InvitationLookup>("/api/v1/auth/invitations/lookup", { method: "POST", body: { token } }), gcTime: 0 });
export const startSignup = (token: string) => apiFetch<ChallengeIssued>("/api/v1/auth/signup/challenges", { method: "POST", body: { token } });
export const completeSignup = (body: { challenge_id: string; code: string; locale: ApiLocale; time_zone: string; remember_me: boolean }) =>
  apiFetch<LoginResult>("/api/v1/auth/signup/verify", { method: "POST", body });

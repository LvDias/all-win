import type { StepUpProof } from "../api/types";

export type StepUpRunner = <T>(request: { action: string; target?: string; send: (proof: StepUpProof) => Promise<T> }) => Promise<T>;

/**
 * INTERIM until Vitral ships `useStepUp()` in shared (contract 2.0.0+, proof = {challenge_id, code}).
 * Tests inject a stub through the `stepUp` prop; this fallback fails closed with the Core's own 403.
 */
export function useStepUpRunner(): StepUpRunner {
  return ({ send }) => send({ challenge_id: "", code: "" });
}

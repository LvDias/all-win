import { MutationObserver, QueryClient } from "@tanstack/react-query";
import { ApiError } from "@/shared/api/client";
import { onSessionExpired } from "@/shared/session/expiry";
import { createQueryClient } from "./providers";

describe("app query client", () => {
  it("drops mutation results (secrets) as soon as they settle", async () => {
    const client = createQueryClient();
    const observer = new MutationObserver<{ webhook_url: string }, Error, { token: string }>(client, { mutationFn: async () => ({ webhook_url: "https://x/secret" }) });
    await observer.mutate({ token: "hunter2" });
    observer.reset();
    await new Promise((resolve) => setTimeout(resolve, 20));
    expect(client.getMutationCache().getAll()).toHaveLength(0);
  });

  it("empties the whole cache when a request answers auth.unauthenticated, then goes to login", async () => {
    const client = createQueryClient();
    const expired = vi.fn();
    const stop = onSessionExpired(expired);
    client.setQueryData(["sources", { includeRevoked: false }], [{ name: "user A source" }]);
    await expect(
      client.fetchQuery({
        queryKey: ["connections"],
        queryFn: () => {
          throw new ApiError({ code: "auth.unauthenticated", status: 401 });
        },
      }),
    ).rejects.toBeInstanceOf(ApiError);
    expect(client.getQueryCache().getAll()).toHaveLength(0);
    expect(expired).toHaveBeenCalledOnce();
    stop();
  });

  it("keeps a plain QueryClient untouched (control)", () => {
    expect(new QueryClient().getMutationCache().getAll()).toHaveLength(0);
  });
});

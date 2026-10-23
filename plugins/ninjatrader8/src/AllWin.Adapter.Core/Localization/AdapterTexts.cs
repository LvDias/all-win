using System.Collections.Generic;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Core.Localization
{
    /// <summary>
    /// User-visible plugin texts in en and pt-BR (CLAUDE.md §6: no hard-coded UI text). Keys are the
    /// stable codes the engine exposes; the window picks the language from welcome.locale, else the OS.
    /// A parity test checks that every key exists in both languages.
    /// </summary>
    public static class AdapterTexts
    {
        private static readonly Dictionary<string, string[]> Texts = new Dictionary<string, string[]>
        {
            // key: { en, pt-BR }
            ["session.offline"] = new[] { "Disconnected from All Win. Reconnecting…", "Desconectado do All Win. Reconectando…" },
            ["session.awaiting_welcome"] = new[] { "Connecting to All Win…", "Conectando ao All Win…" },
            ["session.ready"] = new[] { "Connected to All Win", "Conectado ao All Win" },
            ["session.stopped"] = new[] { "Not connected", "Não conectado" },
            ["stop.pair_again"] = new[] { "This NinjaTrader is no longer paired. Pair it again in All Win.", "Este NinjaTrader não está mais pareado. Pareie novamente no All Win." },
            ["stop.update_plugin"] = new[] { "This plugin version is no longer supported. Update the plugin.", "Esta versão do plugin não é mais suportada. Atualize o plugin." },
            ["stop.connected_elsewhere"] = new[] { "This connection was opened on another NinjaTrader.", "Esta conexão foi aberta em outro NinjaTrader." },
            ["stop.store_failure"] = new[] { "The plugin could not save its local state and stopped trading. Contact support.", "O plugin não conseguiu gravar seu estado local e parou de operar. Fale com o suporte." },
            ["notice.local_oco"] = new[]
            {
                "Stop and target are only fully protected while NinjaTrader is running and connected.",
                "Stop e alvo só ficam totalmente protegidos enquanto o NinjaTrader estiver aberto e conectado.",
            },
            ["account.not_simulation"] = new[] { "Not a simulation account: this plugin build does not trade it.", "Não é uma conta de simulação: esta versão do plugin não opera nela." },
            ["account.out_of_sync"] = new[] { "Out of sync: no new entries until you resolve it in All Win.", "Fora de sincronia: nenhuma nova entrada até você resolver no All Win." },

            [ReasonCodes.SyncBracketIntegrityUnknown] = new[] { "Bracket integrity unknown while the account cannot be observed.", "Integridade do stop/alvo desconhecida enquanto a conta não pode ser observada." },
            [ReasonCodes.SyncBracketOrphanLeg] = new[] { "A remaining stop or target was cancelled after the position closed.", "Um stop ou alvo restante foi cancelado após a posição fechar." },
            [ReasonCodes.SyncBracketDoubleFill] = new[] { "Both stop and target filled: an unintended position was opened.", "Stop e alvo foram executados: uma posição não desejada foi aberta." },
            [ReasonCodes.SyncBracketIncomplete] = new[] { "Open position without a working stop or target. Check the account.", "Posição aberta sem stop ou alvo ativo. Verifique a conta." },
            [ReasonCodes.SyncSimulationReset] = new[] { "Simulation account reset or Playback rewind.", "Conta de simulação reiniciada ou Playback rebobinado." },
            ["sync.disconnected_in_trade"] = new[] { "Disconnected with an open position or a command in progress.", "Desconectado com posição aberta ou comando em andamento." },
            ["sync.unknown"] = new[] { "Out of sync.", "Fora de sincronia." },

            [ReasonCodes.CommandDuplicate] = new[] { "Already executed; not repeated.", "Já executado; não foi repetido." },
            [ReasonCodes.CommandStale] = new[] { "Too old to execute; ignored.", "Antigo demais para executar; ignorado." },
            [ReasonCodes.CommandExpired] = new[] { "Arrived too late; the entry was not sent.", "Chegou tarde demais; a entrada não foi enviada." },
            [ReasonCodes.CommandPrerequisiteFailed] = new[] { "Previous step failed; not sent.", "A etapa anterior falhou; não foi enviado." },
            [ReasonCodes.CommandAccountNotMapped] = new[] { "Account not linked in All Win.", "Conta não vinculada no All Win." },
            [ReasonCodes.CommandAccountNotDestination] = new[] { "Account is not a destination.", "A conta não é de destino." },
            [ReasonCodes.CommandAccountNotSimulation] = new[] { "Not a simulation account.", "Não é uma conta de simulação." },
            [ReasonCodes.CommandInstrumentUnavailable] = new[] { "Instrument not available in NinjaTrader.", "Instrumento indisponível no NinjaTrader." },
            [ReasonCodes.CommandTargetNotFound] = new[] { "Order to change was not found.", "A ordem a alterar não foi encontrada." },
            [ReasonCodes.CommandAlreadyFlat] = new[] { "Nothing to close.", "Nada a fechar." },
            [ReasonCodes.CommandPlatformDisconnected] = new[] { "Account connection is down; not sent.", "A conexão da conta caiu; não foi enviado." },
            [ReasonCodes.CommandNothingToReduce] = new[] { "Nothing to reduce.", "Nada a reduzir." },
            [ReasonCodes.OrderQuantityCapped] = new[] { "Quantity limited to the open position.", "Quantidade limitada à posição aberta." },
            [ReasonCodes.OrderPlatformRejected] = new[] { "Rejected by NinjaTrader.", "Rejeitada pelo NinjaTrader." },
            [ReasonCodes.OrderOrphanCancelled] = new[] { "Cancelled: the position is closed.", "Cancelada: a posição foi fechada." },
            [ReasonCodes.OrderProtectionFailed] = new[] { "Stop or target could not be placed or changed.", "Não foi possível colocar ou alterar o stop ou alvo." },
            [ReasonCodes.OrderCancelFailed] = new[] { "Cancellation failed.", "O cancelamento falhou." },
            [ReasonCodes.RiskAccountBlocked] = new[] { "Account blocked by its risk limit until the reset.", "Conta bloqueada pelo limite de risco até o reinício." },
        };

        public static IEnumerable<string> Keys => Texts.Keys;

        public static string Get(string key, Locale locale)
        {
            if (!Texts.TryGetValue(key, out var pair)) return key.StartsWith("sync.") ? Get("sync.unknown", locale) : key;
            return locale == Locale.PtBr ? pair[1] : pair[0];
        }

        internal static bool HasBoth(string key) => Texts.TryGetValue(key, out var p) && p.Length == 2 && p[0].Length > 0 && p[1].Length > 0;
    }
}

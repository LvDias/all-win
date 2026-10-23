# Alertas do TradingView para o All Win — guia rápido

> Rascunho S17 · contrato de instrução v1 (`schema_version` "1.0") · [English](GUIDE.en.md)

## 1. Rotina diária

1. Cole os níveis do dia no indicador.
2. **Apague o alerta de ontem** (o alerta guarda os níveis e a versão do indicador de quando foi criado).
3. Crie **um alerta por gráfico**: Condição = o indicador → **"Qualquer chamada da função alert()"** ("Any alert() function call").
4. Em *Notificações*, marque **Webhook URL** e cole o endereço da sua fonte no All Win. **Não altere a mensagem**: o indicador monta o JSON.
5. Recrie o alerta sempre que mudar níveis, entradas ou a versão do indicador.

**Segredo:** o token secreto fica **somente na URL do webhook**. Nunca o coloque na mensagem, em prints ou em conversas. Se o corpo contiver algo parecido com um token, o All Win rejeita o alerta.

Segundo a [página oficial do TradingView](https://www.tradingview.com/support/solutions/43000529348-how-to-configure-webhook-alerts/), webhooks podem ocasionalmente não chegar ao destino (veja a coluna *Webhook status* no log de alertas do TradingView). Confira no **log de auditoria** do All Win se cada alerta foi recebido ou recusado, e com qual motivo.

## 2. Tipos de sinal

| Sinal do indicador | `signal_type` |
|---|---|
| Gatilhos (Setups 1/2/3/4/6) | `trigger` |
| Trade Plan | `trade_plan` |
| Triggers do NQ | `nq_trigger` |

Cada conta escolhe quais tipos aceita (tela de roteamento).

## 3. Campos por ação

Sempre: `schema_version` = `"1.0"`, `message_id` (único, gerado pelo indicador), `sent_at` (UTC, termina em `Z`), `source.type` = `"tradingview"`, `signal_type`, `action`, `instrument.symbol` = `MNQ` (o **futuro operado** na V1; `QQQ`/`SPY` são só referência).

**Não envie `contract_month`:** o All Win usa o contrato ativo do catálogo e faz a rolagem na data configurada pelo administrador. Um mês digitado no Pine fica velho na rolagem e operaria o contrato errado.

| Ação | Envie também | Não envie | Efeito em cada conta |
|---|---|---|---|
| `buy` / `sell` | `quantity` (inteiro 1–1000, antes do multiplicador da conta) | `side`, `order`, `position_effect` | abre/aumenta; se a conta estiver na posição oposta, é **recusado** |
| `close` | — | `quantity`, `side`, `order` | zera **toda** a posição e cancela stop/alvo |
| `reverse` | `quantity` e `side` (`long`/`short` = lado final) | `position_effect` | zera e abre o lado pedido; zerado → abre |

**Recusados pelo All Win (V1):** `modify`, `cancel`, `position_effect: "reduce"`, ordens que não sejam a mercado (`order.type` limit/stop), `signal_type: "master_account"`, `schema_version` diferente de `1.0`. **Stop e alvo nunca vão na mensagem:** vêm do perfil de cada conta.

Quantidade vai **sem aspas**: `1`. Não envie `reference_price` enquanto o gráfico for QQQ/SPY (o preço seria de outro ativo).

## 4. Exemplos válidos

Compra (gatilho):
```json
{"schema_version":"1.0","message_id":"tv-MNQ-trigger-s6-buy-QQQ-5-1792679400000","sent_at":"2026-10-22T14:31:05.123Z","source":{"type":"tradingview","name":"nq-es-strikes","version":"4.0"},"signal_type":"trigger","action":"buy","instrument":{"symbol":"MNQ"},"quantity":1,"details":{"setup":"Setup 6","zone":"Defense","trigger":"engulfing"}}
```
Venda (Trade Plan):
```json
{"schema_version":"1.0","message_id":"tv-MNQ-trade_plan-tp-sell-QQQ-5-1792681200000","sent_at":"2026-10-22T15:00:00.120Z","source":{"type":"tradingview","name":"nq-es-strikes","version":"4.0"},"signal_type":"trade_plan","action":"sell","instrument":{"symbol":"MNQ"},"quantity":2}
```
Venda (Trigger do NQ):
```json
{"schema_version":"1.0","message_id":"tv-MNQ-nq_trigger-nq-sell-NQ1-1-1792682000000","sent_at":"2026-10-22T15:13:20.004Z","source":{"type":"tradingview","name":"nq-es-strikes","version":"4.0"},"signal_type":"nq_trigger","action":"sell","instrument":{"symbol":"MNQ"},"quantity":1}
```
Zerar:
```json
{"schema_version":"1.0","message_id":"tv-MNQ-trigger-s6-close-QQQ-5-1792683000000","sent_at":"2026-10-22T15:30:00.000Z","source":{"type":"tradingview","name":"nq-es-strikes","version":"4.0"},"signal_type":"trigger","action":"close","instrument":{"symbol":"MNQ"}}
```
Virar para vendido:
```json
{"schema_version":"1.0","message_id":"tv-MNQ-trigger-s3-reverse_short-QQQ-5-1792684000000","sent_at":"2026-10-22T15:46:40.250Z","source":{"type":"tradingview","name":"nq-es-strikes","version":"4.0"},"signal_type":"trigger","action":"reverse","side":"short","instrument":{"symbol":"MNQ"},"quantity":1}
```
As mesmas mensagens estão em `examples/accepted/`.

## 5. Como validar um alerta

- **No uso normal:** abra o **log de auditoria** do All Win. Cada alerta aparece como recebido, duplicado ignorado ou recusado, com o motivo.
- **Durante o desenvolvimento:** o indicador grava cada mensagem em **Pine Logs**. Cole **só o JSON da mensagem, nunca a URL**, na conversa com o time; o time o testa no ambiente de desenvolvimento local (nada é exposto) e devolve aceito / duplicado / recusado + código. Reenviar o mesmo JSON dá "duplicado": é o esperado.

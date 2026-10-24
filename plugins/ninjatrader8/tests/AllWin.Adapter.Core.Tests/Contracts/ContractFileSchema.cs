using System;
using System.IO;
using System.Text.Json.Nodes;
using AllWin.Adapter.Core.Protocol.Schema;
using Json.Schema;

namespace AllWin.Adapter.Core.Tests.Contracts
{
    /// <summary>
    /// Validates a frame against the CURRENT contracts/adapter-protocol/v1/messages.schema.json read from the repository,
    /// not the embedded copy (which may lag behind contracts/; see Embedded_schemas_match_contracts_files).
    /// JsonSchema.Net is not safe to evaluate concurrently: every evaluation takes the lock.
    /// </summary>
    public static class ContractFileSchema
    {
        private static readonly object Gate = new object();

        private static readonly Lazy<(JsonSchema Schema, EvaluationOptions Options)> Adapter = new Lazy<(JsonSchema, EvaluationOptions)>(() =>
        {
            var options = new EvaluationOptions { OutputFormat = OutputFormat.Flag };
            options.SchemaRegistry.Register(JsonSchema.FromText(File.ReadAllText(Path.Combine(RepoPaths.Root, ContractSchemas.InstructionV1Path))));
            return (JsonSchema.FromText(File.ReadAllText(Path.Combine(RepoPaths.Root, ContractSchemas.AdapterProtocolV1Path))), options);
        });

        public static bool IsValidAdapterFrame(string json)
        {
            var node = JsonNode.Parse(json);
            lock (Gate)
            {
                var (schema, options) = Adapter.Value;
                return schema.Evaluate(node, options).IsValid;
            }
        }
    }
}

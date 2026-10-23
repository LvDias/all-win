using System;
using System.Text.Json.Nodes;
using AllWin.Adapter.Core.Protocol.Schema;
using Json.Schema;

namespace AllWin.Adapter.Core.Tests.Contracts
{
    /// <summary>
    /// The independent JSON Schema implementation (JsonSchema.Net) used as a second opinion on adapter frames.
    /// One schema for the whole test run, resolving the instruction schema through its own options registry
    /// (not <see cref="SchemaRegistry.Global"/>). JsonSchema.Net is not safe to evaluate concurrently and xUnit
    /// runs test classes in parallel, so every evaluation takes the lock (QA T1).
    /// </summary>
    public static class ReferenceSchema
    {
        private static readonly object Gate = new object();

        private static readonly Lazy<(JsonSchema Schema, EvaluationOptions Options)> Adapter = new Lazy<(JsonSchema, EvaluationOptions)>(() =>
        {
            var options = new EvaluationOptions { OutputFormat = OutputFormat.Flag };
            options.SchemaRegistry.Register(JsonSchema.FromText(ContractSchemas.InstructionV1));
            return (JsonSchema.FromText(ContractSchemas.AdapterProtocolV1), options);
        });

        /// <summary>Whether <paramref name="json"/> is a valid adapter protocol v1 frame.</summary>
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

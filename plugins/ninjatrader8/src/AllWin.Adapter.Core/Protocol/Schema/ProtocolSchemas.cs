using System;
using System.Collections.Generic;
using Newtonsoft.Json.Linq;

namespace AllWin.Adapter.Core.Protocol.Schema
{
    /// <summary>The canonical contract schemas (adapter protocol v1 + embedded instruction v1).</summary>
    public static class ProtocolSchemas
    {
        public const string AdapterProtocolV1Id = "https://allwin.app/contracts/adapter-protocol/v1/messages.schema.json";
        public const string InstructionV1Id = "https://allwin.app/contracts/instruction/v1/instruction.schema.json";

        private static readonly Lazy<JsonSchemaValidator> Validator = new Lazy<JsonSchemaValidator>(Create);

        /// <summary>Validates one WebSocket frame (envelope + payload) against messages.schema.json.</summary>
        public static IReadOnlyList<SchemaError> ValidateFrame(JToken frame) => Validator.Value.Validate(AdapterProtocolV1Id, frame);

        /// <summary>Validates a standard instruction (ADR-0003) against instruction.schema.json.</summary>
        public static IReadOnlyList<SchemaError> ValidateInstruction(JToken instruction) => Validator.Value.Validate(InstructionV1Id, instruction);

        private static JsonSchemaValidator Create()
        {
            var validator = new JsonSchemaValidator();
            validator.AddSchema(ProtocolJson.ParseObject(ContractSchemas.InstructionV1));
            validator.AddSchema(ProtocolJson.ParseObject(ContractSchemas.AdapterProtocolV1));
            return validator;
        }
    }
}

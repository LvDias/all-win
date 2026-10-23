using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json.Nodes;
using AllWin.Adapter.Core.Protocol;
using AllWin.Adapter.Core.Protocol.Schema;
using Json.Schema;
using Xunit;

namespace AllWin.Adapter.Core.Tests.Contracts
{
    /// <summary>Our in-process validator must agree with the contract fixtures and with an independent implementation.</summary>
    public class SchemaConformanceTests
    {
        public static IEnumerable<object[]> AdapterFixtures() => Fixtures(RepoPaths.AdapterExamples);
        public static IEnumerable<object[]> InstructionFixtures() => Fixtures(RepoPaths.InstructionExamples);

        private static IEnumerable<object[]> Fixtures(string dir) =>
            new[] { "valid", "invalid" }
                .SelectMany(kind => Directory.GetFiles(Path.Combine(dir, kind), "*.json").OrderBy(f => f)
                    .Select(f => new object[] { kind, Path.GetFileName(f) }));

        [Theory]
        [MemberData(nameof(AdapterFixtures))]
        public void Adapter_fixture_verdict_matches_its_folder(string kind, string file)
        {
            var frame = ProtocolJson.Parse(File.ReadAllText(Path.Combine(RepoPaths.AdapterExamples, kind, file)));
            var errors = ProtocolSchemas.ValidateFrame(frame);
            if (kind == "valid") Assert.True(errors.Count == 0, string.Join("; ", errors));
            else Assert.NotEmpty(errors);
        }

        [Theory]
        [MemberData(nameof(InstructionFixtures))]
        public void Instruction_fixture_verdict_matches_its_folder(string kind, string file)
        {
            var instruction = ProtocolJson.Parse(File.ReadAllText(Path.Combine(RepoPaths.InstructionExamples, kind, file)));
            var errors = ProtocolSchemas.ValidateInstruction(instruction);
            if (kind == "valid") Assert.True(errors.Count == 0, string.Join("; ", errors));
            else Assert.NotEmpty(errors);
        }

        [Fact]
        public void Fixture_sets_are_not_empty()
        {
            Assert.True(AdapterFixtures().Count() >= 50);
            Assert.True(InstructionFixtures().Count() >= 40);
        }

        [Theory]
        [MemberData(nameof(AdapterFixtures))]
        public void Independent_validator_agrees(string kind, string file)
        {
            var reference = ReferenceSchemas.Value;
            var json = File.ReadAllText(Path.Combine(RepoPaths.AdapterExamples, kind, file));
            var expected = reference.Evaluate(JsonNode.Parse(json), new EvaluationOptions { OutputFormat = OutputFormat.Flag }).IsValid;
            var ours = ProtocolSchemas.ValidateFrame(ProtocolJson.Parse(json)).Count == 0;
            Assert.Equal(expected, ours);
        }

        [Fact]
        public void Embedded_schemas_match_contracts_files()
        {
            const string hint = "Embedded schema drifted from contracts/: run plugins/ninjatrader8/tools/gen-contract-schemas.py";
            Assert.True(Read(ContractSchemas.AdapterProtocolV1Path) == ContractSchemas.AdapterProtocolV1, hint);
            Assert.True(Read(ContractSchemas.InstructionV1Path) == ContractSchemas.InstructionV1, hint);
            static string Read(string rel) => File.ReadAllText(Path.Combine(RepoPaths.Root, rel)).Replace("\r\n", "\n");
        }

        [Fact]
        public void Dollar_anchor_does_not_accept_trailing_newline()
        {
            var frame = ProtocolJson.ParseObject(File.ReadAllText(Path.Combine(RepoPaths.AdapterExamples, "valid", "heartbeat.json")));
            frame["msg_id"] = frame.Value<string>("msg_id") + "\n";
            Assert.Contains(ProtocolSchemas.ValidateFrame(frame), e => e.Keyword == "pattern");
        }

        [Fact]
        public void Duplicate_keys_are_rejected_by_the_parser()
        {
            Assert.ThrowsAny<Newtonsoft.Json.JsonException>(() => ProtocolJson.Parse("{\"a\":1,\"a\":2}"));
        }

        private static readonly System.Lazy<JsonSchema> ReferenceSchemas = new System.Lazy<JsonSchema>(() =>
        {
            var instruction = JsonSchema.FromText(ContractSchemas.InstructionV1);
            SchemaRegistry.Global.Register(instruction);
            return JsonSchema.FromText(ContractSchemas.AdapterProtocolV1);
        });
    }
}

using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Numerics;
using System.Text;
using System.Text.RegularExpressions;
using Newtonsoft.Json.Linq;

namespace AllWin.Adapter.Core.Protocol.Schema
{
    /// <summary>
    /// Minimal JSON Schema 2020-12 validator for the keywords used by the All Win contracts.
    /// It runs inside NinjaTrader without extra dependencies, so every frame is checked against
    /// the canonical schema in <c>contracts/</c> instead of a hand-written copy of its rules.
    /// Unknown keywords fail at load time: a contract change can never be ignored silently.
    /// </summary>
    public sealed class JsonSchemaValidator
    {
        private static readonly HashSet<string> Annotations = new HashSet<string>(StringComparer.Ordinal)
        {
            "$schema", "$id", "$defs", "$comment", "title", "description", "examples", "default",
        };

        private static readonly HashSet<string> Supported = new HashSet<string>(StringComparer.Ordinal)
        {
            "$ref", "type", "properties", "required", "additionalProperties", "enum", "const", "pattern",
            "minLength", "maxLength", "minimum", "maximum", "minItems", "maxItems", "uniqueItems", "items",
            "allOf", "anyOf", "oneOf", "not", "if", "then", "else", "minProperties", "maxProperties",
        };

        private readonly Dictionary<string, JObject> _documents = new Dictionary<string, JObject>(StringComparer.Ordinal);
        private readonly Dictionary<string, Regex> _regexCache = new Dictionary<string, Regex>(StringComparer.Ordinal);
        private readonly object _regexLock = new object();

        /// <summary>Registers a schema document by its <c>$id</c>. All keywords are checked eagerly.</summary>
        public void AddSchema(JObject schema)
        {
            if (schema == null) throw new ArgumentNullException(nameof(schema));
            var id = (string?)schema["$id"];
            if (string.IsNullOrEmpty(id)) throw new ArgumentException("Schema has no $id.", nameof(schema));
            CheckKeywords(schema, "#");
            _documents[id!] = schema;
        }

        /// <summary>Validates <paramref name="instance"/> against the schema registered as <paramref name="schemaId"/>.</summary>
        public IReadOnlyList<SchemaError> Validate(string schemaId, JToken instance)
        {
            if (!_documents.TryGetValue(schemaId, out var root))
                throw new ArgumentException("Unknown schema " + schemaId, nameof(schemaId));
            var errors = new List<SchemaError>();
            Evaluate(root, root, instance, "", errors);
            return errors;
        }

        private void CheckKeywords(JToken node, string path)
        {
            if (node.Type == JTokenType.Boolean) return;
            if (!(node is JObject obj)) throw new NotSupportedException("Schema node is not an object at " + path);
            foreach (var p in obj.Properties())
            {
                if (!Supported.Contains(p.Name) && !Annotations.Contains(p.Name))
                    throw new NotSupportedException("Unsupported schema keyword '" + p.Name + "' at " + path);
            }
            if (obj["$defs"] is JObject defs)
                foreach (var d in defs.Properties()) CheckKeywords(d.Value, path + "/$defs/" + d.Name);
            if (obj["properties"] is JObject props)
                foreach (var d in props.Properties()) CheckKeywords(d.Value, path + "/properties/" + d.Name);
            foreach (var key in new[] { "additionalProperties", "items", "not", "if", "then", "else" })
            {
                var sub = obj[key];
                if (sub != null) CheckKeywords(sub, path + "/" + key);
            }
            foreach (var key in new[] { "allOf", "anyOf", "oneOf" })
            {
                if (obj[key] is JArray arr)
                    for (var i = 0; i < arr.Count; i++) CheckKeywords(arr[i], path + "/" + key + "/" + i);
            }
            if (obj["pattern"] != null) GetRegex((string)obj["pattern"]!);
        }

        private void Evaluate(JObject root, JToken schemaToken, JToken instance, string path, List<SchemaError> errors)
        {
            if (schemaToken.Type == JTokenType.Boolean)
            {
                if (!(bool)schemaToken) errors.Add(new SchemaError(path, "false", "no value is allowed"));
                return;
            }
            var schema = (JObject)schemaToken;

            var reference = (string?)schema["$ref"];
            if (reference != null)
            {
                var (targetRoot, target) = Resolve(root, reference);
                Evaluate(targetRoot, target, instance, path, errors);
            }

            var type = schema["type"];
            if (type != null && !MatchesType(type, instance))
            {
                errors.Add(new SchemaError(path, "type", "expected " + type.ToString(Newtonsoft.Json.Formatting.None)));
                return;
            }

            if (schema["enum"] is JArray enumValues && !enumValues.Any(v => JsonEquals(v, instance)))
                errors.Add(new SchemaError(path, "enum", "value not allowed"));

            var constValue = schema["const"];
            if (constValue != null && !JsonEquals(constValue, instance))
                errors.Add(new SchemaError(path, "const", "value not allowed"));

            if (instance.Type == JTokenType.String) EvaluateString(schema, (string)instance!, path, errors);
            if (IsNumber(instance)) EvaluateNumber(schema, instance, path, errors);
            if (instance is JObject obj) EvaluateObject(root, schema, obj, path, errors);
            if (instance is JArray arr) EvaluateArray(root, schema, arr, path, errors);

            if (schema["allOf"] is JArray allOf)
                foreach (var sub in allOf) Evaluate(root, sub, instance, path, errors);

            if (schema["anyOf"] is JArray anyOf && !anyOf.Any(sub => IsValid(root, sub, instance, path)))
                errors.Add(new SchemaError(path, "anyOf", "no alternative matches"));

            if (schema["oneOf"] is JArray oneOf && oneOf.Count(sub => IsValid(root, sub, instance, path)) != 1)
                errors.Add(new SchemaError(path, "oneOf", "exactly one alternative must match"));

            var not = schema["not"];
            if (not != null && IsValid(root, not, instance, path))
                errors.Add(new SchemaError(path, "not", "value matches a forbidden schema"));

            var ifSchema = schema["if"];
            if (ifSchema != null)
            {
                var branch = IsValid(root, ifSchema, instance, path) ? schema["then"] : schema["else"];
                if (branch != null) Evaluate(root, branch, instance, path, errors);
            }
        }

        private bool IsValid(JObject root, JToken schema, JToken instance, string path)
        {
            var scratch = new List<SchemaError>();
            Evaluate(root, schema, instance, path, scratch);
            return scratch.Count == 0;
        }

        private void EvaluateString(JObject schema, string value, string path, List<SchemaError> errors)
        {
            var length = CodePointLength(value);
            var minLength = schema["minLength"];
            if (minLength != null && length < (long)minLength) errors.Add(new SchemaError(path, "minLength", "too short"));
            var maxLength = schema["maxLength"];
            if (maxLength != null && length > (long)maxLength) errors.Add(new SchemaError(path, "maxLength", "too long"));
            var pattern = (string?)schema["pattern"];
            if (pattern != null && !GetRegex(pattern).IsMatch(value)) errors.Add(new SchemaError(path, "pattern", "does not match " + pattern));
        }

        private static void EvaluateNumber(JObject schema, JToken value, string path, List<SchemaError> errors)
        {
            var minimum = schema["minimum"];
            if (minimum != null && CompareNumbers(value, minimum) < 0) errors.Add(new SchemaError(path, "minimum", "below minimum"));
            var maximum = schema["maximum"];
            if (maximum != null && CompareNumbers(value, maximum) > 0) errors.Add(new SchemaError(path, "maximum", "above maximum"));
        }

        private void EvaluateObject(JObject root, JObject schema, JObject obj, string path, List<SchemaError> errors)
        {
            if (schema["required"] is JArray required)
                foreach (var name in required.Select(r => (string)r!))
                    if (obj.Property(name, StringComparison.Ordinal) == null)
                        errors.Add(new SchemaError(path + "/" + name, "required", "missing"));

            var count = obj.Count;
            var minProperties = schema["minProperties"];
            if (minProperties != null && count < (long)minProperties) errors.Add(new SchemaError(path, "minProperties", "too few properties"));
            var maxProperties = schema["maxProperties"];
            if (maxProperties != null && count > (long)maxProperties) errors.Add(new SchemaError(path, "maxProperties", "too many properties"));

            var properties = schema["properties"] as JObject;
            var additional = schema["additionalProperties"];
            foreach (var property in obj.Properties())
            {
                var childPath = path + "/" + property.Name;
                var propertySchema = properties?.Property(property.Name, StringComparison.Ordinal);
                if (propertySchema != null) Evaluate(root, propertySchema.Value, property.Value, childPath, errors);
                else if (additional != null) Evaluate(root, additional, property.Value, childPath, errors);
            }
        }

        private void EvaluateArray(JObject root, JObject schema, JArray arr, string path, List<SchemaError> errors)
        {
            var minItems = schema["minItems"];
            if (minItems != null && arr.Count < (long)minItems) errors.Add(new SchemaError(path, "minItems", "too few items"));
            var maxItems = schema["maxItems"];
            if (maxItems != null && arr.Count > (long)maxItems) errors.Add(new SchemaError(path, "maxItems", "too many items"));
            var unique = schema["uniqueItems"];
            if (unique != null && (bool)unique)
            {
                for (var i = 0; i < arr.Count; i++)
                    for (var j = i + 1; j < arr.Count; j++)
                        if (JsonEquals(arr[i], arr[j])) { errors.Add(new SchemaError(path, "uniqueItems", "duplicate items")); i = arr.Count; break; }
            }
            var items = schema["items"];
            if (items != null)
                for (var i = 0; i < arr.Count; i++) Evaluate(root, items, arr[i], path + "/" + i, errors);
        }

        private (JObject Root, JToken Target) Resolve(JObject currentRoot, string reference)
        {
            var hash = reference.IndexOf('#');
            var documentId = hash < 0 ? reference : reference.Substring(0, hash);
            var pointer = hash < 0 ? "" : reference.Substring(hash + 1);
            var root = currentRoot;
            if (documentId.Length > 0 && !_documents.TryGetValue(documentId, out root!))
                throw new InvalidOperationException("Unresolvable $ref " + reference);
            JToken target = root;
            foreach (var segment in pointer.Split(new[] { '/' }, StringSplitOptions.RemoveEmptyEntries))
            {
                var name = segment.Replace("~1", "/").Replace("~0", "~");
                target = target[name] ?? throw new InvalidOperationException("Unresolvable $ref " + reference);
            }
            return (root, target);
        }

        private static bool MatchesType(JToken type, JToken instance)
        {
            if (type is JArray types) return types.Any(t => MatchesType(t, instance));
            switch ((string?)type)
            {
                case "object": return instance.Type == JTokenType.Object;
                case "array": return instance.Type == JTokenType.Array;
                case "string": return instance.Type == JTokenType.String;
                case "boolean": return instance.Type == JTokenType.Boolean;
                case "null": return instance.Type == JTokenType.Null;
                case "number": return IsNumber(instance);
                case "integer":
                    if (instance.Type == JTokenType.Integer) return true;
                    if (instance.Type == JTokenType.Float)
                    {
                        // 2020-12: a number with a zero fractional part is an integer.
                        var v = ((JValue)instance).Value;
                        if (v is decimal d) return decimal.Truncate(d) == d;
                        if (v is double f) return !double.IsInfinity(f) && Math.Floor(f) == f;
                    }
                    return false;
                default: throw new NotSupportedException("Unsupported type " + type);
            }
        }

        private static bool IsNumber(JToken token) => token.Type == JTokenType.Integer || token.Type == JTokenType.Float;

        private static int CompareNumbers(JToken left, JToken right)
        {
            var l = ToBigRational(left);
            var r = ToBigRational(right);
            return (l.Numerator * r.Denominator).CompareTo(r.Numerator * l.Denominator);
        }

        private static (BigInteger Numerator, BigInteger Denominator) ToBigRational(JToken token)
        {
            var value = ((JValue)token).Value;
            switch (value)
            {
                case BigInteger b: return (b, BigInteger.One);
                case long l: return (l, BigInteger.One);
                case int i: return (i, BigInteger.One);
                case decimal d: return FromDecimal(d);
                case double f: return FromDecimal((decimal)f);
                default: return FromDecimal(Convert.ToDecimal(value, CultureInfo.InvariantCulture));
            }
        }

        private static (BigInteger, BigInteger) FromDecimal(decimal d)
        {
            var bits = decimal.GetBits(d);
            var scale = (bits[3] >> 16) & 0xFF;
            var mantissa = new BigInteger((uint)bits[0]) | (new BigInteger((uint)bits[1]) << 32) | (new BigInteger((uint)bits[2]) << 64);
            if (bits[3] < 0) mantissa = -mantissa;
            return (mantissa, BigInteger.Pow(10, scale));
        }

        /// <summary>JSON equality: numbers compare by value (1 == 1.0), everything else structurally.</summary>
        private static bool JsonEquals(JToken a, JToken b)
        {
            if (IsNumber(a) && IsNumber(b)) return CompareNumbers(a, b) == 0;
            if (a.Type != b.Type) return false;
            switch (a)
            {
                case JObject oa:
                    var ob = (JObject)b;
                    if (oa.Count != ob.Count) return false;
                    foreach (var p in oa.Properties())
                    {
                        var other = ob.Property(p.Name, StringComparison.Ordinal);
                        if (other == null || !JsonEquals(p.Value, other.Value)) return false;
                    }
                    return true;
                case JArray aa:
                    var ab = (JArray)b;
                    if (aa.Count != ab.Count) return false;
                    for (var i = 0; i < aa.Count; i++) if (!JsonEquals(aa[i], ab[i])) return false;
                    return true;
                default:
                    return JToken.DeepEquals(a, b);
            }
        }

        private static int CodePointLength(string value)
        {
            var count = 0;
            for (var i = 0; i < value.Length; i++)
            {
                if (char.IsHighSurrogate(value[i]) && i + 1 < value.Length && char.IsLowSurrogate(value[i + 1])) i++;
                count++;
            }
            return count;
        }

        private Regex GetRegex(string pattern)
        {
            lock (_regexLock)
            {
                if (!_regexCache.TryGetValue(pattern, out var regex))
                {
                    regex = new Regex(ToDotNetPattern(pattern), RegexOptions.ECMAScript | RegexOptions.CultureInvariant);
                    _regexCache[pattern] = regex;
                }
                return regex;
            }
        }

        /// <summary>
        /// ECMA-262 <c>$</c> matches only at the end of input; .NET also matches before a final
        /// newline. Unescaped <c>$</c> outside character classes becomes an absolute end anchor.
        /// </summary>
        internal static string ToDotNetPattern(string pattern)
        {
            var sb = new StringBuilder(pattern.Length + 16);
            var inClass = false;
            for (var i = 0; i < pattern.Length; i++)
            {
                var c = pattern[i];
                if (c == '\\' && i + 1 < pattern.Length) { sb.Append(c).Append(pattern[++i]); continue; }
                if (c == '[') inClass = true;
                else if (c == ']') inClass = false;
                if (c == '$' && !inClass) { sb.Append(@"(?![\s\S])"); continue; }
                sb.Append(c);
            }
            return sb.ToString();
        }
    }

    /// <summary>One validation failure: JSON pointer of the instance location and the failing keyword.</summary>
    public sealed class SchemaError
    {
        public SchemaError(string path, string keyword, string message)
        {
            Path = path.Length == 0 ? "/" : path;
            Keyword = keyword;
            Message = message;
        }

        public string Path { get; }
        public string Keyword { get; }
        public string Message { get; }

        public override string ToString() => Path + " [" + Keyword + "] " + Message;
    }
}

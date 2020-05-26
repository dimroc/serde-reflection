initSidebarItems({"enum":[["ContainerFormat","Serde-based serialization format for named \"container\" types. In Rust, those are enums and structs."],["Error","Error type used in this crate."],["Format","Serde-based serialization format for anonymous \"value\" types."],["Value","A structured Serde value. Meant to be easily recorded while tracing serialization and easily used while tracing deserialization."],["VariantFormat","Description of a variant in an enum."]],"struct":[["Named","A named value. Used for named parameters or variants."],["Samples","User inputs, aka \"samples\", recorded during serialization. This will help passing user-defined checks during deserialization."],["Tracer","Structure to drive the tracing of Serde serialization and deserialization. This typically aims at computing a `Registry`."],["TracerConfig","Configuration object to create a tracer."],["Variable","A mutable holder for an initially unknown value."]],"trait":[["FormatHolder","Common methods for nodes in the AST of formats."]],"type":[["Registry","A map of container formats."],["Result","Result type used in this crate."]]});
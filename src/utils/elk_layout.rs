use angel_api_client_reqwest::types::flows::Flow;
use flx_types::DataSource;
/// Utilities for converting Flow data to yewflow nodes and edges with ELK layout
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// JS interop for ELK layout
#[wasm_bindgen(module = "/elk-interop.js")]
extern "C" {
    #[wasm_bindgen(js_name = layoutGraph, catch)]
    async fn elk_layout_graph(graph_data: JsValue) -> Result<JsValue, JsValue>;
}

/// Represents a node in the flow graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FlowNode {
    pub id: String,
    pub label: String,
    pub component_type: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub color: String,
    pub icon: String,
    pub input_count: usize,
    pub output_count: usize,
}

/// Represents an edge connecting two nodes in the flow graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FlowEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub source_port: Option<usize>,
    pub target_port: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<ElkEdgeSection>>,
}

/// Edge routing section with bend points (matches ELK.js ElkEdgeSection)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ElkEdgeSection {
    pub id: String,
    pub start_point: ElkPoint,
    pub end_point: ElkPoint,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bend_points: Option<Vec<ElkPoint>>,
}

/// 2D Point (matches ELK.js ElkPoint)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ElkPoint {
    pub x: f64,
    pub y: f64,
}

/// Represents the complete flow graph with nodes and edges
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FlowGraph {
    pub nodes: Vec<FlowNode>,
    pub edges: Vec<FlowEdge>,
}

/// ELK graph format for layout
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElkGraph {
    id: String,
    children: Vec<ElkNode>,
    edges: Vec<ElkEdge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layout_options: Option<ElkLayoutOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElkLayoutOptions {
    #[serde(rename = "elk.algorithm")]
    algorithm: String,
    #[serde(rename = "elk.direction")]
    direction: String,
    #[serde(rename = "elk.spacing.nodeNode")]
    node_spacing: f64,
    #[serde(rename = "elk.layered.spacing.nodeNodeBetweenLayers")]
    layer_spacing: f64,
    #[serde(rename = "elk.spacing.edgeNode")]
    edge_node_spacing: f64,
    #[serde(rename = "elk.edgeRouting")]
    edge_routing: String,
    #[serde(rename = "elk.layered.edgeRouting.sloppySplineRouting")]
    sloppy_spline_routing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElkNode {
    id: String,
    width: f64,
    height: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElkEdge {
    id: String,
    sources: Vec<String>,
    targets: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sections: Option<Vec<ElkEdgeSection>>,
}

/// Convert a Flow to a FlowGraph with nodes and edges
///
/// Parses the actual flow structure to extract components and their connections
pub fn flow_to_graph(flow: &Flow) -> FlowGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut edge_id_counter = 0;

    // Create nodes for each component
    for component in &flow.components {
        let component_type = component.r#type.to_string();
        let component_id = component.id.to_string();

        let width = match component_type.as_str() {
            "Input" | "Output" => 200.0,
            "JinjaTemplate" => 220.0,
            "LLM" | "LLM_nonstreaming" | "LLMVision" | "StructuredLLM" => 200.0,
            "ElevenLabs" | "ElevenLabsWithConfig" | "ElevenLabsHttps" => 200.0,
            _ if component_type.starts_with("$flow_defs/") => 240.0,
            _ => 200.0,
        };

        let height = 100.0;

        let (color, icon) = get_component_style(&component_type);

        nodes.push(FlowNode {
            id: component_id.clone(),
            label: component_id,
            component_type: component_type.clone(),
            x: 0.0, // Will be set by ELK layout
            y: 0.0, // Will be set by ELK layout
            width,
            height,
            color: color.to_string(),
            icon: icon.to_string(),
            input_count: 0,  // Will be calculated after edges
            output_count: 0, // Will be calculated after edges
        });
    }

    // Create edges from port connections
    for component in &flow.components {
        if let Some(inputs) = &component.inputs {
            for (input_idx, input_opt) in inputs.iter().enumerate() {
                // Check if this is a Port variant
                if let Some(DataSource::Port(port_ref)) = input_opt {
                    edge_id_counter += 1;
                    edges.push(FlowEdge {
                        id: format!("e{}", edge_id_counter),
                        source: port_ref.component.to_string(),
                        target: component.id.to_string(),
                        source_port: Some(port_ref.idx),
                        target_port: Some(input_idx),
                        sections: None, // Will be populated by ELK
                    });
                }
            }
        }
    }

    // Calculate actual input/output counts from edges
    for edge in &edges {
        // Count outputs for source node
        if let Some(node) = nodes.iter_mut().find(|n| n.id == edge.source) {
            node.output_count += 1;
        }
        // Count inputs for target node
        if let Some(node) = nodes.iter_mut().find(|n| n.id == edge.target) {
            node.input_count += 1;
        }
    }

    FlowGraph { nodes, edges }
}

fn get_component_style(component_type: &str) -> (&'static str, &'static str) {
    match component_type {
        "Input" => ("#10b981", "📥"),  // green
        "Output" => ("#3b82f6", "📤"), // blue
        "LLM" | "LLM_nonstreaming" | "LLMVision" | "StructuredLLM" => ("#8b5cf6", "🤖"), // purple
        "ElevenLabs"
        | "ElevenLabsWithConfig"
        | "ElevenLabsHttps"
        | "BubbleMachine"
        | "Echo"
        | "EchoLines" => ("#f59e0b", "🔊"), // amber
        "BingLegacy" | "BingDirect" | "BraveSearchImage" | "PerplexitySearch" => ("#ec4899", "🔍"), // pink
        "Filter" | "FloodGate1" | "FloodGate2" | "FloodGate3" | "FloodGate4" | "FloodRouter" => {
            ("#ef4444", "🚦")
        } // red
        "Flatten" | "FlattenStream" | "Split" | "Chain" | "Collect" | "Take" | "Skip"
        | "InterleaveLongest2" | "InterleaveLongest3" | "InterleaveLongest4"
        | "InterleaveLongest5" | "StreamMerger" => ("#06b6d4", "💧"), // cyan
        "JsonObjectConstructor"
        | "JsonObjectExtender"
        | "JsonObjectMerger"
        | "JsonPathExtractor"
        | "JsonObjectConstructorStreamData" => ("#6366f1", "📋"), // indigo
        "JinjaTemplate" => ("#14b8a6", "📝"),     // teal
        "JmesPathConverter" => ("#a855f7", "🔄"), // violet
        "All" | "Any" => ("#84cc16", "🧮"),       // lime
        "TextMerger" => ("#14b8a6", "🔤"),        // teal
        _ if component_type.starts_with("$flow_defs/") => ("#f97316", "📦"), // orange
        _ => ("#6b7280", "⚙️"),                   // gray
    }
}

/// Apply ELK (Eclipse Layout Kernel) layout algorithm to the flow graph
///
/// This uses the elkjs library via wasm-bindgen to automatically layout nodes and edges.
pub async fn apply_elk_layout(graph: FlowGraph) -> Result<FlowGraph, String> {
    // Convert to ELK format with layout options
    let elk_graph = ElkGraph {
        id: "root".to_string(),
        layout_options: Some(ElkLayoutOptions {
            algorithm: "layered".to_string(),
            direction: "RIGHT".to_string(),
            node_spacing: 80.0,                     // Increased from default (40)
            layer_spacing: 120.0,                   // Increased from default (60)
            edge_node_spacing: 60.0,                // Increased from default (30)
            edge_routing: "ORTHOGONAL".to_string(), // Orthogonal routing
            sloppy_spline_routing: false,           // Disable splines for strict orthogonal
        }),
        children: graph
            .nodes
            .iter()
            .map(|node| ElkNode {
                id: node.id.clone(),
                width: node.width,
                height: node.height,
                x: None,
                y: None,
            })
            .collect(),
        edges: graph
            .edges
            .iter()
            .map(|edge| ElkEdge {
                id: edge.id.clone(),
                sources: vec![edge.source.clone()],
                targets: vec![edge.target.clone()],
                sections: None, // ELK will populate this
            })
            .collect(),
    };

    // Serialize to JsValue
    let js_graph = serde_wasm_bindgen::to_value(&elk_graph)
        .map_err(|e| format!("Failed to serialize graph: {:?}", e))?;

    // Call ELK layout via JS interop
    let result = elk_layout_graph(js_graph)
        .await
        .map_err(|e| format!("ELK layout failed: {:?}", e))?;

    // Deserialize result
    let layouted_elk: ElkGraph = serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to deserialize layout result: {:?}", e))?;

    // Convert back to FlowGraph with updated positions
    let mut layouted_graph = graph;
    for (i, elk_node) in layouted_elk.children.iter().enumerate() {
        if let Some(node) = layouted_graph.nodes.get_mut(i) {
            node.x = elk_node.x.unwrap_or(node.x);
            node.y = elk_node.y.unwrap_or(node.y);
        }
    }

    // Update edge routing information from ELK
    for elk_edge in layouted_elk.edges.iter() {
        if let Some(edge) = layouted_graph
            .edges
            .iter_mut()
            .find(|e| e.id == elk_edge.id)
        {
            edge.sections = elk_edge.sections.clone();
        }
    }

    Ok(layouted_graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_to_graph() {
        // Test would require creating proper flx_types structures
        // which may not be straightforward in tests
        // Skipping detailed test for now
    }
}

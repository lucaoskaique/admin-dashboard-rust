// ELK (Eclipse Layout Kernel) JavaScript interop
// Load ELK from CDN (loaded in index.html)

// Layout a graph using ELK algorithm
export async function layoutGraph(graphData) {
  try {
    // Wait for ELK to be loaded
    if (typeof window.ELK === 'undefined') {
      throw new Error('ELK library not loaded. Make sure elk.bundled.js is included in index.html');
    }
    
    // Initialize ELK instance
    const elk = new window.ELK();
    const layoutedGraph = await elk.layout(graphData);
    // Return the object directly, not as JSON string
    return layoutedGraph;
  } catch (error) {
    console.error('ELK layout error:', error);
    throw error;
  }
}

// Export for wasm-bindgen to call
window.elkLayout = layoutGraph;

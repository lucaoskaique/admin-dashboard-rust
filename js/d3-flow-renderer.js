// D3.js Flow Renderer - Edges Only
// This module provides functions to render flow graph edges using D3.js
// Nodes are now rendered by Yew components

// Import D3 from CDN (loaded in index.html)
const d3 = window.d3;

export function renderEdges(containerId, nodesJson, edgesJson) {
    try {
        const nodes = JSON.parse(nodesJson);
        const edges = JSON.parse(edgesJson);
        
        console.log('Rendering edges with D3:', edges.length, 'edges');
        
        const container = document.getElementById(containerId);
        if (!container) {
            console.error('Container not found:', containerId);
            return;
        }
        
        // Calculate the actual content bounds from nodes
        let maxX = 0, maxY = 0;
        nodes.forEach(node => {
            maxX = Math.max(maxX, node.x + node.width);
            maxY = Math.max(maxY, node.y + node.height);
        });
        
        // Add padding for edge curves and arrowheads
        const padding = 100;
        const contentWidth = maxX + padding;
        const contentHeight = maxY + padding;
        
        console.log('Content bounds:', contentWidth, 'x', contentHeight);
        
        // Clear previous content
        container.innerHTML = '';
        
        // Create SVG with dimensions matching actual content (1:1 coordinate mapping)
        const svg = d3.select(container)
            .append('svg')
            .attr('width', contentWidth)
            .attr('height', contentHeight)
            .attr('viewBox', `0 0 ${contentWidth} ${contentHeight}`)
            .style('position', 'absolute')
            .style('top', '0')
            .style('left', '0')
            .style('pointer-events', 'none');
        
        // Define arrowhead marker
        svg.append('defs').append('marker')
            .attr('id', 'arrowhead')
            .attr('markerWidth', 10)
            .attr('markerHeight', 10)
            .attr('refX', 9)
            .attr('refY', 3)
            .attr('orient', 'auto')
            .attr('markerUnits', 'strokeWidth')
            .append('path')
            .attr('d', 'M0,0 L0,6 L9,3 z')
            .attr('fill', '#6b7280');
        

        // Render edges
        const edgeGroup = svg.append('g').attr('class', 'edges');
        renderEdgesOnly(edgeGroup, edges, nodes);
        
    } catch (error) {
        console.error('Error rendering edges:', error);
    }
}

export function clearFlow(containerId) {
    const container = document.getElementById(containerId);
    if (container) {
        container.innerHTML = '';
    }
}

function renderEdgesOnly(group, edges, nodes) {
    console.log('=== renderEdgesOnly called ===');
    console.log('Number of edges to render:', edges.length);
    console.log('Number of nodes available:', nodes.length);
    
    // Create a lookup map for nodes
    const nodeMap = new Map(nodes.map(n => [n.id, n]));
    console.log('Node IDs in map:', Array.from(nodeMap.keys()));
    
    const edgeGroups = group.selectAll('g.edge')
        .data(edges)
        .enter()
        .append('g')
        .attr('class', 'edge');
    
    console.log('Created edge groups:', edgeGroups.size());
    
    edgeGroups.each(function(edge, i) {
        const sourceNode = nodeMap.get(edge.source);
        const targetNode = nodeMap.get(edge.target);
        
        if (!sourceNode || !targetNode) {
            console.warn('Missing node for edge:', edge);
            return;
        }
        
        console.log(`\n=== Rendering edge ${edge.id} (${edge.source} -> ${edge.target}) ===`);
        console.log('Source:', sourceNode.id, 'pos:', sourceNode.x, sourceNode.y, 'size:', sourceNode.width, 'x', sourceNode.height);
        console.log('Target:', targetNode.id, 'pos:', targetNode.x, targetNode.y, 'size:', targetNode.width, 'x', targetNode.height);
        
        const edgeGroup = d3.select(this);
        let pathData;
        
        if (edge.sections && edge.sections.length > 0) {
            const section = edge.sections[0];
            console.log('ELK start:', section.startPoint.x, section.startPoint.y);
            console.log('ELK end:', section.endPoint.x, section.endPoint.y);
            
            // Use ELK's coordinates directly
            const startX = section.startPoint.x;
            const startY = section.startPoint.y;
            const endX = section.endPoint.x;
            const endY = section.endPoint.y;
            
            console.log('Using ELK points directly:', startX, startY, '->', endX, endY);
            
            pathData = `M ${startX} ${startY}`;
            
            if (section.bendPoints && section.bendPoints.length > 0) {
                console.log('Using', section.bendPoints.length, 'bend points');
                // Use orthogonal routing with bend points
                section.bendPoints.forEach(point => {
                    pathData += ` L ${point.x} ${point.y}`;
                });
            }
            
            pathData += ` L ${endX} ${endY}`;
        } else {
            console.log('No sections - using straight line');
            const startX = sourceNode.x + sourceNode.width;
            const startY = sourceNode.y + (sourceNode.height / 2);
            const endX = targetNode.x;
            const endY = targetNode.y + (targetNode.height / 2);
            
            pathData = `M ${startX} ${startY} L ${endX} ${endY}`;
        }
        
        // Edge path
        const pathElement = edgeGroup.append('path')
            .attr('d', pathData)
            .attr('fill', 'none')
            .attr('stroke', '#6b7280')
            .attr('stroke-width', 2)
            .attr('marker-end', 'url(#arrowhead)')
            .style('pointer-events', 'auto')
            .style('cursor', 'pointer')
            .on('mouseover', function() {
                d3.select(this).attr('stroke', '#3b82f6').attr('stroke-width', 3);
            })
            .on('mouseout', function() {
                d3.select(this).attr('stroke', '#6b7280').attr('stroke-width', 2);
            });
        
        // Optional: render port indices 
        const startX = sourceNode.x + sourceNode.width;
        const startY = sourceNode.y + (sourceNode.height / 2);
        const endX = targetNode.x;
        const endY = targetNode.y + (targetNode.height / 2);
        
        if (edge.source_port !== undefined && edge.source_port !== null) {
            edgeGroup.append('text')
                .attr('x', startX + 5)
                .attr('y', startY - 5)
                .attr('font-size', 10)
                .attr('fill', '#6b7280')
                .style('user-select', 'none')
                .text(`[${edge.source_port}]`);
        }
        
        if (edge.target_port !== undefined && edge.target_port !== null) {
            edgeGroup.append('text')
                .attr('x', endX - 15)
                .attr('y', endY - 5)
                .attr('font-size', 10)
                .attr('fill', '#6b7280')
                .style('user-select', 'none')
                .text(`[${edge.target_port}]`);
        }
    });
}

// Keep the old function for backward compatibility during transition
export function renderFlow(containerId, nodesJson, edgesJson) {
    renderEdges(containerId, edgesJson);
}

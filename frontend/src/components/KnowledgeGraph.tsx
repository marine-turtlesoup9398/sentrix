import React, { useEffect, useRef, useState } from 'react';

interface Node {
  id: string;
  name: string;
  type: string;
  path?: string;
  x?: number;
  y?: number;
  vx?: number;
  vy?: number;
}

interface Edge {
  source: string;
  target: string;
  type: string;
}

interface GraphData {
  nodes: Node[];
  edges: Edge[];
}

export const KnowledgeGraph: React.FC<{ data: GraphData | null; onSelectNode: (node: Node) => void }> = ({ data, onSelectNode }) => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [zoom, setZoom] = useState(1);
  const [pan, setPan] = useState({ x: 0, y: 0 });
  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });

  useEffect(() => {
    if (!data || !canvasRef.current) return;
    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    let width = (canvas.width = canvas.parentElement?.clientWidth || 800);
    let height = (canvas.height = canvas.parentElement?.clientHeight || 600);

    // Initialize node positions in a circle layout
    const radius = Math.min(width, height) / 3;
    const nodes = data.nodes.map((n, i) => {
      const angle = (i / data.nodes.length) * 2 * Math.PI;
      return {
        ...n,
        x: width / 2 + radius * Math.cos(angle) + (Math.random() - 0.5) * 50,
        y: height / 2 + radius * Math.sin(angle) + (Math.random() - 0.5) * 50,
      };
    });

    const nodeMap = new Map<string, Node>();
    nodes.forEach((n) => nodeMap.set(n.id, n));

    let animationFrameId: number = 0;

    const render = () => {
      ctx.clearRect(0, 0, width, height);
      ctx.save();
      ctx.translate(pan.x, pan.y);
      ctx.scale(zoom, zoom);

      // Draw Edges
      ctx.lineWidth = 1;
      data.edges.forEach((edge) => {
        const src = nodeMap.get(edge.source);
        const tgt = nodeMap.get(edge.target);
        if (src && tgt && src.x && src.y && tgt.x && tgt.y) {
          ctx.beginPath();
          ctx.moveTo(src.x, src.y);
          ctx.lineTo(tgt.x, tgt.y);
          ctx.strokeStyle = edge.type === 'Affects' ? 'rgba(244, 63, 94, 0.6)' : 'rgba(100, 116, 139, 0.3)';
          ctx.stroke();
        }
      });

      // Draw Nodes
      nodes.forEach((node) => {
        if (!node.x || !node.y) return;
        ctx.beginPath();
        ctx.arc(node.x, node.y, node.type === 'File' ? 8 : 5, 0, 2 * Math.PI);
        
        if (node.type === 'File') ctx.fillStyle = '#3b82f6';
        else if (node.type === 'Function') ctx.fillStyle = '#10b981';
        else if (node.type === 'ApiEndpoint') ctx.fillStyle = '#8b5cf6';
        else if (node.type === 'Finding') ctx.fillStyle = '#f43f5e';
        else ctx.fillStyle = '#06b6d4';

        ctx.fill();
        ctx.strokeStyle = '#1e293b';
        ctx.lineWidth = 2;
        ctx.stroke();

        // Node Label
        ctx.fillStyle = '#94a3b8';
        ctx.font = '10px Inter, sans-serif';
        ctx.fillText(node.name.length > 15 ? node.name.slice(0, 12) + '...' : node.name, node.x + 10, node.y + 3);
      });

      ctx.restore();
    };

    render();

    const handleMouseDown = (e: MouseEvent) => {
      setIsDragging(true);
      setDragStart({ x: e.clientX - pan.x, y: e.clientY - pan.y });
    };

    const handleMouseMove = (e: MouseEvent) => {
      if (isDragging) {
        setPan({ x: e.clientX - dragStart.x, y: e.clientY - dragStart.y });
      }
    };

    const handleMouseUp = () => setIsDragging(false);

    canvas.addEventListener('mousedown', handleMouseDown);
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);

    return () => {
      canvas.removeEventListener('mousedown', handleMouseDown);
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
      cancelAnimationFrame(animationFrameId);
    };
  }, [data, zoom, pan, isDragging, dragStart]);

  return (
    <div className="graph-container">
      <div className="graph-controls">
        <button className="btn-icon" onClick={() => setZoom((z) => Math.min(z + 0.2, 3))}>+</button>
        <button className="btn-icon" onClick={() => setZoom((z) => Math.max(z - 0.2, 0.4))}>-</button>
        <button className="btn-icon" onClick={() => { setZoom(1); setPan({ x: 0, y: 0 }); }}>Reset</button>
      </div>
      <canvas ref={canvasRef} />
    </div>
  );
};

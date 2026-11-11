// jsdom has no layout: every element is a 0×0 box at (0,0), so react-resizable-panels
// treats every pointer-down as a separator grab and cancels it (no clicks reach tabs or
// inputs). Tests use these plain containers instead; the real panels are checked by hand.
import { useImperativeHandle, useState, type Ref } from "react";
import type { PanelImperativeHandle } from "react-resizable-panels";

export function ResizablePanelGroup({ children, className }: { children?: React.ReactNode; className?: string }) {
  return <div className={className}>{children}</div>;
}

export function ResizablePanel({
  children,
  panelRef,
  id,
}: {
  children?: React.ReactNode;
  panelRef?: Ref<PanelImperativeHandle | null>;
  id?: string;
}) {
  const [collapsed, setCollapsed] = useState(false);
  useImperativeHandle(panelRef, () => ({
    collapse: () => setCollapsed(true),
    expand: () => setCollapsed(false),
    isCollapsed: () => collapsed,
    getSize: () => ({ asPercentage: collapsed ? 0 : 50, inPixels: collapsed ? 0 : 400 }),
    resize: () => {},
  }) as unknown as PanelImperativeHandle);
  return (
    <div data-panel-id={id} data-collapsed={collapsed || undefined} hidden={collapsed}>
      {children}
    </div>
  );
}

export function ResizableHandle() {
  return <div role="separator" />;
}

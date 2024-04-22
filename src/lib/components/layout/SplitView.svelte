<script lang="ts">
  import type { ComponentAndProps as Component } from "../../utils";

  export let components: [Component, Component, ...Component[]];
  export let size: [number, ...number[]] = [50];
  export let direction: "t" | "b" | "l" | "r" = "l";

  let gridStyle = "";
  let isDragging: number = 0;
  let splitContainer: HTMLDivElement;

  $: {
    if (components.length - 1 !== size.length)
      throw new Error(
        "Size array must be one element shorter than components array",
      );

    let valueString = "";
    for (let i = 0; i < size.length; i++) {
      valueString += `minmax(50px,${size[i]}px) 4px `;
    }
    valueString += "minmax(50px,1fr)";

    if (direction === "r" || direction === "b") {
      valueString = valueString.split(" ").reverse().join(" ");
    }

    gridStyle = `grid-template-${direction === "t" || direction === "b" ? "rows" : "columns"}: ${valueString};`;
  }

  const dragStarted = (i: number) => {
    isDragging = i;
  };

  const dragStopped = () => {
    isDragging = 0;
  };

  const drag = (event: MouseEvent) => {
    if (isDragging !== 0) {
      let prev =
        size.slice(0, isDragging - 1).reduce((a, b) => a + b, 0) +
        4 * (isDragging - 1);
      let newValue = 0;
      switch (direction) {
        case "b":
          newValue =
            splitContainer.clientHeight -
            (event.clientY - splitContainer.offsetTop - prev);
          break;
        case "t":
          newValue = event.clientY - splitContainer.offsetTop - prev;
          break;
        case "r":
          newValue =
            splitContainer.clientWidth -
            (event.clientX - splitContainer.offsetLeft - prev);
          break;
        case "l":
          newValue = event.clientX - splitContainer.offsetLeft - prev;
          break;
      }

      newValue = Math.max(50, newValue);
      let next = Math.max(
        0,
        Math.max(50, size[isDragging - 1]) -
          Math.max(50, newValue) +
          size[isDragging],
      );

      if (next < 50) {
        newValue = size[isDragging - 1] - 50 + size[isDragging];
        next = 50;
      }
      if (isDragging === size.length) {
        let newTotal =
          size.reduce((a, b) => a + b, 0) -
          size[isDragging - 1] +
          4 * size.length;
        let containerSize =
          direction === "t" || direction === "b"
            ? splitContainer.clientHeight
            : splitContainer.clientWidth;

        if (containerSize - newTotal - newValue < 50) {
          newValue = containerSize - newTotal - 50;
        }
      }

      size = size.map((s, i) =>
        i === isDragging - 1 ? newValue : i === isDragging ? next : s,
      ) as [number, ...number[]];
    }
  };
</script>

<svelte:window on:mousemove={drag} on:mouseup={dragStopped} />

<div
  class="splitview"
  bind:this={splitContainer}
  style={(isDragging !== 0
    ? `cursor: ${direction === "l" || direction === "r" ? "ew" : "ns"}-resize;`
    : "") + gridStyle}
>
  <slot name="0" />
  {#each components as component, i}
    {#if i > 0}
      <button
        class="slider"
        class:dragging={isDragging === i}
        on:mousedown={() => dragStarted(i)}
        style={`cursor: ${
          direction === "l" || direction === "r" ? "ew" : "ns"
        }-resize;`}
      ></button>
    {/if}
    <div class="component">
      <svelte:component this={component.component} {...component.props} />
    </div>
  {/each}
</div>

<style>
  .splitview {
    display: grid;
    height: 100%;
  }

  .slider {
    background-color: transparent;
    padding: 0;
    border: none;
  }

  .slider:hover,
  .dragging {
    background-color: var(--color-accent);
  }
</style>

<script lang="ts">
  type sizeType = `${"t" | "b" | "l" | "r"}${number}`;
  export let size: sizeType = "t10";

  let gridStyle = "";
  let isDragging = false;
  let splitDirection = "";
  let splitContainer: HTMLDivElement;

  $: {
    const { direction, value } = parseSize(size);
    splitDirection = direction;
    let valueString = `minmax(50px,${value}px) 4px minmax(50px,1fr)`;
    if (direction === "r" || direction === "b") {
      valueString = valueString.split(" ").reverse().join(" ");
    }
    gridStyle = `grid-template-${direction === "t" || direction === "b" ? "rows" : "columns"}: ${valueString};`;
  }

  const parseSize = (size: sizeType) => {
    const [complete, direction, value] = size.match(/([tb]|[lr])(\d+)/) as [
      string,
      string,
      number,
    ];
    return { complete, direction, value };
  };

  const dragStarted = () => {
    isDragging = true;
  };

  const dragStopped = () => {
    isDragging = false;
  };

  const drag = (event: MouseEvent) => {
    if (isDragging) {
      let newValue = 0;
      switch (splitDirection) {
        case "b":
          newValue =
            splitContainer.clientHeight -
            (event.clientY - splitContainer.offsetTop);
          break;
        case "t":
          newValue = event.clientY - splitContainer.offsetTop;
          break;
        case "r":
          newValue =
            splitContainer.clientWidth -
            (event.clientX - splitContainer.offsetLeft);
          break;
        case "l":
          newValue = event.clientX - splitContainer.offsetLeft;
          break;
      }
      size = `${splitDirection}${Math.max(0, newValue)}px` as sizeType;
    }
  };
</script>

<svelte:window on:mousemove={drag} on:mouseup={dragStopped} />

<div class="splitview" style={gridStyle} bind:this={splitContainer}>
  <slot name="first" />
  <button
    class="slider"
    class:dragging={isDragging}
    on:mousedown={dragStarted}
    style="cursor: {splitDirection === 'l' || splitDirection === 'r'
      ? 'ew-resize'
      : 'ns-resize'};"
  ></button>
  <slot name="second" />
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

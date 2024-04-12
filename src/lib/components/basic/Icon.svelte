<script lang="ts">
  export let src = "";
  export let color = "";

  let html = "";

  $: html, load();

  const load = () => {
    fetch(src)
      .then((res) => res.text())
      .then((data) => {
        html = data.replace(/\$COLOR/g, color);
      });
  };
</script>

<div class="icon">
  {@html html}
</div>

<style>
  .icon {
    width: 50%;
    height: 50%;
  }
</style>

<script lang="ts">
  let count: number = $state(0)
</script>

<button type="button" class="counter" onclick={() => {
    count += 1
  }}>
  Count is {count}
</button>
